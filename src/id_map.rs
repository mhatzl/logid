//! Contains the [`LogIdMap`] definition used to capture [`LogId`]s and their [`LogIdEntry`]s.

use std::{
    collections::{HashMap, HashSet},
    sync::{RwLock, RwLockWriteGuard},
};

use once_cell::sync::Lazy;

use crate::{
    id_entry::LogIdEntry,
    log_id::{LogId, INVALID_LOG_ID}, capturing::MappedLogId,
};

/// Map to capture [`LogId`]s, and combine all informations set
/// for  a [`LogId`] inside a [`LogIdEntry`].
pub struct LogIdMap {
    /// Map to capture entries for set [`LogId`]s.
    /// Multiple entries per [`LogId`] might be possible
    /// if the [`LogId`] is used at multiple positions.
    pub(crate) map: RwLock<HashMap<LogId, HashSet<LogIdEntry>>>,
    /// Used to keep track of the last [`LogId`] that was
    /// entered in the map, and got marked as `drainable`.
    pub(crate) last_finalized_id: RwLock<LogId>,
}

pub(crate) static LOG_ID_MAP: Lazy<LogIdMap> = Lazy::new(LogIdMap::new);

impl Default for LogIdMap {
    fn default() -> Self {
        Self::new()
    }
}

/// Drain global [`LogIdMap`].  Returning all `drainable` entries of all captured [`LogId`]s of the map so far.
pub fn drain_map() -> Option<HashMap<LogId, HashSet<LogIdEntry>>> {
    LOG_ID_MAP.drain_map()
}

/// Trait used to implement functions on the [`LogIdEntry`] HashSet.
pub trait LogIdEntrySet {
    /// Tries to get a [`LogIdEntry`] that is identified by the given [`MappedLogId`].
    /// 
    /// # Arguments
    /// 
    /// - `mapped_id` - [`MappedLogId`] used to identify the [`LogIdEntry`]
    fn get_logid(&self, mapped_id: &MappedLogId) -> Option<&LogIdEntry>;

    /// Tries to retrieve a [`LogIdEntry`] that is identified by the given [`MappedLogId`].
    /// 
    /// # Arguments
    /// 
    /// - `mapped_id` - [`MappedLogId`] used to identify the [`LogIdEntry`]
    fn take_logid(&mut self, mapped_id: &MappedLogId) -> Option<LogIdEntry>;
}

impl LogIdEntrySet for HashSet<LogIdEntry> {
    fn get_logid(&self, mapped_id: &MappedLogId) -> Option<&LogIdEntry> {
        self.get(&LogIdEntry::shallow_new(mapped_id.hash))
    }

    fn take_logid(&mut self, mapped_id: &MappedLogId) -> Option<LogIdEntry> {
        self.take(&LogIdEntry::shallow_new(mapped_id.hash))
    }
}

impl LogIdMap {
    /// Create a new [`LogIdMap`].
    pub fn new() -> Self {
        LogIdMap {
            map: RwLock::new(HashMap::new()),
            last_finalized_id: RwLock::new(INVALID_LOG_ID),
        }
    }

    /// Returns the last [`LogId`] that was
    /// entered in the map, and got marked as `drainable`.
    pub fn get_last_finalized_id(&self) -> LogId {
        match self.last_finalized_id.read() {
            Ok(last) => *last,
            Err(_) => INVALID_LOG_ID,
        }
    }

    /// Drain this [`LogIdMap`]. Returning all `drainable` entries of all captured [`LogId`]s of the map so far.
    pub fn drain_map(&self) -> Option<HashMap<LogId, HashSet<LogIdEntry>>> {
        if let Ok(mut last) = self.last_finalized_id.write() {
            *last = INVALID_LOG_ID;
        }

        //Note: Due to RWLock, mutable access to map is fine
        match self.map.write() {
            Ok(mut map) => {
                let mut safe_map = HashMap::new();
                let mut keys = Vec::new();
                for key in (*map).keys() {
                    keys.push(*key);
                }

                for id in keys {
                    let drain_res = drain_entries(&mut map, id);
                    if let Some(entries) = drain_res.0 {
                        safe_map.insert(id, entries);
                    }
                }

                if safe_map.is_empty() {
                    None
                } else {
                    Some(safe_map)
                }
            }
            Err(_) => None,
        }
    }

