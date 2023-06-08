use std::{marker::PhantomData, sync::mpsc::Receiver, thread::JoinHandle};

use logid_core::{
    evident::event::Event,
    log_id::{LogId, LogLevel},
    logging::{event_entry::LogEventEntry, intermediary_event::IntermediaryLogEvent, LOGGER},
};

/// Notify listeners to stop logging.
///
/// Note: Uses `LogLevel::Error` to ensure it is not ignored by any filter
const STOP_LOGGING: LogId = logid_core::new_log_id!("STOP_LOGGING", LogLevel::Error);

pub struct LogEventHandler {
    log_thread: JoinHandle<()>,
}

impl LogEventHandler {
    pub fn shutdown(self) {
        crate::evident::event::set_event::<_, LogEventEntry, IntermediaryLogEvent>(
            STOP_LOGGING,
            crate::evident::this_origin!(),
        )
        .finalize();

        let _ = self.log_thread.join();
        LOGGER.shutdown();
    }
}

pub struct NoKind;

pub struct AllLogs;

pub struct SpecificLogs;

pub struct LogEventHandlerBuilder<K> {
    log_ids: Vec<LogId>,
    handler: Vec<Box<dyn FnMut(Event<LogId, LogEventEntry>) -> () + std::marker::Send + 'static>>,
    sub_kind: PhantomData<K>,
}

impl LogEventHandlerBuilder<NoKind> {
    pub fn new() -> Self {
        LogEventHandlerBuilder {
            log_ids: Vec::new(),
            handler: Vec::new(),
            sub_kind: PhantomData,
        }
    }

    pub fn add_handler(
        mut self,
        handler: impl FnMut(Event<LogId, LogEventEntry>) -> () + std::marker::Send + 'static,
    ) -> Self {
        self.handler.push(Box::new(handler));
        self
    }

    pub fn write_to_console(mut self) -> Self {
        self.handler.push(Box::new(console_writer));
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

impl LogEventHandlerBuilder<AllLogs> {
    pub fn build(self) -> LogEventHandler {
        let handle = std::thread::spawn(|| {
            if let Ok(recv) = LOGGER.subscribe_to_all_events() {
                event_listener(self.handler, recv.get_receiver());
            }
        });

        LogEventHandler { log_thread: handle }
    }
}

impl LogEventHandlerBuilder<SpecificLogs> {
    pub fn build(self) -> LogEventHandler {
        let handle = std::thread::spawn(|| {
            if let Ok(recv) = LOGGER.subscribe_to_many(self.log_ids) {
                event_listener(self.handler, recv.get_receiver());
            }
        });

        LogEventHandler { log_thread: handle }
    }
}

fn event_listener<F: FnMut(Event<LogId, LogEventEntry>) -> ()>(
    mut fns: Vec<F>,
    recv: &Receiver<Event<LogId, LogEventEntry>>,
) {
    while let Ok(log_event) = recv.recv() {
        if log_event.get_id() == &STOP_LOGGING {
            break;
        }

        fns.iter_mut().for_each(|f| f(log_event.clone()));
    }
}

fn console_writer(log_event: Event<LogId, LogEventEntry>) {
    println!(
        "{}: {}",
        log_event.get_id().get_log_level(),
        log_event.get_msg()
    );
}
