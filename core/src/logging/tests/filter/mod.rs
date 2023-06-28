use evident::event::{intermediary::IntermediaryEvent, origin::Origin, Event};

use crate::{
    log_id::LogId,
    logging::{event_entry::LogEventEntry, intermediary_event::IntermediaryLogEvent},
};

pub mod addons;
pub mod global_ids;
pub mod only_general;
pub mod only_module;
pub mod rule_mix;

fn test_event(log_id: LogId, origin: Origin) -> Event<LogId, LogEventEntry> {
    Event::new(IntermediaryLogEvent::new(log_id, "", origin).take_entry())
}