    /// Returns all captured entries marked as `drainable` for the given [`LogId`].
    ///
    /// # Arguments
    ///
    /// * `id` - the [`LogId`] used to search for map entries
    pub fn get_entries(&self, id: LogId) -> Option<HashSet<LogIdEntry>> {
        match self.map.read() {
            Ok(map) => match (*map).get(&id) {
                Some(entries) => {
                    let mut safe_entries: HashSet<LogIdEntry> = HashSet::new();
                    for entry in entries {
                        if entry.drainable() {
                            safe_entries.insert((*entry).clone());
                        }
                    }
                    if safe_entries.is_empty() {
                        None
                    } else {
                        Some(safe_entries)
                    }
                }
                None => None,
            },
            Err(_) => None,
        }
    }

    /// Returns all captured entries for the given [`LogId`].
    /// Entries must not be marked as `drainable`.
    /// Therefore, not all information might have been captured for an entry.
    ///
    /// # Arguments
    ///
    /// * `id` - the [`LogId`] used to search for map entries
    pub fn get_entries_unsafe(&self, id: LogId) -> Option<HashSet<LogIdEntry>> {
        match self.map.read() {
            Ok(map) => match (*map).get(&id) {
                Some(entries) => {
                    let mut safe_entries: HashSet<LogIdEntry> = HashSet::new();
                    for entry in entries {
                        safe_entries.insert((*entry).clone());
                    }
                    if safe_entries.is_empty() {
                        None
                    } else {
                        Some(safe_entries)
                    }
                }
                None => None,
            },
            Err(_) => None,
        }
    }

    /// Drains all captured entries marked as `drainable` for the given [`LogId`].
    /// Non-drainable entries remain in the map.
    ///
    /// # Arguments
    ///
    /// * `id` - the [`LogId`] used to search for map entries
    pub fn drain_entries(&self, id: LogId) -> Option<HashSet<LogIdEntry>> {
        let mut drained = false;
        let res = match self.map.write() {
            Ok(mut map) => {
                let drain_res = drain_entries(&mut map, id);
                drained = drain_res.1;
                drain_res.0
            }
            Err(_) => None,
        };

        if drained {
            if let Ok(mut last) = self.last_finalized_id.write() {
                if *last == id {
                    *last = INVALID_LOG_ID;
                }
            }
        }

        res
    }
}

/// Function to drain `drainable` map entries using an aquired write-lock.
/// 
/// Returns set of drained entries, and `true` if all entries were drained.
fn drain_entries(
    write_lock_map: &mut RwLockWriteGuard<HashMap<LogId, HashSet<LogIdEntry>>>,
    id: LogId,
) -> (Option<HashSet<LogIdEntry>>, bool) {
    let mut drained = true;
    match (*write_lock_map).remove(&id) {
        Some(mut entries) => {
            // TODO: Wait until unstable feature is supported
            //let drained_entries = entries.drain_filter(|entry| entry.drainable()).collect();

            let mut safe_entries = HashSet::new();
            let tmp_entries : HashSet<LogIdEntry> = entries.drain().collect();
            for entry in tmp_entries {
                if entry.drainable() {
                    safe_entries.insert(entry);
                    
                } else {
                    entries.insert(entry);
                }
            }

            if !entries.is_empty() {
                write_lock_map.insert(id, entries);
                drained = false;
            }
            if safe_entries.is_empty() {
                (None, drained)
            } else {
                (Some(safe_entries), drained)
            }
        }
        None => (None, drained),
    }
}
