//! Contains the `LogId` type and functions to create and set log IDs

/// Type to represent a LogId.
///
/// **Note:** Wrapper of `isize` for easier `id <=> enum` conversion.
pub type LogId = isize;

/// Represents an invalid log-id
pub const INVALID_LOG_ID: LogId = 0;
/// Bit shift in the log-id to place the [`EventLevel`] value
const EVENT_LEVEL_SHIFT: i16 = 9;

/// Log kind a log-id can represent.
#[derive(Debug)]
pub enum EventLevel {
    /// Log-id error kind
    Error = 3,
    /// Log-id warning kind
    Warn = 2,
    /// Log-id info kind
    Info = 1,
    /// Log-id debug kind
    Debug = 0,
}

/// Trait to use [`LogId`] for tracing.
pub trait LogIdTracing {
    /// Set an event for a [`LogId`]
    ///
    /// # Arguments
    ///
    /// * `msg` - main message that is set for this log-id (should be a user-centered event description)
    /// * `filename` - name of the source file where the event is set (Note: use `file!()`)
    /// * `line_nr` - line number where the event is set (Note: use `line!()`)
    fn set_event(self, msg: &str, filename: &str, line_nr: u32) -> Self;

    /// Add a message describing the cause for this log-id
    fn add_cause(self, msg: &str) -> Self;

    /// Add an info message for this log-id
    fn add_info(self, msg: &str) -> Self;

    /// Add a debug message for this log-id
    fn add_debug(self, msg: &str) -> Self;

    /// Add a trace message for this log-id
    fn add_trace(self, msg: &str) -> Self;

    /// Get the [`EventLevel`] of this log-id
    fn get_level(self) -> EventLevel;
}

impl LogIdTracing for LogId {
    fn set_event(self, msg: &str, filename: &str, line_nr: u32) -> LogId {
        let kind = self.get_level();
        // Note: It is not possible to set `target` via parameter, because it requires `const`
        match kind {
            EventLevel::Error => tracing::error!("{}: {}", self, msg),
            EventLevel::Warn => tracing::warn!("{}: {}", self, msg),
            EventLevel::Info => tracing::info!("{}: {}", self, msg),
            EventLevel::Debug => tracing::debug!("{}: {}", self, msg),
        }

        tracing::trace!(
            "{}: Occured in file \"{}\" at line: {}",
            self,
            filename,
            line_nr
        );
        self
    }

    fn add_cause(self, msg: &str) -> LogId {
        tracing::info!("{}(cause): {}", self, msg);
        self
    }

    fn add_info(self, msg: &str) -> LogId {
        tracing::info!("{}(addon): {}", self, msg);
        self
    }

    fn add_debug(self, msg: &str) -> LogId {
        tracing::debug!("{}(addon): {}", self, msg);
        self
    }

    fn add_trace(self, msg: &str) -> LogId {
        tracing::trace!("{}(addon): {}", self, msg);
        self
    }

    fn get_level(self) -> EventLevel {
        // get EventLevel bits
        let kind = (self >> EVENT_LEVEL_SHIFT) & 3;

        if kind == (EventLevel::Error as isize) {
            EventLevel::Error
        } else if kind == (EventLevel::Warn as isize) {
            EventLevel::Warn
        } else if kind == (EventLevel::Info as isize) {
            EventLevel::Info
        } else if kind == (EventLevel::Debug as isize) {
            EventLevel::Debug
        } else {
            tracing::trace!("{}: Invalid level: '{}'", self, kind);
            EventLevel::Error
        }
    }
}

/// Returns a 16-bit log-id that is used to identify a logID message across a project.
/// The log-id is a unique signed integer value that is identified by bit shifting given group numbers and event level.
///
/// The log-id bits are represented as follows:
///
/// `16-15 bit = main group | 14-11 bit = sub group | 10-9 bit = event level | remaining 8 bit = local number`
///
/// # Arguments
///
/// * `main_grp` - main group the log-id is assigned to (possible range: 0 .. 3)
/// * `sub_grp` - sub group the log-id is assigned to (possible range: 0 .. 15)
/// * `log_kind` - the ['EventLevel'] of the log-id
/// * `local_nr` - the local number of the log-id (possible range: 0 .. 255)
///
/// # Example
///
/// ~~~
/// use unimarkup_core::log_id::{get_log_id, EventLevel};
///
/// assert_eq!(get_log_id(0, 0, EventLevel::Debug, 1), 1);
/// assert_eq!(get_log_id(1, 0, EventLevel::Error, 1), 17153);
/// ~~~
pub const fn get_log_id(main_grp: u8, sub_grp: u8, event_level: EventLevel, local_nr: u8) -> LogId {
    let event_level_number: i16 = event_level as i16;

    // Id = 0 is not allowed
    //
    // TODO: needs unstable "panic!() in const fn" feature. Uncomment after feature is in stable
    //panic!((main_grp == 0) && (sub_grp == 0) && (event_level == 0) && (local_nr == 0), "Log ID 0 is not allowed!");
    //panic!((main_grp >= 2^2) || (sub_grp >= 2^4), "At least one log ID subrange is invalid.");

    (((main_grp as i16) << 15)
        + ((sub_grp as i16) << 11)
        + (event_level_number << EVENT_LEVEL_SHIFT)
        + (local_nr as i16)) as LogId
}
