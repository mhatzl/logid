use crate::{
    log_id::LogLevel,
    logging::{filter::InnerLogFilter, intermediary_event::IntermediaryLogEvent},
    new_log_id,
};
use evident::{
    event::{filter::Filter, intermediary::IntermediaryEvent},
    this_origin,
};

#[test]
fn single_module() {
    let filter = InnerLogFilter::new("logid-core::logid_core::logging = warn");

    let warn_id = new_log_id!("warn_id", LogLevel::Warn);
    let mut log_event = IntermediaryLogEvent::new(warn_id, "", this_origin!());
    assert!(
        filter.allow_event(&mut log_event),
        "Warn level LogId not allowed by filter."
    );

    let info_id = new_log_id!("info_id", LogLevel::Info);
    let mut log_event = IntermediaryLogEvent::new(info_id, "", this_origin!());
    assert!(
        !filter.allow_event(&mut log_event),
        "Info level LogId allowed by filter."
    );
}

#[test]
fn only_crate_name() {
    let filter = InnerLogFilter::new("logid-core = warn");

    let warn_id = new_log_id!("warn_id", LogLevel::Warn);
    let mut log_event = IntermediaryLogEvent::new(warn_id, "", this_origin!());
    assert!(
        filter.allow_event(&mut log_event),
        "Warn level LogId not allowed by filter."
    );

    let info_id = new_log_id!("info_id", LogLevel::Info);
    let mut log_event = IntermediaryLogEvent::new(info_id, "", this_origin!());
    assert!(
        !filter.allow_event(&mut log_event),
        "Info level LogId allowed by filter."
    );
}

#[test]
fn multiple_modules() {
    let filter = InnerLogFilter::new(
        "logid-core::logid_core::logging::tests = warn, logid-core::logid_core::logging::event_entry = info",
    );

    let warn_id = new_log_id!("warn_id", LogLevel::Warn);
    let mut log_event = IntermediaryLogEvent::new(warn_id, "", this_origin!());
    assert!(
        filter.allow_event(&mut log_event),
        "Warn level LogId not allowed by filter."
    );

    let info_id = new_log_id!("info_id", LogLevel::Info);
    let mut log_event = IntermediaryLogEvent::new(info_id, "", this_origin!());
    assert!(
        !filter.allow_event(&mut log_event),
        "Info level LogId allowed by filter."
    );
}

#[test]
fn module_with_id() {
    let filter = InnerLogFilter::new(
        "logid-core::logid_core::logging[logid-core::logid_core::logging::tests::filter::only_module::info_id] = error",
    );

    let warn_id = new_log_id!("warn_id", LogLevel::Warn);
    let mut log_event = IntermediaryLogEvent::new(warn_id, "", this_origin!());
    assert!(
        !filter.allow_event(&mut log_event),
        "Warn level LogId allowed by filter."
    );

    let info_id = new_log_id!("info_id", LogLevel::Info);
    let mut log_event = IntermediaryLogEvent::new(info_id, "", this_origin!());
    assert!(
        filter.allow_event(&mut log_event),
        "Explicitly allowed LogId not allowed by filter."
    );
}

#[test]
fn module_with_id_allowed_only() {
    let filter = InnerLogFilter::new(
        "logid-core::logid_core::logging[logid-core::logid_core::logging::tests::filter::only_module::info_id]",
    );

    let error_id = new_log_id!("error_id", LogLevel::Error);
    let mut log_event = IntermediaryLogEvent::new(error_id, "", this_origin!());
    assert!(
        !filter.allow_event(&mut log_event),
        "Error level LogId allowed by filter."
    );

    let info_id = new_log_id!("info_id", LogLevel::Info);
    let mut log_event = IntermediaryLogEvent::new(info_id, "", this_origin!());
    assert!(
        filter.allow_event(&mut log_event),
        "Explicitly allowed LogId not allowed by filter."
    );
}
