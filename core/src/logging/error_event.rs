use crate::log_id::LogId;

use super::{
    event_addons::LogEventAddons, event_entry::LogEventEntry,
    intermediary_event::IntermediaryLogEvent,
};

#[derive(Default, Clone, PartialEq, Eq)]
pub struct ErrLogEvent<E: std::error::Error + Into<LogId> + Clone> {
    pub(crate) error: E,
    pub(crate) interm_event: IntermediaryLogEvent,
}

impl<E: std::error::Error + Into<LogId> + Clone> ErrLogEvent<E> {
    pub fn new(error: E, origin: crate::evident::event::origin::Origin) -> Self {
        let interm_event = evident::event::set_event::<LogId, LogEventEntry, IntermediaryLogEvent>(
            error.clone().into(),
            origin,
        );
        ErrLogEvent {
            error,
            interm_event,
        }
    }

    pub fn new_with_msg(
        error: E,
        msg: &str,
        origin: crate::evident::event::origin::Origin,
    ) -> Self {
        let interm_event = evident::event::set_event_with_msg::<
            LogId,
            LogEventEntry,
            IntermediaryLogEvent,
        >(error.clone().into(), msg, origin);
        ErrLogEvent {
            error,
            interm_event,
        }
    }

    pub fn into_err(self) -> E {
        self.interm_event.finalize();
        self.error
    }
}

impl<E: std::error::Error + Into<LogId> + Clone> LogEventAddons for ErrLogEvent<E> {
    fn add_info(mut self, msg: &str) -> Self {
        self.interm_event = self.interm_event.add_info(msg);
        self
    }

    fn add_debug(mut self, msg: &str) -> Self {
        self.interm_event = self.interm_event.add_debug(msg);
        self
    }

    fn add_trace(mut self, msg: &str) -> Self {
        self.interm_event = self.interm_event.add_trace(msg);
        self
    }

    #[cfg(feature = "diagnostics")]
    fn add_diagnostic(mut self, diagnostic: crate::lsp_types::Diagnostic) -> Self {
        self.interm_event = self.interm_event.add_diagnostic(diagnostic);
        self
    }

    #[cfg(feature = "payloads")]
    fn add_payload(mut self, payload: serde_json::value::Value) -> Self {
        self.interm_event = self.interm_event.add_payload(payload);
        self
    }
}
