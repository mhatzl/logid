crate::evident::create_set_event_macro!(
    logid::log_id::LogId,
    logid::logging::event_entry::LogEventEntry,
    logid::logging::intermediary_event::IntermediaryLogEvent
);

#[macro_export]
macro_rules! log {
    ($any:expr) => {
        $crate::set_event!(($any).clone().into(), &$any.to_string()).finalize()
    };
    ($any:expr, $(add:$addon:expr),*) => {
        $crate::set_event!(($any).into(), &$any.to_string())$(.add_addon($addon))*.finalize()
    };
    ($any:expr, $msg:expr) => {
        $crate::set_event!(($any).into(), $msg).finalize()
    };
    ($any:expr, $msg:expr, $(add:$addon:expr),*) => {
        $crate::set_event!(($any).into(), $msg)$(.add_addon($addon))*.finalize()
    };
}

#[macro_export]
macro_rules! err {
    ($error:expr) => {
        {
            $crate::log!($error);
            Err($error)
        }
    };
    ($error:expr, $(add:$addon:expr),*) => {
        {
            $crate::log!($error, $(add:$addon),*);
            Err($error)
        }
    };
    ($error:expr, $msg:expr) => {
        {
            $crate::log!($error, $msg);
            Err($error)
        }
    };
    ($error:expr, $msg:expr, $(add:$addon:expr),*) => {
        {
            $crate::log!($error, $msg, $(add:$addon),*);
            Err($error)
        }
    };
}

#[macro_export]
macro_rules! pipe {
    ($any:expr) => {
        {
            $crate::log!($any);
            $any
        }
    };
    ($any:expr, $(add:$addon:expr),*) => {
        {
            $crate::log!($any, $(add:$addon),*);
            $any
        }
    };
    ($any:expr, $msg:expr) => {
        {
            $crate::log!($any, $msg);
            $any
        }
    };
    ($any:expr, $msg:expr, $(add:$addon:expr),*) => {
        {
            $crate::log!($any, $msg, $(add:$addon),*);
            $any
        }
    };
}