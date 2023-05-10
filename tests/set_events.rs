//! Tests capturing functionalities

use logid::{
    event::origin::Origin,
    log_id::{get_log_id, LogLevel},
    publisher, set_event, set_silent_event,
};

#[test]
fn capture_single_logid() {
    let log_id = get_log_id(0, 0, LogLevel::Error, 2);
    let msg = "Set first log message";

    let recv = publisher::subscribe(log_id, env!("CARGO_PKG_NAME")).unwrap();

    set_event!(log_id, msg).finalize();

    let event = recv
        .recv_timeout(std::time::Duration::from_millis(10))
        .unwrap();

    let entry = event.entry;
    assert_eq!(
        *entry.get_id(),
        log_id,
        "Set and stored log-ids are not equal"
    );
    assert_eq!(
        *entry.get_level(),
        LogLevel::Error,
        "Set and stored event levels are not equal"
    );
    assert_eq!(
        *entry.get_msg(),
        msg,
        "Set and stored messages are not equal"
    );
    assert_eq!(
        *entry.get_origin(),
        Origin::new(file!(), line!() - 24, module_path!()), //Note: Event is set 24 lines above
        "Set and stored origins are not equal"
    );
}

// #[cfg(feature = "causes")]
// #[test]
// fn capture_single_logid_with_cause() {
//     drain_map!();

//     let log_id = get_log_id(0, 0, EventLevel::Warn, 1);
//     let msg = "Set first log message";
//     let cause = "Something caused this log-id";
//     let line = line!() + 1;
//     let event = set_event!(log_id, msg).add_cause(cause);
//     event.clone().finalize();

//     let map = delayed_map_drain();

//     assert_eq!(map.len(), 1, "More than one or no event captured!");
//     assert!(map.contains_key(&log_id), "Log-id not inside captured map!");

//     let entries = map.get(&log_id).unwrap();
//     assert_eq!(
//         entries.len(),
//         1,
//         "More than one or no entry for the same log-id"
//     );

//     let entry = entries.get_entry(&event).unwrap();
//     assert_eq!(
//         *entry.get_id(),
//         log_id,
//         "Set and stored log-ids are not equal"
//     );
//     assert_eq!(
//         *entry.get_level(),
//         EventLevel::Warn,
//         "Set and stored event levels are not equal"
//     );
//     assert_eq!(
//         *entry.get_msg(),
//         msg,
//         "Set and stored messages are not equal"
//     );
//     assert_eq!(
//         *entry.get_origin(),
//         Origin::new(file!(), line),
//         "Set and stored origins are not equal"
//     );

//     assert_eq!(entry.causes.len(), 1, "More than one or no cause was set");
//     let act_cause = entry.causes.last().unwrap();
//     assert_eq!(act_cause, cause, "Set and stored messages are not equal");
// }

#[test]
fn capture_single_logid_with_info() {
    let log_id = get_log_id(0, 1, LogLevel::Info, 1);
    let msg = "Set first log message";
    let info = "Additional info for this log-id";

    let recv = publisher::subscribe(log_id, env!("CARGO_PKG_NAME")).unwrap();

    set_event!(log_id, msg).add_info(info).finalize();

    let event = recv
        .recv_timeout(std::time::Duration::from_millis(10))
        .unwrap();

    let entry = event.entry;
    assert_eq!(
        *entry.get_id(),
        log_id,
        "Set and stored log-ids are not equal"
    );
    assert_eq!(
        *entry.get_level(),
        LogLevel::Info,
        "Set and stored event levels are not equal"
    );

    assert_eq!(
        entry.get_infos().len(),
        1,
        "More than one or no info was set"
    );
    let act_info = entry.get_infos().last().unwrap();
    assert_eq!(act_info, info, "Set and stored messages are not equal");
}

#[test]
fn capture_single_logid_with_debug() {
    let log_id = get_log_id(1, 1, LogLevel::Debug, 0);
    let msg = "Set first log message";
    let debug = "Additional debug info for this log-id";

    let recv = publisher::subscribe(log_id, env!("CARGO_PKG_NAME")).unwrap();

    set_event!(log_id, msg).add_debug(debug).finalize();

    let event = recv
        .recv_timeout(std::time::Duration::from_millis(10))
        .unwrap();

    let entry = event.entry;
    assert_eq!(
        *entry.get_id(),
        log_id,
        "Set and stored log-ids are not equal"
    );
    assert_eq!(
        *entry.get_level(),
        LogLevel::Debug,
        "Set and stored event levels are not equal"
    );

    assert_eq!(
        entry.get_debugs().len(),
        1,
        "More than one or no debug info was set"
    );
    let act_debug = entry.get_debugs().last().unwrap();
    assert_eq!(act_debug, debug, "Set and stored messages are not equal");
}

// #[test]
// fn capture_single_logid_with_trace() {
//     drain_map!();

//     let log_id = get_log_id(1, 1, EventLevel::Debug, 0);
//     let msg = "Set first log message";
//     let trace = "Additional debug info for this log-id";
//     let event = set_event!(log_id, msg).add_trace(trace);
//     event.clone().finalize();

//     let map = delayed_map_drain();

//     assert_eq!(map.len(), 1, "More than one or no event captured!");
//     assert!(map.contains_key(&log_id), "Log-id not inside captured map!");

//     let entries = map.get(&log_id).unwrap();
//     assert_eq!(
//         entries.len(),
//         1,
//         "More than one or no entry for the same log-id"
//     );

