use evident::event::{entry::EventEntry, origin::Origin};

use crate::log_id::LogId;

use super::{
    event_entry::{AddonKind, LogEventEntry},
    msg::LogMsg,
    LOGGER,
};

/// Struct linking a [`LogId`] to the map the entry for the ID was added to.
#[derive(Default, Clone, PartialEq, Eq)]
pub struct IntermediaryLogEvent {
    /// [`EventEntry`] storing all event information.
    pub(crate) entry: LogEventEntry,
}

impl evident::event::intermediary::IntermediaryEvent<LogId, LogMsg, LogEventEntry>
    for IntermediaryLogEvent
{
    fn new(event_id: LogId, msg: Option<impl Into<LogMsg>>, origin: Origin) -> Self {
        IntermediaryLogEvent {
            entry: LogEventEntry::new(event_id, msg, origin),
        }
    }

    fn get_entry(&self) -> &LogEventEntry {
        &self.entry
    }

    fn take_entry(&mut self) -> LogEventEntry {
        std::mem::take(&mut self.entry)
    }
}

impl std::fmt::Debug for IntermediaryLogEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LogId-IntermEvent")
            .field("event_id", &self.entry.event_id)
            .field("entry_id", &self.entry.entry_id)
            .field("origin", &self.entry.origin)
            .finish()
    }
}

impl IntermediaryLogEvent {
    /// Returns the [`LogId`] of this log-id event
    pub fn get_event_id(&self) -> LogId {
        self.entry.event_id
    }

    /// Returns the [`Entry`] of this log-id event
    pub fn get_entry(&self) -> &LogEventEntry {
        &self.entry
    }

    pub fn add_addon(mut self, kind: AddonKind) -> Self {
        if let Some(filter) = LOGGER.get_filter() {
            if !filter.allow_addon(self.get_event_id(), &self.entry.origin, &kind) {
                return self;
            }
        }

        match kind {
            AddonKind::Info(msg) => self.entry.infos.push(msg),
            AddonKind::Debug(msg) => self.entry.debugs.push(msg),
            AddonKind::Trace(msg) => self.entry.traces.push(msg),
            AddonKind::Related(finalized_event) => self.entry.related.push(finalized_event),

            #[cfg(feature = "hint_note")]
            AddonKind::Hint(msg) => self.entry.hints.push(msg),
            #[cfg(feature = "hint_note")]
            AddonKind::Note(msg) => self.entry.notes.push(msg),

            #[cfg(feature = "diagnostics")]
            AddonKind::Diagnostic(diag) => self.entry.diagnostics.push(diag),

            #[cfg(feature = "payloads")]
            AddonKind::Payload(payload) => self.entry.payloads.push(payload),
        }
        self
    }
}
