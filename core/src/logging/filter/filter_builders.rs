use crate::log_id::LogLevel;

use super::{AddonFilter, FilterConfig, LogIdAddonFilter, LogIdModuleFilter};

#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct FilterConfigBuilder {
    logging_enabled: bool,
    general_level: LogLevel,
    general_addons: Vec<AddonFilter>,
    /// LogIds set with `on[LogId]`
    allowed_global_ids: Vec<LogIdAddonFilter>,
    allowed_modules: Vec<LogIdModuleFilter>,
}

impl FilterConfigBuilder {
    /// Creates a new builder for [`FilterConfig`].
    pub fn new(level: LogLevel) -> Self {
        Self {
            general_level: level,
            ..Default::default()
        }
    }

    /// Disable logging in general.
    pub fn disabled_logging(mut self) -> Self {
        self.logging_enabled = false;
        self
    }

    /// Add addons allowed by the filter.
    pub fn allowed_addons<I>(mut self, addons: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<AddonFilter>,
    {
        self.general_addons
            .extend(addons.into_iter().map(Into::into));
        self
    }

    /// Add LogIDs allowed by the filter.
    pub fn global_ids<I>(mut self, global_ids: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<LogIdAddonFilter>,
    {
        self.allowed_global_ids
            .extend(global_ids.into_iter().map(Into::into));
        self
    }

    /// Add modules allowed by the filter.
    pub fn modules<I>(mut self, modules: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<LogIdModuleFilter>,
    {
        self.allowed_modules
            .extend(modules.into_iter().map(Into::into));
        self
    }

    /// Build [`FilterConfig`] with configuration constructed using the builder.
    pub fn build(self) -> FilterConfig {
        FilterConfig {
            logging_enabled: self.logging_enabled,
            general_level: self.general_level,
            general_addons: self.general_addons,
            allowed_global_ids: self.allowed_global_ids,
            allowed_modules: self.allowed_modules,
        }
    }
}
