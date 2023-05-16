#[macro_export]
macro_rules! err {
    ($error:ident) => {
        Err($crate::logging::error_event::ErrLogEvent<_>::new($error, $crate::evident::this_origin!()).into_err())
    };
    ($error:ident, $msg:expr) => {
        Err($crate::logging::error_event::ErrLogEvent<_>::new_with_msg($error, $msg, $crate::evident::this_origin!()).into_err())
    };

    ($enum_name:ident::$variant:ident) => {
        Err($crate::logging::error_event::ErrLogEvent::new($enum_name::$variant, $crate::evident::this_origin!()).into_err())
    };
    ($enum_name:ident::$variant:ident, $msg:expr) => {
        Err($crate::logging::error_event::ErrLogEvent::new_with_msg($enum_name::$variant, $msg, $crate::evident::this_origin!()).into_err())
    };
}

#[macro_export]
macro_rules! intermediary_err {
    ($error:ident) => {
        $crate::logging::error_event::ErrLogEvent<_>::new($error, $crate::evident::this_origin!())
    };
    ($error:ident, $msg:expr) => {
        $crate::logging::error_event::ErrLogEvent<_>::new_with_msg($error, $msg, $crate::evident::this_origin!())
    };

    ($enum_name:ident::$variant:ident) => {
        $crate::logging::error_event::ErrLogEvent::new($enum_name::$variant, $crate::evident::this_origin!())
    };
    ($enum_name:ident::$variant:ident, $msg:expr) => {
        $crate::logging::error_event::ErrLogEvent::new_with_msg($enum_name::$variant, $msg, $crate::evident::this_origin!())
    };
}

#[macro_export]
macro_rules! log {
    ($any:ident) => {
        $crate::set_event!(($enum_name::$variant).into(), $any.to_string()).finalize()
    };
    ($any:ident, $msg:expr) => {
        $crate::set_event!(($enum_name::$variant).into(), $any.to_string()).finalize()
    };

    ($enum_name:ident::$variant:ident:ident) => {
        $crate::set_event!(
            ($enum_name::$variant).into(),
            ($enum_name::$variant).to_string()
        )
        .finalize()
    };
    ($enum_name:ident::$variant:ident, $msg:expr) => {
        $crate::set_event!(($enum_name::$variant).into(), $msg).finalize()
    };
}

#[macro_export]
macro_rules! intermediary_log {
    ($any:ident) => {
        $crate::set_event!(($enum_name::$variant).into(), $any.to_string())
    };
    ($any:ident, $msg:expr) => {
        $crate::set_event!(($enum_name::$variant).into(), $any.to_string())
    };

    ($enum_name:ident::$variant:ident:ident) => {
        $crate::set_event!(
            ($enum_name::$variant).into(),
            ($enum_name::$variant).to_string()
        )
    };
    ($enum_name:ident::$variant:ident, $msg:expr) => {
        $crate::set_event!(($enum_name::$variant).into(), $msg)
    };
}
