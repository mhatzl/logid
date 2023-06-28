use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread::JoinHandle,
};

use logid_core::{
    log_id::{LogId, LogLevel},
    logging::{event_entry::LogEventEntry, intermediary_event::IntermediaryLogEvent},
    new_log_id,
};

pub mod builder;
pub mod terminal;

pub(self) const HANDLER_START_LOGGING: LogId = new_log_id!("HANDLER_START_LOGGING", LogLevel::Info);
pub(self) const HANDLER_STOP_LOGGING: LogId = new_log_id!("HANDLER_STOP_LOGGING", LogLevel::Info);
pub(self) const SHUTDOWN_HANDLER: LogId = new_log_id!("SHUTDOWN_HANDLER", LogLevel::Info);

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
