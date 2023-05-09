//! Offers functionality to set an event on a [`LogId`], and capture its content in a [`LogIdMap`].

use crate::{
    id_entry::{EntryKind, LogIdEntry},
    log_id::{EventLevel, LogId}, publisher::{PUBLISHER, EventMsg},
};

#[cfg(feature = "diagnostics")]
use crate::id_entry::Diagnostic;

/// Trait to use [`LogId`] for tracing.
pub trait LogIdTracing {
    /// Set an event for a [`LogId`], and storing it inside the [`LogIdMap`] of the given crate name.
    ///
    /// # Arguments
    ///
    /// * `crate_name` ... Name of the crate to identify the [`LogIdMap`]
    /// * `msg` ... Main message that is set for this log-id (should be a user-centered event description)
    /// * `filename` ... Name of the source file where the event is set (Note: use `file!()`)
    /// * `line_nr` ... Line number where the event is set (Note: use `line!()`)
    fn set_event(
        self,
        crate_name: &'static str,
        msg: &str,
        filename: &str,
        line_nr: u32,
    ) -> LogIdEvent;

    /// Set an event for a [`LogId`] **without** adding it to a [`LogIdMap`].
    ///
    /// # Arguments
    ///
    /// * `msg` ... Main message that is set for this event (should be a user-centered event description)
    /// * `filename` ... Name of the source file where the event is set (Note: use `file!()`)
    /// * `line_nr` ... Line number where the event is set (Note: use `line!()`)
    fn set_silent_event(self, msg: &str, filename: &str, line_nr: u32) -> LogIdEvent;
}

/// Macro to set an event for the given [`LogId`] using the caller crate to identify the [`LogIdMap`].\
/// The caller crate is identified using the environment variable `CARGO_PKG_NAME` set by cargo.
///
/// **Arguments:**
///
/// * `logid` ... Must be a valid `LogId`
/// * `msg` ... `String` variable or literal of the main message set for the event (should be a user-centered event description)
#[macro_export]
macro_rules! set_event {
    ($logid:ident, $msg:ident) => {
        $crate::capturing::LogIdTracing::set_event(
            $crate::logid!($logid),
            env!("CARGO_PKG_NAME"),
            $msg,
            file!(),
            line!(),
        )
    };
    ($logid:ident, $msg:literal) => {
        $crate::capturing::LogIdTracing::set_event(
            $crate::logid!($logid),
            env!("CARGO_PKG_NAME"),
            $msg,
            file!(),
            line!(),
        )
    };
    ($logid:ident, $msg:expr) => {
        $crate::capturing::LogIdTracing::set_event(
            $crate::logid!($logid),
            env!("CARGO_PKG_NAME"),
            $msg,
            file!(),
            line!(),
        )
    };
    ($logid:expr, $msg:expr) => {
        $crate::capturing::LogIdTracing::set_event(
            $crate::logid!($logid),
            env!("CARGO_PKG_NAME"),
            $msg,
            file!(),
            line!(),
        )
    };
}

/// Macro to set a silent log event.\
/// This is a convenient wrapper arounf [`LogIdTracing::set_silent_event`] that automatically converts the given [`LogId`].
///
/// **Arguments:**
///
/// * `logid` ... Must be a valid `LogId`
/// * `msg` ... `String` variable or literal of the main message set for the event
#[macro_export]
macro_rules! set_silent_event {
    ($logid:ident, $msg:ident) => {
        $crate::capturing::LogIdTracing::set_silent_event(
            $crate::logid!($logid),
            $msg,
            file!(),
            line!(),
        )
    };
    ($logid:ident, $msg:literal) => {
        $crate::capturing::LogIdTracing::set_silent_event(
            $crate::logid!($logid),
            $msg,
            file!(),
            line!(),
        )
    };
    ($logid:ident, $msg:expr) => {
        $crate::capturing::LogIdTracing::set_silent_event(
            $crate::logid!($logid),
            $msg,
            file!(),
            line!(),
        )
    };
    ($logid:expr, $msg:expr) => {
        $crate::capturing::LogIdTracing::set_silent_event(
            $crate::logid!($logid),
            $msg,
            file!(),
            line!(),
        )
    };
}

/// Traces a [`LogIdEntry`] creation.
fn trace_entry_creation(id: LogId, msg: &str, filename: &str, line_nr: u32) -> LogIdEntry {
    let id_entry = LogIdEntry::new(id, msg, filename, line_nr);

    // Note: It is not possible to set `target` via parameter, because it requires `const`
    // Same goes for `level` for the `event` macro => match and code duplication needed
    match id_entry.level {
        EventLevel::Error => tracing::error!("{}: {}", id, msg),
        EventLevel::Warn => tracing::warn!("{}: {}", id, msg),
        EventLevel::Info => tracing::info!("{}: {}", id, msg),
        EventLevel::Debug => tracing::debug!("{}: {}", id, msg),
    }

    tracing::trace!("{}(origin): {}", id, String::from(&id_entry.origin));

    id_entry
}

