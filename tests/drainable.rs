//! Tests drainable flag for [`LogIdEntry`], and related functions

use logid::{log_id::{get_log_id, EventLevel}, id_map::LogIdMap, capturing::LogIdTracing};

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

    let entries = log_map.get_entries_safe(log_id).unwrap();

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

    assert!(log_map.get_entries_safe(log_id).is_none(), "Entries marked as drainable");
}

#[test]
fn entries_not_drainable_not_removed() {
    let log_id = get_log_id(0, 0, EventLevel::Error, 2);
    let msg = "Set first log message";
    let log_map = LogIdMap::new();

    {
      // Mapped id **not** dropped => entry **not** set as `drainable`
      let _mapped_id = log_id.set_event_with(&log_map, msg, file!(), line!());

      let result = log_map.drain_entries_safe(log_id);
      assert!(result.is_none(), "Entries marked as drainable");
    }

    // Now drainable, because out-of-scope
    let entries = log_map.get_entries_safe(log_id).unwrap();
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

    let entries = log_map.drain_entries_safe(log_id).unwrap();
    // Now drainable, because out-of-scope
    assert_eq!(
        entries.len(),
        1,
        "More than one or no entry for the same log-id"
    );

    let entry = entries.last().unwrap();
    assert_eq!(entry.id, log_id, "Set and stored log-ids are not equal");
    assert!(entry.drainable(), "Entry not marked as drainable");

    assert!(log_map.get_entries_safe(log_id).is_none(), "Entries not removed from map");
}
