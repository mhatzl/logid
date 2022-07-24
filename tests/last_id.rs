//! Tests that `last_finalized_id` is set correctly

use logid::{
    capturing::LogIdTracing,
    id_map::LogIdMap,
    log_id::{get_log_id, EventLevel, INVALID_LOG_ID},
};

#[test]
fn last_id_updated_to_finalized_logid() {
    let log_id = get_log_id(0, 0, EventLevel::Error, 2);
    let msg = "Set first log message";
    let log_map = LogIdMap::new();

    assert_eq!(
        log_map.get_last_finalized_id(),
        INVALID_LOG_ID,
        "Initialized last id was wrong"
    );

    log_id
        .set_event_with(&log_map, msg, file!(), line!())
        .finalize();

    assert_eq!(
        log_map.get_last_finalized_id(),
        log_id,
        "Last finalized id not updated"
    );
}

#[test]
fn last_id_updated_after_last_got_drained() {
    let log_id = get_log_id(0, 0, EventLevel::Error, 2);
    let msg = "Set first log message";
    let log_map = LogIdMap::new();

    assert_eq!(
        log_map.get_last_finalized_id(),
        INVALID_LOG_ID,
        "Initialized last id was wrong"
    );

    log_id
        .set_event_with(&log_map, msg, file!(), line!())
        .finalize();

    assert_eq!(
        log_map.get_last_finalized_id(),
        log_id,
        "Last finalized id not updated"
    );

    let _ = log_map.drain_entries(log_id);

    assert_eq!(
        log_map.get_last_finalized_id(),
        INVALID_LOG_ID,
        "Last finalized id was not reset"
    );
}

#[test]
fn last_id_updated_after_map_got_drained() {
    let log_id = get_log_id(0, 0, EventLevel::Error, 2);
    let msg = "Set first log message";
    let log_map = LogIdMap::new();

    assert_eq!(
        log_map.get_last_finalized_id(),
        INVALID_LOG_ID,
        "Initialized last id was wrong"
    );

    log_id
        .set_event_with(&log_map, msg, file!(), line!())
        .finalize();

    assert_eq!(
        log_map.get_last_finalized_id(),
        log_id,
        "Last finalized id not updated"
    );

    let _ = log_map.drain_map();

    assert_eq!(
        log_map.get_last_finalized_id(),
        INVALID_LOG_ID,
        "Last finalized id was not reset"
    );
}

#[test]
fn last_id_updated_to_latest_finalized_id() {
    let log_id_1 = get_log_id(0, 0, EventLevel::Error, 2);
    let msg = "Set first log message";
    let log_map = LogIdMap::new();

    assert_eq!(
        log_map.get_last_finalized_id(),
        INVALID_LOG_ID,
        "Initialized last id was wrong"
    );

    log_id_1
        .set_event_with(&log_map, msg, file!(), line!())
        .finalize();

    assert_eq!(
        log_map.get_last_finalized_id(),
        log_id_1,
        "Last finalized id not updated"
    );

    let log_id_2 = get_log_id(1, 0, EventLevel::Error, 2);
    log_id_2
        .set_event_with(&log_map, msg, file!(), line!())
        .finalize();

    assert_eq!(
        log_map.get_last_finalized_id(),
        log_id_2,
        "Last finalized id not updated"
    );
}
