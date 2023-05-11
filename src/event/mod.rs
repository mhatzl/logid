use crate::{
    log_id::{LogId, LogLevel},
};

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

    /// Set an event for a [`LogId`] **without** adding it to a [`LogIdMap`].
    ///
    /// # Arguments
    ///
    /// * `msg` ... Main message that is set for this event (should be a user-centered event description)
    /// * `filename` ... Name of the source file where the event is set (Note: use `file!()`)
    /// * `line_nr` ... Line number where the event is set (Note: use `line!()`)
    /// * `module_path` ... Module path where the event is set (Note: use `module_path!()`)
    fn set_silent_event(
        self,
        crate_name: &'static str,
        msg: &str,
        filename: &str,
        line_nr: u32,
        module_path: &str,
    ) -> IntermediaryEvent;
}

/// Traces a [`Entry`] creation.
fn create_entry(id: LogId, msg: &str, filename: &str, line_nr: u32, module_path: &str) -> EventEntry {
    let id_entry = EventEntry::new(id, msg, filename, line_nr, module_path);

    // Note: It is not possible to set `target` via parameter, because it requires `const`
    // Same goes for `level` for the `event` macro => match and code duplication needed
    match id_entry.level {
        LogLevel::Error => tracing::error!("{}(id={}): {}", id_entry.hash, id, msg),
        LogLevel::Warn => tracing::warn!("{}(id={}): {}", id_entry.hash, id, msg),
        LogLevel::Info => tracing::info!("{}(id={}): {}", id_entry.hash, id, msg),
        LogLevel::Debug => tracing::debug!("{}(id={}): {}", id_entry.hash, id, msg),
    }

    tracing::trace!(
        "{}(origin): {}",
        id_entry.hash,
        String::from(&id_entry.origin)
    );

    id_entry
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
            entry: create_entry(self, msg, filename, line_nr, module_path),
            crate_name,
            is_silent: false,
        }
    }

    fn set_silent_event(
        self,
        crate_name: &'static str,
        msg: &str,
        filename: &str,
        line_nr: u32,
        module_path: &str,
    ) -> IntermediaryEvent {
        IntermediaryEvent {
            entry: create_entry(self, msg, filename, line_nr, module_path),
            crate_name,
            is_silent: true,
        }
    }
}

#[derive(Default, Clone, PartialEq, Eq)]
pub struct Event {
    pub crate_name: &'static str,
    pub entry: EventEntry,
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

