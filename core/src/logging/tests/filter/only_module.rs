use crate::{
    log_id::LogLevel,
    logging::{filter::FilterConfig, tests::filter::test_entry},
    new_log_id,
};
use evident::{event::filter::Filter, this_origin};

#[test]
fn single_module() {
    let filter = FilterConfig::new("logid_core::logging = warn");

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
fn only_crate_name_as_module() {
    let filter = FilterConfig::new("logid_core = warn");

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
fn multiple_modules() {
    let filter = FilterConfig::new(
        "logid_core::logging::tests = warn, logid_core::logging::event_entry = info",
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
fn module_with_id() {
    let filter = FilterConfig::new(
        "logid_core::logging[logid_core::logging::tests::filter::only_module::info_id] = error",
    );

    let warn_id = new_log_id!("warn_id", LogLevel::Warn);
    assert!(
        !filter.allow_entry(&test_entry(warn_id, this_origin!())),
        "Warn level LogId allowed by filter."
    );

    let info_id = new_log_id!("info_id", LogLevel::Info);
    assert!(
        filter.allow_entry(&test_entry(info_id, this_origin!())),
        "Explicitly allowed LogId not allowed by filter."
    );
}

#[test]
fn module_with_id_allowed_only() {
    let filter = FilterConfig::new(
        "logid_core::logging[logid_core::logging::tests::filter::only_module::info_id]",
    );

    let error_id = new_log_id!("error_id", LogLevel::Error);
    assert!(
        !filter.allow_entry(&test_entry(error_id, this_origin!())),
        "Error level LogId allowed by filter."
    );

    let info_id = new_log_id!("info_id", LogLevel::Info);
    assert!(
        filter.allow_entry(&test_entry(info_id, this_origin!())),
        "Explicitly allowed LogId not allowed by filter."
    );
}
