crate::evident::create_set_event_macro!(
    logid::log_id::LogId,
    logid::logging::event_entry::LogEventEntry,
    logid::logging::intermediary_event::IntermediaryLogEvent
);

#[macro_export]
macro_rules! err {
    ($error:ident) => {
        Err(
            $crate::logging::error_event::ErrLogEvent::new($error, $crate::evident::this_origin!())
                .into_err(),
        )
    };
    ($error:ident, $(add:$addon:expr),*) => {
        Err(
            $crate::logging::error_event::ErrLogEvent::new($error, $crate::evident::this_origin!())
                $(.add_addon($addon))*
                .into_err(),
        )
    };
    ($error:ident, $msg:expr) => {
        Err($crate::logging::error_event::ErrLogEvent::new_with_msg(
            $error,
            $msg,
            $crate::evident::this_origin!(),
        )
        .into_err())
    };
    ($error:ident, $msg:expr, $(add:$addon:expr),*) => {
        Err($crate::logging::error_event::ErrLogEvent::new_with_msg(
            $error,
            $msg,
            $crate::evident::this_origin!(),
        )
        $(.add_addon($addon))*
        .into_err())
    };

    ($enum_name:ident::$variant:ident) => {
        Err($crate::logging::error_event::ErrLogEvent::new(
            $enum_name::$variant,
            $crate::evident::this_origin!(),
        )
        .into_err())
    };
    ($enum_name:ident::$variant:ident, $(add:$addon:expr),*) => {
        Err($crate::logging::error_event::ErrLogEvent::new(
            $enum_name::$variant,
            $crate::evident::this_origin!(),
        )
        $(.add_addon($addon))*
        .into_err())
    };
    ($enum_name:ident::$variant:ident, $msg:expr) => {
        Err($crate::logging::error_event::ErrLogEvent::new_with_msg(
            $enum_name::$variant,
            $msg,
            $crate::evident::this_origin!(),
        )
        .into_err())
    };
    ($enum_name:ident::$variant:ident, $msg:expr, $(add:$addon:expr),*) => {
        Err($crate::logging::error_event::ErrLogEvent::new_with_msg(
            $enum_name::$variant,
            $msg,
            $crate::evident::this_origin!(),
        )
        $(.add_addon($addon))*
        .into_err())
    };
}

#[macro_export]
macro_rules! log {
    ($any:ident) => {
        $crate::set_event!(($any).clone().into(), &$any.to_string()).finalize()
    };
    ($any:ident, $(add:$addon:expr),*) => {
        $crate::set_event!(($any).into(), &$any.to_string())$(.add_addon($addon))*.finalize()
    };
    ($any:ident, $msg:expr) => {
        $crate::set_event!(($any).into(), &$any.to_string()).finalize()
    };
    ($any:ident, $msg:expr, $(add:$addon:expr),*) => {
        $crate::set_event!(($any).into(), &$any.to_string())$(.add_addon($addon))*.finalize()
    };

    ($enum_name:ident::$variant:ident) => {
        $crate::set_event!(
            ($enum_name::$variant).into(),
            &($enum_name::$variant).to_string()
        )
        .finalize()
    };
    ($enum_name:ident::$variant:ident($data:expr)) => {
        $crate::set_event!(
            ($enum_name::$variant($data)).into(),
            &($enum_name::$variant($data)).to_string()
        )
        .finalize()
    };
    ($enum_name:ident::$variant:ident, $(add:$addon:expr),*) => {
        $crate::set_event!(
            ($enum_name::$variant).into(),
            &($enum_name::$variant).to_string()
        )
        $(.add_addon($addon))*
        .finalize()
    };
    ($enum_name:ident::$variant:ident, $msg:expr) => {
        $crate::set_event!(($enum_name::$variant).into(), $msg).finalize()
    };
    ($enum_name:ident::$variant:ident, $msg:expr, $(add:$addon:expr),*) => {
        $crate::set_event!(($enum_name::$variant).into(), $msg)$(.add_addon($addon))*.finalize()
    };
}
