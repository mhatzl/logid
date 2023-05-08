//! Contains the [`LogIdMap`] definition used to capture [`LogId`]s and their [`LogIdEntry`]s.

use std::{
    collections::{HashMap, HashSet},
    sync::{
        atomic::{AtomicIsize, Ordering},
        Arc, RwLock,
    },
};

use crate::{
    crate_map::CRATES_MAP,
    id_entry::LogIdEntry,
    log_id::{LogId, INVALID_LOG_ID},
};

/// Map to capture [`LogId`]s, and combine all informations set
/// for  a [`LogId`] inside a [`LogIdEntry`].
pub(crate) struct LogIdMap {
    /// Map to capture entries for set [`LogId`]s.
    /// Multiple entries per [`LogId`] might be possible
    /// if the [`LogId`] is used at multiple positions.
    pub(crate) map: Arc<RwLock<HashMap<LogId, HashSet<LogIdEntry>>>>,
    /// Used to keep track of the last [`LogId`] that was
    /// entered in the map, and got marked as `drainable`.
    pub(crate) last_finalized_id: Arc<AtomicIsize>,
}

/// Returns the last [`LogId`] that was
/// entered and marked as `drainable` in the [`LogIdMap`] identified by the given crate name.
///
/// ## Arguments
///
/// * `crate_name` ... The name of the crate the drained [`LogIdMap`] is assigned to
///
/// ## Returns
///
/// Returns the last [`LogId`] that was marked `drainable`, or `None` if no [`LogIdMap`] was found
/// for the given crate name, or if no [`LogId`] is `drainable`.
pub fn get_last_finalized_id(crate_name: &str) -> Option<LogId> {
    match CRATES_MAP.map.read().ok()?.get(crate_name) {
        Some(map) => map.get_last_finalized_id(),
        None => None,
    }
}

/// Wrapper around [`get_last_finalized_id`] using the crate name of the caller.
#[macro_export]
macro_rules! get_last_finalized_id {
    () => {
        $crate::id_map::get_last_finalized_id(env!("CARGO_PKG_NAME"))
    };
}

/// Drain the [`LogIdMap`] of the given crate name.
///
/// ## Arguments
///
/// * `crate_name` ... The name of the crate the drained [`LogIdMap`] is assigned to
///
/// ## Returns
///
/// Returning all `drainable` entries of all captured [`LogId`]s of the map so far.
pub fn drain_map(crate_name: &str) -> Option<HashMap<LogId, HashSet<LogIdEntry>>> {
    match CRATES_MAP.map.write().ok()?.get_mut(crate_name) {
        Some(map) => map.drain_map(),
        None => None,
    }
}

/// Wrapper around [`drain_map`] using the crate name of the caller.
#[macro_export]
macro_rules! drain_map {
    () => {
        $crate::id_map::drain_map(env!("CARGO_PKG_NAME"))
    };
}

/// Returns all captured [`LogIdEntry`]s marked as `drainable` for the given [`LogId`].
///
/// # Arguments
///
/// * `crate_name` ... The name of the crate the drained [`LogIdMap`] is assigned to
/// * `id` ... The [`LogId`] used to search for log entries
///
/// ## Returns
///
/// Returns all captured [`LogIdEntry`]s marked as `drainable` for the given [`LogId`].
/// If no entries are `drainable` or the [`LogIdMap`] could not be accessed, `None` is returned.
pub fn get_entries(crate_name: &str, id: LogId) -> Option<HashSet<LogIdEntry>> {
    match CRATES_MAP.map.read().ok()?.get(crate_name) {
        Some(map) => map.get_entries(id),
        None => None,
    }
}

/// Wrapper around [`get_entries`] using the crate name of the caller.
#[macro_export]
macro_rules! get_entries {
    ($logid:ident) => {
        $crate::id_map::get_entries(env!("CARGO_PKG_NAME"), $logid)
    };
    ($logid:expr) => {
        $crate::id_map::get_entries(env!("CARGO_PKG_NAME"), $logid)
    };
}

