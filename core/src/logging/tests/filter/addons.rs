use crate::{
    log_id::LogLevel,
    logging::{
        event_entry::AddonKind, filter::FilterConfig, intermediary_event::IntermediaryLogEvent,
        msg::NO_MSG, tests::filter::test_entry,
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
    let filter = FilterConfig::new(&format!(
        "on[{}::{}(infos)]",
        log_id.get_module_path(),
        log_id.get_identifier()
    ));

    assert!(
        filter.allow_entry(&test_entry(log_id, this_origin!())),
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
    let filter = FilterConfig::new(&format!(
        "on[{}::{}(infos & origin)]",
        log_id.get_module_path(),
        log_id.get_identifier()
    ));

    assert!(
        filter.allow_entry(&test_entry(log_id, this_origin!())),
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
    let filter = FilterConfig::new(&format!(
        "on[{}::{}(related)]",
        log_id.get_module_path(),
        log_id.get_identifier()
    ));

    assert!(
        filter.allow_entry(&test_entry(log_id, this_origin!())),
        "Explicitly allowed LogId not allowed by filter."
    );

    let log_event = IntermediaryLogEvent::new(log_id, NO_MSG, this_origin!());
    let finalized = log_event.finalize();
    assert!(
        filter.allow_addon(log_id, &this_origin!(), &AddonKind::Related(finalized)),
        "Related addon not allowed by filter."
    );
}

#[test]
fn allow_single_id_with_all_addons() {
    let log_id = new_log_id!("log_id", LogLevel::Info);
    let filter = FilterConfig::new(&format!(
        "on[{}::{}(all)]",
        log_id.get_module_path(),
        log_id.get_identifier()
    ));

    assert!(
        filter.allow_entry(&test_entry(log_id, this_origin!())),
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

    #[cfg(feature = "hint_note")]
    assert!(
        filter.allow_addon(
            log_id,
            &this_origin!(),
            &AddonKind::Hint("Some info".to_string())
        ),
        "Hint addon not allowed by filter."
    );
    #[cfg(feature = "hint_note")]
    assert!(
        filter.allow_addon(
            log_id,
            &this_origin!(),
            &AddonKind::Note("Some info".to_string())
        ),
        "Note addon not allowed by filter."
    );

    assert!(
        filter.show_origin_info(log_id, &this_origin!()),
        "Origin info not allowed by filter."
    );

    assert!(
        filter.show_id(log_id, &this_origin!()),
        "Event info not allowed by filter."
    );
}

#[test]
fn allow_module_with_infos_addon() {
    let log_id = new_log_id!("log_id", LogLevel::Error);
    let filter = FilterConfig::new("logid_core::logging(infos) = error");

    assert!(
        filter.allow_entry(&test_entry(log_id, this_origin!())),
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
    let filter = FilterConfig::new("logid_core(infos) = error");

    assert!(
        filter.allow_entry(&test_entry(log_id, this_origin!())),
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
    let filter = FilterConfig::new("error(infos)");

    assert!(
        filter.allow_entry(&test_entry(log_id, this_origin!())),
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
