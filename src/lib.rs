//! Library providing functionalities to set and capture [`log_id::LogId`]s.
//!
//! **Usage:**
//!
//! ~~~
//! use once_cell::sync::Lazy;
//! use logid::{log_id::{LogId, EventLevel, get_log_id}, id_map::LogIdMap, capturing::LogIdTracing};
//!
//! const SOME_ERROR: LogId = get_log_id(0, 0, EventLevel::Error, 0);
//!
//! static GLOBAL_LOG_MAP: Lazy<LogIdMap> = Lazy::new(LogIdMap::new);
//! logid::setup_logid_map!(&GLOBAL_LOG_MAP);
//!
//! fn my_func() -> Result<usize, LogId> {
//!   // some code ...
//!
//!   // on error
//!   Err(set_event!(SOME_ERROR, "Some error message")
//!       .add_debug("Add debug information").into()
//!   )
//! }
//! ~~~

pub mod capturing;
pub mod id_entry;
pub mod id_map;
pub mod log_id;
