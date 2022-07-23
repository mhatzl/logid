use logid::{
    id_entry::Origin,
    id_map::{drain_map, LogIdMap},
    log_id::{get_log_id, EventLevel}, capturing::LogIdTracing,
};

#[test]
fn capture_single_logid() {
    // Note: Only use global map in this test to prevent test collisions!
    // Make sure global map is empty
    let _ = drain_map();

    let log_id = get_log_id(0, 0, EventLevel::Error, 2);
    let msg = "Set first log message";
    log_id.set_event(msg, file!(), line!());

    let map = drain_map();

    assert_eq!(map.len(), 1, "More than one or no event captured!");
    assert!(map.contains_key(&log_id), "Log-id not inside captured map!");

    let entries = map.get(&log_id).unwrap();
    assert_eq!(
        entries.len(), 1,
        "More than one or no entry for the same log-id"
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
        Origin::new(file!(), 15),
        "Set and stored origins are not equal"
    );
}


#[test]
fn capture_single_logid_with_cause() {
    let log_id = get_log_id(0, 0, EventLevel::Warn, 1);
    let msg = "Set first log message";
    let cause = "Something caused this log-id";
    let log_map = LogIdMap::new();
    log_id.set_event_with(&log_map, msg, file!(), line!()).add_cause(cause);

    let map = log_map.drain_map();

    assert_eq!(map.len(), 1, "More than one or no event captured!");
    assert!(map.contains_key(&log_id), "Log-id not inside captured map!");

    let entries = map.get(&log_id).unwrap();
    assert_eq!(
        entries.len(), 1,
        "More than one or no entry for the same log-id"
    );

    let entry = entries.last().unwrap();
    assert_eq!(entry.id, log_id, "Set and stored log-ids are not equal");
    assert_eq!(
        entry.level,
        EventLevel::Warn,
        "Set and stored event levels are not equal"
    );
    assert_eq!(entry.msg, msg, "Set and stored messages are not equal");
    assert_eq!(
        entry.origin,
        Origin::new(file!(), 50),
        "Set and stored origins are not equal"
    );

    assert_eq!(entry.causes.len(), 1, "More than one or no cause was set");
    let act_cause = entry.causes.last().unwrap();
    assert_eq!(act_cause, cause, "Set and stored messages are not equal");
}

#[test]
fn capture_single_logid_with_info() {
    let log_id = get_log_id(0, 1, EventLevel::Info, 1);
    let msg = "Set first log message";
    let info = "Additional info for this log-id";
    let log_map = LogIdMap::new();
    log_id.set_event_with(&log_map, msg, file!(), line!()).add_info(info);

    let map = log_map.drain_map();

    assert_eq!(map.len(), 1, "More than one or no event captured!");
    assert!(map.contains_key(&log_id), "Log-id not inside captured map!");

    let entries = map.get(&log_id).unwrap();
    assert_eq!(
        entries.len(), 1,
        "More than one or no entry for the same log-id"
    );

    let entry = entries.last().unwrap();
    assert_eq!(entry.id, log_id, "Set and stored log-ids are not equal");
    assert_eq!(
        entry.level,
        EventLevel::Info,
        "Set and stored event levels are not equal"
    );

    assert_eq!(entry.infos.len(), 1, "More than one or no info was set");
    let act_info = entry.infos.last().unwrap();
    assert_eq!(act_info, info, "Set and stored messages are not equal");
}

#[test]
fn capture_single_logid_with_debug() {
    let log_id = get_log_id(1, 1, EventLevel::Debug, 0);
    let msg = "Set first log message";
    let debug = "Additional debug info for this log-id";
    let log_map = LogIdMap::new();
    log_id.set_event_with(&log_map, msg, file!(), line!()).add_debug(debug);

    let map = log_map.drain_map();

    assert_eq!(map.len(), 1, "More than one or no event captured!");
    assert!(map.contains_key(&log_id), "Log-id not inside captured map!");

    let entries = map.get(&log_id).unwrap();
    assert_eq!(
        entries.len(), 1,
        "More than one or no entry for the same log-id"
    );

    let entry = entries.last().unwrap();
    assert_eq!(entry.id, log_id, "Set and stored log-ids are not equal");
    assert_eq!(
        entry.level,
        EventLevel::Debug,
        "Set and stored event levels are not equal"
    );

    assert_eq!(entry.debugs.len(), 1, "More than one or no debug info was set");
    let act_debug = entry.debugs.last().unwrap();
    assert_eq!(act_debug, debug, "Set and stored messages are not equal");
}

