use logid::log_id::{LogId, LogLevel};
use logid_derive::ErrLogId;

#[derive(PartialEq, Eq, Debug, Default, ErrLogId)]
enum LogIdEnum {
    #[default]
    First,
    Second,
    Third,
}

#[derive(PartialEq, Eq, Debug, Default, ErrLogId)]
enum OtherLogIdEnum {
    One,
    Two,
    #[default]
    Three,
}

#[test]
fn enum_as_err_log_id() {
    let first_id: LogId = LogIdEnum::First.into();
    let first_enum: LogIdEnum = first_id.into();

    assert_eq!(
        LogIdEnum::from(first_id),
        LogIdEnum::First,
        "Conversion back to enum using `from()` failed.",
    );
    assert_eq!(
        first_enum,
        LogIdEnum::First,
        "Conversion back to enum using `into()` failed.",
    );
    assert_eq!(first_id.get_log_level(), LogLevel::Error, "LogLevel .",);
}
