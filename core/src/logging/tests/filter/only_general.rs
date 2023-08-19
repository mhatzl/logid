use crate::{
    log_id::LogLevel,
    logging::{filter::FilterConfig, tests::filter::test_entry},
    new_log_id,
};
use evident::{event::filter::Filter, this_origin};

#[test]
fn logging_turned_off() {
    let filter = FilterConfig::new("off");

    let error_id = new_log_id!("err_id", LogLevel::Error);

    assert!(
        !filter.allow_entry(&test_entry(error_id, this_origin!())),
        "Error level LogId allowed by filter."
    );
}

#[test]
fn empty_filter_means_logging_turned_off() {
    let filter = FilterConfig::new("");

    let error_id = new_log_id!("err_id", LogLevel::Error);

    assert!(
        !filter.allow_entry(&test_entry(error_id, this_origin!())),
        "Error level LogId allowed by filter."
    );
}

#[test]
fn only_allow_error() {
    let filter = FilterConfig::new("error");

    let error_id = new_log_id!("err_id", LogLevel::Error);
    assert!(
        filter.allow_entry(&test_entry(error_id, this_origin!())),
        "Error level LogId not allowed by filter."
    );

    let warn_id = new_log_id!("warn_id", LogLevel::Warn);
    assert!(
        !filter.allow_entry(&test_entry(warn_id, this_origin!())),
        "Warn level LogId allowed by filter."
    );
}

#[test]
fn allow_error_and_warning() {
    let filter = FilterConfig::new("warn");

    let error_id = new_log_id!("err_id", LogLevel::Error);
    assert!(
        filter.allow_entry(&test_entry(error_id, this_origin!())),
        "Error level LogId not allowed by filter."
    );

    let warn_id = new_log_id!("warn_id", LogLevel::Warn);
    assert!(
        filter.allow_entry(&test_entry(warn_id, this_origin!())),
        "Warn level LogId not allowed by filter."
    );

    let info_id = new_log_id!("info_id", LogLevel::Info);
    assert!(
        !filter.allow_entry(&test_entry(info_id, this_origin!())),
        "Info level LogId allowed by filter."
    );
}

#[test]
fn allow_error_warning_and_info() {
    let filter = FilterConfig::new("info");

    let error_id = new_log_id!("err_id", LogLevel::Error);
    assert!(
        filter.allow_entry(&test_entry(error_id, this_origin!())),
        "Error level LogId not allowed by filter."
    );

    let warn_id = new_log_id!("warn_id", LogLevel::Warn);
    assert!(
        filter.allow_entry(&test_entry(warn_id, this_origin!())),
        "Warn level LogId not allowed by filter."
    );

    let info_id = new_log_id!("info_id", LogLevel::Info);
    assert!(
        filter.allow_entry(&test_entry(info_id, this_origin!())),
        "Info level LogId not allowed by filter."
    );

    let debug_id = new_log_id!("debug_id", LogLevel::Debug);
    assert!(
        !filter.allow_entry(&test_entry(debug_id, this_origin!())),
        "Debug level LogId allowed by filter."
    );
}

#[test]
fn allow_error_warning_info_and_debug() {
    let filter = FilterConfig::new("debug");

    let error_id = new_log_id!("err_id", LogLevel::Error);
    assert!(
        filter.allow_entry(&test_entry(error_id, this_origin!())),
        "Error level LogId not allowed by filter."
    );

    let warn_id = new_log_id!("warn_id", LogLevel::Warn);
    assert!(
        filter.allow_entry(&test_entry(warn_id, this_origin!())),
        "Warn level LogId not allowed by filter."
    );

    let info_id = new_log_id!("info_id", LogLevel::Info);
    assert!(
        filter.allow_entry(&test_entry(info_id, this_origin!())),
        "Info level LogId not allowed by filter."
    );

    let debug_id = new_log_id!("debug_id", LogLevel::Debug);
    assert!(
        filter.allow_entry(&test_entry(debug_id, this_origin!())),
        "Debug level LogId not allowed by filter."
    );

    let trace_id = new_log_id!("trace_id", LogLevel::Trace);
    assert!(
        !filter.allow_entry(&test_entry(trace_id, this_origin!())),
        "Trace level LogId allowed by filter."
    );
}

#[test]
fn allow_error_warning_info_debug_and_trace() {
    let filter = FilterConfig::new("trace");

    let error_id = new_log_id!("err_id", LogLevel::Error);
    assert!(
        filter.allow_entry(&test_entry(error_id, this_origin!())),
        "Error level LogId not allowed by filter."
    );

    let warn_id = new_log_id!("warn_id", LogLevel::Warn);
    assert!(
        filter.allow_entry(&test_entry(warn_id, this_origin!())),
        "Warn level LogId not allowed by filter."
    );

    let info_id = new_log_id!("info_id", LogLevel::Info);
    assert!(
        filter.allow_entry(&test_entry(info_id, this_origin!())),
        "Info level LogId not allowed by filter."
    );

    let debug_id = new_log_id!("debug_id", LogLevel::Debug);
    assert!(
        filter.allow_entry(&test_entry(debug_id, this_origin!())),
        "Debug level LogId not allowed by filter."
    );

    let trace_id = new_log_id!("trace_id", LogLevel::Trace);
    assert!(
        filter.allow_entry(&test_entry(trace_id, this_origin!())),
        "Trace level LogId not allowed by filter."
    );
}

#[test]
fn logging_on_equal_to_trace() {
    let filter = FilterConfig::new("on");

    let error_id = new_log_id!("err_id", LogLevel::Error);
    assert!(
        filter.allow_entry(&test_entry(error_id, this_origin!())),
        "Error level LogId not allowed by filter."
    );

    let warn_id = new_log_id!("warn_id", LogLevel::Warn);
    assert!(
        filter.allow_entry(&test_entry(warn_id, this_origin!())),
        "Warn level LogId not allowed by filter."
    );

    let info_id = new_log_id!("info_id", LogLevel::Info);
    assert!(
        filter.allow_entry(&test_entry(info_id, this_origin!())),
        "Info level LogId not allowed by filter."
    );

    let debug_id = new_log_id!("debug_id", LogLevel::Debug);
    assert!(
        filter.allow_entry(&test_entry(debug_id, this_origin!())),
        "Debug level LogId not allowed by filter."
    );

    let trace_id = new_log_id!("trace_id", LogLevel::Trace);
    assert!(
        filter.allow_entry(&test_entry(trace_id, this_origin!())),
        "Trace level LogId not allowed by filter."
    );
}
