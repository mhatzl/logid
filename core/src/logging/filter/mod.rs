use std::sync::{Arc, RwLock};

use evident::event::origin::Origin;

use crate::log_id::{LogId, LogLevel};

use super::{event_entry::AddonKind, msg::LogMsg};

mod filter_builders;

pub use filter_builders::*;

#[derive(Default, Debug)]
pub struct LogFilter {
    filter: Arc<RwLock<FilterConfig>>,
}

/// Returns `true` if logid is configured to allow the given level.
/// Debug and Trace levels must explicitly be allowed by enabling features `log_debugs` or `log_traces`.
///
/// Without using this function as early filter return, debug and trace logs would mostly go through all filter steps, which decreases performance.
#[allow(unreachable_code)]
#[allow(unused_variables)]
fn allow_level(level: LogLevel) -> bool {
    #[cfg(feature = "log_traces")]
    return true; // Note: No level is below Trace

    #[cfg(feature = "log_debugs")]
    return level >= LogLevel::Debug;

    level > LogLevel::Debug
}

impl LogFilter {
    pub fn new() -> Self {
        let filter_config = FilterConfig::new(&filter_config());

        LogFilter {
            filter: Arc::new(RwLock::new(filter_config)),
        }
    }

    pub fn set_filter(&self, filter_config: FilterConfig) -> Result<(), FilterError> {
        match self.filter.write() {
            Ok(mut locked_filter) => {
                locked_filter.replace(filter_config);
            }
            Err(mut err) => {
                // lock poisoned, replace inner filter completely
                **err.get_mut() = filter_config;
            }
        }

        Ok(())
    }

    pub fn allow_addon(&self, id: LogId, origin: &Origin, addon: &AddonKind) -> bool {
        if !allow_level(id.log_level) {
            return false;
        } else if let Ok(addon_filter) = AddonFilter::try_from(addon) {
            if addon_filter == AddonFilter::Debugs && !allow_level(LogLevel::Debug)
                || addon_filter == AddonFilter::Traces && !allow_level(LogLevel::Trace)
            {
                return false;
            }
        }

        match self.filter.read() {
            Ok(locked_filter) => locked_filter.allow_addon(id, origin, addon),
            Err(_) => false,
        }
    }

    pub fn show_origin_info(&self, id: LogId, origin: &Origin) -> bool {
        match self.filter.read() {
            Ok(locked_filter) => locked_filter.show_origin_info(id, origin),
            Err(_) => false,
        }
    }

    pub fn show_id(&self, id: LogId, origin: &Origin) -> bool {
        match self.filter.read() {
            Ok(locked_filter) => locked_filter.show_id(id, origin),
            Err(_) => false,
        }
    }
}

fn filter_config() -> String {
    if cfg!(feature = "test_filter") {
        return "trace(all)".to_string();
    }

    match std::env::var("LOGID_FILTER") {
        Ok(config) => config,
        Err(_) => "error".to_string(),
    }
}

pub fn set_filter<T>(into_filter: T) -> Result<(), crate::logging::filter::FilterError>
where
    T: Into<FilterConfig>,
{
    if let Some(filter) = crate::logging::LOGGER.get_filter() {
        filter.set_filter(into_filter.into())
    } else {
        Err(crate::logging::filter::FilterError::SettingFilter)
    }
}

