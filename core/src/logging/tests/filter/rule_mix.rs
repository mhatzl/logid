use crate::{
    log_id::LogLevel,
    logging::{filter::FilterConfig, tests::filter::test_entry},
    new_log_id,
};
use evident::{event::filter::Filter, this_origin};

#[test]
fn global_id_and_general_error() {
    let log_id = new_log_id!("log_id", LogLevel::Info);
    let err_id = new_log_id!("err_id", LogLevel::Error);
    let warn_id = new_log_id!("warn_id", LogLevel::Warn);

    let filter = FilterConfig::new(&format!(
        "on[{}::{}], error",
        log_id.get_module_path(),
        log_id.get_identifier()
    ));

    assert!(
        filter.allow_entry(&test_entry(log_id, this_origin!())),
        "Explicitly allowed LogId not allowed by filter."
    );

    assert!(
        filter.allow_entry(&test_entry(err_id, this_origin!())),
        "Error level not allowed by filter."
    );

    assert!(
        !filter.allow_entry(&test_entry(warn_id, this_origin!())),
        "Warn level allowed by filter."
    );
}
