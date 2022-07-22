//! Contains the [`IdEntry`] definition used to capture messages for a log-id.
use lazy_static::lazy_static;
use regex::Regex;

use crate::log_id::{EventLevel, LogId};

/// Structure representing the origin of a log-id.
#[derive(Debug, Default)]
pub struct Origin {
    /// Filename where the log-id was set
    filename: String,
    /// Linenumber where the log-id was set
    line_nr: u32,
}

impl Origin {
    pub(crate) fn new(filename: &str, line_nr: u32) -> Self {
        Origin {
            filename: filename.to_string(),
            line_nr,
        }
    }
}

impl From<Origin> for String {
    fn from(origin: Origin) -> Self {
        format!(
            "Occured in file=\"{}\" at line={}",
            origin.filename, origin.line_nr
        )
    }
}

impl From<String> for Origin {
    fn from(s: String) -> Self {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r#"Occured in file="(?P<file>.+)" at line=(?P<line>\d+)"#).unwrap();
        }

        if let Some(captures) = RE.captures(&s) {
            let line: u32 = match &captures["line"].parse::<u32>() {
                Ok(number) => *number,
                Err(_) => 0,
            };
            return Origin {
                filename: captures["file"].to_string(),
                line_nr: line,
            };
        }
        Origin::default()
    }
}

impl core::fmt::Display for Origin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "file=\"{}\", line={}", self.filename, self.line_nr)
    }
}

/// Structure to capture all messages set for a log-id.
#[derive(Debug, Default)]
pub struct LogIdEntry {
    /// The log-id
    pub(crate) id: LogId,
    /// The level of the log-id
    pub(crate) level: EventLevel,
    /// The main message set when creating the log-id
    pub(crate) msg: String,
    /// List of causes for this log-id
    pub(crate) causes: Option<Vec<String>>,
    /// List of additional informations for this log-id
    pub(crate) infos: Option<Vec<String>>,
    /// List of additional debug informations for this log-id
    pub(crate) debugs: Option<Vec<String>>,
    /// List of additional trace information for this log-id
    pub(crate) traces: Option<Vec<String>>,
    /// Code position where the log-id was created
    pub(crate) origin: Origin,
    /// Name of the span that was current when the log-id event was set
    pub(crate) span: &'static str,
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