//     let entry = entries.get_entry(&event).unwrap();
//     assert_eq!(
//         *entry.get_id(),
//         log_id,
//         "Set and stored log-ids are not equal"
//     );
//     assert_eq!(
//         *entry.get_level(),
//         EventLevel::Debug,
//         "Set and stored event levels are not equal"
//     );

//     assert_eq!(
//         entry.get_traces().len(),
//         1,
//         "More than one or no trace was set"
//     );
//     let act_trace = entry.get_traces().last().unwrap();
//     assert_eq!(act_trace, trace, "Set and stored messages are not equal");
// }

#[test]
fn single_logid_without_capture() {
    let log_id = get_log_id(0, 0, LogLevel::Error, 2);
    let msg = "Set first log message";

    let recv = publisher::subscribe(log_id, env!("CARGO_PKG_NAME")).unwrap();

    let event = set_silent_event!(log_id, msg);
    event.clone().finalize();

    let result = recv.recv_timeout(std::time::Duration::from_millis(10));

    if let Ok(recv_event) = result {
        assert_ne!(event.entry().get_origin(), recv_event.entry.get_origin(), "Silent event was captured");
    }
}

#[test]
fn logid_correctly_set_in_silent_event() {
    let log_id = get_log_id(0, 0, LogLevel::Error, 2);
    let msg = "Set first log message";

    let event = set_silent_event!(log_id, msg);
    event.clone().finalize();

    assert!(event == log_id, "LogIdEvent and LogId are not equal");
    assert!(log_id == event, "LogId and LogIdEvent are not equal");
}

// #[test]
// fn logid_with_span() {
//     drain_map!();

//     tracing_subscriber::fmt::init();
//     const SPAN_NAME: &str = "mySpan";

//     let log_id = get_log_id(0, 0, EventLevel::Info, 2);
//     let msg = "Set first log message";
//     let span = tracing::span!(tracing::Level::ERROR, SPAN_NAME);
//     // Assignment only to initialize mapped => silent_event
//     let mut event = set_silent_event!(log_id, msg);

//     let _ = span.in_scope(|| {
//         event = set_event!(log_id, msg);
//         event.clone().finalize()
//     });

//     let map = delayed_map_drain();

//     let entries = map.get(&log_id).unwrap();
//     let entry = entries.get_entry(&event).unwrap();

//     assert_eq!(entry.get_span(), SPAN_NAME, "Span names are not equal");
// }

// #[test]
// fn capture_same_logid_twice_with_different_origin() {
//     drain_map!();

//     let log_id = get_log_id(0, 0, EventLevel::Error, 2);
//     let msg = "Set first log message";
//     let line_1 = line!() + 1;
//     let event_1 = set_event!(log_id, msg);
//     event_1.clone().finalize();
//     let line_2 = line!() + 1;
//     let event_2 = set_event!(log_id, msg);
//     event_2.clone().finalize();

//     let map = delayed_map_drain();

//     assert_eq!(map.len(), 1, "More than two or less events captured!");
//     assert!(map.contains_key(&log_id), "Log-id not inside captured map!");

//     let entries = map.get(&log_id).unwrap();
//     assert_eq!(
//         entries.len(),
//         2,
//         "More than one or no entry for the same log-id"
//     );

//     let entry_1 = entries.get_entry(&event_1).unwrap();
//     assert_eq!(
//         *entry_1.get_id(),
//         log_id,
//         "Set and stored log-ids are not equal"
//     );
//     assert_eq!(
//         entry_1.get_origin().line_nr,
//         line_1,
//         "Set and stored line numbers are not equal"
//     );

//     let entry_2 = entries.get_entry(&event_2).unwrap();
//     assert_eq!(
//         *entry_2.get_id(),
//         log_id,
//         "Set and stored log-ids are not equal"
//     );
//     assert_eq!(
//         entry_2.get_origin().line_nr,
//         line_2,
//         "Set and stored line numbers are not equal"
//     );
// }

// #[test]
// fn capture_same_logid_twice_with_same_origin() {
//     drain_map!();

//     let log_id = get_log_id(0, 0, EventLevel::Error, 2);
//     let msg = "Set first log message";
//     let file = file!();
//     let line = line!();
//     let event_1 = log_id.set_event(env!("CARGO_PKG_NAME"), msg, file, line);
//     event_1.clone().finalize();
//     let event_2 = log_id.set_event(env!("CARGO_PKG_NAME"), msg, file, line);
//     event_2.clone().finalize();

//     let map = delayed_map_drain();

//     assert_eq!(map.len(), 1, "More than two or less events captured!");
//     assert!(map.contains_key(&log_id), "Log-id not inside captured map!");

//     let entries = map.get(&log_id).unwrap();
//     assert_eq!(
//         entries.len(),
//         2,
//         "More than one or no entry for the same log-id"
//     );

//     let entry_1 = entries.get_entry(&event_1).unwrap();
//     assert_eq!(
//         *entry_1.get_id(),
//         log_id,
//         "Set and stored log-ids are not equal"
//     );
//     assert_eq!(
//         entry_1.get_origin().line_nr,
//         line,
//         "Set and stored line numbers are not equal"
//     );

//     let entry_2 = entries.get_entry(&event_2).unwrap();
//     assert_eq!(
//         *entry_2.get_id(),
//         log_id,
//         "Set and stored log-ids are not equal"
//     );
//     assert_eq!(
//         entry_2.get_origin().line_nr,
//         line,
//         "Set and stored line numbers are not equal"
//     );
// }
