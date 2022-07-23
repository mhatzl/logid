//! Contains the [`IdEntry`] definition used to capture messages for a log-id.
use crate::log_id::{EventLevel, LogId};

/// Structure representing the origin of a log-id.
#[derive(Debug, Default, PartialEq)]
pub struct Origin {
    /// Filename where the log-id was set
    filename: String,
    /// Linenumber where the log-id was set
    line_nr: u32,
}

impl Origin {
    pub fn new(filename: &str, line_nr: u32) -> Self {
        Origin {
            filename: filename.to_string(),
            line_nr,
        }
    }
}

impl From<&Origin> for String {
    fn from(origin: &Origin) -> Self {
        format!(
            "file=\"{}\", line={}",
            origin.filename, origin.line_nr
        )
    }
}

impl core::fmt::Display for Origin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from(self))
    }
}

/// Structure to capture all messages set for a log-id.
#[derive(Debug, Default)]
pub struct LogIdEntry {
    /// The log-id
    pub id: LogId,
    /// The level of the log-id
    pub level: EventLevel,
    /// The main message set when creating the log-id
    pub msg: String,
    /// List of causes for this log-id
    pub causes: Option<Vec<String>>,
    /// List of additional informations for this log-id
    pub infos: Option<Vec<String>>,
    /// List of additional debug informations for this log-id
    pub debugs: Option<Vec<String>>,
    /// List of additional trace information for this log-id
    pub traces: Option<Vec<String>>,
    /// Code position where the log-id was created
    pub origin: Origin,
    /// Name of the span that was current when the log-id event was set
    pub span: &'static str,
}

impl LogIdEntry {
    pub(crate) fn add_cause(&mut self, cause: String) {
        if let Some(causes) = self.causes.as_mut() {
            causes.push(cause);
        } else {
            self.causes = Some([cause].into());
        }
    }

    pub(crate) fn add_addon(&mut self, level: &tracing::Level, addon: String) {
        let addons = match *level {
            tracing::Level::INFO => self.infos.as_mut(),
            tracing::Level::DEBUG => self.debugs.as_mut(),
            tracing::Level::TRACE => self.traces.as_mut(),
            _ => {
                return;
            }
        };

        if let Some(addons) = addons {
            addons.push(addon);
        } else {
            match *level {
                tracing::Level::INFO => self.infos = Some([addon].into()),
                tracing::Level::DEBUG => self.debugs = Some([addon].into()),
                tracing::Level::TRACE => self.traces = Some([addon].into()),
                _ => (),
            }
        }
    }
}