#[test]
fn capture_single_logid_with_trace() {
    let log_id = get_log_id(1, 1, EventLevel::Debug, 0);
    let msg = "Set first log message";
    let trace = "Additional debug info for this log-id";
    let log_map = LogIdMap::new();
    log_id.set_event_with(&log_map, msg, file!(), line!()).add_trace(trace);

    let map = log_map.drain_map();

    assert_eq!(map.len(), 1, "More than one or no event captured!");
    assert!(map.contains_key(&log_id), "Log-id not inside captured map!");

    let entries = map.get(&log_id).unwrap();
    assert_eq!(
        entries.len(), 1,
        "More than one or no entry for the same log-id"
    );

    let entry = entries.last().unwrap();
    assert_eq!(entry.id, log_id, "Set and stored log-ids are not equal");
    assert_eq!(
        entry.level,
        EventLevel::Debug,
        "Set and stored event levels are not equal"
    );

    assert_eq!(entry.traces.len(), 1, "More than one or no trace was set");
    let act_trace = entry.traces.last().unwrap();
    assert_eq!(act_trace, trace, "Set and stored messages are not equal");
}

#[test]
fn capture_single_logid_with_custom_map() {
    let log_id = get_log_id(0, 0, EventLevel::Error, 2);
    let msg = "Set first log message";
    let log_map = LogIdMap::new();
    log_id.set_event_with(&log_map, msg, file!(), line!());

    let map = log_map.drain_map();

    assert_eq!(map.len(), 1, "More than one or no event captured!");
    assert!(map.contains_key(&log_id), "Log-id not inside captured map!");

    let entries = map.get(&log_id).unwrap();
    assert_eq!(
        entries.len(), 1,
        "More than one or no entry for the same log-id"
    );

    let entry = entries.last().unwrap();
    assert_eq!(entry.id, log_id, "Set and stored log-ids are not equal");
    assert_eq!(
        entry.level,
        EventLevel::Error,
        "Set and stored event levels are not equal"
    );
    assert_eq!(entry.msg, msg, "Set and stored messages are not equal");
}

#[test]
fn capture_two_logids_with_custom_map() {
    let log_id_1 = get_log_id(0, 0, EventLevel::Error, 2);
    let log_id_2 = get_log_id(1, 0, EventLevel::Error, 2);
    let msg = "Set first log message";
    let log_map = LogIdMap::new();
    log_id_1.set_event_with(&log_map, msg, file!(), line!());
    log_id_2.set_event_with(&log_map, msg, file!(), line!());

    let map = log_map.drain_map();

    assert_eq!(map.len(), 2, "More than two or less events captured!");
    assert!(map.contains_key(&log_id_1), "Log-id_1 not inside captured map!");
    assert!(map.contains_key(&log_id_2), "Log-id_2 not inside captured map!");

    let entries_1 = map.get(&log_id_1).unwrap();
    assert_eq!(
        entries_1.len(), 1,
        "More than one or no entry for the same log-id"
    );
    let entry_1 = entries_1.last().unwrap();
    assert_eq!(entry_1.id, log_id_1, "Set and stored log-ids are not equal");

    let entries_2 = map.get(&log_id_2).unwrap();
    assert_eq!(
        entries_2.len(), 1,
        "More than one or no entry for the same log-id"
    );
    let entry_2 = entries_2.last().unwrap();
    assert_eq!(entry_2.id, log_id_2, "Set and stored log-ids are not equal");
}

#[test]
fn single_logid_without_capture() {
    // Make sure global map is empty
    let _ = drain_map();

    let log_id = get_log_id(0, 0, EventLevel::Error, 2);
    let msg = "Set first log message";
    log_id.set_silent_event(msg, file!(), line!());

    let map = drain_map();

    assert!(map.is_empty(), "Silent event captured!");
    assert!(!map.contains_key(&log_id), "Log-id inside captured map!");
}

#[test]
fn logid_with_span() {
    // Make sure global map is empty
    let _ = drain_map();

    tracing_subscriber::fmt::init();

    let log_id = get_log_id(0, 0, EventLevel::Info, 2);
    let msg = "Set first log message";
    const SPAN_NAME: &str = "mySpan";
    let span = tracing::span!(tracing::Level::ERROR, SPAN_NAME);
    let _ = span.in_scope(|| log_id.set_event(msg, file!(), line!()));

    let map = drain_map();
    
    let entries = map.get(&log_id).unwrap();
    let entry = entries.last().unwrap();

    assert_eq!(entry.span, SPAN_NAME, "Span names are not equal");
}