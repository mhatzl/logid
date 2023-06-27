use std::{
    marker::PhantomData,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::Receiver,
        Arc,
    },
    thread::JoinHandle,
};

use colored::*;
use logid_core::{
    evident::event::{finalized::FinalizedEvent, Event},
    log_id::{LogId, LogLevel, START_LOGGING, STOP_LOGGING},
    logging::{event_entry::LogEventEntry, intermediary_event::IntermediaryLogEvent, LOGGER},
    new_log_id,
};

const HANDLER_START_LOGGING: LogId = new_log_id!("HANDLER_START_LOGGING", LogLevel::Info);
const HANDLER_STOP_LOGGING: LogId = new_log_id!("HANDLER_STOP_LOGGING", LogLevel::Info);
const SHUTDOWN_HANDLER: LogId = new_log_id!("SHUTDOWN_HANDLER", LogLevel::Info);

pub struct LogEventHandler {
    log_thread: JoinHandle<()>,
    /// Start flag needed to have independent handler.
    start: Arc<AtomicBool>,
    /// Stop flag needed to have independent handler.
    stop: Arc<AtomicBool>,
    /// Shutdown flag needed to have independent handler.
    shutdown: Arc<AtomicBool>,
    /// Capture flag needed to have independent handler.
    capturing: Arc<AtomicBool>,
}

impl LogEventHandler {
    pub fn start(&self) {
        self.start.store(true, Ordering::Release);

        crate::evident::event::set_event_with_msg::<_, LogEventEntry, IntermediaryLogEvent>(
            HANDLER_START_LOGGING,
            "Start logging on handler.",
            crate::evident::this_origin!(),
        )
        .finalize();
    }

    pub fn stop(&self) {
        self.stop.store(true, Ordering::Release);

        crate::evident::event::set_event_with_msg::<_, LogEventEntry, IntermediaryLogEvent>(
            HANDLER_STOP_LOGGING,
            "Stop logging on handler.",
            crate::evident::this_origin!(),
        )
        .finalize();
    }

    pub fn shutdown(self) {
        drop(self)
    }

    pub fn is_capturing(&self) -> bool {
        self.capturing.load(Ordering::Acquire)
    }
}

