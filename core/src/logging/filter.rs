use std::sync::{Arc, RwLock};

use evident::event::origin::Origin;

use crate::log_id::{LogId, LogLevel};

use super::event_entry::{AddonKind, LogEventEntry};

#[derive(Default, Debug)]
pub struct LogFilter {
    filter: Arc<RwLock<InnerLogFilter>>,
}

impl LogFilter {
    pub fn new() -> Self {
        LogFilter {
            filter: Arc::new(RwLock::new(InnerLogFilter::new(&filter_config()))),
        }
    }

    pub fn set_filter(&self, config: &str) -> Result<(), FilterError> {
        match self.filter.write() {
            Ok(mut locked_filter) => {
                locked_filter.replace(InnerLogFilter::new(config));
            }
            Err(_) => todo!(),
        }

        Ok(())
    }

    pub fn allow_addon(&self, id: LogId, origin: &Origin, addon: &AddonKind) -> bool {
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

impl evident::event::filter::Filter<LogId, LogEventEntry> for LogFilter {
    fn allow_event(
        &self,
        event: &mut impl evident::event::intermediary::IntermediaryEvent<LogId, LogEventEntry>,
    ) -> bool {
        match self.filter.read() {
            Ok(locked_filter) => locked_filter.allow_event(event),
            Err(_) => false,
        }
    }
}

#[derive(Default, Debug, PartialEq, Eq)]
pub enum AddonFilter {
    #[default]
    Infos,
    Debugs,
    Traces,
    Origin,
    Related,
    AllAllowed,

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

            #[cfg(feature = "diagnostics")]
            AddonKind::Diagnostic(_) => AddonFilter::Diagnostics,

            #[cfg(feature = "payloads")]
            AddonKind::Payload(_) => AddonFilter::Payloads,
        }
    }
}

impl TryFrom<&str> for AddonFilter {
    type Error = FilterError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let addon = match value {
            "infos" => AddonFilter::Infos,
            "debugs" => AddonFilter::Debugs,
            "traces" => AddonFilter::Traces,
            "origin" => AddonFilter::Origin,
            "related" => AddonFilter::Related,
            "all" => AddonFilter::AllAllowed,

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

#[derive(Default, Debug)]
pub struct LogIdFilter {
    crate_name: String,
    module_path: String,
    identifier: String,
}

impl PartialEq<LogId> for LogIdFilter {
    fn eq(&self, other: &LogId) -> bool {
        self.crate_name == other.crate_name
            && self.module_path == other.module_path
            && self.identifier == other.identifier
    }
}

impl TryFrom<&str> for LogIdFilter {
    type Error = FilterError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut parts = value.split("::");
        let len = parts.clone().count();
        if len < 2 {
            return Err(FilterError::ParsingLogId(value.to_string()));
        }

        let crate_name = parts
            .next()
            .ok_or(FilterError::ParsingLogId(value.to_string()))?
            .trim();

        if crate_name.is_empty() {
            return Err(FilterError::ParsingLogId(value.to_string()));
        }

        if len == 2 {
            // Form: <crate name>::<identifier>
            let identifier = parts
                .next()
                .ok_or(FilterError::ParsingLogId(value.to_string()))?
                .trim();

            if identifier.is_empty() {
                return Err(FilterError::ParsingLogId(value.to_string()));
            }

            return Ok(LogIdFilter {
                crate_name: crate_name.to_string(),
                identifier: identifier.to_string(),
                ..Default::default()
            });
        }

        // Form: <crate name>::<module path (might have more '::' between)>::<identifier>

        let mut module_parts = String::default();

        for part in parts.clone().take(len - 2) {
            if part.trim().is_empty() {
                return Err(FilterError::ParsingLogId(value.to_string()));
            }

            module_parts.push_str(part);
            module_parts.push_str("::");
        }
        let module_path = module_parts
            .strip_suffix("::")
            .ok_or(FilterError::ParsingLogId(value.to_string()))?;

        // Skip module path parts, since iterator above was cloned
        let mut parts = parts.skip(len - 2);
        let identifier = parts
            .next()
            .ok_or(FilterError::ParsingLogId(value.to_string()))?
            .trim();

        if identifier.is_empty() || parts.next().is_some() {
            return Err(FilterError::ParsingLogId(value.to_string()));
        }

        Ok(LogIdFilter {
            crate_name: crate_name.to_string(),
            module_path: module_path.to_string(),
            identifier: identifier.to_string(),
        })
    }
}

#[derive(Default, Debug)]
pub struct LogIdAddonFilter {
    log_id: LogIdFilter,
    allowed_addons: Vec<AddonFilter>,
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

#[derive(Default, Debug)]
pub struct LogIdModuleFilter {
    no_general_logging: bool,
    origin_crate_name: String,
    origin_module_path: String,
    level: LogLevel,
    allowed_ids: Vec<LogIdAddonFilter>,
    allowed_addons: Vec<AddonFilter>,
}

impl LogIdModuleFilter {
    pub fn origin_in_module(&self, origin: &Origin) -> bool {
        if self.origin_crate_name != origin.crate_name {
            return false;
        }
        origin.module_path.starts_with(&self.origin_module_path)
    }

    pub fn event_allowed(&self, id: LogId, origin: &Origin) -> bool {
        if !self.origin_in_module(origin) {
            return false;
        }

        (!self.no_general_logging && self.level >= id.log_level)
            || id_allowed(&self.allowed_ids, id)
    }

