use crate::log_id::LogId;

use self::{event_entry::LogEventEntry, intermediary_event::IntermediaryLogEvent};

pub mod error_event;
pub mod event_entry;
pub mod intermediary_event;

evident::create_static_publisher!(
    pub LOGGER,
    LogId,
    LogEventEntry,
    IntermediaryLogEvent,
    CAPTURE_CHANNEL_BOUND = 1000,
    SUBSCRIPTION_CHANNEL_BOUND = 500,
    non_blocking = true
);
evident::create_set_event_macro!(
    logid_core::log_id::LogId,
    logid_core::logging::event_entry::LogEventEntry,
    logid_core::logging::intermediary_event::IntermediaryLogEvent
);
