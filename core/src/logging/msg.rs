#[derive(Debug, Clone)]
pub struct LogMsg {
    msg: String,

    #[cfg(feature = "fmt")]
    fmt: Option<FmtMsg>,
}

pub const NO_MSG: Option<LogMsg> = None;

#[cfg(feature = "fmt")]
impl LogMsg {
    pub fn get_fmt_data(&self) -> Option<&crate::serde_json::Value> {
        self.fmt.as_ref().map(|f| f.get_data())
    }
}

impl evident::event::Msg for LogMsg {}

impl std::fmt::Display for LogMsg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[cfg(feature = "fmt")]
        if let Some(fmt) = &self.fmt {
            return write!(f, "{}", fmt);
        }

        write!(f, "{}", self.msg)
    }
}

impl From<String> for LogMsg {
    fn from(value: String) -> Self {
        LogMsg {
            msg: value,
            #[cfg(feature = "fmt")]
            fmt: None,
        }
    }
}

impl From<&str> for LogMsg {
    fn from(value: &str) -> Self {
        LogMsg {
            msg: value.to_string(),
            #[cfg(feature = "fmt")]
            fmt: None,
        }
    }
}

#[cfg(feature = "fmt")]
impl
    From<(
        for<'a> fn(&'a crate::serde_json::Value) -> String,
        crate::serde_json::Value,
    )> for LogMsg
{
    fn from(
        value: (
            for<'a> fn(&'a crate::serde_json::Value) -> String,
            crate::serde_json::Value,
        ),
    ) -> Self {
        LogMsg {
            msg: String::default(),
            #[cfg(feature = "fmt")]
            fmt: Some(FmtMsg {
                func: value.0,
                data: value.1,
            }),
        }
    }
}

#[cfg(feature = "fmt")]
impl From<FmtMsg> for LogMsg {
    fn from(value: FmtMsg) -> Self {
        LogMsg {
            msg: String::default(),
            #[cfg(feature = "fmt")]
            fmt: Some(value),
        }
    }
}

impl From<LogMsg> for String {
    fn from(value: LogMsg) -> Self {
        #[cfg(feature = "fmt")]
        if let Some(fmt) = value.fmt {
            return fmt.to_string();
        }

        value.msg
    }
}

impl PartialEq<String> for LogMsg {
    fn eq(&self, other: &String) -> bool {
        #[cfg(feature = "fmt")]
        if let Some(fmt) = &self.fmt {
            return fmt.to_string().eq(other);
        }

        self.msg.eq(other)
    }
}

impl PartialEq<str> for LogMsg {
    fn eq(&self, other: &str) -> bool {
        #[cfg(feature = "fmt")]
        if let Some(fmt) = &self.fmt {
            return fmt.to_string().eq(other);
        }

        self.msg.eq(other)
    }
}

#[cfg(feature = "fmt")]
#[derive(Clone)]
pub struct FmtMsg {
    func: for<'a> fn(&'a crate::serde_json::Value) -> String,
    data: crate::serde_json::Value,
}

#[cfg(feature = "fmt")]
impl FmtMsg {
    pub fn new(
        func: for<'a> fn(&'a crate::serde_json::Value) -> String,
        data: crate::serde_json::Value,
    ) -> Self {
        FmtMsg { func, data }
    }

    pub fn get_data(&self) -> &crate::serde_json::Value {
        &self.data
    }
}

#[cfg(feature = "fmt")]
impl std::fmt::Debug for FmtMsg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", (self.func)(&self.data))
    }
}

#[cfg(feature = "fmt")]
impl std::fmt::Display for FmtMsg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", (self.func)(&self.data))
    }
}

#[cfg(feature = "fmt")]
impl PartialEq<FmtMsg> for FmtMsg {
    fn eq(&self, other: &FmtMsg) -> bool {
        (self.func)(&self.data) == (other.func)(&other.data)
    }
}

#[cfg(feature = "fmt")]
impl Eq for FmtMsg {}
