#[cfg(feature = "payloads")]
mod payload_tests {

    use logid::log_id::{LogId, LogLevel};
    use serde::{Deserialize, Serialize};

    #[derive(PartialEq, Eq, Debug, Default, Deserialize, Serialize)]
    struct Payload {
        something: String,
    }

    #[derive(PartialEq, Eq, Debug, Default)]
    enum LogIdEnum {
        First(Payload),
        Second(Payload),
        #[default]
        Third,
    }

    struct PayloadLogId {
        identifier: &'static str,
        payload: Option<serde_json::Value>,
    }

    impl From<PayloadLogId> for LogIdEnum {
        fn from(value: PayloadLogId) -> Self {
            match value.identifier {
                "LogIdEnum::First(_)" => match value.payload {
                    Some(payload) => LogIdEnum::First(serde_json::from_value(payload).unwrap()),
                    None => Self::default(),
                },
                "LogIdEnum::Second(_)" => match value.payload {
                    Some(payload) => LogIdEnum::Second(serde_json::from_value(payload).unwrap()),
                    None => Self::default(),
                },
                "LogIdEnum::Third" => LogIdEnum::Third,
                _ => Self::default(),
            }
        }
    }

    // impl From<logid::logging::intermediary_event::IntermediaryLogEvent> for LogIdEnum {
    //     fn from(value: logid::logging::intermediary_event::IntermediaryLogEvent) -> Self {
    //         value.finalize().into()
    //     }
    // }

    impl From<LogIdEnum> for PayloadLogId {
        fn from(value: LogIdEnum) -> Self {
            match value {
                LogIdEnum::First(payload) => PayloadLogId {
                    identifier: "LogIdEnum::First(_)",
                    payload: Some(serde_json::to_value(payload).unwrap()),
                },
                LogIdEnum::Second(payload) => PayloadLogId {
                    identifier: "LogIdEnum::Second(_)",
                    payload: Some(serde_json::to_value(payload).unwrap()),
                },
                LogIdEnum::Third => PayloadLogId {
                    identifier: "LogIdEnum::Third",
                    payload: None,
                },
            }
        }
    }

    #[test]
    fn enum_into_log_id() {
        let first_enum_id: PayloadLogId = LogIdEnum::First(Payload {
            something: "First".to_owned(),
        })
        .into();
        let second_enum_id: PayloadLogId = LogIdEnum::Second(Payload {
            something: "sec".to_owned(),
        })
        .into();
        let third_enum_id: PayloadLogId = LogIdEnum::Third.into();

        assert_eq!(
            first_enum_id.identifier, "LogIdEnum::First(_)",
            "Derive set wrong identifier name for first variant."
        );
        assert_eq!(
            second_enum_id.identifier, "LogIdEnum::Second(_)",
            "Derive set wrong identifier name for second variant."
        );
        assert_eq!(
            third_enum_id.identifier, "LogIdEnum::Third",
            "Derive set wrong identifier name for third variant."
        );

        assert_eq!(
            LogIdEnum::from(first_enum_id),
            LogIdEnum::First(Payload {
                something: "First".to_owned(),
            }),
            "Derive set wrong identifier name for first variant."
        );
        assert_eq!(
            LogIdEnum::from(second_enum_id),
            LogIdEnum::Second(Payload {
                something: "sec".to_owned(),
            }),
            "Derive set wrong identifier name for second variant."
        );
        assert_eq!(
            LogIdEnum::from(third_enum_id),
            LogIdEnum::Third,
            "Derive set wrong identifier name for third variant."
        );
    }
}
