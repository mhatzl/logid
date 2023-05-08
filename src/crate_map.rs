use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use once_cell::sync::Lazy;

use crate::id_map::LogIdMap;

/// Map to capture [`LogId`]s per crate in separate [`LogIdMap`]s.
pub(crate) struct CratesMap {
    /// Map of [`LogIdMap`]s, where one [`LogIdMap`] collects all set [`LogId`](crate::log_id::LogId)s of one crate.
    /// The crate name is used as key.
    pub(crate) map: Arc<RwLock<HashMap<String, LogIdMap>>>,
    // pub(crate) map: RwLock<HashMap<String, LogIdMap>>,
}

/// Main map to capture all logs.
pub(crate) static CRATES_MAP: Lazy<CratesMap> = Lazy::new(CratesMap::new);

impl CratesMap {
    /// Create a new [`CratesMap`].
    pub(crate) fn new() -> Self {
        CratesMap {
            map: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}
