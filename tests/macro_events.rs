use logid::{
    id_entry::Origin,
    id_map::LogIdMap,
    log_id::{get_log_id, EventLevel},
    set_event_with,
};
use once_cell::sync::Lazy;

#[test]
fn set_event_with_macro() {
    let log_id = get_log_id(0, 0, EventLevel::Error, 2);
    static LOG_MAP: Lazy<LogIdMap> = Lazy::new(LogIdMap::new);
    let msg = "Set first log message";

    let log_id = set_event_with!(log_id, &LOG_MAP, msg).finalize();

    let map = LOG_MAP.drain_map().unwrap();

    assert_eq!(map.len(), 1, "More than one or no event captured!");
    assert!(map.contains_key(&log_id), "Log-id not inside captured map!");

    let entries = map.get(&log_id).unwrap();
    assert_eq!(
        entries.len(),
        1,
        "More than one or no entry for the same log-id"
    );

    let entry = entries.iter().last().unwrap();
    assert_eq!(
        *entry.get_id(),
        log_id,
        "Set and stored log-ids are not equal"
    );
    assert_eq!(
        *entry.get_origin(),
        Origin {
            filename: file!().to_string(),
            line_nr: 15
        },
        "Origin of log_id not set correctly"
    );
    assert_eq!(
        *entry.get_level(),
        EventLevel::Error,
        "Set and stored event levels are not equal"
    );

    assert_eq!(entry.get_infos().len(), 0, "Info was set");
}

#[test]
fn set_event_with_literal_msg() {
    let log_id = get_log_id(0, 0, EventLevel::Error, 2);
    static LOG_MAP: Lazy<LogIdMap> = Lazy::new(LogIdMap::new);

    let log_id = set_event_with!(log_id, &LOG_MAP, "Set first log message").finalize();

    let map = LOG_MAP.drain_map().unwrap();
    let entries = map.get(&log_id).unwrap();
    let entry = entries.iter().last().unwrap();
    assert_eq!(
        *entry.get_origin(),
        Origin {
            filename: file!().to_string(),
            line_nr: 57
        },
        "Origin of log_id not set correctly"
    );
}

// Setup macro with custom global map
static GLOBAL_LOG_MAP: Lazy<LogIdMap> = Lazy::new(LogIdMap::new);
logid::setup_map!(&GLOBAL_LOG_MAP);

#[test]
fn set_event_macro() {
    let log_id = get_log_id(0, 0, EventLevel::Error, 2);

    let log_id = set_event!(log_id, "Set first log message").finalize();

    let map = GLOBAL_LOG_MAP.drain_map().unwrap();
    let entries = map.get(&log_id).unwrap();
    let entry = entries.iter().last().unwrap();
    assert_eq!(
        *entry.get_origin(),
        Origin {
            filename: file!().to_string(),
            line_nr: 80
        },
        "Origin of log_id not set correctly"
    );
}
