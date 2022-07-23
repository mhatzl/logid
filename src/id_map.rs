use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use lazy_static::lazy_static;

use crate::{
    id_entry::LogIdEntry,
    log_id::{LogId, INVALID_LOG_ID},
};

pub struct LogIdMap {
    pub(crate) map: Arc<RwLock<HashMap<LogId, Vec<LogIdEntry>>>>,
    last_log_id: RwLock<LogId>,
}

lazy_static! {
    pub(crate) static ref LOG_ID_MAP: LogIdMap = LogIdMap::new();
}

impl Default for LogIdMap {
    fn default() -> Self {
        Self::new()
    }
}

pub fn drain_map() -> HashMap<LogId, Vec<LogIdEntry>> {
    let map = &mut *LOG_ID_MAP.map.write().unwrap();
    map.drain().collect()
}

impl LogIdMap {
    pub fn new() -> Self {
        LogIdMap {
            map: Arc::new(RwLock::new(HashMap::new())),
            last_log_id: RwLock::new(INVALID_LOG_ID),
        }
    }

    pub fn get_last_log_id(&self) -> LogId {
        *self.last_log_id.read().unwrap()
    }
}
