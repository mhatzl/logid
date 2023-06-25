use crate::{
    log_id::LogLevel,
    logging::{
        event_entry::AddonKind, filter::InnerLogFilter, intermediary_event::IntermediaryLogEvent,
    },
    new_log_id,
};
use evident::{
    event::{filter::Filter, intermediary::IntermediaryEvent},
    this_origin,
};

#[test]
fn allow_single_id_with_infos_addon() {
    let log_id = new_log_id!("log_id", LogLevel::Info);
    let filter = InnerLogFilter::new(&format!(
        "on[{}::{}::{}(infos)]",
        log_id.get_crate_name(),
        log_id.get_module_path(),
        log_id.get_identifier()
    ));

    let mut log_event = IntermediaryLogEvent::new(log_id, "", this_origin!());
    assert!(
        filter.allow_event(&mut log_event),
        "Explicitly allowed LogId not allowed by filter."
    );

    assert!(
        filter.allow_addon(
            log_id,
            &this_origin!(),
            &AddonKind::Info("Some info".to_string())
        ),
        "Info addon not allowed by filter."
    );
}

#[test]
fn allow_single_id_with_infos_and_origin_addon() {
    let log_id = new_log_id!("log_id", LogLevel::Info);
    let filter = InnerLogFilter::new(&format!(
        "on[{}::{}::{}(infos & origin)]",
        log_id.get_crate_name(),
        log_id.get_module_path(),
        log_id.get_identifier()
    ));

    let mut log_event = IntermediaryLogEvent::new(log_id, "", this_origin!());
    assert!(
        filter.allow_event(&mut log_event),
        "Explicitly allowed LogId not allowed by filter."
    );

    assert!(
        filter.allow_addon(
            log_id,
            &this_origin!(),
            &AddonKind::Info("Some info".to_string())
        ),
        "Info addon not allowed by filter."
    );

    assert!(
        filter.show_origin_info(log_id, &this_origin!()),
        "Origin info not allowed by filter."
    );
}

#[test]
fn allow_single_id_with_related_addon() {
    let log_id = new_log_id!("log_id", LogLevel::Info);
    let filter = InnerLogFilter::new(&format!(
        "on[{}::{}::{}(related)]",
        log_id.get_crate_name(),
        log_id.get_module_path(),
        log_id.get_identifier()
    ));

    let mut log_event = IntermediaryLogEvent::new(log_id, "", this_origin!());
    assert!(
        filter.allow_event(&mut log_event),
        "Explicitly allowed LogId not allowed by filter."
    );
    let finalized = log_event.finalize();

    assert!(
        filter.allow_addon(log_id, &this_origin!(), &AddonKind::Related(finalized)),
        "Related addon not allowed by filter."
    );
}

#[test]
fn allow_single_id_with_all_addons() {
    let log_id = new_log_id!("log_id", LogLevel::Info);
    let filter = InnerLogFilter::new(&format!(
        "on[{}::{}::{}(all)]",
        log_id.get_crate_name(),
        log_id.get_module_path(),
        log_id.get_identifier()
    ));

    let mut log_event = IntermediaryLogEvent::new(log_id, "", this_origin!());
    assert!(
        filter.allow_event(&mut log_event),
        "Explicitly allowed LogId not allowed by filter."
    );

    assert!(
        filter.allow_addon(
            log_id,
            &this_origin!(),
            &AddonKind::Info("Some info".to_string())
        ),
        "Info addon not allowed by filter."
    );

    assert!(
        filter.allow_addon(
            log_id,
            &this_origin!(),
            &AddonKind::Debug("Some info".to_string())
        ),
        "Debug addon not allowed by filter."
    );

    assert!(
        filter.allow_addon(
            log_id,
            &this_origin!(),
            &AddonKind::Trace("Some info".to_string())
        ),
        "Trace addon not allowed by filter."
    );

    assert!(
        filter.show_origin_info(log_id, &this_origin!()),
        "Origin info not allowed by filter."
    );
}

#[test]
fn allow_module_with_infos_addon() {
    let log_id = new_log_id!("log_id", LogLevel::Error);
    let filter = InnerLogFilter::new("logid-core::logid_core(infos) = error");

    let mut log_event = IntermediaryLogEvent::new(log_id, "", this_origin!());
    assert!(
        filter.allow_event(&mut log_event),
        "Error level not allowed by filter."
    );

    assert!(
        filter.allow_addon(
            log_id,
            &this_origin!(),
            &AddonKind::Info("Some info".to_string())
        ),
        "Info addon not allowed by filter."
    );
}

#[test]
fn allow_crate_with_infos_addon() {
    let log_id = new_log_id!("log_id", LogLevel::Error);
    let filter = InnerLogFilter::new("logid-core::logid_core(infos) = error");

    let mut log_event = IntermediaryLogEvent::new(log_id, "", this_origin!());
    assert!(
        filter.allow_event(&mut log_event),
        "Error level not allowed by filter."
    );

    assert!(
        filter.allow_addon(
            log_id,
            &this_origin!(),
            &AddonKind::Info("Some info".to_string())
        ),
        "Info addon not allowed by filter."
    );
}

#[test]
fn allow_general_level_with_infos_addon() {
    let log_id = new_log_id!("log_id", LogLevel::Error);
    let filter = InnerLogFilter::new("error(infos)");

    let mut log_event = IntermediaryLogEvent::new(log_id, "", this_origin!());
    assert!(
        filter.allow_event(&mut log_event),
        "Error level not allowed by filter."
    );

    assert!(
        filter.allow_addon(
            log_id,
            &this_origin!(),
            &AddonKind::Info("Some info".to_string())
        ),
        "Info addon not allowed by filter."
    );
}
