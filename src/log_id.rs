//! Contains the [`LogId`] type and functions to create log IDs

/// Type to represent a LogId.
///
/// **Note:** Wrapper of `isize` for easier `id <=> enum` conversion.
pub type LogId = isize;

/// Represents an invalid log-id
pub const INVALID_LOG_ID: LogId = 0;

/// Event level a log-id may represent.
#[derive(Debug, PartialEq, Clone)]
pub enum EventLevel {
    /// Log-id debug kind
    Debug = 0,
    /// Log-id info kind
    Info = 1,
    /// Log-id warning kind
    Warn = 2,
    /// Log-id error kind
    Error = 3,
}

impl Default for EventLevel {
    fn default() -> Self {
        EventLevel::Debug
    }
}

impl From<&tracing::Level> for EventLevel {
    /// Converts tracing::Level to EventLevel.
    /// `DEBUG` and `TRACE` are both converted to `EventLevel::Debug`.
    fn from(level: &tracing::Level) -> Self {
        if level == &tracing::Level::ERROR {
            EventLevel::Error
        } else if level == &tracing::Level::WARN {
            EventLevel::Warn
        } else if level == &tracing::Level::INFO {
            EventLevel::Info
        } else {
            EventLevel::Debug
        }
    }
}

/// Trait needed to implement functions on [`LogId`], due to `isize` wrap.
pub trait LogIdLevel {
    /// Get the [`EventLevel`] of this log-id
    fn get_level(self) -> EventLevel;
}

impl LogIdLevel for LogId {
    fn get_level(self) -> EventLevel {
        // get EventLevel bits
        let level = (self >> EVENT_LEVEL_SHIFT) & 3;

        if level == (EventLevel::Error as isize) {
            EventLevel::Error
        } else if level == (EventLevel::Warn as isize) {
            EventLevel::Warn
        } else if level == (EventLevel::Info as isize) {
            EventLevel::Info
        } else if level == (EventLevel::Debug as isize) {
            EventLevel::Debug
        } else {
            tracing::trace!("Invalid event level={} for id={}", level, self);
            EventLevel::Error
        }
    }
}

/// Bit shift in the log-id to place the [`EventLevel`] value
const EVENT_LEVEL_SHIFT: i16 = 8;

/// Returns a 16-bit log-id that is used to identify a log-id message across a project.
/// The log-id is a unique unsigned integer value that is identified by bit shifting given group numbers and event level.
/// The 16-bit result in a possible log-id range of [0 .. 65535].
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
/// use logid::log_id::{get_log_id, EventLevel};
///
/// assert_eq!(get_log_id(0, 0, EventLevel::Debug, 1), 1);
/// assert_eq!(get_log_id(1, 0, EventLevel::Error, 1), 17153);
/// assert_eq!(get_log_id(3, 15, EventLevel::Error, 255), 65535);
/// ~~~
pub const fn get_log_id(main_grp: u8, sub_grp: u8, event_level: EventLevel, local_nr: u8) -> LogId {
    let event_level_number: u16 = event_level as u16;

    if (main_grp == 0) && (sub_grp == 0) && (event_level_number == 0) && (local_nr == 0) {
        panic!("Log-id `0` is not allowed!");
    } else if (main_grp > 3) || (sub_grp > 15) {
        panic!("At least one log-id subrange is invalid.");
    }

    (((main_grp as u16) << 14)
        + ((sub_grp as u16) << 10)
        + (event_level_number << EVENT_LEVEL_SHIFT)
        + (local_nr as u16)) as LogId
}

#[cfg(test)]
mod tests {
    use super::{get_log_id, EventLevel, LogIdLevel};

    #[test]
    fn create_log_id_with_error() {
        let log_id = get_log_id(0, 0, EventLevel::Error, 0);

        assert_eq!(
            log_id.get_level(),
            EventLevel::Error,
            "Log-id levels are not equal"
        );
    }

    #[test]
    fn main_log_id_set_1() {
        let log_id = get_log_id(1, 0, EventLevel::Debug, 0);

        assert_eq!(
            log_id, 0b0100000000000000,
            "Log-id value not shifted correctly"
        );
    }

    #[test]
    fn main_log_id_set_3() {
        let log_id = get_log_id(3, 0, EventLevel::Debug, 0);

        assert_eq!(
            log_id, 0b1100000000000000,
            "Log-id value not shifted correctly"
        );
    }

    #[test]
    fn sub_log_id_set_3() {
        let log_id = get_log_id(0, 3, EventLevel::Debug, 0);

        assert_eq!(
            log_id, 0b0000110000000000,
            "Log-id value not shifted correctly"
        );
    }

    #[test]
    fn local_log_id_set_3() {
        let log_id = get_log_id(0, 0, EventLevel::Debug, 3);

        assert_eq!(
            log_id, 0b0000000000000011,
            "Log-id value not shifted correctly"
        );
    }

    #[test]
    fn log_id_level_set_warning() {
        let log_id = get_log_id(0, 0, EventLevel::Warn, 0);

        assert_eq!(
            log_id, 0b0000001000000000,
            "Log-id value not shifted correctly"
        );
    }

    #[test]
    #[should_panic(expected = "Log-id `0` is not allowed!")]
    fn invalid_log_id_set() {
        let _log_id = get_log_id(0, 0, EventLevel::Debug, 0);

        unreachable!("Should have panicked");
    }

    #[test]
    #[should_panic(expected = "At least one log-id subrange is invalid.")]
    fn log_id_main_out_of_bounds() {
        let _log_id = get_log_id(4, 0, EventLevel::Debug, 1);

        unreachable!("Should have panicked");
    }

    #[test]
    #[should_panic(expected = "At least one log-id subrange is invalid.")]
    fn log_id_sub_out_of_bounds() {
        let _log_id = get_log_id(0, 16, EventLevel::Debug, 1);

        unreachable!("Should have panicked");
    }

}
