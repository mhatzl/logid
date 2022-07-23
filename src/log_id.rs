//! Contains the `LogId` type and functions to create and set log IDs

use crate::{
    id_entry::{LogIdEntry, Origin},
    id_map::LOG_ID_MAP,
};

/// Type to represent a LogId.
///
/// **Note:** Wrapper of `isize` for easier `id <=> enum` conversion.
pub type LogId = isize;

/// Represents an invalid log-id
pub const INVALID_LOG_ID: LogId = 0;
/// Bit shift in the log-id to place the [`EventLevel`] value
const EVENT_LEVEL_SHIFT: i16 = 9;

/// Event level a log-id can represent.
#[derive(Debug, PartialEq)]
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
        let id_entry = LogIdEntry {
            id: self,
            level: self.get_level(),
            msg: msg.to_string(),
            origin: Origin::new(filename, line_nr),
            span: if let Some(span) = tracing::span::Span::current().metadata() {
                span.name()
            } else {
                "event not in span"
            },
            ..Default::default()
        };

        // Note: It is not possible to set `target` via parameter, because it requires `const`
        // Same goes for `level` for the `event` macro => match and code duplication needed
        match id_entry.level {
            EventLevel::Error => tracing::error!("{}: {}", self, msg),
            EventLevel::Warn => tracing::warn!("{}: {}", self, msg),
            EventLevel::Info => tracing::info!("{}: {}", self, msg),
            EventLevel::Debug => tracing::debug!("{}: {}", self, msg),
        }

        tracing::trace!(
            "{}(origin): {}", self, String::from(&id_entry.origin)
        );

        let update_map = LOG_ID_MAP.map.write();
        if let Ok(mut map) = update_map {
            match map.get_mut(&id_entry.id) {
                Some(entries) => entries.push(id_entry),
                None => {
                    map.insert(id_entry.id, [id_entry].into());
                }
            };
        }

        self
    }

    fn add_cause(self, msg: &str) -> LogId {
        tracing::info!("{}(cause): {}", self, msg);

        let update_map = LOG_ID_MAP.map.write();
        if let Ok(mut map) = update_map {
            match map.get_mut(&self) {
                Some(entries) => {
                    if let Some(last) = entries.last_mut() {
                        last.add_cause(msg.to_string());
                    };
                }
                None => {
                    tracing::warn!(
                        "Got cause=\"{}\" for log-id={}, but no base for log-id was set!",
                        msg,
                        self
                    )
                }
            };
        }

        self
    }

    fn add_info(self, msg: &str) -> LogId {
        tracing::info!("{}(addon): {}", self, msg);
        add_addon_to_map(self, msg, &tracing::Level::INFO);
        self
    }

    fn add_debug(self, msg: &str) -> LogId {
        tracing::debug!("{}(addon): {}", self, msg);
        add_addon_to_map(self, msg, &tracing::Level::DEBUG);
        self
    }

    fn add_trace(self, msg: &str) -> LogId {
        tracing::trace!("{}(addon): {}", self, msg);
        add_addon_to_map(self, msg, &tracing::Level::TRACE);
        self
    }

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

fn add_addon_to_map(id: LogId, msg: &str, level: &tracing::Level) {
    let update_map = LOG_ID_MAP.map.write();
    if let Ok(mut map) = update_map {
        match map.get_mut(&id) {
            Some(entries) => {
                if let Some(last) = entries.last_mut() {
                    last.add_addon(level, msg.to_string());
                };
            }
            None => {
                tracing::warn!(
                    "Got addon=\"{}\" for log-id={}, but no base for log-id was set!",
                    msg,
                    id
                )
            }
        };
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
