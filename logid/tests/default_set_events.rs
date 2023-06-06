use logid::{err, log, set_event};
use logid_core::{
    evident::event::{entry::EventEntry, origin::Origin},
    log_id::{LogId, LogLevel},
    logging::LOGGER,
};
use logid_derive::ErrLogId;
use thiserror::Error;

#[derive(Debug, Default, ErrLogId, PartialEq, Clone, Error)]
enum TestErrId {
    #[error("Error on `TestErrId::One`")]
    One,

    #[error("Error on `TestErrId::Two`")]
    #[default]
    Two,
}

#[test]
fn capture_single_logid() {
    let msg = "Set first log message";

    let recv = LOGGER.subscribe(TestErrId::One.into()).unwrap();

    log!(TestErrId::One, msg);

    let event = recv
        .get_receiver()
        .recv_timeout(std::time::Duration::from_millis(10))
        .unwrap();

    let entry = event.get_entry();
    assert_eq!(
        *entry.get_event_id(),
        TestErrId::One.into(),
        "Set and stored log-ids are not equal"
    );
    assert_eq!(
        entry.get_level(),
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
        Origin::new(
            env!("CARGO_PKG_NAME"),
            module_path!(),
            file!(),
            line!() - 29
        ), //Note: Event is set 29 lines above
        "Set and stored origins are not equal"
    );
}

fn failing_fn(msg: &str) -> Result<(), TestErrId> {
    err!(TestErrId::One, msg)
}

#[test]
fn set_event_for_err_result() {
    let msg = "Set first log message";

    let recv = LOGGER.subscribe(TestErrId::One.into()).unwrap();

    let res = failing_fn(msg);

    assert_eq!(
        res.unwrap_err(),
        TestErrId::One,
        "Converted LogId from result is wrong"
    );

    let event = recv
        .get_receiver()
        .recv_timeout(std::time::Duration::from_millis(10))
        .unwrap();

    let entry = event.get_entry();
    assert_eq!(
        *entry.get_event_id(),
        TestErrId::One.into(),
        "Set and stored log-ids are not equal"
    );
    assert_eq!(
        *entry.get_msg(),
        msg,
        "Set and stored messages are not equal"
    );
}

#[test]
fn capture_logid_with_custom_identifier() {
    let msg = "Set log message";
    let identifier = "log_id";
    let log_id = LogId::new(
        env!("CARGO_PKG_NAME"),
        module_path!(),
        identifier,
        LogLevel::Trace,
    );

    let recv = LOGGER.subscribe(log_id).unwrap();

    set_event!(log_id, msg).finalize();

    let event = recv
        .get_receiver()
        .recv_timeout(std::time::Duration::from_millis(10))
        .unwrap();

    let entry = event.get_entry();
    assert_eq!(
        *entry.get_event_id(),
        log_id,
        "Set and stored log-ids are not equal"
    );
    assert_eq!(
        entry.get_level(),
        LogLevel::Trace,
        "Set and stored event levels are not equal"
    );
    assert_eq!(
        *entry.get_msg(),
        msg,
        "Set and stored messages are not equal"
    );
    assert_eq!(
        entry.get_event_id().get_identifier(),
        identifier,
        "Set and stored identifiers are not equal"
    );
}

// #[test]
// fn capture_single_logid_with_cause() {
//     let cause_log_id = get_log_id(0, 0, LogLevel::Warn, 1);
//     let cause_msg = "Cause log message";
//     let log_id = get_log_id(0, 0, LogLevel::Error, 2);
//     let msg = "My log message";

//     let recv = publisher::subscribe_to_logs(
//         vec![cause_log_id, log_id].iter().copied(),
//         env!("CARGO_PKG_NAME"),
//     )
//     .unwrap();

//     set_event!(cause_log_id, cause_msg).finalize();

//     let cause_event = recv
//         .recv_timeout(std::time::Duration::from_millis(10))
//         .unwrap();

//     set_event!(log_id, msg).add_cause(cause_event).finalize();

