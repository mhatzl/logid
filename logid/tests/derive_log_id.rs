use logid::log_id::{LogId, LogLevel};
use logid::{DbgLogId, ErrLogId, FromLogId, InfoLogId, TraceLogId, WarnLogId};

#[derive(PartialEq, Eq, Debug, Default, ErrLogId, FromLogId)]
enum LogIdEnum {
    #[default]
    First,
    Second,
    Third,
}

#[derive(PartialEq, Eq, Debug, Default, ErrLogId, FromLogId)]
enum OtherLogIdEnum {
    One,
    Two,
    #[default]
    Three,
}

#[test]
fn enum_into_log_id() {
    let first_enum_id: LogId = LogIdEnum::First.into();
    let second_enum_id: LogId = LogIdEnum::Second.into();
    let third_enum_id: LogId = LogIdEnum::Third.into();

    assert_eq!(
        first_enum_id.get_module_path(),
        module_path!(),
        "Derive set wrong module path."
    );
    assert_eq!(
        first_enum_id.get_identifier(),
        "LogIdEnum::First",
        "Derive set wrong identifier name for first variant."
    );
    assert_eq!(
        first_enum_id.get_log_level(),
        LogLevel::Error,
        "Derive set wrong log level."
    );
    assert_eq!(
        second_enum_id.get_identifier(),
        "LogIdEnum::Second",
        "Derive set wrong identifier name for second variant."
    );
    assert_eq!(
        third_enum_id.get_identifier(),
        "LogIdEnum::Third",
        "Derive set wrong identifier name for third variant."
    );
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
    assert_eq!(
        first_id.get_log_level(),
        LogLevel::Error,
        "LogLevel::Error was not set using ErrLogId derive macro.",
    );
}

#[test]
fn enums_as_log_id_differ() {
    let first_enum_id: LogId = LogIdEnum::First.into();
    let other_enum_id: LogId = OtherLogIdEnum::One.into();

    assert_ne!(
        first_enum_id, other_enum_id,
        "Converted LogIds from different enums are equal.",
    );
}

#[derive(PartialEq, Eq, Debug, Default, WarnLogId)]
enum WarnLogId {
    #[default]
    First,
    Second,
}

#[test]
fn enum_as_warn_log_id() {
    let warn_id: LogId = WarnLogId::Second.into();

    assert_eq!(
        warn_id.get_identifier(),
        "WarnLogId::Second",
        "Derive set wrong identifier name for second variant.",
    );

    assert_eq!(
        warn_id.get_log_level(),
        LogLevel::Warn,
        "LogLevel::Warn was not set using WarnLogId derive macro.",
    );
}

#[derive(PartialEq, Eq, Debug, Default, InfoLogId)]
enum InfoLogId {
    #[default]
    First,
    Second,
}

#[test]
fn enum_as_info_log_id() {
    let info_id: LogId = InfoLogId::Second.into();

    assert_eq!(
        info_id.get_identifier(),
        "InfoLogId::Second",
        "Derive set wrong identifier name for second variant.",
    );

    assert_eq!(
        info_id.get_log_level(),
        LogLevel::Info,
        "LogLevel::Info was not set using InfoLogId derive macro.",
    );
}

#[derive(PartialEq, Eq, Debug, Default, DbgLogId)]
enum DbgLogId {
    #[default]
    First,
    Second,
}

#[test]
fn enum_as_dbg_log_id() {
    let dbg_id: LogId = DbgLogId::Second.into();

    assert_eq!(
        dbg_id.get_identifier(),
        "DbgLogId::Second",
        "Derive set wrong identifier name for second variant.",
    );

    assert_eq!(
        dbg_id.get_log_level(),
        LogLevel::Debug,
        "LogLevel::Debug was not set using DbgLogId derive macro.",
    );
}

#[derive(PartialEq, Eq, Debug, Default, TraceLogId)]
enum TraceLogId {
    #[default]
    First,
    Second,
}

#[test]
fn enum_as_trace_log_id() {
    let trace_id: LogId = TraceLogId::Second.into();

    assert_eq!(
        trace_id.get_identifier(),
        "TraceLogId::Second",
        "Derive set wrong identifier name for second variant.",
    );

    assert_eq!(
        trace_id.get_log_level(),
        LogLevel::Trace,
        "LogLevel::Trace was not set using TraceLogId derive macro.",
    );
}
