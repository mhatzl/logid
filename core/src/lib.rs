//! Library providing functionalities to set and capture [`log_id::LogId`] events.
//! [`log_id::LogId`]s are used to identify and group events.
//! An event is captured by a central [`LOGGER`](crate::logging::LOGGER) once the event is *finalized*.
//! *Finalizing* is done explicitly using `.finalize()`, converting the event back to the [`log_id::LogId`], or implicitly when the event is dropped.
//!
//! The [`LOGGER`](crate::logging::LOGGER) is created using [evident]'s [`Publisher`](evident::publisher) struct.
//! This allows to add subscribers to one or more [`log_id::LogId`]s to receive events set with those [`log_id::LogId`]s.
//! For more information about subscription variants, checkout [`evident`].
//!
//! **Usage:**
//!
//! ~~~
//! use logid::{log_id::{LogId, LogLevel}, err};
//! use logid_derive::ErrLogId;
//! use thiserror::Error;
//!
//! #[derive(Debug, Default, Clone, ErrLogId, Error)]
//! enum CrateErrors {
//!     #[error("`SomeError` description.")]
//!     SomeError,
//!
//!     #[error("`InternalError` description.")]
//!     #[default]
//!     InternalError,
//! }
//!
//! fn my_func() -> Result<(), CrateErrors> {
//!     // some code ...
//!
//!     // on error
//!     err!(CrateErrors::SomeError)
//! }
//! ~~~

pub mod log_id;
pub mod logging;
pub mod set_macros;

// Re-exports
pub use evident;
