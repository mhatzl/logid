use crate::log_id::LogLevel;

use super::{AddonFilter, FilterConfig, LogIdAddonFilter, LogIdModuleFilter};

#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct FilterConfigBuilder {
    // TODO: negation is confusing, use affirmation instead (no_general_logging -> general_logging)
    no_general_logging: bool,
    general_level: LogLevel,
    general_addons: Vec<AddonFilter>,
    /// LogIds set with `on[LogId]`
    allowed_global_ids: Vec<LogIdAddonFilter>,
    allowed_modules: Vec<LogIdModuleFilter>,
}

impl FilterConfigBuilder {
    pub fn new(level: LogLevel) -> Self {
        Self {
            general_level: level,
            ..Default::default()
        }
    }

    pub fn level(mut self, level: LogLevel) -> Self {
        self.general_level = level;
        self
    }

    pub fn no_general_logging(mut self) -> Self {
        self.no_general_logging = true;
        self
    }

    pub fn addons<T>(mut self, addons: T) -> Self
    where
        T: IntoIterator<Item = AddonFilter>,
    {
        self.general_addons.extend(addons.into_iter());
        self
    }

    pub fn global_ids<T>(mut self, global_ids: T) -> Self
    where
        T: IntoIterator<Item = LogIdAddonFilter>,
    {
        self.allowed_global_ids.extend(global_ids.into_iter());
        self
    }

    pub fn modules<T>(mut self, modules: T) -> Self
    where
        T: IntoIterator<Item = LogIdModuleFilter>,
    {
        self.allowed_modules.extend(modules.into_iter());
        self
    }

    pub fn build(self) -> FilterConfig {
        FilterConfig {
            no_general_logging: self.no_general_logging,
            general_level: self.general_level,
            general_addons: self.general_addons,
            allowed_global_ids: self.allowed_global_ids,
            allowed_modules: self.allowed_modules,
        }
    }
}
