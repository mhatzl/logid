//! Tests that `last_finalized_id` is set correctly

use logid::{
    drain_entries, drain_map, get_last_finalized_id,
    log_id::{get_log_id, EventLevel},
    set_event,
};

#[test]
fn last_id_updated_to_finalized_logid() {
    drain_map!();

    let log_id = get_log_id(0, 0, EventLevel::Error, 2);
    let msg = "Set first log message";

    assert_eq!(
        get_last_finalized_id!(),
        None,
        "Initialized last id was wrong"
    );

    set_event!(log_id, msg).finalize();

    assert_eq!(
        get_last_finalized_id!(),
        Some(log_id),
        "Last finalized id not updated"
    );
}

#[test]
fn last_id_updated_after_last_got_drained() {
    drain_map!();

    let log_id = get_log_id(0, 0, EventLevel::Error, 2);
    let msg = "Set first log message";

    assert_eq!(
        get_last_finalized_id!(),
        None,
        "Initialized last id was wrong"
    );

    set_event!(log_id, msg).finalize();

    assert_eq!(
        get_last_finalized_id!(),
        Some(log_id),
        "Last finalized id not updated"
    );

    let _ = drain_entries!(log_id);

    assert_eq!(
        get_last_finalized_id!(),
        None,
        "Last finalized id was not reset"
    );
}

#[test]
fn last_id_updated_after_map_got_drained() {
    drain_map!();

    let log_id = get_log_id(0, 0, EventLevel::Error, 2);
    let msg = "Set first log message";

    assert_eq!(
        get_last_finalized_id!(),
        None,
        "Initialized last id was wrong"
    );

    set_event!(log_id, msg).finalize();

    assert_eq!(
        get_last_finalized_id!(),
        Some(log_id),
        "Last finalized id not updated"
    );

    let _ = drain_map!();

    assert_eq!(
        get_last_finalized_id!(),
        None,
        "Last finalized id was not reset"
    );
}

#[test]
fn last_id_updated_to_latest_finalized_id() {
    drain_map!();

    let log_id_1 = get_log_id(0, 0, EventLevel::Error, 2);
    let msg = "Set first log message";

    assert_eq!(
        get_last_finalized_id!(),
        None,
        "Initialized last id was wrong"
    );

    set_event!(log_id_1, msg).finalize();

    assert_eq!(
        get_last_finalized_id!(),
        Some(log_id_1),
        "Last finalized id not updated"
    );

    let log_id_2 = get_log_id(1, 0, EventLevel::Error, 2);
    set_event!(log_id_2, msg).finalize();

    assert_eq!(
        get_last_finalized_id!(),
        Some(log_id_2),
        "Last finalized id not updated"
    );
}
