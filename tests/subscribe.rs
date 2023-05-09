use logid::{log_id::{get_log_id, EventLevel}, set_event, publisher::subscribe, subscribe, logid};

#[test]
fn subscribe_to_one_logid() {
  let log_id = get_log_id(0, 0, EventLevel::Error, 2);
  let msg = "Set first log message";
  
  let recv = subscribe(log_id, env!("CARGO_PKG_NAME")).unwrap();

  set_event!(log_id, msg).finalize();

  let event = recv.recv_timeout(std::time::Duration::from_millis(10)).unwrap();
  assert_eq!(event.crate_name, env!("CARGO_PKG_NAME"), "Event received from wrong crate.");
  assert_eq!(event.entry.get_id(), &log_id, "Received event has wrong LogId.");
  assert_eq!(event.entry.get_msg(), msg, "Received event has wrong msg.");
}

#[test]
fn subscribe_macro() {
  let log_id = get_log_id(0, 0, EventLevel::Error, 2);
  let msg = "Set first log message";
  
  let recv = subscribe!(log_id).unwrap();

  set_event!(log_id, msg).finalize();

  let event = recv.recv_timeout(std::time::Duration::from_millis(10)).unwrap();
  assert_eq!(event.crate_name, env!("CARGO_PKG_NAME"), "Event received from wrong crate.");
  assert_eq!(event.entry.get_id(), &log_id, "Received event has wrong LogId.");
  assert_eq!(event.entry.get_msg(), msg, "Received event has wrong msg.");
}

enum TestLogId {
  Id = get_log_id(0, 0, EventLevel::Error, 2),
}

#[test]
fn subscribe_macro_with_logid_enum() {
  let msg = "Set first log message";
  
  let recv = subscribe!(TestLogId::Id).unwrap();

  set_event!(TestLogId::Id, msg).finalize();

  let event = recv.recv_timeout(std::time::Duration::from_millis(10)).unwrap();
  assert_eq!(event.crate_name, env!("CARGO_PKG_NAME"), "Event received from wrong crate.");
  assert_eq!(event.entry.get_id(), &logid!(TestLogId::Id), "Received event has wrong LogId.");
  assert_eq!(event.entry.get_msg(), msg, "Received event has wrong msg.");
}

#[test]
fn two_log_ids_separate_receiver() {
  let log_id_1 = get_log_id(0, 0, EventLevel::Error, 2);
  let msg_1 = "Set first log message";
  let log_id_2 = get_log_id(0, 0, EventLevel::Error, 3);
  let msg_2 = "Set second log message";
  
  let recv_1 = subscribe!(log_id_1).unwrap();
  let recv_2 = subscribe!(log_id_2).unwrap();

  set_event!(log_id_1, msg_1).finalize();
  set_event!(log_id_2, msg_2).finalize();

  let event_1 = recv_1.recv_timeout(std::time::Duration::from_millis(10)).unwrap();
  assert_eq!(event_1.entry.get_id(), &log_id_1, "Received event 1 has wrong LogId.");
  assert_eq!(event_1.entry.get_msg(), msg_1, "Received event 1 has wrong msg.");

  let event_2 = recv_2.recv_timeout(std::time::Duration::from_millis(10)).unwrap();
  assert_eq!(event_2.entry.get_id(), &log_id_2, "Received event 2 has wrong LogId.");
  assert_eq!(event_2.entry.get_msg(), msg_2, "Received event 2 has wrong msg.");
}
