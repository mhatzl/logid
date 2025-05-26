crate::evident::create_set_event_macro!(
    id_type = logid::log_id::LogId,
    msg_type = logid::logging::msg::LogMsg,
    entry_type = logid::logging::event_entry::LogEventEntry,
    interm_event_type = logid::logging::intermediary_event::IntermediaryLogEvent
);

#[macro_export]
macro_rules! log {
    ($any:expr) => {
        {
            let s = $any.to_string();
            $crate::set_event!(($any).into(), s).finalize()
        }
    };
    ($any:expr, $(add:$addon:expr),*) => {
        {
            let s = $any.to_string();
            $crate::set_event!(($any).into(), s)$(.add_addon($addon))*.finalize()
        }
    };
    ($any:expr, $msg:expr) => {
        $crate::set_event!(($any).into(), $msg).finalize()
    };
    ($any:expr, $msg:expr, $(add:$addon:expr),*) => {
        $crate::set_event!(($any).into(), $msg)$(.add_addon($addon))*.finalize()
    };

    // Note: It is not possible to check for "fmt" feature flag here
    ($any:expr, $fmt_fn:expr, $fmt_data:expr) => {
        $crate::set_event!(($any).into(), $crate::logging::msg::FmtMsg::new($fmt_fn, $fmt_data)).finalize()
    };
    ($any:expr, $fmt_fn:expr, $fmt_data:expr, $(add:$addon:expr),*) => {
        $crate::set_event!(($any).into(), $crate::logging::msg::FmtMsg::new($fmt_fn, $fmt_data))$(.add_addon($addon))*.finalize()
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

    ($error:expr, $fmt_fn:expr, $fmt_data:expr) => {
        {
            $crate::log!($error, $fmt_fn, $fmt_data);
            Err($error)
        }
    };
    ($error:expr, $fmt_fn:expr, $fmt_data:expr, $(add:$addon:expr),*) => {
        {
            $crate::log!($error, $fmt_fn, $fmt_data, $(add:$addon),*);
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

    ($any:expr, $fmt_fn:expr, $fmt_data:expr) => {
        {
            $crate::log!($any, $fmt_fn, $fmt_data);
            $any
        }
    };
    ($any:expr, $fmt_fn:expr, $fmt_data:expr, $(add:$addon:expr),*) => {
        {
            $crate::log!($any, $fmt_fn, $fmt_data, $(add:$addon),*);
            $any
        }
    };
}
