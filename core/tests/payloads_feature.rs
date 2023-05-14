#[cfg(feature = "payloads")]
mod payload_tests {
    use evident::event::entry::EventEntry;
    use logid::{log_id::LogLevel, logging::LOGGER, set_event};
    use logid_derive::TraceLogId;

    #[derive(Debug, Default, TraceLogId, PartialEq)]
    enum TestTraceId {
        #[default]
        One,
        Two,
    }

    #[test]
    fn capture_single_logid_with_paylod() {
        let msg = "Set first log message";
        let payload = serde_json::json!({
            "code": 200,
            "success": true,
            "payload": {
                "features": [
                    "serde",
                    "json"
                ]
            }
        });

        let recv = LOGGER.subscribe(TestTraceId::One.into()).unwrap();

        set_event!(TestTraceId::One, msg)
            .add_payload(payload.clone())
            .finalize();

        let event = recv
            .get_receiver()
            .recv_timeout(std::time::Duration::from_millis(10))
            .unwrap();

        let entry = event.get_entry();
        assert_eq!(
            TestTraceId::from(*entry.get_event_id()),
            TestTraceId::One,
            "Set and stored log-ids are not equal"
        );
        assert_eq!(
            entry.get_level(),
            LogLevel::Trace,
            "Set and stored event levels are not equal"
        );

        assert_eq!(
            entry.get_payloads().len(),
            1,
            "More than one or no payload was set"
        );
        let act_payload = entry.get_payloads().last().unwrap();
        assert_eq!(
            act_payload, &payload,
            "Set and stored payloads are not equal"
        );
    }
}