    pub fn addon_allowed(&self, id: LogId, origin: &Origin, addon: &AddonFilter) -> bool {
        if !self.origin_in_module(origin) {
            return false;
        }

        (!self.no_general_logging
            && self.level >= id.log_level
            && self.allowed_addons.contains(addon))
            || addon_allowed(&self.allowed_ids, id, addon)
    }

    fn try_from(
        s: &str,
        ids: Vec<LogIdAddonFilter>,
        addons: Vec<AddonFilter>,
    ) -> Result<Self, FilterError> {
        let mut module_parts = s.split('=');
        let len = module_parts.clone().count();
        if len != 1 && len != 2 {
            return Err(FilterError::ParsingModule(s.to_string()));
        }

        let mut origin_crate_name = String::new();
        let mut origin_module_path = String::new();

        let module = module_parts
            .next()
            .ok_or(FilterError::ParsingModule(s.to_string()))?
            .trim();

        if module.is_empty() {
            return Err(FilterError::ParsingModule(s.to_string()));
        }

        if let Some((crate_name, module_path)) = module.split_once("::") {
            if crate_name.is_empty() || module_path.is_empty() {
                return Err(FilterError::ParsingModule(s.to_string()));
            }

            origin_crate_name.push_str(crate_name);
            origin_module_path.push_str(module_path);
        } else {
            origin_crate_name.push_str(module);
        }

        if len == 1 {
            return Ok(LogIdModuleFilter {
                no_general_logging: true,
                origin_crate_name,
                origin_module_path,
                allowed_ids: ids,
                allowed_addons: addons,
                ..Default::default()
            });
        }

        let level_part = module_parts
            .next()
            .ok_or(FilterError::ParsingModule(s.to_string()))?
            .trim();

        let level =
            try_into_log_level(level_part).ok_or(FilterError::ParsingModule(s.to_string()))?;

        Ok(LogIdModuleFilter {
            no_general_logging: false,
            origin_crate_name,
            origin_module_path,
            level,
            allowed_ids: ids,
            allowed_addons: addons,
        })
    }
}

#[derive(Default, Debug)]
pub(super) struct InnerLogFilter {
    no_general_logging: bool,
    general_level: LogLevel,
    general_addons: Vec<AddonFilter>,
    /// LogIds set with `on[LogId]`
    allowed_global_ids: Vec<LogIdAddonFilter>,
    allowed_modules: Vec<LogIdModuleFilter>,
}

impl InnerLogFilter {
    pub fn new(filter: &str) -> Self {
        if filter.trim().is_empty() || filter.to_lowercase() == "off" {
            return InnerLogFilter {
                no_general_logging: true,
                ..Default::default()
            };
        }

        let mut log_filter = InnerLogFilter {
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
}

impl evident::event::filter::Filter<LogId, LogEventEntry> for InnerLogFilter {
    fn allow_event(
        &self,
        event: &mut impl evident::event::intermediary::IntermediaryEvent<LogId, LogEventEntry>,
    ) -> bool {
        // Note: `Error` starts at `0`
        if !self.no_general_logging && self.general_level >= event.get_event_id().log_level {
            return true;
        }

        id_allowed(&self.allowed_global_ids, *event.get_event_id())
            || id_allowed_in_origin(
                &self.allowed_modules,
                *event.get_event_id(),
                &event.get_entry().origin,
            )
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
                        addons.push(AddonFilter::Infos);
                        addons.push(AddonFilter::Debugs);
                        addons.push(AddonFilter::Traces);
                        addons.push(AddonFilter::Origin);
                        addons.push(AddonFilter::Related);

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
        let log_id_filter = LogIdFilter::try_from("my_crate::my_module::my_id").unwrap();

        assert_eq!(
            log_id_filter.crate_name, "my_crate",
            "Crate name extraction was not correct"
        );
        assert_eq!(
            log_id_filter.module_path, "my_module",
            "Module path extraction was not correct"
        );
        assert_eq!(
            log_id_filter.identifier, "my_id",
            "Identifier extraction was not correct"
        );
    }

    #[test]
    fn valid_log_id_filter_without_module() {
        let log_id_filter = LogIdFilter::try_from("my_crate::my_id").unwrap();

        assert_eq!(
            log_id_filter.crate_name, "my_crate",
            "Crate name extraction was not correct"
        );
        assert_eq!(
            log_id_filter.module_path, "",
            "Module path extraction was not correct"
        );
        assert_eq!(
            log_id_filter.identifier, "my_id",
            "Identifier extraction was not correct"
        );
    }

    #[test]
    fn valid_log_id_filter_with_sub_module() {
        let log_id_filter =
            LogIdFilter::try_from("my_crate::my_module::sub_module::my_id").unwrap();

        assert_eq!(
            log_id_filter.crate_name, "my_crate",
            "Crate name extraction was not correct"
        );
        assert_eq!(
            log_id_filter.module_path, "my_module::sub_module",
            "Module path extraction was not correct"
        );
        assert_eq!(
            log_id_filter.identifier, "my_id",
            "Identifier extraction was not correct"
        );
    }

    #[test]
    fn invalid_log_id_filter_with_empty_crate_name() {
        let log_id_filter = LogIdFilter::try_from("::my_module::my_id");

        assert!(
            log_id_filter.is_err(),
            "Parsing invalid LogIdFilter did not result in error."
        );
    }

    #[test]
    fn invalid_log_id_filter_with_empty_module_path() {
        let log_id_filter = LogIdFilter::try_from("my_crate::::my_id");

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