/// Drains all captured [`LogIdEntry`]s marked as `drainable` for the given [`LogId`].
///
/// # Arguments
///
/// * `crate_name` ... The name of the crate the drained [`LogIdMap`] is assigned to
/// * `id` ... The [`LogId`] used to search for log entries
///
/// ## Returns
///
/// Returns all captured [`LogIdEntry`]s marked as `drainable` for the given [`LogId`].
/// If no entries are `drainable` or the [`LogIdMap`] could not be accessed, `None` is returned.
pub fn drain_entries(crate_name: &str, id: LogId) -> Option<HashSet<LogIdEntry>> {
    match CRATES_MAP.map.read().ok()?.get(crate_name) {
        Some(map) => map.drain_entries(id),
        None => None,
    }
}

/// Wrapper around [`drain_entries`] using the crate name of the caller.
#[macro_export]
macro_rules! drain_entries {
    ($logid:ident) => {
        $crate::id_map::drain_entries(env!("CARGO_PKG_NAME"), $logid)
    };
    ($logid:expr) => {
        $crate::id_map::drain_entries(env!("CARGO_PKG_NAME"), $logid)
    };
}

impl Default for LogIdMap {
    fn default() -> Self {
        Self::new()
    }
}

impl LogIdMap {
    /// Create a new [`LogIdMap`].
    pub fn new() -> Self {
        LogIdMap {
            map: Arc::new(RwLock::new(HashMap::new())),
            last_finalized_id: Arc::new(AtomicIsize::new(INVALID_LOG_ID)),
        }
    }

    pub fn new_with<I>(values: I) -> Self
    where
        I: Iterator<Item = (LogId, HashSet<LogIdEntry>)>,
    {
        LogIdMap {
            map: Arc::new(RwLock::new(HashMap::from_iter(values))),
            last_finalized_id: Arc::new(AtomicIsize::new(INVALID_LOG_ID)),
        }
    }

    /// Returns the last [`LogId`] that was
    /// entered in the map, and got marked as `drainable`,
    /// or returns `None` if no [`LogId`] is `drainable`.
    pub fn get_last_finalized_id(&self) -> Option<LogId> {
        let id: LogId = self.last_finalized_id.load(Ordering::Relaxed);
        if id == INVALID_LOG_ID {
            None
        } else {
            Some(id)
        }
    }

    /// Drain this [`LogIdMap`].
    pub fn drain_map(&self) -> Option<HashMap<LogId, HashSet<LogIdEntry>>> {
        self.last_finalized_id
            .store(INVALID_LOG_ID, Ordering::Relaxed);

        if self.map.read().ok()?.is_empty() {
            return None;
        }
        // Note: Since map is behind a RwLock, write access is safe even though we do not require *mut* for self.
        match self.map.write() {
            Ok(mut locked_map) => {
                // let mut map: HashMap<LogId, HashSet<LogIdEntry>> = HashMap::new();
                // locked_map.alter_all(|id, entry| {
                //     map.insert(*id, entry);
                //     HashSet::new()
                // });
                // locked_map.clear();

                Some(locked_map.drain().collect())
            }
            Err(_) => None,
        }
    }

    /// Returns all captured entries for the given [`LogId`].
    ///
    /// # Arguments
    ///
    /// * `id` ... The [`LogId`] used to search for entries
    pub fn get_entries(&self, id: LogId) -> Option<HashSet<LogIdEntry>> {
        self.map.read().ok()?.get(&id).map(|entry| entry.to_owned())
    }

    /// Drains all captured entries for the given [`LogId`].
    ///
    /// # Arguments
    ///
    /// * `id` ... The [`LogId`] used to search for entries
    pub fn drain_entries(&self, id: LogId) -> Option<HashSet<LogIdEntry>> {
        if Some(id) == self.get_last_finalized_id() {
            self.last_finalized_id
                .store(INVALID_LOG_ID, Ordering::Relaxed);
        }

        self.map.write().ok()?.remove(&id)
    }
}
