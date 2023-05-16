use crate::log_id::LogId;

use super::{
    event_entry::{EntryKind, LogEventEntry},
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

    pub fn add_addon(mut self, kind: EntryKind) -> Self {
        match kind {
            EntryKind::Info(msg) => self.interm_event.entry.infos.push(msg),
            EntryKind::Debug(msg) => self.interm_event.entry.debugs.push(msg),
            EntryKind::Trace(msg) => self.interm_event.entry.traces.push(msg),

            #[cfg(feature = "diagnostics")]
            EntryKind::Diagnostic(diag) => self.interm_event.entry.diagnostics.push(diag),

            #[cfg(feature = "payloads")]
            EntryKind::Payload(payload) => self.interm_event.entry.payloads.push(payload),
        }

        self
    }
}
