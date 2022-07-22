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
    map: Arc<RwLock<HashMap<LogId, Vec<LogIdEntry>>>>,
    last_log_id: RwLock<LogId>,
}

lazy_static! {
    pub static ref LOG_ID_MAP: LogIdMap = LogIdMap {
        map: Arc::new(RwLock::new(HashMap::new())),
        last_log_id: RwLock::new(INVALID_LOG_ID),
    };
}

impl LogIdMap {
    pub fn get_last_log_id(&self) -> LogId {
        *self.last_log_id.read().unwrap()
    }

    pub fn drain_map(&mut self) -> HashMap<LogId, Vec<LogIdEntry>> {
        let map = &mut *self.map.write().unwrap();
        map.drain().collect()
    }
}
