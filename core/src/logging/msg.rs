#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogMsg {
    msg: String,
}

pub const NO_MSG: Option<LogMsg> = None;

impl evident::event::Msg for LogMsg {}

impl std::fmt::Display for LogMsg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl From<String> for LogMsg {
    fn from(value: String) -> Self {
        LogMsg { msg: value }
    }
}

impl From<&str> for LogMsg {
    fn from(value: &str) -> Self {
        LogMsg {
            msg: value.to_string(),
        }
    }
}

impl From<LogMsg> for String {
    fn from(value: LogMsg) -> Self {
        value.msg
    }
}

impl PartialEq<String> for LogMsg {
    fn eq(&self, other: &String) -> bool {
        self.msg.eq(other)
    }
}

impl PartialEq<str> for LogMsg {
    fn eq(&self, other: &str) -> bool {
        self.msg.eq(other)
    }
}
