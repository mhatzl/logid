use evident::{
    event::{filter::Filter, intermediary::IntermediaryEvent},
    this_origin,
};
use logid_core::{
    log_id::LogLevel,
    logging::{filter::LogFilter, intermediary_event::IntermediaryLogEvent},
    new_log_id,
};

#[test]
fn allow_single_id() {
    let log_id = new_log_id!("log_id", LogLevel::Info);
    let filter = LogFilter::new(&format!(
        "on[{}::{}::{}]",
        log_id.get_crate_name(),
        log_id.get_module_path(),
        log_id.get_identifier()
    ));

    let mut log_event = IntermediaryLogEvent::new(log_id, "", this_origin!());
    assert!(
        filter.allow_event(&mut log_event),
        "Explicitly allowed LogId not allowed by filter."
    );
}

#[test]
fn allow_multiple_ids() {
    let log_id_1 = new_log_id!("log_id_1", LogLevel::Info);
    let log_id_2 = new_log_id!("log_id_2", LogLevel::Debug);

    let filter = LogFilter::new(&format!(
        "on[{}::{}::{} | {}::{}::{}]",
        log_id_1.get_crate_name(),
        log_id_1.get_module_path(),
        log_id_1.get_identifier(),
        log_id_2.get_crate_name(),
        log_id_2.get_module_path(),
        log_id_2.get_identifier()
    ));

    let mut log_event = IntermediaryLogEvent::new(log_id_1, "", this_origin!());
    assert!(
        filter.allow_event(&mut log_event),
        "Explicitly allowed first LogId not allowed by filter."
    );

    let mut log_event = IntermediaryLogEvent::new(log_id_2, "", this_origin!());
    assert!(
        filter.allow_event(&mut log_event),
        "Explicitly allowed second LogId not allowed by filter."
    );
}

#[test]
fn invalid_ids_syntax() {
    let log_id = new_log_id!("log_id", LogLevel::Info);

    let filter = LogFilter::new(&format!(
        "on]{}::{}::{}[",
        log_id.get_crate_name(),
        log_id.get_module_path(),
        log_id.get_identifier(),
    ));

    let mut log_event = IntermediaryLogEvent::new(log_id, "", this_origin!());
    assert!(
        !filter.allow_event(&mut log_event),
        "Invalid filter syntax allowed LogId by filter."
    );
}
