use crate::log_id::LogId;

use self::{entry::EventEntry, intermediary::IntermediaryEvent};

pub mod entry;
pub mod intermediary;
pub mod macros;
pub mod origin;

/// Trait to use [`LogId`] for tracing.
pub trait EventFns {
    /// Set an event for a [`LogId`], and storing it inside the [`LogIdMap`] of the given crate name.
    ///
    /// # Arguments
    ///
    /// * `crate_name` ... Name of the crate to identify the [`LogIdMap`]
    /// * `msg` ... Main message that is set for this log-id (should be a user-centered event description)
    /// * `filename` ... Name of the source file where the event is set (Note: use `file!()`)
    /// * `line_nr` ... Line number where the event is set (Note: use `line!()`)
    /// * `module_path` ... Module path where the event is set (Note: use `module_path!()`)
    fn set_event(
        self,
        crate_name: &'static str,
        msg: &str,
        filename: &str,
        line_nr: u32,
        module_path: &str,
    ) -> IntermediaryEvent;
}

impl EventFns for LogId {
    fn set_event(
        self,
        crate_name: &'static str,
        msg: &str,
        filename: &str,
        line_nr: u32,
        module_path: &str,
    ) -> IntermediaryEvent {
        IntermediaryEvent {
            entry: EventEntry::new(self, msg, filename, line_nr, module_path),
            crate_name,
        }
    }
}

#[derive(Default, Clone, PartialEq, Eq)]
pub struct Event {
    pub(crate) crate_name: &'static str,
    pub(crate) entry: EventEntry,
}

impl std::fmt::Debug for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LogId-EventMsg")
            .field("crate", &self.crate_name)
            .field("id", &self.entry.id)
            .field("origin", &self.entry.origin)
            .field("hash", &self.entry.hash)
            .finish()
    }
}

impl Event {
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
}
