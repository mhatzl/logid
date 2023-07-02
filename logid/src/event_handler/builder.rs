use std::{
    marker::PhantomData,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::Receiver,
        Arc,
    },
};

use logid_core::{
    evident::event::Event,
    log_id::{LogId, START_LOGGING, STOP_LOGGING},
    logging::{event_entry::LogEventEntry, msg::LogMsg, LOGGER},
};

use super::{
    terminal::{stderr_writer, stdout_writer},
    LogEventHandler, HANDLER_START_LOGGING, HANDLER_STOP_LOGGING, SHUTDOWN_HANDLER,
};

#[derive(Default)]
pub struct NoKind;

#[derive(Default)]
pub struct AllLogs;

#[derive(Default)]
pub struct SpecificLogs;

type Handler =
    Box<dyn FnMut(Arc<Event<LogId, LogMsg, LogEventEntry>>) + std::marker::Send + 'static>;

#[derive(Default)]
pub struct LogEventHandlerBuilder<K> {
    log_ids: Vec<LogId>,
    handler: Vec<Handler>,
    sub_kind: PhantomData<K>,
}

impl LogEventHandlerBuilder<NoKind> {
    pub fn new() -> Self {
        LogEventHandlerBuilder {
            // Note: Make sure control IDs are received by handler.
            log_ids: vec![
                logid_core::log_id::START_LOGGING,
                logid_core::log_id::STOP_LOGGING,
                HANDLER_START_LOGGING,
                HANDLER_STOP_LOGGING,
                SHUTDOWN_HANDLER,
            ],
            handler: Vec::new(),
            sub_kind: PhantomData,
        }
    }

    pub fn add_handler(
        mut self,
        handler: impl FnMut(Arc<Event<LogId, LogMsg, LogEventEntry>>) + std::marker::Send + 'static,
    ) -> Self {
        self.handler.push(Box::new(handler));
        self
    }

    pub fn to_stderr(mut self) -> Self {
        self.handler.push(Box::new(stderr_writer));
        self
    }

    pub fn to_stdout(mut self) -> Self {
        self.handler.push(Box::new(stdout_writer));
        self
    }

    pub fn all_log_events(self) -> LogEventHandlerBuilder<AllLogs> {
        LogEventHandlerBuilder {
            log_ids: self.log_ids,
            handler: self.handler,
            sub_kind: PhantomData,
        }
    }

    pub fn for_log_ids(
        mut self,
        ids: impl Iterator<Item = LogId>,
    ) -> LogEventHandlerBuilder<SpecificLogs> {
        self.log_ids.extend(ids);

        LogEventHandlerBuilder {
            log_ids: self.log_ids,
            handler: self.handler,
            sub_kind: PhantomData,
        }
    }
}

impl<K> LogEventHandlerBuilder<K> {
    fn create(self, subscribe_specific: bool) -> Result<LogEventHandler, LogEventHandlerError> {
        let start = Arc::new(AtomicBool::new(false));
        let moved_start = start.clone();

        let stop = Arc::new(AtomicBool::new(false));
        let moved_stop = stop.clone();

        let shutdown = Arc::new(AtomicBool::new(false));
        let moved_shutdown = shutdown.clone();

        let capturing = Arc::new(AtomicBool::new(true));
        let moved_capturing = capturing.clone();

        let sub_res = if subscribe_specific {
            LOGGER.subscribe_to_many(self.log_ids)
        } else {
            LOGGER.subscribe_to_all_events()
        };

        match sub_res {
            Ok(recv) => {
                let log_thread = std::thread::spawn(move || {
                    event_listener(
                        self.handler,
                        recv.get_receiver(),
                        moved_start,
                        moved_stop,
                        moved_shutdown,
                        moved_capturing,
                    );
                });

                Ok(LogEventHandler {
                    log_thread,
                    start,
                    stop,
                    shutdown,
                    capturing,
                })
            }
            Err(_) => Err(LogEventHandlerError::CreatingSubscription),
        }
    }
}

impl LogEventHandlerBuilder<AllLogs> {
    pub fn build(self) -> Result<LogEventHandler, LogEventHandlerError> {
        self.create(false)
    }
}

impl LogEventHandlerBuilder<SpecificLogs> {
    pub fn build(self) -> Result<LogEventHandler, LogEventHandlerError> {
        self.create(true)
    }
}

#[derive(Debug, Clone)]
pub enum LogEventHandlerError {
    CreatingSubscription,
}

impl std::fmt::Display for LogEventHandlerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogEventHandlerError::CreatingSubscription => write!(
                f,
                "Could not create LOGGER subscription for the LogEventHandler."
            ),
        }
    }
}

fn event_listener<F: FnMut(Arc<Event<LogId, LogMsg, LogEventEntry>>)>(
    mut fns: Vec<F>,
    recv: &Receiver<Arc<Event<LogId, LogMsg, LogEventEntry>>>,
    start: Arc<AtomicBool>,
    stop: Arc<AtomicBool>,
    shutdown: Arc<AtomicBool>,
    capturing: Arc<AtomicBool>,
) {
    let mut shutdown_received = false;

    while !shutdown_received {
        if capturing.load(Ordering::Acquire) {
            while let Ok(log_event) = recv.recv() {
                let id = log_event.get_event_id();

                // Note: Due to channel buffer, handler flags might already be set, but not all events are processed => required check on flag AND event id

                if id == &STOP_LOGGING {
                    capturing.store(false, Ordering::Release);
                    break;
                } else if stop.load(Ordering::Acquire) && id == &HANDLER_STOP_LOGGING {
                    stop.store(false, Ordering::Release);
                    capturing.store(false, Ordering::Release);
                    break;
                } else if shutdown.load(Ordering::Acquire) && id == &SHUTDOWN_HANDLER {
                    shutdown_received = true;
                    break;
                } else if id != &HANDLER_START_LOGGING
                    && id != &HANDLER_STOP_LOGGING
                    && id != &SHUTDOWN_HANDLER
                    && id != &START_LOGGING
                {
                    fns.iter_mut().for_each(|f| f(log_event.clone()));
                }
            }
        } else {
            while let Ok(log_event) = recv.recv() {
                let id = log_event.get_event_id();

                if id == &START_LOGGING {
                    capturing.store(true, Ordering::Release);
                    break;
                } else if start.load(Ordering::Acquire) && id == &HANDLER_START_LOGGING {
                    start.store(false, Ordering::Release);
                    capturing.store(true, Ordering::Release);
                    break;
                } else if shutdown.load(Ordering::Acquire) && id == &SHUTDOWN_HANDLER {
                    shutdown_received = true;
                    break;
                }
            }
        }
    }
}
