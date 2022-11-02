//! Offers functionality to set an event on a [`LogId`] and capture its content in a [`LogIdMap`].

use crate::{
    id_entry::{LogIdEntry, Origin},
    id_map::{LogIdEntrySet, LogIdMap, LOG_ID_MAP},
    log_id::{EventLevel, LogId},
};

#[cfg(feature = "diagnostics")]
use crate::id_entry::Diagnostic;

/// Trait to use [`LogId`] for tracing.
pub trait LogIdTracing {
    /// Set an event for a [`LogId`] using the global [`LogIdMap`] reference [`LOG_ID_MAP`].
    ///
    /// # Arguments
    ///
    /// * `msg` - main message that is set for this log-id (should be a user-centered event description)
    /// * `filename` - name of the source file where the event is set (Note: use `file!()`)
    /// * `line_nr` - line number where the event is set (Note: use `line!()`)
    fn set_event(self, msg: &str, filename: &str, line_nr: u32) -> MappedLogId;

    /// Set an event for a [`LogId`] using a given [`LogIdMap`].
    ///
    /// # Arguments
    ///
    /// * `log_map` - the map the log-id and all its addons are captured in
    /// * `msg` - main message that is set for this log-id (should be a user-centered event description)
    /// * `filename` - name of the source file where the event is set (Note: use `file!()`)
    /// * `line_nr` - line number where the event is set (Note: use `line!()`)
    fn set_event_with(
        self,
        log_map: &'static LogIdMap,
        msg: &str,
        filename: &str,
        line_nr: u32,
    ) -> MappedLogId;

    /// Set an event for a [`LogId`] **without** adding it to a [`LogIdMap`].
    ///
    /// # Arguments
    ///
    /// * `msg` - main message that is set for this log-id (should be a user-centered event description)
    /// * `filename` - name of the source file where the event is set (Note: use `file!()`)
    /// * `line_nr` - line number where the event is set (Note: use `line!()`)
    fn set_silent_event(self, msg: &str, filename: &str, line_nr: u32) -> MappedLogId;
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
    fn set_event(self, msg: &str, filename: &str, line_nr: u32) -> MappedLogId {
        self.set_event_with(&*LOG_ID_MAP, msg, filename, line_nr)
    }

    fn set_event_with(
        self,
        log_map: &'static LogIdMap,
        msg: &str,
        filename: &str,
        line_nr: u32,
    ) -> MappedLogId {
        let entry = trace_entry_creation(self, msg, filename, line_nr);
        let hash = entry.hash;
        let origin = entry.origin.clone();

        let update_map = log_map.map.write();
        if let Ok(mut map) = update_map {
            match map.get_mut(&self) {
                Some(entries) => {
                    entries.insert(entry);
                }
                None => {
                    map.insert(self, [entry].into());
                }
            };
        }

        MappedLogId {
            hash,
            id: self,
            origin,
            map: Some(log_map),
        }
    }

    fn set_silent_event(self, msg: &str, filename: &str, line_nr: u32) -> MappedLogId {
        let id_entry = trace_entry_creation(self, msg, filename, line_nr);

        MappedLogId {
            hash: id_entry.hash,
            id: id_entry.id,
            origin: id_entry.origin,
            map: None,
        }
    }
}

/// Struct linking a [`LogId`] to the map the entry for the ID was added to.
#[derive(Clone)]
pub struct MappedLogId {
    /// Hash to identify the exact [`LogIdEntry`] this [`MappedLogId`] is mapped to in the linked [`LogIdMap`]
    pub(crate) hash: u64,
    /// [`LogId`] of this [`MappedLogId`]
    id: LogId,
    /// [`Origin`] of this [`MappedLogId`]
    origin: Origin,
    /// [`LogIdMap`] this [`MappedLogId`] is mapped to, or none for silent events
    map: Option<&'static LogIdMap>,
}

impl PartialEq<LogId> for MappedLogId {
    fn eq(&self, other: &LogId) -> bool {
        self.id == *other
    }
}

impl PartialEq<MappedLogId> for LogId {
    fn eq(&self, other: &MappedLogId) -> bool {
        *self == other.id
    }
}

impl Drop for MappedLogId {
    /// [`MappedLogId`] is finalized on drop.
    fn drop(&mut self) {
        self.finalize();
    }
}

