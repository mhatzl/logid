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
pub fn drain_map() -> HashMap<LogId, Vec<LogIdEntry>> {
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
        *self.last_log_id.read().unwrap()
    }

    /// Drain this [`LogIdMap`]. Returning all captured [`LogId`]s of the map so far.
    pub fn drain_map(&self) -> HashMap<LogId, Vec<LogIdEntry>> {
        //Note: Due to RWLock, mutable access to map is fine
        let map = &mut *self.map.write().unwrap();
        map.drain().collect()
    }
}
