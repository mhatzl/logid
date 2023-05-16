use evident::event::{entry::EventEntry, origin::Origin};

use crate::log_id::LogId;

use super::event_entry::{EntryKind, LogEventEntry};

/// Struct linking a [`LogId`] to the map the entry for the ID was added to.
#[derive(Default, Clone, PartialEq, Eq)]
pub struct IntermediaryLogEvent {
    /// [`EventEntry`] storing all event information.
    pub(crate) entry: LogEventEntry,
}

impl evident::event::intermediary::IntermediaryEvent<LogId, LogEventEntry>
    for IntermediaryLogEvent
{
    fn new(event_id: LogId, msg: &str, origin: Origin) -> Self {
        IntermediaryLogEvent {
            entry: LogEventEntry::new(event_id.into(), msg, origin),
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

    /// Returns the name of the associated crate of this log-id event
    pub fn get_crate_name(&self) -> &'static str {
        self.entry.origin.crate_name
    }

    /// Returns the [`Entry`] of this log-id event
    pub fn get_entry(&self) -> &LogEventEntry {
        &self.entry
    }

    pub fn add_addon(mut self, kind: EntryKind) -> Self {
        match kind {
            EntryKind::Info(msg) => self.entry.infos.push(msg),
            EntryKind::Debug(msg) => self.entry.debugs.push(msg),
            EntryKind::Trace(msg) => self.entry.traces.push(msg),

            #[cfg(feature = "diagnostics")]
            EntryKind::Diagnostic(diag) => self.entry.diagnostics.push(diag),

            #[cfg(feature = "payloads")]
            EntryKind::Payload(payload) => self.entry.payloads.push(payload),
        }
        self
    }
}