impl LogIdTracing for LogId {
    fn set_event(
        self,
        crate_name: &'static str,
        msg: &str,
        filename: &str,
        line_nr: u32,
    ) -> LogIdEvent {
        let entry = trace_entry_creation(self, msg, filename, line_nr);

        LogIdEvent {
            entry,
            crate_name: Some(crate_name),
        }
    }

    fn set_silent_event(self, msg: &str, filename: &str, line_nr: u32) -> LogIdEvent {
        let entry = trace_entry_creation(self, msg, filename, line_nr);

        LogIdEvent {
            entry,
            crate_name: None,
        }
    }
}

/// Struct linking a [`LogId`] to the map the entry for the ID was added to.
#[derive(Default, Clone)]
pub struct LogIdEvent {
    /// Crate name identifying the [`LogIdMap`] the [`LogIdEvent`] is associated with, or none for silent events.
    crate_name: Option<&'static str>,
    /// [`LogIdEntry`] for the [`LogIdEvent`] storing all event information.
    pub(crate) entry: LogIdEntry,
}

impl From<LogIdEvent> for LogId {
    fn from(mapped_id: LogIdEvent) -> Self {
        mapped_id.finalize()
    }
}

impl PartialEq<LogId> for LogIdEvent {
    fn eq(&self, other: &LogId) -> bool {
        self.entry.id == *other
    }
}

impl PartialEq<LogIdEvent> for LogId {
    fn eq(&self, other: &LogIdEvent) -> bool {
        *self == other.entry.id
    }
}

impl Drop for LogIdEvent {
    /// Drops the [`LogIdEvent`].
    /// If the event was not created *silently*, it moves the entry into the [`LogIdMap`] associated with the event.
    fn drop(&mut self) {
        if self.crate_name.is_none() {
            return;
        }
        let id = self.entry.id;
        let crate_name = self.crate_name.unwrap();
        if let Err(err) = PUBLISHER.sender.send(EventMsg {
            crate_name,
            entry: std::mem::take(&mut self.entry),
        }) {
            tracing::error!("{}(send): {}", id, "Failed sending log-id to central map.");
            tracing::debug!("{}(send-cause): {}", id, err);
        }
    }
}

impl std::fmt::Debug for LogIdEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LogIdEvent")
            .field("id", &self.entry.id)
            .field("origin", &self.entry.origin)
            .finish()
    }
}

impl LogIdEvent {
    /// Returns the [`LogId`] of the [`MappedLogId`].
    pub fn id(&self) -> LogId {
        self.entry.id
    }

    /// Add an info message for this log-id
    pub fn add_info(mut self, msg: &str) -> Self {
        tracing::info!("{}(addon): {}", self.entry.id, msg);
        add_addon_to_entry(&mut self, EntryKind::Info(msg.to_owned()));
        self
    }

    /// Add a debug message for this log-id
    pub fn add_debug(mut self, msg: &str) -> Self {
        tracing::debug!("{}(addon): {}", self.entry.id, msg);
        add_addon_to_entry(&mut self, EntryKind::Debug(msg.to_owned()));
        self
    }

    /// Add a trace message for this log-id
    pub fn add_trace(mut self, msg: &str) -> Self {
        tracing::trace!("{}(addon): {}", self.entry.id, msg);
        add_addon_to_entry(&mut self, EntryKind::Trace(msg.to_owned()));
        self
    }

    /// Add a message describing the cause for this log-id
    #[cfg(feature = "causes")]
    pub fn add_cause(mut self, msg: &str) -> Self {
        tracing::info!("{}(cause): {}", self.entry.id, msg);
        add_addon_to_entry(&mut self, EntryKind::Cause(msg.to_owned()));
        self
    }

    /// Add diagnostics for this log-id
    #[cfg(feature = "diagnostics")]
    pub fn add_diagnostic(mut self, diagnostic: Diagnostic) -> Self {
        tracing::trace!("{}(diag): {:?}", self.entry.id, diagnostic);
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

fn add_addon_to_entry(id_event: &mut LogIdEvent, kind: EntryKind) {
    // Note: Silent events cannot move entries to a LogIdMap, so there is no need to store it either.
    if id_event.crate_name.is_none() {
        return;
    }

    match kind {
        EntryKind::Info(msg) => id_event.entry.infos.push(msg),
        EntryKind::Debug(msg) => id_event.entry.debugs.push(msg),
        EntryKind::Trace(msg) => id_event.entry.traces.push(msg),
        EntryKind::Cause(msg) => id_event.entry.causes.push(msg),
        EntryKind::Diagnostic(diag) => id_event.entry.diagnostics.push(diag),
    }
}
