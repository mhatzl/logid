/// Macro to set an event for the given [`LogId`] using the caller crate to identify the [`LogIdMap`].\
/// The caller crate is identified using the environment variable `CARGO_PKG_NAME` set by cargo.
///
/// **Arguments:**
///
/// * `logid` ... Must be a valid `LogId`
/// * `msg` ... `String` variable or literal of the main message set for the event (should be a user-centered event description)
#[macro_export]
macro_rules! set_event {
    ($logid:ident, $msg:ident) => {
        $crate::event::EventFns::set_event(
            $crate::logid!($logid),
            env!("CARGO_PKG_NAME"),
            $msg,
            file!(),
            line!(),
            module_path!(),
        )
    };
    ($logid:ident, $msg:literal) => {
        $crate::event::EventFns::set_event(
            $crate::logid!($logid),
            env!("CARGO_PKG_NAME"),
            $msg,
            file!(),
            line!(),
            module_path!(),
        )
    };
    ($logid:ident, $msg:expr) => {
        $crate::event::EventFns::set_event(
            $crate::logid!($logid),
            env!("CARGO_PKG_NAME"),
            $msg,
            file!(),
            line!(),
            module_path!(),
        )
    };
    ($logid:expr, $msg:expr) => {
        $crate::event::EventFns::set_event(
            $crate::logid!($logid),
            env!("CARGO_PKG_NAME"),
            $msg,
            file!(),
            line!(),
            module_path!(),
        )
    };
}

/// Macro to set a silent log event.\
/// This is a convenient wrapper around [`LogIdTracing::set_silent_event`] that automatically converts the given [`LogId`].
///
/// **Arguments:**
///
/// * `logid` ... Must be a valid `LogId`
/// * `msg` ... `String` variable or literal of the main message set for the event
#[macro_export]
macro_rules! set_silent_event {
    ($logid:ident, $msg:ident) => {
        $crate::event::EventFns::set_silent_event(
            $crate::logid!($logid),
            $msg,
            file!(),
            line!(),
            module_path!(),
        )
    };
    ($logid:ident, $msg:literal) => {
        $crate::event::EventFns::set_silent_event(
            $crate::logid!($logid),
            $msg,
            file!(),
            line!(),
            module_path!(),
        )
    };
    ($logid:ident, $msg:expr) => {
        $crate::event::EventFns::set_silent_event(
            $crate::logid!($logid),
            $msg,
            file!(),
            line!(),
            module_path!(),
        )
    };
    ($logid:expr, $msg:expr) => {
        $crate::event::EventFns::set_silent_event(
            $crate::logid!($logid),
            $msg,
            file!(),
            line!(),
            module_path!(),
        )
    };
}
