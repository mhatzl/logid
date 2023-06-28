use crate::{
    log_id::LogLevel,
    logging::{filter::InnerLogFilter, tests::filter::test_event},
    new_log_id,
};
use evident::{event::filter::Filter, this_origin};

#[test]
fn logging_turned_off() {
    let filter = InnerLogFilter::new("off");

    let error_id = new_log_id!("err_id", LogLevel::Error);

    assert!(
        !filter.allow_event(&test_event(error_id, this_origin!())),
        "Error level LogId allowed by filter."
    );
}

#[test]
fn empty_filter_means_logging_turned_off() {
    let filter = InnerLogFilter::new("");

    let error_id = new_log_id!("err_id", LogLevel::Error);

    assert!(
        !filter.allow_event(&test_event(error_id, this_origin!())),
        "Error level LogId allowed by filter."
    );
}

#[test]
fn only_allow_error() {
    let filter = InnerLogFilter::new("error");

    let error_id = new_log_id!("err_id", LogLevel::Error);
    assert!(
        filter.allow_event(&test_event(error_id, this_origin!())),
        "Error level LogId not allowed by filter."
    );

    let warn_id = new_log_id!("warn_id", LogLevel::Warn);
    assert!(
        !filter.allow_event(&test_event(warn_id, this_origin!())),
        "Warn level LogId allowed by filter."
    );
}

#[test]
fn allow_error_and_warning() {
    let filter = InnerLogFilter::new("warn");

    let error_id = new_log_id!("err_id", LogLevel::Error);
    assert!(
        filter.allow_event(&test_event(error_id, this_origin!())),
        "Error level LogId not allowed by filter."
    );

    let warn_id = new_log_id!("warn_id", LogLevel::Warn);
    assert!(
        filter.allow_event(&test_event(warn_id, this_origin!())),
        "Warn level LogId not allowed by filter."
    );

    let info_id = new_log_id!("info_id", LogLevel::Info);
    assert!(
        !filter.allow_event(&test_event(info_id, this_origin!())),
        "Info level LogId allowed by filter."
    );
}

#[test]
fn allow_error_warning_and_info() {
    let filter = InnerLogFilter::new("info");

    let error_id = new_log_id!("err_id", LogLevel::Error);
    assert!(
        filter.allow_event(&test_event(error_id, this_origin!())),
        "Error level LogId not allowed by filter."
    );

    let warn_id = new_log_id!("warn_id", LogLevel::Warn);
    assert!(
        filter.allow_event(&test_event(warn_id, this_origin!())),
        "Warn level LogId not allowed by filter."
    );

    let info_id = new_log_id!("info_id", LogLevel::Info);
    assert!(
        filter.allow_event(&test_event(info_id, this_origin!())),
        "Info level LogId not allowed by filter."
    );

    let debug_id = new_log_id!("debug_id", LogLevel::Debug);
    assert!(
        !filter.allow_event(&test_event(debug_id, this_origin!())),
        "Debug level LogId allowed by filter."
    );
}

#[test]
fn allow_error_warning_info_and_debug() {
    let filter = InnerLogFilter::new("debug");

    let error_id = new_log_id!("err_id", LogLevel::Error);
    assert!(
        filter.allow_event(&test_event(error_id, this_origin!())),
        "Error level LogId not allowed by filter."
    );

    let warn_id = new_log_id!("warn_id", LogLevel::Warn);
    assert!(
        filter.allow_event(&test_event(warn_id, this_origin!())),
        "Warn level LogId not allowed by filter."
    );

    let info_id = new_log_id!("info_id", LogLevel::Info);
    assert!(
        filter.allow_event(&test_event(info_id, this_origin!())),
        "Info level LogId not allowed by filter."
    );

    let debug_id = new_log_id!("debug_id", LogLevel::Debug);
    assert!(
        filter.allow_event(&test_event(debug_id, this_origin!())),
        "Debug level LogId not allowed by filter."
    );

    let trace_id = new_log_id!("trace_id", LogLevel::Trace);
    assert!(
        !filter.allow_event(&test_event(trace_id, this_origin!())),
        "Trace level LogId allowed by filter."
    );
}

#[test]
fn allow_error_warning_info_debug_and_trace() {
    let filter = InnerLogFilter::new("trace");

    let error_id = new_log_id!("err_id", LogLevel::Error);
    assert!(
        filter.allow_event(&test_event(error_id, this_origin!())),
        "Error level LogId not allowed by filter."
    );

    let warn_id = new_log_id!("warn_id", LogLevel::Warn);
    assert!(
        filter.allow_event(&test_event(warn_id, this_origin!())),
        "Warn level LogId not allowed by filter."
    );

    let info_id = new_log_id!("info_id", LogLevel::Info);
    assert!(
        filter.allow_event(&test_event(info_id, this_origin!())),
        "Info level LogId not allowed by filter."
    );

    let debug_id = new_log_id!("debug_id", LogLevel::Debug);
    assert!(
        filter.allow_event(&test_event(debug_id, this_origin!())),
        "Debug level LogId not allowed by filter."
    );

    let trace_id = new_log_id!("trace_id", LogLevel::Trace);
    assert!(
        filter.allow_event(&test_event(trace_id, this_origin!())),
        "Trace level LogId not allowed by filter."
    );
}

#[test]
fn logging_on_equal_to_trace() {
    let filter = InnerLogFilter::new("on");

    let error_id = new_log_id!("err_id", LogLevel::Error);
    assert!(
        filter.allow_event(&test_event(error_id, this_origin!())),
        "Error level LogId not allowed by filter."
    );

    let warn_id = new_log_id!("warn_id", LogLevel::Warn);
    assert!(
        filter.allow_event(&test_event(warn_id, this_origin!())),
        "Warn level LogId not allowed by filter."
    );

    let info_id = new_log_id!("info_id", LogLevel::Info);
    assert!(
        filter.allow_event(&test_event(info_id, this_origin!())),
        "Info level LogId not allowed by filter."
    );

    let debug_id = new_log_id!("debug_id", LogLevel::Debug);
    assert!(
        filter.allow_event(&test_event(debug_id, this_origin!())),
        "Debug level LogId not allowed by filter."
    );

    let trace_id = new_log_id!("trace_id", LogLevel::Trace);
    assert!(
        filter.allow_event(&test_event(trace_id, this_origin!())),
        "Trace level LogId not allowed by filter."
    );
}
