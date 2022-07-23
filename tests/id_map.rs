use logid::{
    id_entry::Origin,
    id_map::drain_map,
    log_id::{get_log_id, EventLevel}, capturing::LogIdTracing,
};

#[test]
fn capture_single_logid() {
    let log_id = get_log_id(0, 0, EventLevel::Error, 2);
    let msg = "Set first log message";
    log_id.set_event(msg, file!(), line!());

    let map = drain_map();

    assert!(map.len() == 1, "More than one event captured!");
    assert!(map.contains_key(&log_id), "Log-id not inside captured map!");

    let entries = map.get(&log_id).unwrap();
    assert!(
        entries.len() == 1,
        "More than one entry for the same log-id"
    );

    let entry = entries.last().unwrap();
    assert_eq!(entry.id, log_id, "Set and stored log-ids are not equal");
    assert_eq!(
        entry.level,
        EventLevel::Error,
        "Set and stored event levels are not equal"
    );
    assert_eq!(entry.msg, msg, "Set and stored messages are not equal");
    assert_eq!(
        entry.origin,
        Origin::new(file!(), 11),
        "Set and stored origins are not equal"
    );
}
