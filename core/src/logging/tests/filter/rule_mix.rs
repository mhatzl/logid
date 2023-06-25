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

    let mut log_event = IntermediaryLogEvent::new(log_id, "", this_origin!());
    assert!(
        filter.allow_event(&mut log_event),
        "Explicitly allowed LogId not allowed by filter."
    );

    let mut log_event = IntermediaryLogEvent::new(err_id, "", this_origin!());
    assert!(
        filter.allow_event(&mut log_event),
        "Error level not allowed by filter."
    );

    let mut log_event = IntermediaryLogEvent::new(warn_id, "", this_origin!());
    assert!(
        !filter.allow_event(&mut log_event),
        "Warn level allowed by filter."
    );
}