impl std::fmt::Debug for MappedLogId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MappedLogId")
            .field("id", &self.id)
            .field("origin", &self.origin)
            .finish()
    }
}

impl MappedLogId {
    /// Returns the [`LogId`] of the [`MappedLogId`].
    pub fn id(&self) -> LogId {
        self.id
    }

    /// Add a message describing the cause for this log-id
    #[cfg(feature = "causes")]
    pub fn add_cause(self, msg: &str) -> Self {
        tracing::info!("{}(cause): {}", self.id, msg);

        if let Some(log_map) = self.map {
            let update_map = log_map.map.write();
            if let Ok(mut map) = update_map {
                match map.get_mut(&self.id) {
                    Some(entries) => {
                        if let Some(mut entry) = entries.take_logid(&self) {
                            entry.add_cause(msg.to_string());
                            entries.insert(entry);
                        };
                    }
                    None => {
                        tracing::warn!(
                            "Got cause=\"{}\" for log-id={}, but no base for log-id was set!",
                            msg,
                            self.id
                        )
                    }
                };
            }
        }

        self
    }

    /// Add an info message for this log-id
    pub fn add_info(self, msg: &str) -> Self {
        tracing::info!("{}(addon): {}", self.id, msg);
        add_addon_to_map(&self, msg, &tracing::Level::INFO);
        self
    }

    /// Add a debug message for this log-id
    pub fn add_debug(self, msg: &str) -> Self {
        tracing::debug!("{}(addon): {}", self.id, msg);
        add_addon_to_map(&self, msg, &tracing::Level::DEBUG);
        self
    }

    /// Add a trace message for this log-id
    pub fn add_trace(self, msg: &str) -> Self {
        tracing::trace!("{}(addon): {}", self.id, msg);
        add_addon_to_map(&self, msg, &tracing::Level::TRACE);
        self
    }

    /// Finalizing a [`MappedLogId`] marks the map entry that
    /// no more information will be added to it.
    ///
    /// Besides the [`LogId`], also the [`Origin`] of the [`LogIdEntry`] is compared for identification.
    pub fn finalize(&self) -> LogId {
        let mut finalized = false;
        if let Some(log_map) = self.map {
            if let Ok(mut map) = log_map.map.write() {
                if let Some(entries) = map.get_mut(&self.id) {
                    if let Some(mut entry) = entries.take_logid(self) {
                        entry.finalize();
                        entries.insert(entry);
                        finalized = true;
                    };
                }
            }
            // flag used to shorten access to write-lock
            if finalized {
                if let Ok(mut last_id) = log_map.last_finalized_id.write() {
                    *last_id = self.id;
                }
            }
        }

        self.id
    }

    #[cfg(feature = "diagnostics")]
    pub fn add_diagnostic(self, diagnostic: Diagnostic) -> Self {
        tracing::trace!("{}(diag): {:?}", self.id, diagnostic);

        if let Some(log_map) = self.map {
            let update_map = log_map.map.write();
            if let Ok(mut map) = update_map {
                match map.get_mut(&self.id) {
                    Some(entries) => {
                        if let Some(mut entry) = entries.take_logid(&self) {
                            entry.add_diagnostic(diagnostic);
                            entries.insert(entry);
                        };
                    }
                    None => {
                        tracing::warn!(
                            "Got diagnostic=\"{:?}\" for log-id={}, but no base for log-id was set!",
                            diagnostic,
                            self.id
                        )
                    }
                };
            }
        }

        self
    }
}

fn add_addon_to_map(mapped_id: &MappedLogId, msg: &str, level: &tracing::Level) {
    if let Some(log_map) = mapped_id.map {
        let update_map = log_map.map.write();
        if let Ok(mut map) = update_map {
            match map.get_mut(&mapped_id.id) {
                Some(entries) => {
                    if let Some(mut entry) = entries.take_logid(mapped_id) {
                        entry.add_addon(level, msg.to_string());
                        entries.insert(entry);
                    };
                }
                None => {
                    tracing::warn!(
                        "Got addon=\"{}\" for log-id={}, but no base for log-id was set!",
                        msg,
                        mapped_id.id
                    )
                }
            };
        }
    }
}
