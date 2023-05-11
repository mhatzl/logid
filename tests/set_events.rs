//! Tests capturing functionalities

use logid::{
    event::{origin::Origin, EventFns},
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

#[cfg(feature = "causes")]
#[test]
fn capture_single_logid_with_cause() {
    let cause_log_id = get_log_id(0, 0, LogLevel::Warn, 1);
    let cause_msg = "Cause log message";
    let log_id = get_log_id(0, 0, LogLevel::Error, 2);
    let msg = "My log message";

    let recv = publisher::subscribe_to_logs(
        vec![cause_log_id, log_id].iter().copied(),
        env!("CARGO_PKG_NAME"),
    )
    .unwrap();

    set_event!(cause_log_id, cause_msg).finalize();

    let cause_event = recv
        .recv_timeout(std::time::Duration::from_millis(10))
        .unwrap();

    set_event!(log_id, msg)
        .add_cause(cause_event)
        .finalize();

    let event = recv
        .recv_timeout(std::time::Duration::from_millis(10))
        .unwrap();

    assert_eq!(
        event.entry.get_id(),
        &log_id,
        "Set and received log-ids are not equal"
    );
    assert_eq!(
        event.entry.get_causes().get(0).unwrap().get_entry().get_id(),
        &cause_log_id,
        "Set and received causing log-ids are not equal"
    );
    assert_eq!(
        event.entry.get_causes().get(0).unwrap().get_entry().get_msg(),
        &cause_msg,
        "Set and received causing msgs are not equal"
    );
}

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

#[test]
fn capture_single_logid_with_trace() {
    let log_id = get_log_id(1, 1, LogLevel::Debug, 0);
    let msg = "Set first log message";
    let trace = "Additional debug info for this log-id";

    let recv = publisher::subscribe(log_id, env!("CARGO_PKG_NAME")).unwrap();

    set_event!(log_id, msg).add_trace(trace).finalize();

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
        entry.get_traces().len(),
        1,
        "More than one or no trace was set"
    );
    let act_trace = entry.get_traces().last().unwrap();
    assert_eq!(act_trace, trace, "Set and stored messages are not equal");
}

#[test]
fn single_logid_without_capture() {
    let log_id = get_log_id(0, 0, LogLevel::Error, 2);
    let msg = "Set first log message";

    let recv = publisher::subscribe(log_id, env!("CARGO_PKG_NAME")).unwrap();

    let event = set_silent_event!(log_id, msg);
    event.clone().finalize();

    let result = recv.recv_timeout(std::time::Duration::from_millis(10));

    if let Ok(recv_event) = result {
        assert_ne!(
            event.get_entry().get_origin(),
            recv_event.entry.get_origin(),
            "Silent event was captured"
        );
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

#[test]
fn logid_with_span() {
    tracing_subscriber::fmt::init();
    const SPAN_NAME: &str = "mySpan";

    let log_id = get_log_id(0, 0, LogLevel::Info, 2);
    let msg = "Set first log message";

    let recv = publisher::subscribe(log_id, env!("CARGO_PKG_NAME")).unwrap();

    let span = tracing::span!(tracing::Level::ERROR, SPAN_NAME);
    span.in_scope(|| {
        set_event!(log_id, msg);
    });

    let event = recv
        .recv_timeout(std::time::Duration::from_millis(10))
        .unwrap();

    let entry = event.entry;
    assert_eq!(entry.get_span(), SPAN_NAME, "Span names are not equal");
}

#[test]
fn capture_same_logid_twice_with_different_origin() {
    let log_id = get_log_id(0, 0, LogLevel::Error, 2);
    let msg = "Set first log message";

    let recv = publisher::subscribe(log_id, env!("CARGO_PKG_NAME")).unwrap();

    let line_1 = line!() + 1;
    set_event!(log_id, msg).finalize();

    let line_2 = line!() + 1;
    set_event!(log_id, msg).finalize();

    let event_1 = recv
        .recv_timeout(std::time::Duration::from_millis(10))
        .unwrap();
    let event_2 = recv
        .recv_timeout(std::time::Duration::from_millis(10))
        .unwrap();

    let entry_1 = event_1.entry;
    assert_eq!(
        *entry_1.get_id(),
        log_id,
        "Set and stored log-ids are not equal"
    );
    assert_eq!(
        entry_1.get_origin().line_nr,
        line_1,
        "Set and stored line numbers are not equal"
    );

    let entry_2 = event_2.entry;
    assert_eq!(
        *entry_2.get_id(),
        log_id,
        "Set and stored log-ids are not equal"
    );
    assert_eq!(
        entry_2.get_origin().line_nr,
        line_2,
        "Set and stored line numbers are not equal"
    );
}

#[test]
fn capture_same_logid_twice_with_same_origin() {
    let log_id = get_log_id(0, 0, LogLevel::Error, 2);
    let msg = "Set first log message";
    let file = file!();
    let line = line!();

    let recv = publisher::subscribe(log_id, env!("CARGO_PKG_NAME")).unwrap();

    log_id
        .set_event(env!("CARGO_PKG_NAME"), msg, file, line, module_path!())
        .finalize();
    log_id
        .set_event(env!("CARGO_PKG_NAME"), msg, file, line, module_path!())
        .finalize();

    let event_1 = recv
        .recv_timeout(std::time::Duration::from_millis(10))
        .unwrap();
    let event_2 = recv
        .recv_timeout(std::time::Duration::from_millis(10))
        .unwrap();

    let entry_1 = event_1.entry;
    assert_eq!(
        *entry_1.get_id(),
        log_id,
        "Set and stored log-ids are not equal"
    );
    assert_eq!(
        entry_1.get_origin().line_nr,
        line,
        "Set and stored line numbers are not equal"
    );

    let entry_2 = event_2.entry;
    assert_eq!(
        *entry_2.get_id(),
        log_id,
        "Set and stored log-ids are not equal"
    );
    assert_eq!(
        entry_2.get_origin().line_nr,
        line,
        "Set and stored line numbers are not equal"
    );

    assert_ne!(entry_1, entry_2, "Received events have the same entry");
}

#[cfg(feature = "diagnostics")]
#[test]
fn capture_single_logid_with_diagnostics() {
    use lsp_types::{Diagnostic, DiagnosticSeverity, Position, Range};

    let log_id = get_log_id(1, 1, LogLevel::Debug, 0);
    let msg = "Set first log message";
    let diagnostics = Diagnostic {
        range: Range {
            start: Position {
                line: 0,
                character: 4,
            },
            end: Position {
                line: 0,
                character: 10,
            },
        },
        severity: Some(DiagnosticSeverity::INFORMATION),
        message: "Some diagnostic information useful for lsp implementations.".to_owned(),
        ..Default::default()
    };

    let recv = publisher::subscribe(log_id, env!("CARGO_PKG_NAME")).unwrap();

    set_event!(log_id, msg)
        .add_diagnostic(diagnostics.clone())
        .finalize();

    let event = recv
        .recv_timeout(std::time::Duration::from_millis(10))
        .unwrap();

    let entry = event.entry;
    let act_diagnostics = entry.get_diagnostics().last().unwrap();
    assert_eq!(
        act_diagnostics, &diagnostics,
        "Set and stored diagnostics are not equal"
    );
}
