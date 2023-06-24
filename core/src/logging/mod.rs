use crate::log_id::LogId;

use self::{
    event_entry::LogEventEntry, filter::LogFilter, intermediary_event::IntermediaryLogEvent,
};

pub mod event_entry;
pub mod filter;
pub mod intermediary_event;

evident::create_static_publisher!(
    pub LOGGER,
    id_type = LogId,
    entry_type = LogEventEntry,
    interm_event_type = IntermediaryLogEvent,
    filter_type = LogFilter,
    filter = LogFilter::new(""),
    capture_channel_bound = 1000,
    subscription_channel_bound = 500,
    non_blocking = true
);
