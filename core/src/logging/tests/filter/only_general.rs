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
fn logging_turned_off() {
    let filter = InnerLogFilter::new("off");

    let error_id = new_log_id!("err_id", LogLevel::Error);
    let mut error_event = IntermediaryLogEvent::new(error_id, "", this_origin!());
    assert!(
        !filter.allow_event(&mut error_event),
        "Error level LogId allowed by filter."
    );
}

#[test]
fn empty_filter_means_logging_turned_off() {
    let filter = InnerLogFilter::new("");

    let error_id = new_log_id!("err_id", LogLevel::Error);
    let mut error_event = IntermediaryLogEvent::new(error_id, "", this_origin!());
    assert!(
        !filter.allow_event(&mut error_event),
        "Error level LogId allowed by filter."
    );
}

#[test]
fn only_allow_error() {
    let filter = InnerLogFilter::new("error");

    let error_id = new_log_id!("err_id", LogLevel::Error);
    let mut error_event = IntermediaryLogEvent::new(error_id, "", this_origin!());
    assert!(
        filter.allow_event(&mut error_event),
        "Error level LogId not allowed by filter."
    );

    let warn_id = new_log_id!("warn_id", LogLevel::Warn);
    let mut warn_event = IntermediaryLogEvent::new(warn_id, "", this_origin!());
    assert!(
        !filter.allow_event(&mut warn_event),
        "Warn level LogId allowed by filter."
    );
}

#[test]
fn allow_error_and_warning() {
    let filter = InnerLogFilter::new("warn");

    let error_id = new_log_id!("err_id", LogLevel::Error);
    let mut error_event = IntermediaryLogEvent::new(error_id, "", this_origin!());
    assert!(
        filter.allow_event(&mut error_event),
        "Error level LogId not allowed by filter."
    );

    let warn_id = new_log_id!("warn_id", LogLevel::Warn);
    let mut warn_event = IntermediaryLogEvent::new(warn_id, "", this_origin!());
    assert!(
        filter.allow_event(&mut warn_event),
        "Warn level LogId not allowed by filter."
    );

    let info_id = new_log_id!("info_id", LogLevel::Info);
    let mut info_event = IntermediaryLogEvent::new(info_id, "", this_origin!());
    assert!(
        !filter.allow_event(&mut info_event),
        "Info level LogId allowed by filter."
    );
}

#[test]
fn allow_error_warning_and_info() {
    let filter = InnerLogFilter::new("info");

    let error_id = new_log_id!("err_id", LogLevel::Error);
    let mut error_event = IntermediaryLogEvent::new(error_id, "", this_origin!());
    assert!(
        filter.allow_event(&mut error_event),
        "Error level LogId not allowed by filter."
    );

    let warn_id = new_log_id!("warn_id", LogLevel::Warn);
    let mut warn_event = IntermediaryLogEvent::new(warn_id, "", this_origin!());
    assert!(
        filter.allow_event(&mut warn_event),
        "Warn level LogId not allowed by filter."
    );

    let info_id = new_log_id!("info_id", LogLevel::Info);
    let mut info_event = IntermediaryLogEvent::new(info_id, "", this_origin!());
    assert!(
        filter.allow_event(&mut info_event),
        "Info level LogId not allowed by filter."
    );

    let debug_id = new_log_id!("debug_id", LogLevel::Debug);
    let mut debug_event = IntermediaryLogEvent::new(debug_id, "", this_origin!());
    assert!(
        !filter.allow_event(&mut debug_event),
        "Debug level LogId allowed by filter."
    );
}

