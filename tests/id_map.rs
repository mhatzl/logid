use logid::{
    id_entry::Origin,
    id_map::{drain_map, LogIdMap},
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


#[test]
fn capture_single_logid_with_cause() {
    let log_id = get_log_id(0, 0, EventLevel::Warn, 1);
    let msg = "Set first log message";
    let cause = "Something caused this log-id";
    log_id.set_event(msg, file!(), line!()).add_cause(cause);

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
        EventLevel::Warn,
        "Set and stored event levels are not equal"
    );
    assert_eq!(entry.msg, msg, "Set and stored messages are not equal");
    assert_eq!(
        entry.origin,
        Origin::new(file!(), 45),
        "Set and stored origins are not equal"
    );

    assert!(entry.causes.is_some(), "No cause added to log-id");
    let causes = entry.causes.as_ref().unwrap();
    assert!(causes.len() == 1, "More than one cause was set");
    let act_cause = causes.last().unwrap();
    assert_eq!(act_cause, cause, "Set and stored messages are not equal");
}

#[test]
fn capture_single_logid_with_info() {
    let log_id = get_log_id(0, 1, EventLevel::Info, 1);
    let msg = "Set first log message";
    let info = "Additional info for this log-id";
    log_id.set_event(msg, file!(), line!()).add_info(info);

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
        EventLevel::Info,
        "Set and stored event levels are not equal"
    );

    assert!(entry.infos.is_some(), "No info added to log-id");
    let infos = entry.infos.as_ref().unwrap();
    assert!(infos.len() == 1, "More than one info was set");
    let act_info = infos.last().unwrap();
    assert_eq!(act_info, info, "Set and stored messages are not equal");
}

#[test]
fn capture_single_logid_with_debug() {
    let log_id = get_log_id(1, 1, EventLevel::Debug, 0);
    let msg = "Set first log message";
    let debug = "Additional debug info for this log-id";
    log_id.set_event(msg, file!(), line!()).add_debug(debug);

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
        EventLevel::Debug,
        "Set and stored event levels are not equal"
    );

    assert!(entry.debugs.is_some(), "No debug info added to log-id");
    let debugs = entry.debugs.as_ref().unwrap();
    assert!(debugs.len() == 1, "More than one debug info was set");
    let act_debug = debugs.last().unwrap();
    assert_eq!(act_debug, debug, "Set and stored messages are not equal");
}

#[test]
fn capture_single_logid_with_trace() {
    let log_id = get_log_id(1, 1, EventLevel::Debug, 0);
    let msg = "Set first log message";
    let trace = "Additional debug info for this log-id";
    log_id.set_event(msg, file!(), line!()).add_trace(trace);

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
        EventLevel::Debug,
        "Set and stored event levels are not equal"
    );

    assert!(entry.traces.is_some(), "No trace added to log-id");
    let traces = entry.traces.as_ref().unwrap();
    assert!(traces.len() == 1, "More than one trace was set");
    let act_trace = traces.last().unwrap();
    assert_eq!(act_trace, trace, "Set and stored messages are not equal");
}

#[test]
fn capture_single_logid_with_custom_map() {
    let log_id = get_log_id(0, 0, EventLevel::Error, 2);
    let msg = "Set first log message";
    let log_map = LogIdMap::new();
    log_id.set_event_with(&log_map, msg, file!(), line!());

    let map = log_map.drain_map();

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

    assert!(map.len() == 2, "Map did not capture two log-ids!");
    assert!(map.contains_key(&log_id_1), "Log-id_1 not inside captured map!");
    assert!(map.contains_key(&log_id_2), "Log-id_2 not inside captured map!");

    let entries_1 = map.get(&log_id_1).unwrap();
    assert!(
        entries_1.len() == 1,
        "More than one entry for the same log-id"
    );
    let entry_1 = entries_1.last().unwrap();
    assert_eq!(entry_1.id, log_id_1, "Set and stored log-ids are not equal");

    let entries_2 = map.get(&log_id_2).unwrap();
    assert!(
        entries_2.len() == 1,
        "More than one entry for the same log-id"
    );
    let entry_2 = entries_2.last().unwrap();
    assert_eq!(entry_2.id, log_id_2, "Set and stored log-ids are not equal");
}

#[test]
fn single_logid_without_capture() {
    let log_id = get_log_id(0, 0, EventLevel::Error, 2);
    let msg = "Set first log message";
    log_id.set_silent_event(msg, file!(), line!());

    let map = drain_map();

    assert!(map.is_empty(), "Silent event captured!");
    assert!(!map.contains_key(&log_id), "Log-id inside captured map!");
}

#[test]
fn logid_with_span() {
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
