use std::error::Error;

use logid::{
    err,
    log_id::{LogId, LogLevel},
    logging::{event_entry::EntryKind, LOGGER},
    new_log_id,
};

#[derive(Debug, Clone, Copy)]
struct TestDummy {}

impl std::fmt::Display for TestDummy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TestDummy")
    }
}

impl Error for TestDummy {}

impl From<TestDummy> for LogId {
    fn from(_value: TestDummy) -> Self {
        new_log_id!("TestDummy", LogLevel::Error)
    }
}

#[test]
fn add_single_info_for_err() {
    let dummy = TestDummy {};
    let info_msg = EntryKind::Info("Test".to_owned());

    let recv = LOGGER.subscribe(dummy.into()).unwrap();

    let _: Result<(), _> = err!(dummy, addon: info_msg.clone());

    let event = recv
        .get_receiver()
        .recv_timeout(std::time::Duration::from_millis(10))
        .unwrap();

    let entry = event.get_entry();
    assert_eq!(
        entry.get_infos().len(),
        1,
        "Additional info was not added to entry"
    );
    assert_eq!(
        EntryKind::Info(entry.get_infos().first().unwrap().to_string()),
        info_msg,
        "Set additional info was not stored in the entry"
    );
}

#[test]
fn add_multiple_infos_for_err() {
    let dummy = TestDummy {};
    let info_msg = EntryKind::Info("Test".to_owned());

    let recv = LOGGER.subscribe(dummy.into()).unwrap();

    let _: Result<(), _> = err!(
        dummy,
        addon: info_msg.clone(),
        addon: info_msg.clone(),
        addon: info_msg.clone()
    );

    let event = recv
        .get_receiver()
        .recv_timeout(std::time::Duration::from_millis(10))
        .unwrap();

    let entry = event.get_entry();
    assert_eq!(
        entry.get_infos().len(),
        3,
        "Additional info was not added to entry"
    );
    assert_eq!(
        EntryKind::Info(entry.get_infos().first().unwrap().to_string()),
        info_msg,
        "Set additional info was not stored in the entry"
    );
}
