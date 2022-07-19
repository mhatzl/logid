use std::{sync::{RwLock, Arc}, collections::HashMap};

use lazy_static::lazy_static;

use crate::{log_id::{LogId, INVALID_LOG_ID}, id_entry::LogIdEntry};


pub struct LogIdMap {
  map: Arc<RwLock<HashMap<LogId, LogIdEntry>>>,
  last_log_id: RwLock<LogId>, 
}

lazy_static!{
  pub static ref LOG_ID_MAP: LogIdMap = LogIdMap{
    map: Arc::new(RwLock::new(HashMap::new())),
    last_log_id: RwLock::new(INVALID_LOG_ID),
  };  
}

impl LogIdMap {

  pub fn get_last_log_id(&self) -> LogId {
    *self.last_log_id.read().unwrap()
  }

  pub fn drain_map(&mut self) -> HashMap<LogId, LogIdEntry> {
    let map = &mut *self.map.write().unwrap();
    map.drain().collect()
  }

}
