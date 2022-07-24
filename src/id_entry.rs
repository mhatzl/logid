//! Contains the [`LogIdEntry`] definition used to capture messages for a log-id.
use crate::log_id::{EventLevel, LogId, LogIdLevel};

/// Structure representing the origin of a log-id.
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Origin {
    /// Filename where the log-id was set
    filename: String,
    /// Linenumber where the log-id was set
    line_nr: u32,
}

impl Origin {
    /// Create a new [`Origin`].
    pub fn new(filename: &str, line_nr: u32) -> Self {
        Origin {
            filename: filename.to_string(),
            line_nr,
        }
    }
}

impl From<&Origin> for String {
    /// Outputs given [`Origin`] as `file="<filename>", line=<line number>`.
    fn from(origin: &Origin) -> Self {
        format!("file=\"{}\", line={}", origin.filename, origin.line_nr)
    }
}

impl core::fmt::Display for Origin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from(self))
    }
}

/// Structure to capture all messages set for a log-id.
#[derive(Debug, Default, Clone)]
pub struct LogIdEntry {
    /// The log-id
    pub id: LogId,
    /// The level of the log-id
    pub level: EventLevel,
    /// The main message set when creating the log-id
    pub msg: String,
    /// List of causes for this log-id
    pub causes: Vec<String>,
    /// List of additional informations for this log-id
    pub infos: Vec<String>,
    /// List of additional debug informations for this log-id
    pub debugs: Vec<String>,
    /// List of additional trace information for this log-id
    pub traces: Vec<String>,
    /// Code position where the log-id was created
    pub origin: Origin,
    /// Name of the span that was current when the log-id event was set
    pub span: &'static str,

    /// Flag to inform that an entry may be safely drained.
    /// This is the case, when no more information is added to the entry.
    drainable: bool,
}

impl LogIdEntry {
    /// Create a new [`LogIdEntry`].
    pub(crate) fn new(id: LogId, msg: &str, filename: &str, line_nr: u32) -> Self {
        LogIdEntry {
            id,
            level: id.get_level(),
            msg: msg.to_string(),
            origin: Origin::new(filename, line_nr),
            span: if let Some(span) = tracing::span::Span::current().metadata() {
                span.name()
            } else {
                "event not in span"
            },
            ..Default::default()
        }
    }

    /// Add cause to given [`LogIdEntry`].
    pub(crate) fn add_cause(&mut self, cause: String) {
        self.causes.push(cause);
    }

    /// Add additional information to given [`LogIdEntry`].
    /// The destination depends on the given [`tracing::Level`].
    ///
    /// # Arguments
    ///
    /// * `level` - [tracing::Level`] defining the destination of the addon
    /// * `addon` - the additional information that is added to the [`LogIdEntry`]
    pub(crate) fn add_addon(&mut self, level: &tracing::Level, addon: String) {
        let addons = match *level {
            tracing::Level::INFO => &mut self.infos,
            tracing::Level::DEBUG => &mut self.debugs,
            tracing::Level::TRACE => &mut self.traces,
            _ => {
                return;
            }
        };

        addons.push(addon);
    }

    /// Finalizing an entry sets the `drainable` flag,
    /// and marks that no more information will be added to this entry.
    pub(crate) fn finalize(&mut self) {
        self.drainable = true;
    }

    /// Returns `true` if the entry is safe to drain.
    /// Meaning that no more additional information is added to this entry.
    pub fn drainable(&self) -> bool {
        self.drainable
    }
}
