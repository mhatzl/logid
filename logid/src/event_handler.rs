use std::{marker::PhantomData, sync::mpsc::Receiver, thread::JoinHandle};

use logid_core::{
    evident::event::Event,
    log_id::LogId,
    logging::{event_entry::LogEventEntry, intermediary_event::IntermediaryLogEvent, LOGGER},
};

pub struct LogEventHandler {
    log_thread: JoinHandle<()>,
}

impl LogEventHandler {
    pub fn shutdown(self) {
        crate::evident::event::set_event::<_, LogEventEntry, IntermediaryLogEvent>(
            logid_core::log_id::STOP_LOGGING,
            crate::evident::this_origin!(),
        )
        .finalize();

        let _ = self.log_thread.join();
    }
}

#[derive(Default)]
pub struct NoKind;

#[derive(Default)]
pub struct AllLogs;

#[derive(Default)]
pub struct SpecificLogs;

type Handler = Box<dyn FnMut(Event<LogId, LogEventEntry>) + std::marker::Send + 'static>;

#[derive(Default)]
pub struct LogEventHandlerBuilder<K> {
    log_ids: Vec<LogId>,
    handler: Vec<Handler>,
    sub_kind: PhantomData<K>,
}

impl LogEventHandlerBuilder<NoKind> {
    pub fn new() -> Self {
        LogEventHandlerBuilder {
            log_ids: vec![logid_core::log_id::STOP_LOGGING],
            handler: Vec::new(),
            sub_kind: PhantomData,
        }
    }

    pub fn add_handler(
        mut self,
        handler: impl FnMut(Event<LogId, LogEventEntry>) + std::marker::Send + 'static,
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

fn event_listener<F: FnMut(Event<LogId, LogEventEntry>)>(
    mut fns: Vec<F>,
    recv: &Receiver<Event<LogId, LogEventEntry>>,
) {
    while let Ok(log_event) = recv.recv() {
        if log_event.get_id() == &logid_core::log_id::STOP_LOGGING {
            break;
        }

        fns.iter_mut().for_each(|f| f(log_event.clone()));
    }
}

fn stderr_writer(log_event: Event<LogId, LogEventEntry>) {
    console_writer(log_event, true);
}

fn stdout_writer(log_event: Event<LogId, LogEventEntry>) {
    console_writer(log_event, false);
}

fn console_writer(log_event: Event<LogId, LogEventEntry>, to_stderr: bool) {
    let id = log_event.get_id();
    let mut content = format!("{}: {}\n", id.get_log_level(), log_event.get_msg());

    if let Some(filter) = LOGGER.get_filter() {
        let origin = log_event.get_origin();
        if filter.show_origin_info(*id, origin) {
            content.push_str(&format!("|--> Origin: {}\n", origin));
        }
    }

    let entry = log_event.get_entry();

    // Note: Addon filter is already applied on capture side, so printing what is captured is fine here

    for info in entry.get_infos() {
        content.push_str(&format!("|--> Info: {}\n", info));
    }

    for debug in entry.get_debugs() {
        content.push_str(&format!("|--> Debug: {}\n", debug));
    }

    for trace in entry.get_traces() {
        content.push_str(&format!("|--> Trace: {}\n", trace));
    }

    #[cfg(feature = "diagnostics")]
    for diag in entry.get_diagnostics() {
        // TODO: make diag output prettier
        content.push_str(&format!("|--> Diagnostics: {}\n", diag.message));
    }

    #[cfg(feature = "payloads")]
    for payload in entry.get_payloads() {
        content.push_str(&format!("|--> Payload: {}\n", payload));
    }

    if to_stderr {
        eprint!("{}", content);
    } else {
        print!("{}", content);
    };
}
