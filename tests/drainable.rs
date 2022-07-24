//! Tests drainable flag for [`LogIdEntry`], and related functions

use logid::{
    capturing::LogIdTracing,
    id_map::LogIdMap,
    log_id::{get_log_id, EventLevel},
};

#[test]
fn finalize_logid_manually() {
    let log_id = get_log_id(0, 0, EventLevel::Error, 2);
    let msg = "Set first log message";
    let log_map = LogIdMap::new();
    log_id
        .set_event_with(&log_map, msg, file!(), line!())
        .finalize();

    let map = log_map.drain_map().unwrap();

    let entries = map.get(&log_id).unwrap();
    assert_eq!(
        entries.len(),
        1,
        "More than one or no entry for the same log-id"
    );

    let entry = entries.last().unwrap();
    assert_eq!(entry.id, log_id, "Set and stored log-ids are not equal");
    assert!(entry.drainable(), "Entry not marked as drainable");
}

#[test]
fn finalize_logid_on_drop() {
    let log_id = get_log_id(0, 0, EventLevel::Error, 2);
    let msg = "Set first log message";
    let log_map = LogIdMap::new();

    {
        // Mapped id dropped => entry set as `drainable`
        let _mapped_id = log_id.set_event_with(&log_map, msg, file!(), line!());
    }

    let map = log_map.drain_map().unwrap();

    let entries = map.get(&log_id).unwrap();
    assert_eq!(
        entries.len(),
        1,
        "More than one or no entry for the same log-id"
    );

    let entry = entries.last().unwrap();
    assert_eq!(entry.id, log_id, "Set and stored log-ids are not equal");
    assert!(entry.drainable(), "Entry not marked as drainable");
}

#[test]
fn get_drainable_entries() {
    let log_id = get_log_id(0, 0, EventLevel::Error, 2);
    let msg = "Set first log message";
    let log_map = LogIdMap::new();

    {
        // Mapped id dropped => entry set as `drainable`
        let _mapped_id = log_id.set_event_with(&log_map, msg, file!(), line!());
    }

    let entries = log_map.get_entries(log_id).unwrap();

    assert_eq!(
        entries.len(),
        1,
        "More than one or no entry for the same log-id"
    );

    let entry = entries.last().unwrap();
    assert_eq!(entry.id, log_id, "Set and stored log-ids are not equal");
    assert!(entry.drainable(), "Entry not marked as drainable");
}

#[test]
fn entries_not_drainable() {
    let log_id = get_log_id(0, 0, EventLevel::Error, 2);
    let msg = "Set first log message";
    let log_map = LogIdMap::new();

    // Mapped id **not** dropped => entry **not** set as `drainable`
    let _mapped_id = log_id.set_event_with(&log_map, msg, file!(), line!());

    assert!(
        log_map.get_entries(log_id).is_none(),
        "Entries marked as drainable"
    );
}

#[test]
fn entries_not_drainable_not_removed() {
    let log_id = get_log_id(0, 0, EventLevel::Error, 2);
    let msg = "Set first log message";
    let log_map = LogIdMap::new();

    {
        // Mapped id **not** dropped => entry **not** set as `drainable`
        let _mapped_id = log_id.set_event_with(&log_map, msg, file!(), line!());

        let result = log_map.drain_entries(log_id);
        assert!(result.is_none(), "Entries marked as drainable");
    }

    // Now drainable, because out-of-scope
    let entries = log_map.get_entries(log_id).unwrap();
    assert_eq!(
        entries.len(),
        1,
        "More than one or no entry for the same log-id"
    );

    let entry = entries.last().unwrap();
    assert_eq!(entry.id, log_id, "Set and stored log-ids are not equal");
    assert!(entry.drainable(), "Entry not marked as drainable");
}

#[test]
fn entries_drainable_and_removed() {
    let log_id = get_log_id(0, 0, EventLevel::Error, 2);
    let msg = "Set first log message";
    let log_map = LogIdMap::new();

    {
        // Mapped id dropped => entry set as `drainable`
        let _mapped_id = log_id.set_event_with(&log_map, msg, file!(), line!());
    }

    let entries = log_map.drain_entries(log_id).unwrap();
    // Now drainable, because out-of-scope
    assert_eq!(
        entries.len(),
        1,
        "More than one or no entry for the same log-id"
    );

    let entry = entries.last().unwrap();
    assert_eq!(entry.id, log_id, "Set and stored log-ids are not equal");
    assert!(entry.drainable(), "Entry not marked as drainable");

    assert!(
        log_map.get_entries(log_id).is_none(),
        "Entries not removed from map"
    );
}

#[test]
fn entries_not_drainable_in_map() {
    let log_id = get_log_id(0, 0, EventLevel::Error, 2);
    let msg = "Set first log message";
    let log_map = LogIdMap::new();

    // Not `finalizing´ on purpose here
    let _mapped_id = log_id.set_event_with(&log_map, msg, file!(), line!());

    assert!(
        log_map.drain_map().is_none(),
        "Map drained non-drainable entries"
    );
}

#[test]
fn entries_partially_drainable_in_map_same_id() {
    let log_id = get_log_id(0, 0, EventLevel::Error, 2);
    let msg_1 = "Set first log message";
    let msg_2 = "Set second log message";
    let log_map = LogIdMap::new();

    {
        let _mapped_id_1 = log_id.set_event_with(&log_map, msg_1, file!(), line!());
    }

    // Not `finalizing´ on purpose here
    let _mapped_id_2 = log_id.set_event_with(&log_map, msg_2, file!(), line!());

    let drained_res = log_map.drain_map().unwrap();
    let entries = drained_res.get(&log_id).unwrap();
    assert_eq!(
        entries.len(),
        1,
        "More than one or no entry for the same log-id"
    );
    let entry = entries.last().unwrap();
    assert!(entry.drainable(), "Entry not marked as drainable");
    assert_eq!(entry.msg, msg_1, "Wrong entry drained");

    let remaining_entries = log_map.get_entries_unsafe(log_id).unwrap();
    assert_eq!(
        remaining_entries.len(),
        1,
        "More than one or no entry for the same log-id"
    );
    let entry = remaining_entries.last().unwrap();
    assert!(!entry.drainable(), "Entry marked as drainable");
    assert_eq!(entry.msg, msg_2, "Wrong entry drained");
}
