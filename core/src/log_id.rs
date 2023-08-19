//! Contains the [`LogId`] struct.

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LogId {
    pub(crate) module_path: &'static str,

    pub(crate) identifier: &'static str,

    pub(crate) log_level: LogLevel,
}

impl evident::event::Id for LogId {}

impl evident::publisher::CaptureControl for LogId {
    fn start(id: &Self) -> bool {
        id == &START_LOGGING
    }

    fn start_id() -> Self {
        START_LOGGING
    }

    fn stop(id: &Self) -> bool {
        id == &STOP_LOGGING
    }

    fn stop_id() -> Self {
        STOP_LOGGING
    }
}

/// Notify LOGGER and listeners to start logging.
///
/// **Note:** Filter does not affect capturing of this LogId. It is up to the handler to decide wether to filter it or not.
pub const START_LOGGING: LogId = crate::new_log_id!("START_LOGGING", LogLevel::Info);
/// Notify LOGGER and listeners to stop logging.
///
/// **Note:** Filter does not affect capturing of this LogId. It is up to the handler to decide wether to filter it or not.
pub const STOP_LOGGING: LogId = crate::new_log_id!("STOP_LOGGING", LogLevel::Info);

impl LogId {
    pub const fn new(
        module_path: &'static str,
        identifier: &'static str,
        log_level: LogLevel,
    ) -> Self {
        LogId {
            module_path,
            identifier,
            log_level,
        }
    }

    pub fn get_module_path(&self) -> &'static str {
        self.module_path
    }

    pub fn get_identifier(&self) -> &'static str {
        self.identifier
    }

    pub fn get_log_level(&self) -> LogLevel {
        self.log_level
    }
}

impl std::fmt::Display for LogId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "id='{}::{}'", self.module_path, self.identifier)
    }
}

/// Log level a [`LogId`] may represent.
#[derive(Debug, Default, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, std::hash::Hash)]
pub enum LogLevel {
    Trace = 0,
    #[default]
    Debug = 1,
    Info = 2,
    Warn = 3,
    Error = 4,
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            LogLevel::Error => "ERR",
            LogLevel::Warn => "WARN",
            LogLevel::Info => "INFO",
            LogLevel::Debug => "DEBUG",
            LogLevel::Trace => "TRACE",
        };

        write!(f, "{s}")
    }
}

/// Macro to create a [`LogId`] with a custom identifier and [`LogLevel`].
///
/// **Note:** The identifier must be a string literal.
///
/// ## Usage
///
/// ```
/// use logid_core::{new_log_id, log_id::LogLevel};
///
/// let id = new_log_id!("custom_ident", LogLevel::Debug);
/// ```
#[macro_export]
macro_rules! new_log_id {
    ($identifier:expr, $log_level:expr) => {
        $crate::log_id::LogId::new(module_path!(), $identifier, $log_level)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_log_id_with_macro() {
        let log_id = new_log_id!("custom_ident", LogLevel::Debug);

        assert_eq!(
            log_id.module_path,
            module_path!(),
            "Module path was not set correctly using `log_id!()` macro."
        );
        assert_eq!(
            log_id.identifier, "custom_ident",
            "Identifier was not set correctly using `log_id!()` macro."
        );
        assert_eq!(
            log_id.log_level,
            LogLevel::Debug,
            "Log level was not set correctly using `log_id!()` macro."
        );
    }
}
