use crate::{
    log_id::LogLevel,
    logging::{filter::InnerLogFilter, tests::filter::test_event},
    new_log_id,
};
use evident::{event::filter::Filter, this_origin};

#[test]
fn allow_single_id() {
    let log_id = new_log_id!("log_id", LogLevel::Info);
    let filter = InnerLogFilter::new(&format!(
        "on[{}::{}::{}]",
        log_id.get_crate_name(),
        log_id.get_module_path(),
        log_id.get_identifier()
    ));

    assert!(
        filter.allow_event(&test_event(log_id, this_origin!())),
        "Explicitly allowed LogId not allowed by filter."
    );
}

#[test]
fn allow_multiple_ids() {
    let log_id_1 = new_log_id!("log_id_1", LogLevel::Info);
    let log_id_2 = new_log_id!("log_id_2", LogLevel::Debug);

    let filter = InnerLogFilter::new(&format!(
        "on[{}::{}::{} | {}::{}::{}]",
        log_id_1.get_crate_name(),
        log_id_1.get_module_path(),
        log_id_1.get_identifier(),
        log_id_2.get_crate_name(),
        log_id_2.get_module_path(),
        log_id_2.get_identifier()
    ));

    assert!(
        filter.allow_event(&test_event(log_id_1, this_origin!())),
        "Explicitly allowed first LogId not allowed by filter."
    );

    assert!(
        filter.allow_event(&test_event(log_id_2, this_origin!())),
        "Explicitly allowed second LogId not allowed by filter."
    );
}

#[test]
fn invalid_ids_syntax() {
    let log_id = new_log_id!("log_id", LogLevel::Info);

    let filter = InnerLogFilter::new(&format!(
        "on]{}::{}::{}[",
        log_id.get_crate_name(),
        log_id.get_module_path(),
        log_id.get_identifier(),
    ));

    assert!(
        !filter.allow_event(&test_event(log_id, this_origin!())),
        "Invalid filter syntax allowed LogId by filter."
    );
}
