//! Contains the [`LogId`] type and functions to create log IDs

/// Type to represent a LogId.
///
/// **Note:** Wrapper of `isize` for easier `id <=> enum` conversion.
pub type LogId = isize;

/// Macro to convert an enum to a [`LogId`].
#[macro_export]
macro_rules! logid {
    ($id:ident) => {
        ($id as $crate::log_id::LogId)
    };
}

/// Represents an invalid log-id
pub const INVALID_LOG_ID: LogId = 0;

/// Event level a log-id may represent.
#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub enum EventLevel {
    /// Log-id debug kind
    #[default]
    Debug = 0,
    /// Log-id info kind
    Info = 1,
    /// Log-id warning kind
    Warn = 2,
    /// Log-id error kind
    Error = 3,
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
pub trait LogIdParts {
    /// Get the main group of this log-id
    fn get_main_grp(self) -> u8;
    /// Get the sub group of this log-id
    fn get_sub_grp(self) -> u8;
    /// Get the [`EventLevel`] of this log-id
    fn get_level(self) -> EventLevel;
    /// Get the local number of this log-id
    fn get_local_nr(self) -> u8;
}

impl LogIdParts for LogId {
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

    fn get_main_grp(self) -> u8 {
        (self >> MAIN_GRP_SHIFT).try_into().unwrap_or(0)
    }

    fn get_sub_grp(self) -> u8 {
        // 15 to get 1111
        ((self >> SUB_GRP_SHIFT) & 15).try_into().unwrap()
    }

    fn get_local_nr(self) -> u8 {
        // 63 to get 111111
        ((self) & 63).try_into().unwrap()
    }
}

/// The number of bits of a log-id.
///
/// **Note:** The restriction to 16 bit is used to get support for logid on all platforms.
const LOG_ID_BIT_RANGE: i16 = 16;
/// Bit shift in the log-id to place the main group value
const MAIN_GRP_SHIFT: i16 = 12;
/// Bit shift in the log-id to place the sub group value
const SUB_GRP_SHIFT: i16 = 8;
/// Bit shift in the log-id to place the [`EventLevel`] value
const EVENT_LEVEL_SHIFT: i16 = 6;

/// Returns a 16-bit log-id that is used to identify a log-id message across a project.
/// The log-id is a unique unsigned integer value that is identified by bit shifting given group numbers and event level.
/// The 16-bit result in a possible log-id range of [0 .. 65535].
///
/// The log-id bits are represented as follows:
///
/// `16-13 bit = main group | 12-9 bit = sub group | 8-7 bit = event level | remaining 6 bit = local number`
///
/// # Arguments
///
/// * `main_grp` - main group the log-id is assigned to (possible range: 0 .. 15)
/// * `sub_grp` - sub group the log-id is assigned to (possible range: 0 .. 15)
/// * `log_kind` - the ['EventLevel'] of the log-id
/// * `local_nr` - the local number of the log-id (possible range: 0 .. 63)
///
/// # Example
///
/// ~~~
/// use logid::log_id::{get_log_id, EventLevel};
///
/// assert_eq!(get_log_id(0, 0, EventLevel::Debug, 1), 1);
/// assert_eq!(get_log_id(1, 0, EventLevel::Error, 1), 4289);
/// assert_eq!(get_log_id(15, 15, EventLevel::Error, 63), 65535);
/// ~~~
pub const fn get_log_id(main_grp: u8, sub_grp: u8, event_level: EventLevel, local_nr: u8) -> LogId {
    let event_level_number: u16 = event_level as u16;

    if (main_grp == 0) && (sub_grp == 0) && (event_level_number == 0) && (local_nr == 0) {
        panic!("Log-id `0` is not allowed!");
    } else if main_grp >= (1 << (LOG_ID_BIT_RANGE - MAIN_GRP_SHIFT)) {
        panic!("Given main group is too big for a valid log-id.");
    } else if sub_grp >= (1 << (MAIN_GRP_SHIFT - SUB_GRP_SHIFT)) {
        panic!("Given sub group is too big for a valid log-id.");
    } else if local_nr >= (1 << EVENT_LEVEL_SHIFT) {
        panic!("Given local number is too big for a valid log-id.");
    }

    (((main_grp as u16) << MAIN_GRP_SHIFT)
        + ((sub_grp as u16) << SUB_GRP_SHIFT)
        + (event_level_number << EVENT_LEVEL_SHIFT)
        + (local_nr as u16)) as LogId
}

#[cfg(test)]
mod tests {
    use super::{get_log_id, EventLevel, LogIdParts};

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
    fn main_set_1() {
        let log_id = get_log_id(1, 0, EventLevel::Debug, 0);

        assert_eq!(
            log_id, 0b0001000000000000,
            "Log-id value not shifted correctly"
        );
        assert_eq!(log_id.get_main_grp(), 1, "Did not get correct main group");
    }

    #[test]
    fn main_set_15() {
        let log_id = get_log_id(15, 0, EventLevel::Debug, 0);

        assert_eq!(
            log_id, 0b1111000000000000,
            "Log-id value not shifted correctly"
        );
        assert_eq!(log_id.get_main_grp(), 15, "Did not get correct main group");
    }

    #[test]
    fn sub_set_4() {
        let log_id = get_log_id(0, 4, EventLevel::Debug, 0);

        assert_eq!(
            log_id, 0b0000010000000000,
            "Log-id value not shifted correctly"
        );
        assert_eq!(log_id.get_sub_grp(), 4, "Did not get correct sub group");
    }

    #[test]
    fn sub_set_15() {
        let log_id = get_log_id(0, 15, EventLevel::Debug, 0);

        assert_eq!(
            log_id, 0b0000111100000000,
            "Log-id value not shifted correctly"
        );
        assert_eq!(log_id.get_sub_grp(), 15, "Did not get correct sub group");
    }

    #[test]
    fn local_set_3() {
        let log_id = get_log_id(0, 0, EventLevel::Debug, 3);

        assert_eq!(
            log_id, 0b0000000000000011,
            "Log-id value not shifted correctly"
        );
        assert_eq!(log_id.get_local_nr(), 3, "Did not get correct local number");
    }

    #[test]
    fn local_set_63() {
        let log_id = get_log_id(0, 0, EventLevel::Debug, 63);

        assert_eq!(
            log_id, 0b0000000000111111,
            "Log-id value not shifted correctly"
        );
        assert_eq!(
            log_id.get_local_nr(),
            63,
            "Did not get correct local number"
        );
    }

    #[test]
    fn level_set_warning() {
        let log_id = get_log_id(0, 0, EventLevel::Warn, 0);

        assert_eq!(
            log_id, 0b0000000010000000,
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
    #[should_panic(expected = "Given main group is too big for a valid log-id.")]
    fn main_out_of_bounds() {
        let _log_id = get_log_id(16, 0, EventLevel::Debug, 1);

        unreachable!("Should have panicked");
    }

    #[test]
    #[should_panic(expected = "Given sub group is too big for a valid log-id.")]
    fn sub_out_of_bounds() {
        let _log_id = get_log_id(0, 16, EventLevel::Debug, 1);

        unreachable!("Should have panicked");
    }

    #[test]
    #[should_panic(expected = "Given local number is too big for a valid log-id.")]
    fn local_nr_out_of_bounds() {
        let _log_id = get_log_id(0, 0, EventLevel::Debug, 64);

        unreachable!("Should have panicked");
    }
}
