use std::{
    collections::HashMap,
    sync::{
        mpsc::{self, Receiver, SyncSender},
        Arc, RwLock,
    },
    thread,
};

use once_cell::sync::Lazy;

use crate::{event::Event, log_id::LogId};

pub(crate) static PUBLISHER: Lazy<LogIdEventPublisher> = Lazy::new(LogIdEventPublisher::new);

pub(crate) struct LogIdEventPublisher {
    pub(crate) subscriptions: Arc<RwLock<HashMap<SubscriptionKey, Vec<SyncSender<Event>>>>>,
    pub(crate) any_event: Arc<RwLock<Vec<SyncSender<Event>>>>,
    pub(crate) capturer: SyncSender<Event>,
}

/// Buffersize of the broadcast channel for capturing log events.
/// One finalized event is passed over one message in the channel.
const CHANNEL_BOUND: usize = 1000;
// TODO: Use when `unwrap_or` becomes available for const fn.
// match option_env!("LOGID_EVENT_BUFFER") {
//     Some(v) => usize::from_str_radix(v, 10).unwrap_or(1000),
//     None => 1000,
// };

impl LogIdEventPublisher {
    fn new() -> Self {
        let (send, recv) = mpsc::sync_channel(CHANNEL_BOUND);

        thread::spawn(move || loop {
            match recv.recv() {
                Ok(event_msg) => {
                    on_event(event_msg);
                }
                Err(_) => {
                    // Sender got dropped => Publisher got dropped
                    return;
                }
            }
        });

        LogIdEventPublisher {
            subscriptions: Arc::new(RwLock::new(HashMap::new())),
            any_event: Arc::new(RwLock::new(Vec::new())),
            capturer: send,
        }
    }
}

#[derive(Default, Hash, Debug, PartialEq, Eq)]
pub(crate) struct SubscriptionKey {
    crate_name: &'static str,
    log_id: LogId,
}

impl SubscriptionKey {
    pub fn new(crate_name: &'static str, log_id: LogId) -> Self {
        SubscriptionKey { crate_name, log_id }
    }
}

pub fn subscribe(log_id: LogId, crate_name: &'static str) -> Option<Receiver<Event>> {
    let crate_logs = vec![log_id];
    let logs = vec![(crate_name, crate_logs)];
    subscribe_to_crates(&logs)
}

#[macro_export]
macro_rules! subscribe {
    ($logid:ident) => {
        $crate::publisher::subscribe($crate::logid!($logid), env!("CARGO_PKG_NAME"))
    };
    ($logid:expr) => {
        $crate::publisher::subscribe($crate::logid!($logid), env!("CARGO_PKG_NAME"))
    };
}

pub fn subscribe_to_logs<T>(log_ids: T, crate_name: &'static str) -> Option<Receiver<Event>>
where
    T: Iterator<Item = LogId>,
{
    let crate_logs = vec![(crate_name, log_ids.collect())];
    subscribe_to_crates(&crate_logs)
}

#[macro_export]
macro_rules! subscribe_to_logs {
    ($logids:ident) => {
        $crate::publisher::subscribe_to_logs($crate::logids!($logids), env!("CARGO_PKG_NAME"))
    };
    ($logids:expr) => {
        $crate::publisher::subscribe_to_logs($crate::logids!($logids), env!("CARGO_PKG_NAME"))
    };
}

pub fn subscribe_to_crates(
    crate_logs: &Vec<(&'static str, Vec<LogId>)>,
) -> Option<Receiver<Event>> {
    let (send, recv) = mpsc::sync_channel(CHANNEL_BOUND);

    match PUBLISHER.subscriptions.write().ok() {
        Some(mut locked_subs) => {
            for (crate_name, log_ids) in crate_logs {
                for log_id in log_ids {
                    let key = SubscriptionKey::new(crate_name, *log_id);
                    let entry = locked_subs.entry(key);
                    entry
                        .and_modify(|v| v.push(send.clone()))
                        .or_insert(vec![send.clone()]);
                }
            }
        }
        None => {
            return None;
        }
    }

    Some(recv)
}

pub fn subscribe_to_all_events() -> Option<Receiver<Event>> {
    let (send, recv) = mpsc::sync_channel(CHANNEL_BOUND);

    match PUBLISHER.any_event.write().ok() {
        Some(mut locked_vec) => {
            locked_vec.push(send);
        }
        None => {
            return None;
        }
    }

    Some(recv)
}

pub(crate) fn on_event(event_msg: Event) {
    let key = SubscriptionKey::new(event_msg.crate_name, event_msg.entry.id);

    let mut bad_subs = Vec::new();
    let mut bad_any_event = Vec::new();

    if let Ok(locked_subscriptions) = PUBLISHER.subscriptions.read() {
        if let Some(sub_sender) = locked_subscriptions.get(&key) {
            for (i, sender) in sub_sender.iter().enumerate() {
                if sender.send(event_msg.clone()).is_err() {
                    bad_subs.push(i);
                }
            }
        }
    }

    if let Ok(locked_vec) = PUBLISHER.any_event.read() {
        for (i, sender) in locked_vec.iter().enumerate() {
            if sender.send(event_msg.clone()).is_err() {
                bad_any_event.push(i);
            }
        }
    }

    // Remove dead channels
    if !bad_subs.is_empty() {
        if let Ok(mut locked_subscriptions) = PUBLISHER.subscriptions.write() {
            let mut entry = locked_subscriptions.entry(key);
            for i in bad_subs {
                entry = entry.and_modify(|v| {
                    v.remove(i);
                });
            }
        }
    }

    if !bad_any_event.is_empty() {
        if let Ok(mut locked_vec) = PUBLISHER.any_event.write() {
            for i in bad_any_event {
                locked_vec.remove(i);
            }
        }
    }
}
