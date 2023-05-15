use evident::event::{entry::EventEntry, origin::Origin};

use crate::log_id::LogId;

use super::event_entry::{EntryKind, LogEventEntry};

/// Struct linking a [`LogId`] to the map the entry for the ID was added to.
#[derive(Default, Clone, PartialEq, Eq)]
pub struct IntermediaryLogEvent<V: Into<LogId> + Clone> {
    /// [`EventEntry`] storing all event information.
    pub(crate) entry: LogEventEntry,
    creator: V,
}

impl<V: Into<LogId> + Clone>
    evident::event::intermediary::IntermediaryEvent<V, LogId, LogEventEntry>
    for IntermediaryLogEvent<V>
{
    fn new(event_creator: V, msg: &str, origin: Origin) -> Self {
        IntermediaryLogEvent {
            creator: event_creator.clone(),
            entry: LogEventEntry::new(event_creator.into(), msg, origin),
        }
    }

    fn get_entry(&self) -> &LogEventEntry {
        &self.entry
    }

    fn take_entry(&mut self) -> LogEventEntry {
        std::mem::take(&mut self.entry)
    }

    fn get_event_creator(&self) -> &V {
        &self.creator
    }
}

impl<V: Into<LogId> + Clone> std::fmt::Debug for IntermediaryLogEvent<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LogId-IntermEvent")
            .field("event_id", &self.entry.event_id)
            .field("entry_id", &self.entry.entry_id)
            .field("origin", &self.entry.origin)
            .finish()
    }
}

impl<V: Into<LogId> + Clone> IntermediaryLogEvent<V> {
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

    /// Add an warning message to this log-id event
    pub fn add_warning(mut self, msg: &str) -> Self {
        add_addon_to_entry(&mut self, EntryKind::Warning(msg.to_owned()));
        self
    }

    /// Add an info message to this log-id event
    pub fn add_info(mut self, msg: &str) -> Self {
        add_addon_to_entry(&mut self, EntryKind::Info(msg.to_owned()));
        self
    }

    /// Add a debug message to this log-id event
    pub fn add_debug(mut self, msg: &str) -> Self {
        add_addon_to_entry(&mut self, EntryKind::Debug(msg.to_owned()));
        self
    }

    /// Add a trace message to this log-id event
    pub fn add_trace(mut self, msg: &str) -> Self {
        add_addon_to_entry(&mut self, EntryKind::Trace(msg.to_owned()));
        self
    }

    /// Add diagnostic info to this log-id event
    #[cfg(feature = "diagnostics")]
    pub fn add_diagnostic(mut self, diagnostic: lsp_types::Diagnostic) -> Self {
        add_addon_to_entry(&mut self, EntryKind::Diagnostic(diagnostic));
        self
    }

    /// Add a payload to this log-id event
    #[cfg(feature = "payloads")]
    pub fn add_payload(mut self, payload: serde_json::value::Value) -> Self {
        add_addon_to_entry(&mut self, EntryKind::Payload(payload));
        self
    }
}

fn add_addon_to_entry<V: Into<LogId> + Clone>(
    id_event: &mut IntermediaryLogEvent<V>,
    kind: EntryKind,
) {
    match kind {
        EntryKind::Warning(msg) => id_event.entry.warnings.push(msg),
        EntryKind::Info(msg) => id_event.entry.infos.push(msg),
        EntryKind::Debug(msg) => id_event.entry.debugs.push(msg),
        EntryKind::Trace(msg) => id_event.entry.traces.push(msg),

        #[cfg(feature = "diagnostics")]
        EntryKind::Diagnostic(diag) => id_event.entry.diagnostics.push(diag),

        #[cfg(feature = "payloads")]
        EntryKind::Payload(payload) => id_event.entry.payloads.push(payload),
    }
}
