//! Contains the [`LogId`] struct.

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LogId {
    pub(crate) crate_name: &'static str,

    pub(crate) module_path: &'static str,

    pub(crate) identifier: &'static str,

    pub(crate) log_level: LogLevel,
}

impl evident::publisher::Id for LogId {}

impl LogId {
    pub fn new(
        crate_name: &'static str,
        module_path: &'static str,
        identifier: &'static str,
        log_level: LogLevel,
    ) -> Self {
        LogId {
            crate_name,
            module_path,
            identifier,
            log_level,
        }
    }

    pub fn get_crate_name(&self) -> &'static str {
        self.crate_name
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

/// Log level a [`LogId`] may represent.
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, std::hash::Hash)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    #[default]
    Debug,
}
