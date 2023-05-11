use crate::{event::entry::EntryKind, log_id::LogId, publisher::PUBLISHER};

use super::{entry::EventEntry, Event};

#[cfg(feature = "diagnostics")]
use lsp_types::Diagnostic;

/// Struct linking a [`LogId`] to the map the entry for the ID was added to.
#[derive(Default, Clone, PartialEq, Eq)]
pub struct IntermediaryEvent {
    /// Crate name identifying the [`LogIdMap`] the [`LogIdEvent`] is associated with, or none for silent events.
    pub(crate) crate_name: &'static str,
    /// [`Entry`] for the [`LogIdEvent`] storing all event information.
    pub(crate) entry: EventEntry,
}

impl From<IntermediaryEvent> for LogId {
    fn from(mapped_id: IntermediaryEvent) -> Self {
        mapped_id.finalize()
    }
}

impl PartialEq<LogId> for IntermediaryEvent {
    fn eq(&self, other: &LogId) -> bool {
        self.entry.id == *other
    }
}

impl PartialEq<IntermediaryEvent> for LogId {
    fn eq(&self, other: &IntermediaryEvent) -> bool {
        *self == other.entry.id
    }
}

impl Drop for IntermediaryEvent {
    /// On drop, transforms the [`IntermediaryEvent`] into an [`Event`] that gets sent to the central publisher.
    fn drop(&mut self) {
        let hash = self.entry.hash;

        // Note: Since IntermediaryEvent implements `Drop`, the Event struct is needed for message passing.
        // Passing `IntermediaryEvent` directly created unpredictable behaviors in tests.
        if let Err(err) = PUBLISHER.capturer.try_send(Event {
            crate_name: self.crate_name,
            entry: std::mem::take(&mut self.entry),
        }) {
            tracing::error!(
                "{}(send): {}",
                hash,
                "Failed sending log-id to central map."
            );
            tracing::debug!("{}(send-cause): {}", hash, err);
        }
    }
}

impl std::fmt::Debug for IntermediaryEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LogId-Event")
            .field("id", &self.entry.id)
            .field("origin", &self.entry.origin)
            .field("hash", &self.entry.hash)
            .finish()
    }
}

impl IntermediaryEvent {
    /// Returns the [`LogId`] of this log-id event
    pub fn get_id(&self) -> LogId {
        self.entry.id
    }

    /// Returns the name of the associated crate of this log-id event
    pub fn get_crate_name(&self) -> &'static str {
        self.crate_name
    }

    /// Returns the [`Entry`] of this log-id event
    pub fn get_entry(&self) -> &EventEntry {
        &self.entry
    }

    /// Add an info message to this log-id event
    pub fn add_info(mut self, msg: &str) -> Self {
        tracing::info!("{}(addon): {}", self.entry.hash, msg);
        add_addon_to_entry(&mut self, EntryKind::Info(msg.to_owned()));
        self
    }

    /// Add a debug message to this log-id event
    pub fn add_debug(mut self, msg: &str) -> Self {
        tracing::debug!("{}(addon): {}", self.entry.hash, msg);
        add_addon_to_entry(&mut self, EntryKind::Debug(msg.to_owned()));
        self
    }

    /// Add a trace message to this log-id event
    pub fn add_trace(mut self, msg: &str) -> Self {
        tracing::trace!("{}(addon): {}", self.entry.hash, msg);
        add_addon_to_entry(&mut self, EntryKind::Trace(msg.to_owned()));
        self
    }

    /// Add a log-id event that caused this log-id event
    #[cfg(feature = "causes")]
    pub fn add_cause(mut self, event_msg: Event) -> Self {
        tracing::info!("{}(cause): {:?}", self.entry.hash, event_msg);
        add_addon_to_entry(&mut self, EntryKind::Cause(event_msg));
        self
    }

    /// Add diagnostic info to this log-id event
    #[cfg(feature = "diagnostics")]
    pub fn add_diagnostic(mut self, diagnostic: Diagnostic) -> Self {
        tracing::trace!("{}(diag): {:?}", self.entry.hash, diagnostic);
        add_addon_to_entry(&mut self, EntryKind::Diagnostic(diagnostic));
        self
    }

    /// Add a payload to this log-id event
    #[cfg(feature = "payloads")]
    pub fn add_payload(mut self, payload: serde_json::value::Value) -> Self {
        tracing::trace!("{}(payload): {:?}", self.entry.hash, payload);
        add_addon_to_entry(&mut self, EntryKind::Payload(payload));
        self
    }

    /// Finalizing a [`LogIdEvent`] converts it back to a [`LogId`].
    /// This prevents any further information to be added to it.
    /// If the event was not created *silently*, it also moves the entry into the [`LogIdMap`] associated with the event.
    pub fn finalize(self) -> LogId {
        let id = self.entry.id;
        drop(self);
        id
    }
}

fn add_addon_to_entry(id_event: &mut IntermediaryEvent, kind: EntryKind) {
    match kind {
        EntryKind::Info(msg) => id_event.entry.infos.push(msg),
        EntryKind::Debug(msg) => id_event.entry.debugs.push(msg),
        EntryKind::Trace(msg) => id_event.entry.traces.push(msg),

        #[cfg(feature = "causes")]
        EntryKind::Cause(entry) => id_event.entry.causes.push(entry),

        #[cfg(feature = "diagnostics")]
        EntryKind::Diagnostic(diag) => id_event.entry.diagnostics.push(diag),

        #[cfg(feature = "payloads")]
        EntryKind::Payload(payload) => id_event.entry.payloads.push(payload),
    }
}