//     let event = recv
//         .recv_timeout(std::time::Duration::from_millis(10))
//         .unwrap();

//     assert_eq!(
//         event.get_entry().get_id(),
//         &log_id,
//         "Set and received log-ids are not equal"
//     );
//     assert_eq!(
//         event
//             .get_entry()
//             .get_causes()
//             .get(0)
//             .unwrap()
//             .get_entry()
//             .get_id(),
//         &cause_log_id,
//         "Set and received causing log-ids are not equal"
//     );
//     assert_eq!(
//         event
//             .get_entry()
//             .get_causes()
//             .get(0)
//             .unwrap()
//             .get_entry()
//             .get_msg(),
//         &cause_msg,
//         "Set and received causing msgs are not equal"
//     );
// }

// #[test]
// fn capture_single_logid_with_info() {
//     let log_id = get_log_id(0, 1, LogLevel::Info, 1);
//     let msg = "Set first log message";
//     let info = "Additional info for this log-id";

//     let recv = publisher::subscribe(log_id, env!("CARGO_PKG_NAME")).unwrap();

//     set_event!(log_id, msg).add_info(info).finalize();

//     let event = recv
//         .recv_timeout(std::time::Duration::from_millis(10))
//         .unwrap();

//     let entry = event.get_entry();
//     assert_eq!(
//         *entry.get_id(),
//         log_id,
//         "Set and stored log-ids are not equal"
//     );
//     assert_eq!(
//         *entry.get_level(),
//         LogLevel::Info,
//         "Set and stored event levels are not equal"
//     );

//     assert_eq!(
//         entry.get_infos().len(),
//         1,
//         "More than one or no info was set"
//     );
//     let act_info = entry.get_infos().last().unwrap();
//     assert_eq!(act_info, info, "Set and stored messages are not equal");
// }

// #[test]
// fn capture_single_logid_with_debug() {
//     let log_id = get_log_id(1, 1, LogLevel::Debug, 0);
//     let msg = "Set first log message";
//     let debug = "Additional debug info for this log-id";

//     let recv = publisher::subscribe(log_id, env!("CARGO_PKG_NAME")).unwrap();

//     set_event!(log_id, msg).add_debug(debug).finalize();

//     let event = recv
//         .recv_timeout(std::time::Duration::from_millis(10))
//         .unwrap();

//     let entry = event.get_entry();
//     assert_eq!(
//         *entry.get_id(),
//         log_id,
//         "Set and stored log-ids are not equal"
//     );
//     assert_eq!(
//         *entry.get_level(),
//         LogLevel::Debug,
//         "Set and stored event levels are not equal"
//     );

//     assert_eq!(
//         entry.get_debugs().len(),
//         1,
//         "More than one or no debug info was set"
//     );
//     let act_debug = entry.get_debugs().last().unwrap();
//     assert_eq!(act_debug, debug, "Set and stored messages are not equal");
// }

// #[test]
// fn capture_single_logid_with_trace() {
//     let log_id = get_log_id(1, 1, LogLevel::Debug, 0);
//     let msg = "Set first log message";
//     let trace = "Additional debug info for this log-id";

//     let recv = publisher::subscribe(log_id, env!("CARGO_PKG_NAME")).unwrap();

//     set_event!(log_id, msg).add_trace(trace).finalize();

//     let event = recv
//         .recv_timeout(std::time::Duration::from_millis(10))
//         .unwrap();

//     let entry = event.get_entry();
//     assert_eq!(
//         *entry.get_id(),
//         log_id,
//         "Set and stored log-ids are not equal"
//     );
//     assert_eq!(
//         *entry.get_level(),
//         LogLevel::Debug,
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

// #[test]
// fn capture_single_logid_with_cause() {
//     let cause_log_id = get_log_id(0, 0, LogLevel::Warn, 1);
//     let cause_msg = "Cause log message";
//     let log_id = get_log_id(0, 0, LogLevel::Error, 2);
//     let msg = "My log message";

