// use logid::{log_map_err, logged_result::LoggedResult, logging::LOGGER, map_err, set_event};
// use logid_derive::ErrLogId;

// #[derive(Default, ErrLogId)]
// enum InnerErrId {
//     #[default]
//     One,
//     Two,
//     Three,
// }

// #[derive(Default, Debug, ErrLogId, PartialEq, Eq)]
// enum OuterErrId {
//     #[default]
//     InternalErr,
//     SomeErr,
//     AnotherErr,
// }

// fn inner_fn() -> LoggedResult<(), InnerErrId> {
//     Err(set_event!(InnerErrId::Two, "Inner msg").into())
// }

// fn outer_fn() -> LoggedResult<(), OuterErrId> {
//     log_map_err!(inner_fn() -> OuterErrId)?;
//     Err(set_event!(OuterErrId::SomeErr, "Outer msg").into())
// }

// #[test]
// fn use_early_logged_return_with_enums() {
//     let recv = LOGGER.subscribe(OuterErrId::InternalErr).unwrap();

//     let res = outer_fn();

//     assert!(res.is_err(), "Failing function did not fail.");
//     assert_eq!(
//         res.unwrap_err().error,
//         OuterErrId::InternalErr,
//         "Result conversion did not map to `default` variant."
//     );

//     let event = recv
//         .get_receiver()
//         .recv_timeout(std::time::Duration::from_millis(10))
//         .unwrap();

//     assert_eq!(
//         event.get_entry().get_msg(),
//         &format!(
//             "'ERR: logid::logged_results::OuterErrId::InternalErr' caused by 'ERR: logid::logged_results::InnerErrId::Two'. Details see entry: '{}'.",
//             event.get_entry().get_causes().first().unwrap().entry_id
//         ),
//         "Result conversion set wrong msg."
//     );
// }

// fn outer_silent_fn() -> LoggedResult<(), OuterErrId> {
//     map_err!(inner_fn() -> OuterErrId)?;
//     Err(set_event!(OuterErrId::SomeErr, "Outer msg").into())
// }

// #[test]
// fn use_early_silent_return_with_enums() {
//     let recv = LOGGER.subscribe(OuterErrId::InternalErr).unwrap();

//     let res = outer_silent_fn();

//     assert!(res.is_err(), "Failing function did not fail.");
//     assert_eq!(
//         res.unwrap_err().error,
//         OuterErrId::InternalErr,
//         "Result conversion did not map to `default` variant."
//     );

//     let event = recv
//         .get_receiver()
//         .recv_timeout(std::time::Duration::from_millis(10));

//     assert!(event.is_err(), "Silent error conversion set an event.");
// }

// fn outer_silent_fully_qualified() -> LoggedResult<(), OuterErrId> {
//     map_err!(inner_fn() -> OuterErrId::SomeErr)?;
//     Err(set_event!(OuterErrId::AnotherErr, "Outer msg").into())
// }

// #[test]
// fn map_err_with_fully_qualified_enum() {
//     let res = outer_silent_fully_qualified();

//     assert!(res.is_err(), "Failing function did not fail.");
//     assert_eq!(
//         res.unwrap_err().error,
//         OuterErrId::SomeErr,
//         "Result conversion did not map to explicitly set variant."
//     );
// }

// fn outer_log_fully_qualified() -> LoggedResult<(), OuterErrId> {
//     log_map_err!(inner_fn() -> OuterErrId::SomeErr)?;
//     Err(set_event!(OuterErrId::AnotherErr, "Outer msg").into())
// }

// #[test]
// fn log_map_err_with_fully_qualified_enum() {
//     let recv = LOGGER.subscribe(OuterErrId::SomeErr).unwrap();

//     let res = outer_log_fully_qualified();

//     assert!(res.is_err(), "Failing function did not fail.");
//     assert_eq!(
//         res.unwrap_err().error,
//         OuterErrId::SomeErr,
//         "Result conversion did not map to explicitly set variant."
//     );

//     let event = recv
//         .get_receiver()
//         .recv_timeout(std::time::Duration::from_millis(10))
//         .unwrap();

//     assert_eq!(
//         event.get_entry().get_msg(),
//         &format!(
//             "'ERR: logid::logged_results::OuterErrId::SomeErr' caused by 'ERR: logid::logged_results::InnerErrId::Two'. Details see entry: '{}'.",
//             event.get_entry().get_causes().first().unwrap().entry_id
//         ),
//         "Result conversion set wrong msg."
//     );
// }
