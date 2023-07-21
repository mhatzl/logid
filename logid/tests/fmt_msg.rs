#[cfg(feature = "fmt")]
mod fmt_tests {
    use std::error::Error;

    use logid::{err, log, pipe};
    use logid_core::{
        evident::event::entry::EventEntry,
        log_id::{LogId, LogLevel},
        logging::LOGGER,
        new_log_id,
    };
    use serde::{Deserialize, Serialize};

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

    #[derive(Serialize, Deserialize)]
    struct MsgData {
        val: String,
    }

    fn fmt_msg(data: &serde_json::Value) -> String {
        data.to_string()
    }

    #[test]
    fn log_with_fmt_msg() {
        let dummy = TestDummy {};
        let test_data = MsgData {
            val: "Log".to_string(),
        };

        let recv = LOGGER.subscribe(dummy.into()).unwrap();

        let ser: serde_json::Value = serde_json::to_value(test_data).unwrap();
        log!(dummy, fmt_msg, ser.clone());

        let event = recv
            .get_receiver()
            .recv_timeout(std::time::Duration::from_millis(10))
            .unwrap();

        let entry = event.get_entry();
        assert_eq!(
            entry.get_msg().unwrap(),
            &fmt_msg(&ser),
            "Formatted msg was not stored in the entry"
        );
    }

    #[test]
    fn err_with_fmt_msg() {
        let dummy = TestDummy {};
        let test_data = MsgData {
            val: "Error".to_string(),
        };

        let recv = LOGGER.subscribe(dummy.into()).unwrap();

        let ser: serde_json::Value = serde_json::to_value(test_data).unwrap();
        let _: Result<(), _> = err!(dummy, fmt_msg, ser.clone());

        let event = recv
            .get_receiver()
            .recv_timeout(std::time::Duration::from_millis(10))
            .unwrap();

        let entry = event.get_entry();
        assert_eq!(
            entry.get_msg().unwrap(),
            &fmt_msg(&ser),
            "Formatted msg was not stored in the entry"
        );
    }

    #[test]
    fn pipe_with_fmt_msg() {
        let dummy = TestDummy {};
        let test_data = MsgData {
            val: "Pipe".to_string(),
        };

        let recv = LOGGER.subscribe(dummy.into()).unwrap();

        let ser: serde_json::Value = serde_json::to_value(test_data).unwrap();
        let _ = pipe!(dummy, fmt_msg, ser.clone());

        let event = recv
            .get_receiver()
            .recv_timeout(std::time::Duration::from_millis(10))
            .unwrap();

        let entry = event.get_entry();
        assert_eq!(
            entry.get_msg().unwrap(),
            &fmt_msg(&ser),
            "Formatted msg was not stored in the entry"
        );
    }
}
