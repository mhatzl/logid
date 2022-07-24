//! Library providing functionalities to set and capture [`log_id::LogId`]s.
//!
//! **Usage:**
//!
//! ~~~
//! use logid::{log_id::{LogId, EventLevel, get_log_id}, capturing::LogIdTracing};
//!
//! const SOME_ERROR: LogId = get_log_id(0, 0, EventLevel::Error, 0);
//!
//! fn my_func() -> Result<usize, LogId> {
//!   // some code ...
//!
//!   // on error
//!   Err(SOME_ERROR.set_event("Some error message", file!(), line!())
//!       .add_cause("Cause of error -> unknown").finalize()  
//!   )
//! }
//! ~~~

pub mod capturing;
pub mod id_entry;
pub mod id_map;
pub mod log_id;
