#[cfg(feature = "payloads")]
mod payload_tests {
    use logid::{
        evident::event::entry::EventEntry,
        log,
        log_id::LogLevel,
        logging::{event_entry::AddonKind, LOGGER},
    };
    use logid_derive::{FromLogId, TraceLogId};

    #[derive(Debug, Default, PartialEq, Clone, TraceLogId, FromLogId)]
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

        log!(
            TestTraceId::One,
            msg,
            add: AddonKind::Payload(payload.clone())
        );

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