impl evident::event::filter::Filter<LogId, LogMsg> for LogFilter {
    fn allow_entry(&self, entry: &impl evident::event::entry::EventEntry<LogId, LogMsg>) -> bool {
        if !allow_level(entry.get_event_id().log_level) {
            return false;
        }

        match self.filter.read() {
            Ok(locked_filter) => locked_filter.allow_entry(entry),
            Err(_) => false,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum AddonFilter {
    #[default]
    Id,
    Origin,
    Infos,
    Debugs,
    Traces,
    Related,
    AllAllowed,

    #[cfg(feature = "hint_note")]
    Hint,
    #[cfg(feature = "hint_note")]
    Note,

    #[cfg(feature = "diagnostics")]
    Diagnostics,

    #[cfg(feature = "payloads")]
    Payloads,
}

impl From<&AddonKind> for AddonFilter {
    fn from(value: &AddonKind) -> Self {
        match value {
            AddonKind::Info(_) => AddonFilter::Infos,
            AddonKind::Debug(_) => AddonFilter::Debugs,
            AddonKind::Trace(_) => AddonFilter::Traces,
            AddonKind::Related(_) => AddonFilter::Related,

            #[cfg(feature = "fmt")]
            AddonKind::FmtInfo(_) => AddonFilter::Infos,
            #[cfg(feature = "fmt")]
            AddonKind::FmtDebug(_) => AddonFilter::Debugs,
            #[cfg(feature = "fmt")]
            AddonKind::FmtTrace(_) => AddonFilter::Traces,

            #[cfg(feature = "hint_note")]
            AddonKind::Hint(_) => AddonFilter::Hint,
            #[cfg(all(feature = "hint_note", feature = "fmt"))]
            AddonKind::FmtHint(_) => AddonFilter::Hint,
            #[cfg(feature = "hint_note")]
            AddonKind::Note(_) => AddonFilter::Note,
            #[cfg(all(feature = "hint_note", feature = "fmt"))]
            AddonKind::FmtNote(_) => AddonFilter::Note,

            #[cfg(feature = "diagnostics")]
            AddonKind::Diagnostic(_) => AddonFilter::Diagnostics,
            #[cfg(all(feature = "diagnostics", feature = "fmt"))]
            AddonKind::FmtDiagnostic(_) => AddonFilter::Diagnostics,

            #[cfg(feature = "payloads")]
            AddonKind::Payload(_) => AddonFilter::Payloads,
            #[cfg(all(feature = "payloads", feature = "fmt"))]
            AddonKind::FmtPayload(_) => AddonFilter::Payloads,
        }
    }
}

impl TryFrom<&str> for AddonFilter {
    type Error = FilterError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let addon = match value {
            "id" => AddonFilter::Id,
            "origin" => AddonFilter::Origin,
            "infos" => AddonFilter::Infos,
            "debugs" => AddonFilter::Debugs,
            "traces" => AddonFilter::Traces,
            "related" => AddonFilter::Related,
            "all" => AddonFilter::AllAllowed,

            #[cfg(feature = "hint_note")]
            "hints" => AddonFilter::Hint,
            #[cfg(feature = "hint_note")]
            "notes" => AddonFilter::Note,

            #[cfg(feature = "diagnostics")]
            "diagnostics" => AddonFilter::Diagnostics,

            #[cfg(feature = "payloads")]
            "payloads" => AddonFilter::Payloads,

            _ => {
                return Err(FilterError::ParsingAddons(value.to_string()));
            }
        };

        Ok(addon)
    }
}

impl IntoIterator for AddonFilter {
    type Item = Self;

    type IntoIter = std::iter::Once<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        std::iter::once(self)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct LogIdFilter {
    module_path: String,
    identifier: String,
}

impl PartialEq<LogId> for LogIdFilter {
    fn eq(&self, other: &LogId) -> bool {
        self.module_path == other.module_path && self.identifier == other.identifier
    }
}

impl TryFrom<&str> for LogIdFilter {
    type Error = FilterError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parts = value.split("::");
        let len = parts.clone().count();

        if len < 2 {
            return Err(FilterError::ParsingLogId(value.to_string()));
        }

        // Form: <module path (might have more '::' between)>::<identifier>

        let mut module_parts = String::default();
        let mut identifier = String::default();

        for (i, part) in parts.enumerate() {
            if part.trim().is_empty() {
                return Err(FilterError::ParsingLogId(value.to_string()));
            }

            if i < len - 1 {
                module_parts.push_str(part);
                module_parts.push_str("::");
            } else {
                identifier.push_str(part);
            }
        }
        let module_path = module_parts
            .strip_suffix("::")
            .ok_or(FilterError::ParsingLogId(value.to_string()))?;

        if identifier.is_empty() {
            return Err(FilterError::ParsingLogId(value.to_string()));
        }

        Ok(LogIdFilter {
            module_path: module_path.to_string(),
            identifier: identifier.to_string(),
        })
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct LogIdAddonFilter {
    log_id: LogIdFilter,
    allowed_addons: Vec<AddonFilter>,
}

impl IntoIterator for LogIdAddonFilter {
    type Item = Self;

    type IntoIter = std::iter::Once<Self>;

    fn into_iter(self) -> Self::IntoIter {
        std::iter::once(self)
    }
}

impl TryFrom<&str> for LogIdAddonFilter {
    type Error = FilterError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut stripped_id = value.to_string();
        let addons = get_addons(&mut stripped_id);

        Ok(LogIdAddonFilter {
            log_id: LogIdFilter::try_from(stripped_id.as_str())?,
            allowed_addons: addons,
        })
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct LogIdModuleFilter {
    no_general_logging: bool,
    origin_module_path: String,
    level: LogLevel,
    allowed_ids: Vec<LogIdAddonFilter>,
    allowed_addons: Vec<AddonFilter>,
}

impl IntoIterator for LogIdModuleFilter {
    type Item = Self;

    type IntoIter = std::iter::Once<Self>;

    fn into_iter(self) -> Self::IntoIter {
        std::iter::once(self)
    }
}

impl LogIdModuleFilter {
    pub fn origin_in_module(&self, origin: &Origin) -> bool {
        origin.module_path.starts_with(&self.origin_module_path)
    }

    pub fn event_allowed(&self, id: LogId, origin: &Origin) -> bool {
        if !self.origin_in_module(origin) {
            return false;
        }

        (!self.no_general_logging && self.level <= id.log_level)
            || id_allowed(&self.allowed_ids, id)
    }

    pub fn addon_allowed(&self, id: LogId, origin: &Origin, addon: &AddonFilter) -> bool {
        if !self.origin_in_module(origin) {
            return false;
        }

        (!self.no_general_logging
            && self.level <= id.log_level
            && self.allowed_addons.contains(addon))
            || addon_allowed(&self.allowed_ids, id, addon)
    }

    fn try_from(
        s: &str,
        ids: Vec<LogIdAddonFilter>,
        addons: Vec<AddonFilter>,
    ) -> Result<Self, FilterError> {
        let mut module_filter = s.split('=');
        let len = module_filter.clone().count();
        if len != 1 && len != 2 {
            return Err(FilterError::ParsingModule(s.to_string()));
        }

        let module = module_filter
            .next()
            .ok_or(FilterError::ParsingModule(s.to_string()))?
            .trim();

        if module.is_empty() {
            return Err(FilterError::ParsingModule(s.to_string()));
        }

        if len == 1 {
            return Ok(LogIdModuleFilter {
                no_general_logging: true,
                origin_module_path: module.to_string(),
                allowed_ids: ids,
                allowed_addons: addons,
                ..Default::default()
            });
        }

        let level_part = module_filter
            .next()
            .ok_or(FilterError::ParsingModule(s.to_string()))?
            .trim();

        let level =
            try_into_log_level(level_part).ok_or(FilterError::ParsingModule(s.to_string()))?;

        Ok(LogIdModuleFilter {
            no_general_logging: false,
            origin_module_path: module.to_string(),
            level,
            allowed_ids: ids,
            allowed_addons: addons,
        })
    }
}

#[derive(Default, Debug)]
pub struct FilterConfig {
    no_general_logging: bool,
    general_level: LogLevel,
    general_addons: Vec<AddonFilter>,
    /// LogIds set with `on[LogId]`
    allowed_global_ids: Vec<LogIdAddonFilter>,
    allowed_modules: Vec<LogIdModuleFilter>,
}

impl FilterConfig {
    pub fn new(filter: &str) -> Self {
        if filter.trim().is_empty() || filter.to_lowercase() == "off" {
            return FilterConfig {
                no_general_logging: true,
                ..Default::default()
            };
        }

        let mut log_filter = FilterConfig {
            no_general_logging: true,
            general_level: LogLevel::Error,
            general_addons: Vec::new(),
            allowed_global_ids: Vec::new(),
            allowed_modules: Vec::new(),
        };

        for filter_part in filter.split(',') {
            let mut stripped_filter_part = filter_part.to_string();

            let mut ids = get_ids(&mut stripped_filter_part);

            if stripped_filter_part.starts_with("on") && !ids.is_empty() {
                log_filter.allowed_global_ids.append(&mut ids);
            } else {
                let addons = get_addons(&mut stripped_filter_part);

                if let Some(general_level) = try_into_log_level(stripped_filter_part.trim()) {
                    log_filter.no_general_logging = false;
                    log_filter.general_level = general_level;
                    log_filter.general_addons = addons;
                } else if let Ok(module_filter) =
                    LogIdModuleFilter::try_from(&stripped_filter_part, ids, addons)
                {
                    log_filter.allowed_modules.push(module_filter);
                }
            }
        }

        log_filter
    }

    fn replace(&mut self, other: Self) {
        self.allowed_global_ids = other.allowed_global_ids;
        self.allowed_modules = other.allowed_modules;
        self.general_addons = other.general_addons;
        self.general_level = other.general_level;
        self.no_general_logging = other.no_general_logging;
    }

    pub fn allow_addon(&self, id: LogId, origin: &Origin, addon: &AddonKind) -> bool {
        let addon_filter = AddonFilter::from(addon);

        if !self.no_general_logging && self.general_addons.contains(&addon_filter) {
            return true;
        }

        addon_allowed(&self.allowed_global_ids, id, &addon_filter)
            || addon_allowed_in_origin(&self.allowed_modules, id, origin, &addon_filter)
    }

    pub fn show_origin_info(&self, id: LogId, origin: &Origin) -> bool {
        let addon_filter = AddonFilter::Origin;

        if !self.no_general_logging && self.general_addons.contains(&addon_filter) {
            return true;
        }

        addon_allowed(&self.allowed_global_ids, id, &addon_filter)
            || addon_allowed_in_origin(&self.allowed_modules, id, origin, &addon_filter)
    }

    pub fn show_id(&self, id: LogId, origin: &Origin) -> bool {
        let addon_filter = AddonFilter::Id;

        if !self.no_general_logging && self.general_addons.contains(&addon_filter) {
            return true;
        }

        addon_allowed(&self.allowed_global_ids, id, &addon_filter)
            || addon_allowed_in_origin(&self.allowed_modules, id, origin, &addon_filter)
    }

    pub fn builder() -> FilterConfigBuilder {
        FilterConfigBuilder::new(LogLevel::Error)
    }
}

impl evident::event::filter::Filter<LogId, LogMsg> for FilterConfig {
    fn allow_entry(&self, entry: &impl evident::event::entry::EventEntry<LogId, LogMsg>) -> bool {
        // Note: event handler creates unique LogIds per handler => filter on origin
        if entry
            .get_origin()
            .module_path
            .starts_with("logid::event_handler")
        {
            return true;
        }

        // Note: `Trace` starts at `0`
        if !self.no_general_logging && self.general_level <= entry.get_event_id().log_level {
            return true;
        }

        id_allowed(&self.allowed_global_ids, *entry.get_event_id())
            || id_allowed_in_origin(
                &self.allowed_modules,
                *entry.get_event_id(),
                entry.get_origin(),
            )
    }
}

impl<I> From<(LogLevel, I)> for FilterConfig
where
    I: IntoIterator<Item = AddonFilter>,
{
    fn from((level, addons): (LogLevel, I)) -> Self {
        FilterConfig::builder().level(level).addons(addons).build()
    }
}

#[derive(Debug, Clone)]
pub enum FilterError {
    ParsingLogId(String),
    ParsingAddons(String),
    ParsingModule(String),
    SettingFilter,
}

impl std::error::Error for FilterError {}

impl std::fmt::Display for FilterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FilterError::ParsingLogId(bad_id) => {
                write!(f, "Could not parse '{}' as `LogId`.", bad_id)
            }
            FilterError::ParsingAddons(bad_addons) => {
                write!(f, "Could not parse addons '{}'.", bad_addons)
            }
            FilterError::ParsingModule(bad_module) => {
                write!(f, "Could not parse module '{}'.", bad_module)
            }
            FilterError::SettingFilter => {
                write!(f, "Could not set the new filter configuration.")
            }
        }
    }
}

fn id_allowed(ids: &Vec<LogIdAddonFilter>, id: LogId) -> bool {
    for allowed_id in ids {
        if allowed_id.log_id == id {
            return true;
        }
    }

    false
}

fn id_allowed_in_origin(modules: &Vec<LogIdModuleFilter>, id: LogId, origin: &Origin) -> bool {
    for module in modules {
        if module.event_allowed(id, origin) {
            return true;
        }
    }

    false
}

fn addon_allowed(ids: &Vec<LogIdAddonFilter>, id: LogId, addon: &AddonFilter) -> bool {
    for allowed_id in ids {
        if allowed_id.log_id == id && allowed_id.allowed_addons.contains(addon) {
            return true;
        }
    }

    false
}

fn addon_allowed_in_origin(
    modules: &Vec<LogIdModuleFilter>,
    id: LogId,
    origin: &Origin,
    addon: &AddonFilter,
) -> bool {
    for module in modules {
        if module.addon_allowed(id, origin, addon) {
            return true;
        }
    }

    false
}

fn get_addons(s: &mut String) -> Vec<AddonFilter> {
    let mut addons = Vec::new();

    if let (Some(addon_start), Some(addon_end)) = (s.find('('), s.find(')')) {
        if addon_start >= addon_end {
            return addons;
        }

        if let Some(addons_part) = s.get((addon_start + 1)..addon_end) {
            for addon_part in addons_part.split('&') {
                if let Ok(addon) = AddonFilter::try_from(addon_part.trim()) {
                    if addon == AddonFilter::AllAllowed {
                        addons.push(AddonFilter::Id);
                        addons.push(AddonFilter::Origin);
                        addons.push(AddonFilter::Infos);
                        addons.push(AddonFilter::Debugs);
                        addons.push(AddonFilter::Traces);
                        addons.push(AddonFilter::Related);

                        #[cfg(feature = "hint_note")]
                        addons.push(AddonFilter::Hint);
                        #[cfg(feature = "hint_note")]
                        addons.push(AddonFilter::Note);

                        #[cfg(feature = "diagnostics")]
                        addons.push(AddonFilter::Diagnostics);

                        #[cfg(feature = "payloads")]
                        addons.push(AddonFilter::Payloads);
                    } else {
                        addons.push(addon);
                    }
                }
            }
        }

        s.replace_range(addon_start..(addon_end + 1), "");
    }

    addons
}

fn get_ids(s: &mut String) -> Vec<LogIdAddonFilter> {
    let mut ids = Vec::new();

    if let (Some(ids_start), Some(ids_end)) = (s.find('['), s.find(']')) {
        if ids_start >= ids_end {
            return ids;
        }

        if let Some(ids_part) = s.get((ids_start + 1)..ids_end) {
            for id_part in ids_part.split('|') {
                if let Ok(id) = LogIdAddonFilter::try_from(id_part.trim()) {
                    ids.push(id);
                }
            }
        }

        s.replace_range(ids_start..(ids_end + 1), "");
    }

    ids
}

fn try_into_log_level(s: &str) -> Option<LogLevel> {
    match s.to_lowercase().as_str() {
        "error" => Some(LogLevel::Error),
        "warn" => Some(LogLevel::Warn),
        "info" => Some(LogLevel::Info),
        "debug" => Some(LogLevel::Debug),
        "trace" | "on" => Some(LogLevel::Trace),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::LogIdFilter;

    #[test]
    fn valid_log_id_filter() {
        let log_id_filter = LogIdFilter::try_from("my_crate::my_id").unwrap();

        assert_eq!(
            log_id_filter.module_path, "my_crate",
            "Module path extraction was not correct"
        );
        assert_eq!(
            log_id_filter.identifier, "my_id",
            "Identifier extraction was not correct"
        );
    }

    #[test]
    fn valid_log_id_filter_with_one_submodule() {
        let log_id_filter = LogIdFilter::try_from("my_crate::my_module::my_id").unwrap();

        assert_eq!(
            log_id_filter.module_path, "my_crate::my_module",
            "Module path extraction was not correct"
        );
        assert_eq!(
            log_id_filter.identifier, "my_id",
            "Identifier extraction was not correct"
        );
    }

    #[test]
    fn valid_log_id_filter_with_submodule() {
        let log_id_filter =
            LogIdFilter::try_from("my_crate::my_module::sub_module::my_id").unwrap();

        assert_eq!(
            log_id_filter.module_path, "my_crate::my_module::sub_module",
            "Module path extraction was not correct"
        );
        assert_eq!(
            log_id_filter.identifier, "my_id",
            "Identifier extraction was not correct"
        );
    }

    #[test]
    fn invalid_log_id_filter_with_empty_module_path() {
        let log_id_filter = LogIdFilter::try_from("::my_id");

        assert!(
            log_id_filter.is_err(),
            "Parsing invalid LogIdFilter did not result in error."
        );
    }

    #[test]
    fn invalid_log_id_filter_with_empty_identifier() {
        let log_id_filter = LogIdFilter::try_from("my_crate::");

        assert!(
            log_id_filter.is_err(),
            "Parsing invalid LogIdFilter did not result in error."
        );
    }
}
