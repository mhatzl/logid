//! Library providing functionalities to set and capture [`log_id::LogId`]s.
//!
//! **Usage:**
//!
//! ~~~
//! use logid::{log_id::{LogId, EventLevel, get_log_id}, set_event};
//!
//! enum CrateErrors {
//!   SomeError = get_log_id(0, 0, EventLevel::Error, 0),
//! }
//!
//! fn my_func() -> Result<usize, LogId> {
//!   // some code ...
//!
//!   // on error
//!   Err(set_event!(CrateErrors::SomeError, "Some error message")
//!       .add_debug("Add debug information").into()
//!   )
//! }
//! ~~~

pub mod capturing;
pub mod crate_map;
pub mod id_entry;
pub mod id_map;
pub mod log_id;