#[test]
fn allow_error_warning_info_and_debug() {
    let filter = InnerLogFilter::new("debug");

    let error_id = new_log_id!("err_id", LogLevel::Error);
    let mut error_event = IntermediaryLogEvent::new(error_id, "", this_origin!());
    assert!(
        filter.allow_event(&mut error_event),
        "Error level LogId not allowed by filter."
    );

    let warn_id = new_log_id!("warn_id", LogLevel::Warn);
    let mut warn_event = IntermediaryLogEvent::new(warn_id, "", this_origin!());
    assert!(
        filter.allow_event(&mut warn_event),
        "Warn level LogId not allowed by filter."
    );

    let info_id = new_log_id!("info_id", LogLevel::Info);
    let mut info_event = IntermediaryLogEvent::new(info_id, "", this_origin!());
    assert!(
        filter.allow_event(&mut info_event),
        "Info level LogId not allowed by filter."
    );

    let debug_id = new_log_id!("debug_id", LogLevel::Debug);
    let mut debug_event = IntermediaryLogEvent::new(debug_id, "", this_origin!());
    assert!(
        filter.allow_event(&mut debug_event),
        "Debug level LogId not allowed by filter."
    );

    let trace_id = new_log_id!("trace_id", LogLevel::Trace);
    let mut trace_event = IntermediaryLogEvent::new(trace_id, "", this_origin!());
    assert!(
        !filter.allow_event(&mut trace_event),
        "Trace level LogId allowed by filter."
    );
}

#[test]
fn allow_error_warning_info_debug_and_trace() {
    let filter = InnerLogFilter::new("trace");

    let error_id = new_log_id!("err_id", LogLevel::Error);
    let mut error_event = IntermediaryLogEvent::new(error_id, "", this_origin!());
    assert!(
        filter.allow_event(&mut error_event),
        "Error level LogId not allowed by filter."
    );

    let warn_id = new_log_id!("warn_id", LogLevel::Warn);
    let mut warn_event = IntermediaryLogEvent::new(warn_id, "", this_origin!());
    assert!(
        filter.allow_event(&mut warn_event),
        "Warn level LogId not allowed by filter."
    );

    let info_id = new_log_id!("info_id", LogLevel::Info);
    let mut info_event = IntermediaryLogEvent::new(info_id, "", this_origin!());
    assert!(
        filter.allow_event(&mut info_event),
        "Info level LogId not allowed by filter."
    );

    let debug_id = new_log_id!("debug_id", LogLevel::Debug);
    let mut debug_event = IntermediaryLogEvent::new(debug_id, "", this_origin!());
    assert!(
        filter.allow_event(&mut debug_event),
        "Debug level LogId not allowed by filter."
    );

    let trace_id = new_log_id!("trace_id", LogLevel::Trace);
    let mut trace_event = IntermediaryLogEvent::new(trace_id, "", this_origin!());
    assert!(
        filter.allow_event(&mut trace_event),
        "Trace level LogId not allowed by filter."
    );
}

#[test]
fn logging_on_equal_to_trace() {
    let filter = InnerLogFilter::new("on");

    let error_id = new_log_id!("err_id", LogLevel::Error);
    let mut error_event = IntermediaryLogEvent::new(error_id, "", this_origin!());
    assert!(
        filter.allow_event(&mut error_event),
        "Error level LogId not allowed by filter."
    );

    let warn_id = new_log_id!("warn_id", LogLevel::Warn);
    let mut warn_event = IntermediaryLogEvent::new(warn_id, "", this_origin!());
    assert!(
        filter.allow_event(&mut warn_event),
        "Warn level LogId not allowed by filter."
    );

    let info_id = new_log_id!("info_id", LogLevel::Info);
    let mut info_event = IntermediaryLogEvent::new(info_id, "", this_origin!());
    assert!(
        filter.allow_event(&mut info_event),
        "Info level LogId not allowed by filter."
    );

    let debug_id = new_log_id!("debug_id", LogLevel::Debug);
    let mut debug_event = IntermediaryLogEvent::new(debug_id, "", this_origin!());
    assert!(
        filter.allow_event(&mut debug_event),
        "Debug level LogId not allowed by filter."
    );

    let trace_id = new_log_id!("trace_id", LogLevel::Trace);
    let mut trace_event = IntermediaryLogEvent::new(trace_id, "", this_origin!());
    assert!(
        filter.allow_event(&mut trace_event),
        "Trace level LogId not allowed by filter."
    );
}
