use std::{collections::HashMap, sync::RwLock};

use once_cell::sync::Lazy;

use crate::{
    id_entry::LogIdEntry,
    log_id::{LogId, INVALID_LOG_ID},
};

/// Map to capture [`LogId`]s, and combine all informations set
/// for  a [`LogId`] inside a [`LogIdEntry`].
pub struct LogIdMap {
    pub(crate) map: RwLock<HashMap<LogId, Vec<LogIdEntry>>>,
    last_log_id: RwLock<LogId>,
}

pub static LOG_ID_MAP: Lazy<LogIdMap> = Lazy::new(LogIdMap::new);

impl Default for LogIdMap {
    fn default() -> Self {
        Self::new()
    }
}

/// Drain global [`LogIdMap`]. Returning all captured [`LogId`]s of the map so far.
pub fn drain_map() -> Option<HashMap<LogId, Vec<LogIdEntry>>> {
    LOG_ID_MAP.drain_map()
}

impl LogIdMap {
    /// Create a new [`LogIdMap`].
    pub fn new() -> Self {
        LogIdMap {
            map: RwLock::new(HashMap::new()),
            last_log_id: RwLock::new(INVALID_LOG_ID),
        }
    }

    pub fn get_last_log_id(&self) -> LogId {
        match self.last_log_id.read() {
            Ok(last) => *last,
            Err(_) => INVALID_LOG_ID,
        }
    }

    /// Drain this [`LogIdMap`]. Returning all captured [`LogId`]s of the map so far.
    pub fn drain_map(&self) -> Option<HashMap<LogId, Vec<LogIdEntry>>> {
        //Note: Due to RWLock, mutable access to map is fine
        match self.map.write() {
            Ok(mut map) => {
                return Some((*map).drain().collect());
            }
            Err(_) => None,
        }

        // TODO: Replace drain by `drain_filter` to exclude `LogId`s not marked as `drainable`.
        // Needs feature https://github.com/rust-lang/rust/issues/59618
        //
        // map.drain_filter(|_id, entries| {
        //     for entry in entries {
        //         return !entry.drainable();
        //     }
        //     true
        //     }
        // );
    }

    /// Returns all captured entries for the given [`LogId`]
    /// if all entries are safe to drain.
    ///
    /// # Arguments
    ///
    /// * `id` - the [`LogId`] used to search for map entries
    pub fn get_entries_safe(&self, id: LogId) -> Option<Vec<LogIdEntry>> {
        match self.map.read() {
            Ok(map) => match (*map).get(&id) {
                Some(entries) => {
                    if entries.iter().all(|entry| entry.drainable()) {
                        Some(entries.clone())
                    } else {
                        None
                    }
                }
                None => None,
            },
            Err(_) => None,
        }
    }

    /// Drains all captured entries for the given [`LogId`]
    /// if all entries are safe to drain.
    ///
    /// # Arguments
    ///
    /// * `id` - the [`LogId`] used to search for map entries
    pub fn drain_entries_safe(&self, id: LogId) -> Option<Vec<LogIdEntry>> {
        match self.map.write() {
            Ok(mut map) => match (*map).remove(&id) {
                Some(entries) => {
                    if entries.iter().all(|entry| entry.drainable()) {
                        Some(entries)
                    } else {
                        map.insert(id, entries);
                        None
                    }
                }
                None => None,
            },
            Err(_) => None,
        }
    }
}
