use std::{
    collections::HashMap,
    sync::RwLock,
};

use lazy_static::lazy_static;

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

lazy_static! {
    /// Global [`LogIdMap`] that may be used to capture all [`LogId`]s in one place.
    pub(crate) static ref LOG_ID_MAP: LogIdMap = LogIdMap::new();
}

impl Default for LogIdMap {
    fn default() -> Self {
        Self::new()
    }
}

/// Drain global [`LogIdMap`]. Returning all captured [`LogId`]s of the map so far.
pub fn drain_map() -> HashMap<LogId, Vec<LogIdEntry>> {
    (*LOG_ID_MAP).drain_map()
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
        let map = &mut *self.map.write().unwrap();
        map.drain().collect()
    }
}
