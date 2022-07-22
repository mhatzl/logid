use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use lazy_static::lazy_static;

use crate::{
    id_entry::{LogIdEntry, Origin},
    log_id::{EventKind, LogId, INVALID_LOG_ID, NR_LOG_ID_FIELDS},
};

pub struct LogIdLayer {
    map: Arc<RwLock<HashMap<LogId, Vec<LogIdEntry>>>>,
    last_log_id: RwLock<LogId>,
}

lazy_static! {
    pub static ref LOG_ID_LAYER: LogIdLayer = LogIdLayer::new();
}

impl LogIdLayer {
    pub fn new() -> Self {
        LogIdLayer {
            map: Arc::new(RwLock::new(HashMap::new())),
            last_log_id: RwLock::new(INVALID_LOG_ID),
        }
    }

    pub fn get_last_log_id(&self) -> LogId {
        *self.last_log_id.read().unwrap()
    }

    pub fn drain_layer(&mut self) -> HashMap<LogId, Vec<LogIdEntry>> {
        let map = &mut *self.map.write().unwrap();
        map.drain().collect()
    }
}

impl<S: tracing::Subscriber> tracing_subscriber::layer::Layer<S> for LogIdLayer {
    fn on_event(&self, event: &tracing::Event<'_>, ctx: tracing_subscriber::layer::Context<'_, S>) {
        if event.fields().count() > NR_LOG_ID_FIELDS {
            return;
        }

        let mut id_entry = LogIdEntry::default();
        let mut event_kind = EventKind::Invalid;
        let mut msg = String::default();
        for field in event.fields() {
            match field.name() {
                // Ignore events without valid log-ids or EventKinds
                "id" => match field.to_string().parse::<LogId>() {
                    Ok(id) => id_entry.id = id,
                    Err(_) => return,
                },
                "kind" => match field.to_string().parse::<isize>() {
                    Ok(kind) => event_kind = kind.into(),
                    Err(_) => return,
                },
                _ => msg = field.to_string(),
            }
        }
        match event_kind {
            EventKind::Base => {
                id_entry.span = if let Some(span_data) = ctx.current_span().metadata() {
                    span_data.name()
                } else {
                    "event not in span"
                };
                id_entry.level = event.metadata().level().into();
                id_entry.msg = msg;
                let update_map = LOG_ID_LAYER.map.write();
                if let Ok(mut map) = update_map {
                    match map.get_mut(&id_entry.id) {
                        Some(entries) => entries.push(id_entry),
                        None => {
                            map.insert(id_entry.id, [id_entry].into());
                        }
                    };
                }
            }
            EventKind::Location => {
                let origin: Origin = msg.into();
                let update_map = LOG_ID_LAYER.map.write();
                if let Ok(mut map) = update_map {
                    match map.get_mut(&id_entry.id) {
                        Some(entries) => {
                            if let Some(last) = entries.last_mut() {
                                last.origin = origin;
                            }
                        }
                        None => {
                            tracing::warn!("Got location=\"{}\" for log-id={}, but no base for log-id was set!", origin, id_entry.id)
                        }
                    };
                }
            }
            EventKind::Cause => {
                let update_map = LOG_ID_LAYER.map.write();
                if let Ok(mut map) = update_map {
                    match map.get_mut(&id_entry.id) {
                        Some(entries) => {
                            if let Some(last) = entries.last_mut() {
                                last.add_cause(msg);
                            };
                        }
                        None => {
                            tracing::warn!(
                                "Got cause=\"{}\" for log-id={}, but no base for log-id was set!",
                                msg,
                                id_entry.id
                            )
                        }
                    };
                }
            }
            EventKind::Addon => {
                let update_map = LOG_ID_LAYER.map.write();
                if let Ok(mut map) = update_map {
                    match map.get_mut(&id_entry.id) {
                        Some(entries) => {
                            if let Some(last) = entries.last_mut() {
                                last.add_addon(event.metadata().level(), msg);
                            };
                        }
                        None => {
                            tracing::warn!(
                                "Got addon=\"{}\" for log-id={}, but no base for log-id was set!",
                                msg,
                                id_entry.id
                            )
                        }
                    };
                }
            }
            EventKind::Invalid => {
                tracing::warn!("Got invalid event kind for log-id={}!", id_entry.id)
            }
        };
    }
}
