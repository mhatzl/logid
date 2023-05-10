use crate::{
    event::entry::EntryKind,
    log_id::{LogId, LogLevel},
    publisher::PUBLISHER,
};

use self::{entry::Entry, msg::EventMsg};

#[cfg(feature = "diagnostics")]
use lsp_types::Diagnostic;

pub mod entry;
pub mod macros;
pub mod msg;
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
    ) -> Event;

    /// Set an event for a [`LogId`] **without** adding it to a [`LogIdMap`].
    ///
    /// # Arguments
    ///
    /// * `msg` ... Main message that is set for this event (should be a user-centered event description)
    /// * `filename` ... Name of the source file where the event is set (Note: use `file!()`)
    /// * `line_nr` ... Line number where the event is set (Note: use `line!()`)
    /// * `module_path` ... Module path where the event is set (Note: use `module_path!()`)
    fn set_silent_event(self, msg: &str, filename: &str, line_nr: u32, module_path: &str) -> Event;
}

/// Traces a [`Entry`] creation.
fn create_entry(id: LogId, msg: &str, filename: &str, line_nr: u32, module_path: &str) -> Entry {
    let id_entry = Entry::new(id, msg, filename, line_nr, module_path);

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
    ) -> Event {
        Event {
            entry: create_entry(self, msg, filename, line_nr, module_path),
            crate_name: Some(crate_name),
        }
    }

    fn set_silent_event(self, msg: &str, filename: &str, line_nr: u32, module_path: &str) -> Event {
        Event {
            entry: create_entry(self, msg, filename, line_nr, module_path),
            crate_name: None,
        }
    }
}

/// Struct linking a [`LogId`] to the map the entry for the ID was added to.
#[derive(Default, Clone)]
pub struct Event {
    /// Crate name identifying the [`LogIdMap`] the [`LogIdEvent`] is associated with, or none for silent events.
    crate_name: Option<&'static str>,
    /// [`Entry`] for the [`LogIdEvent`] storing all event information.
    pub(crate) entry: Entry,
}

impl From<Event> for LogId {
    fn from(mapped_id: Event) -> Self {
        mapped_id.finalize()
    }
}

impl PartialEq<LogId> for Event {
    fn eq(&self, other: &LogId) -> bool {
        self.entry.id == *other
    }
}

impl PartialEq<Event> for LogId {
    fn eq(&self, other: &Event) -> bool {
        *self == other.entry.id
    }
}

impl Drop for Event {
    /// Drops the [`LogIdEvent`].
    /// If the event was not created *silently*, it moves the entry into the [`LogIdMap`] associated with the event.
    fn drop(&mut self) {
        if self.crate_name.is_none() {
            return;
        }
        let hash = self.entry.hash;
        let crate_name = self.crate_name.unwrap();

        if let Err(err) = PUBLISHER.capturer.send(EventMsg {
            crate_name,
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

impl std::fmt::Debug for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LogId-Event")
            .field("id", &self.entry.id)
            .field("origin", &self.entry.origin)
            .field("hash", &self.entry.hash)
            .finish()
    }
}

impl Event {
    /// Returns the [`LogId`] of the [`MappedLogId`].
    pub fn id(&self) -> LogId {
        self.entry.id
    }

    pub fn entry(&self) -> &Entry {
        &self.entry
    }

    /// Add an info message for this log-id
    pub fn add_info(mut self, msg: &str) -> Self {
        tracing::info!("{}(addon): {}", self.entry.hash, msg);
        add_addon_to_entry(&mut self, EntryKind::Info(msg.to_owned()));
        self
    }

    /// Add a debug message for this log-id
    pub fn add_debug(mut self, msg: &str) -> Self {
        tracing::debug!("{}(addon): {}", self.entry.hash, msg);
        add_addon_to_entry(&mut self, EntryKind::Debug(msg.to_owned()));
        self
    }

    /// Add a trace message for this log-id
    pub fn add_trace(mut self, msg: &str) -> Self {
        tracing::trace!("{}(addon): {}", self.entry.hash, msg);
        add_addon_to_entry(&mut self, EntryKind::Trace(msg.to_owned()));
        self
    }

    /// Add a log-id entry that caused this log-id
    #[cfg(feature = "causes")]
    pub fn add_cause(mut self, entry: Entry) -> Self {
        tracing::info!("{}(cause): {}", self.entry.hash, entry);
        add_addon_to_entry(&mut self, EntryKind::Cause(entry));
        self
    }

    /// Add diagnostics for this log-id
    #[cfg(feature = "diagnostics")]
    pub fn add_diagnostic(mut self, diagnostic: Diagnostic) -> Self {
        tracing::trace!("{}(diag): {:?}", self.entry.hash, diagnostic);
        add_addon_to_entry(&mut self, EntryKind::Diagnostic(diagnostic));
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

fn add_addon_to_entry(id_event: &mut Event, kind: EntryKind) {
    // Note: Silent events are not published, so there is no need to store information either.
    if id_event.crate_name.is_none() {
        return;
    }

    match kind {
        EntryKind::Info(msg) => id_event.entry.infos.push(msg),
        EntryKind::Debug(msg) => id_event.entry.debugs.push(msg),
        EntryKind::Trace(msg) => id_event.entry.traces.push(msg),
        #[cfg(feature = "causes")]
        EntryKind::Cause(entry) => id_event.entry.causes.push(entry),
        #[cfg(feature = "diagnostics")]
        EntryKind::Diagnostic(diag) => id_event.entry.diagnostics.push(diag),
    }
}