impl Drop for LogEventHandler {
    fn drop(&mut self) {
        self.shutdown.store(true, Ordering::Release);

        crate::evident::event::set_event_with_msg::<_, LogEventEntry, IntermediaryLogEvent>(
            SHUTDOWN_HANDLER,
            "Shutdown logging on handler.",
            crate::evident::this_origin!(),
        )
        .finalize();

        // Note: 'join()' needs 'self', but drop() only provides `&mut self`
        // Also in some rare timing issues the shutdown event is not received in the handler thread, if the main thread finished before sending the event to all listeners.
        let mut safe_cnt = 0;
        while !self.log_thread.is_finished() && safe_cnt < 10 {
            std::thread::sleep(std::time::Duration::from_micros(5));
            safe_cnt += 1;
        }
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

impl<K> LogEventHandlerBuilder<K> {
    fn create(self, subscribe_specific: bool) -> LogEventHandler {
        let start = Arc::new(AtomicBool::new(false));
        let moved_start = start.clone();

        let stop = Arc::new(AtomicBool::new(false));
        let moved_stop = stop.clone();

        let shutdown = Arc::new(AtomicBool::new(false));
        let moved_shutdown = shutdown.clone();

        let capturing = Arc::new(AtomicBool::new(true));
        let moved_capturing = capturing.clone();

        let thread_setup = Arc::new(AtomicBool::new(false));
        let moved_thread_setup = thread_setup.clone();

        let log_thread = std::thread::spawn(move || {
            let sub_res = if subscribe_specific {
                LOGGER.subscribe_to_many(self.log_ids)
            } else {
                LOGGER.subscribe_to_all_events()
            };

            match sub_res {
                Ok(recv) => {
                    // Note: Setup-flag in both cases to make sure possible lazy eval of result is no problem.
                    moved_thread_setup.store(true, Ordering::Relaxed);

                    event_listener(
                        self.handler,
                        recv.get_receiver(),
                        moved_start,
                        moved_stop,
                        moved_shutdown,
                        moved_capturing,
                    );
                }
                Err(_) => {
                    moved_thread_setup.store(true, Ordering::Relaxed);
                    eprintln!(
                        "{}: Could not create LOGGER subscription for the LogEventHandler.",
                        "ERR".bold().red()
                    );
                }
            }
        });

        // Note: Needed hack to ensure thread above is created before continuing.
        // This is done by forcing a context switch before returning.
        // Without this, events may not be received correctly if set immediately after build().
        //
        // Note: 'yield_now()' alone is not enough, because the OS might not accept our suggestion.
        while !thread_setup.load(Ordering::Relaxed) {
            std::thread::yield_now();
        }

        LogEventHandler {
            log_thread,
            start,
            stop,
            shutdown,
            capturing,
        }
    }
}

impl LogEventHandlerBuilder<AllLogs> {
    pub fn build(self) -> LogEventHandler {
        self.create(false)
    }
}

impl LogEventHandlerBuilder<SpecificLogs> {
    pub fn build(self) -> LogEventHandler {
        self.create(true)
    }
}

fn event_listener<F: FnMut(Event<LogId, LogEventEntry>)>(
    mut fns: Vec<F>,
    recv: &Receiver<Event<LogId, LogEventEntry>>,
    start: Arc<AtomicBool>,
    stop: Arc<AtomicBool>,
    shutdown: Arc<AtomicBool>,
    capturing: Arc<AtomicBool>,
) {
    let mut shutdown_received = false;

    while !shutdown_received {
        if capturing.load(Ordering::Acquire) {
            while let Ok(log_event) = recv.recv() {
                let id = log_event.get_id();

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
                let id = log_event.get_id();

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

fn stderr_writer(log_event: Event<LogId, LogEventEntry>) {
    console_writer(log_event, true);
}

fn stdout_writer(log_event: Event<LogId, LogEventEntry>) {
    console_writer(log_event, false);
}

fn console_writer(log_event: Event<LogId, LogEventEntry>, to_stderr: bool) {
    let id = log_event.get_id();
    let level = id.get_log_level();
    let mut content = format!("{}: {}\n", colored_level(level), log_event.get_msg());

    if let Some(filter) = LOGGER.get_filter() {
        let origin = log_event.get_origin();

        if filter.show_id(*id, origin) {
            content.push_str(&format!(
                "{} {}: id='{}::{}::{}', entry='{}'\n",
                colored_addon_start(level),
                "Event".bold(),
                id.get_crate_name(),
                id.get_module_path(),
                id.get_identifier(),
                log_event.get_entry_id(),
            ));
        }

        if filter.show_origin_info(*id, origin) {
            content.push_str(&format!(
                "{} {}: {}\n",
                colored_addon_start(level),
                "Origin".bold(),
                origin
            ));
        }
    }

    let entry = log_event.get_entry();

    // Note: Addon filter is already applied on capture side, so printing what is captured is fine here

    for info in entry.get_infos() {
        content.push_str(&format!(
            "{} {}: {}\n",
            colored_addon_start(level),
            "Info".bold().color(get_level_color(LogLevel::Info)),
            info
        ));
    }

    for debug in entry.get_debugs() {
        content.push_str(&format!(
            "{} {}: {}\n",
            colored_addon_start(level),
            "Debug".bold().color(get_level_color(LogLevel::Debug)),
            debug
        ));
    }

    for trace in entry.get_traces() {
        content.push_str(&format!(
            "{} {}: {}\n",
            colored_addon_start(level),
            "Trace".bold().color(get_level_color(LogLevel::Trace)),
            trace
        ));
    }

    for related in entry.get_related() {
        content.push_str(&format!(
            "{} {}: {}\n",
            colored_addon_start(level),
            "Related".bold(),
            colored_related(related)
        ));
    }

    #[cfg(feature = "diagnostics")]
    for diag in entry.get_diagnostics() {
        // TODO: make diag output prettier
        content.push_str(&format!(
            "{} {}: {}\n",
            colored_addon_start(level),
            "Diagnostics".bold(),
            diag.message
        ));
    }

    #[cfg(feature = "payloads")]
    for payload in entry.get_payloads() {
        content.push_str(&format!(
            "{} {}: {}\n",
            colored_addon_start(level),
            "Payload".bold(),
            payload
        ));
    }

    if to_stderr {
        eprint!("{}", content);
    } else {
        print!("{}", content);
    };
}

fn colored_level(level: LogLevel) -> String {
    level
        .to_string()
        .bold()
        .color(get_level_color(level))
        .to_string()
}

fn get_level_color(level: LogLevel) -> colored::Color {
    match level {
        LogLevel::Error => Color::Red,
        LogLevel::Warn => Color::Yellow,
        LogLevel::Info => Color::Green,
        LogLevel::Debug => Color::Blue,
        LogLevel::Trace => Color::Cyan,
    }
}

fn colored_addon_start(level: LogLevel) -> String {
    "|-->".color(get_level_color(level)).to_string()
}

fn colored_related(related: &FinalizedEvent<LogId>) -> String {
    let id = related.event_id;
    format!(
        "id='{}: {}::{}::{}', entry='{}'",
        colored_level(id.get_log_level()),
        id.get_crate_name(),
        id.get_module_path(),
        id.get_identifier(),
        related.entry_id
    )
}
