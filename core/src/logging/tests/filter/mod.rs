use evident::event::{entry::EventEntry, origin::Origin};

use crate::{log_id::LogId, logging::event_entry::LogEventEntry};

pub mod addons;
pub mod global_ids;
pub mod only_general;
pub mod only_module;
pub mod rule_mix;

fn test_entry(log_id: LogId, origin: Origin) -> LogEventEntry {
    LogEventEntry::new(log_id, "", origin)
}