//     let recv = publisher::subscribe_to_logs(
//         vec![cause_log_id, log_id].iter().copied(),
//         env!("CARGO_PKG_NAME"),
//     )
//     .unwrap();

//     set_event!(cause_log_id, cause_msg).finalize();

//     let cause_event = recv
//         .recv_timeout(std::time::Duration::from_millis(10))
//         .unwrap();

//     set_event!(log_id, msg).add_cause(cause_event).finalize();

//     let event = recv
//         .recv_timeout(std::time::Duration::from_millis(10))
//         .unwrap();

//     assert_eq!(
//         event.get_entry().get_id(),
//         &log_id,
//         "Set and received log-ids are not equal"
//     );
//     assert_eq!(
//         event
//             .get_entry()
//             .get_causes()
//             .get(0)
//             .unwrap()
//             .get_entry()
//             .get_id(),
//         &cause_log_id,
//         "Set and received causing log-ids are not equal"
//     );
//     assert_eq!(
//         event
//             .get_entry()
//             .get_causes()
//             .get(0)
//             .unwrap()
//             .get_entry()
//             .get_msg(),
//         &cause_msg,
//         "Set and received causing msgs are not equal"
//     );
// }

// #[test]
// fn capture_single_logid_with_info() {
//     let log_id = get_log_id(0, 1, LogLevel::Info, 1);
//     let msg = "Set first log message";
//     let info = "Additional info for this log-id";

//     let recv = publisher::subscribe(log_id, env!("CARGO_PKG_NAME")).unwrap();

//     set_event!(log_id, msg).add_info(info).finalize();

//     let event = recv
//         .recv_timeout(std::time::Duration::from_millis(10))
//         .unwrap();

//     let entry = event.get_entry();
//     assert_eq!(
//         *entry.get_id(),
//         log_id,
//         "Set and stored log-ids are not equal"
//     );
//     assert_eq!(
//         *entry.get_level(),
//         LogLevel::Info,
//         "Set and stored event levels are not equal"
//     );

//     assert_eq!(
//         entry.get_infos().len(),
//         1,
//         "More than one or no info was set"
//     );
//     let act_info = entry.get_infos().last().unwrap();
//     assert_eq!(act_info, info, "Set and stored messages are not equal");
// }

// #[test]
// fn capture_single_logid_with_debug() {
//     let log_id = get_log_id(1, 1, LogLevel::Debug, 0);
//     let msg = "Set first log message";
//     let debug = "Additional debug info for this log-id";

//     let recv = publisher::subscribe(log_id, env!("CARGO_PKG_NAME")).unwrap();

//     set_event!(log_id, msg).add_debug(debug).finalize();

//     let event = recv
//         .recv_timeout(std::time::Duration::from_millis(10))
//         .unwrap();

//     let entry = event.get_entry();
//     assert_eq!(
//         *entry.get_id(),
//         log_id,
//         "Set and stored log-ids are not equal"
//     );
//     assert_eq!(
//         *entry.get_level(),
//         LogLevel::Debug,
//         "Set and stored event levels are not equal"
//     );

//     assert_eq!(
//         entry.get_debugs().len(),
//         1,
//         "More than one or no debug info was set"
//     );
//     let act_debug = entry.get_debugs().last().unwrap();
//     assert_eq!(act_debug, debug, "Set and stored messages are not equal");
// }

// #[test]
// fn capture_single_logid_with_trace() {
//     let log_id = get_log_id(1, 1, LogLevel::Debug, 0);
//     let msg = "Set first log message";
//     let trace = "Additional debug info for this log-id";

//     let recv = publisher::subscribe(log_id, env!("CARGO_PKG_NAME")).unwrap();

//     set_event!(log_id, msg).add_trace(trace).finalize();

//     let event = recv
//         .recv_timeout(std::time::Duration::from_millis(10))
//         .unwrap();

//     let entry = event.get_entry();
//     assert_eq!(
//         *entry.get_id(),
//         log_id,
//         "Set and stored log-ids are not equal"
//     );
//     assert_eq!(
//         *entry.get_level(),
//         LogLevel::Debug,
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
