use crate::log_id::LogId;

use self::{
    event_entry::LogEventEntry, filter::LogFilter, intermediary_event::IntermediaryLogEvent,
    msg::LogMsg,
};

pub mod event_entry;
pub mod filter;
pub mod intermediary_event;
pub mod msg;

#[cfg(test)]
pub mod tests;

evident::create_static_publisher!(
    pub LOGGER,
    id_type = LogId,
    msg_type = LogMsg,
    entry_type = LogEventEntry,
    interm_event_type = IntermediaryLogEvent,
    filter_type = LogFilter,
    filter = LogFilter::new(),
    capture_channel_bound = 1000,
    subscription_channel_bound = 1000,
    capture_mode = evident::publisher::CaptureMode::Blocking,
    timestamp_kind = evident::publisher::EventTimestampKind::Captured
);
