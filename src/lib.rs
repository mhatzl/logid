//! Library providing functionalities to set and capture [`log_id::LogId`] events.
//! [`log_id::LogId`]s are used to identify and group events.
//! An event is captured by a central map by explicitly using [`finalize()`](capturing::LogIdEvent::finalize()),
//! converting the event into the [`log_id::LogId`] used to set the event,
//! or implicitly on `drop` of the event.
//!
//! To access captured events, checkout the functions and macros provided by the [`id_map`] module.
//!
//! **Usage:**
//!
//! ~~~
//! use logid::{log_id::{LogId, LogLevel, get_log_id}, set_event};
//!
//! enum CrateErrors {
//!   SomeError = get_log_id(0, 0, LogLevel::Error, 0),
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

pub mod event;
pub mod log_id;
pub mod publisher;
