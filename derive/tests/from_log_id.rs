use logid::{logid, log_id::{get_log_id, LogLevel}};
use logid_derive::FromLogId;

#[derive(PartialEq, Eq, Debug, Default, FromLogId)]
enum LogIdEnum {
    #[default]
    First = get_log_id(0, 0, LogLevel::Debug, 1),
    Second = get_log_id(0, 0, LogLevel::Debug, 2),
    Third = get_log_id(0, 0, LogLevel::Debug, 3),
}

#[derive(PartialEq, Eq, Debug, Default, FromLogId)]
enum OtherLogIdEnum {
    One = get_log_id(0, 0, LogLevel::Warn, 1),
    Two = get_log_id(0, 0, LogLevel::Warn, 2),
    #[default]
    Three = get_log_id(0, 0, LogLevel::Warn, 3),
}

#[test]
fn enum_roundtrip_conversion() {
    let first_logid = logid!(LogIdEnum::First);
    let second_logid = logid!(LogIdEnum::Second);
    let third_logid = logid!(LogIdEnum::Third);

    assert_eq!(
        LogIdEnum::from(first_logid),
        LogIdEnum::First,
        "Wrong roundtrip conversion to first enum."
    );
    assert_eq!(
        LogIdEnum::from(second_logid),
        LogIdEnum::Second,
        "Wrong roundtrip conversion to second enum."
    );
    assert_eq!(
        LogIdEnum::from(third_logid),
        LogIdEnum::Third,
        "Wrong roundtrip conversion to third enum."
    );

    assert_eq!(
        std::convert::Into::<LogIdEnum>::into(first_logid),
        LogIdEnum::First,
        "Wrong conversion to first enum."
    );
    assert_eq!(
        std::convert::Into::<LogIdEnum>::into(second_logid),
        LogIdEnum::Second,
        "Wrong conversion to second enum."
    );
    assert_eq!(
        std::convert::Into::<LogIdEnum>::into(third_logid),
        LogIdEnum::Third,
        "Wrong conversion to third enum."
    );
}

