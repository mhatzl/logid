use crate::{
    log_id::LogLevel,
    logging::{filter::InnerLogFilter, tests::filter::test_event},
    new_log_id,
};
use evident::{event::filter::Filter, this_origin};

#[test]
fn global_id_and_general_error() {
    let log_id = new_log_id!("log_id", LogLevel::Info);
    let err_id = new_log_id!("err_id", LogLevel::Error);
    let warn_id = new_log_id!("warn_id", LogLevel::Warn);

    let filter = InnerLogFilter::new(&format!(
        "on[{}::{}::{}], error",
        log_id.get_crate_name(),
        log_id.get_module_path(),
        log_id.get_identifier()
    ));

    assert!(
        filter.allow_event(&test_event(log_id, this_origin!())),
        "Explicitly allowed LogId not allowed by filter."
    );

    assert!(
        filter.allow_event(&test_event(err_id, this_origin!())),
        "Error level not allowed by filter."
    );

    assert!(
        !filter.allow_event(&test_event(warn_id, this_origin!())),
        "Warn level allowed by filter."
    );
}
