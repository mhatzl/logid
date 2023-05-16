use evident::event::{entry::EventEntry, origin::Origin};

use crate::log_id::LogId;

use super::{
    event_addons::LogEventAddons,
    event_entry::{EntryKind, LogEventEntry},
};

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
}

impl LogEventAddons for IntermediaryLogEvent {
    /// Add an info message to this log-id event
    fn add_info(mut self, msg: &str) -> Self {
        add_addon_to_entry(&mut self, EntryKind::Info(msg.to_owned()));
        self
    }

    /// Add a debug message to this log-id event
    fn add_debug(mut self, msg: &str) -> Self {
        add_addon_to_entry(&mut self, EntryKind::Debug(msg.to_owned()));
        self
    }

    /// Add a trace message to this log-id event
    fn add_trace(mut self, msg: &str) -> Self {
        add_addon_to_entry(&mut self, EntryKind::Trace(msg.to_owned()));
        self
    }

    /// Add diagnostic info to this log-id event
    #[cfg(feature = "diagnostics")]
    fn add_diagnostic(mut self, diagnostic: crate::lsp_types::Diagnostic) -> Self {
        add_addon_to_entry(&mut self, EntryKind::Diagnostic(diagnostic));
        self
    }

    /// Add a payload to this log-id event
    #[cfg(feature = "payloads")]
    fn add_payload(mut self, payload: serde_json::value::Value) -> Self {
        add_addon_to_entry(&mut self, EntryKind::Payload(payload));
        self
    }
}

fn add_addon_to_entry(id_event: &mut IntermediaryLogEvent, kind: EntryKind) {
    match kind {
        EntryKind::Info(msg) => id_event.entry.infos.push(msg),
        EntryKind::Debug(msg) => id_event.entry.debugs.push(msg),
        EntryKind::Trace(msg) => id_event.entry.traces.push(msg),

        #[cfg(feature = "diagnostics")]
        EntryKind::Diagnostic(diag) => id_event.entry.diagnostics.push(diag),

        #[cfg(feature = "payloads")]
        EntryKind::Payload(payload) => id_event.entry.payloads.push(payload),
    }
}
