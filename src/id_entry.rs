//! Contains the [`LogIdEntry`] definition used to capture messages for a log-id.

#[cfg(feature = "diagnostics")]
use std::path::PathBuf;

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
#[derive(Debug, Clone)]
pub struct LogIdEntry {
    /// The log-id of this entry
    pub id: LogId,
    /// The level of the log-id of this entry
    pub level: EventLevel,
    /// The main message set when creating the log-id entry
    pub msg: String,
    /// List of causes for this log-id entry
    pub causes: Vec<String>,
    /// List of additional informations for this log-id entry
    pub infos: Vec<String>,
    /// List of additional debug informations for this log-id entry
    pub debugs: Vec<String>,
    /// List of additional trace information for this log-id entry
    pub traces: Vec<String>,
    /// Code position where the log-id entry was created
    pub origin: Origin,
    /// Name of the span that was current when the log-id event was set
    pub span: &'static str,

    /// Flag to inform that an entry may be safely drained.
    /// This is the case, when no more information is added to the entry.
    drainable: bool,

    /// List of diagnostics for this log-id entry
    #[cfg(feature = "diagnostics")]
    pub diagnostics: Vec<Diagnostic>,
}

/// Diagnostic struct offering information about the original input
/// that may be used to create detailed diagnostics (e.g. for language server diagnostics).
#[cfg(feature = "diagnostics")]
#[derive(Debug, Clone)]
pub struct Diagnostic {
    /// Original input that caused this log-id entry
    /// 
    /// **Note:** If `filepath` is set instead, the original input may be read directly from the file. 
    pub input: Option<String>,
    /// Path to the file holding the original input that caused this log-id entry
    pub filepath: Option<PathBuf>,
    /// The range inside the original input that caused this log-id entry
    pub range: Range,
    /// Diagnostic tags that apply to this log-id entry
    pub tags: Vec<DiagnosticTag>,
}

/// Specifies a position inside a text-based 2D-structure.
#[cfg(feature = "diagnostics")]
#[derive(Debug, Clone)]
pub struct Position {
    /// The line number of the position.
    pub line: usize,
    /// The column number of the position
    pub column: usize,
}

/// Specifies a range inside a text-based 2D-structure.
#[cfg(feature = "diagnostics")]
#[derive(Debug, Clone)]
pub struct Range {
    /// The start position of the range.
    pub start: Position,
    /// The end position of the range.
    pub end: Position,
}

/// Diagnostic tags inspired from the language server protocol.
#[cfg(feature = "diagnostics")]
#[derive(Debug, Clone)]
pub enum DiagnosticTag {
    /// Tag to mark unused or unnecessary input
    Unnecessary = 1,
    /// Tag to mark the usage of a certain input as deprecated.
    Deprecated = 2,
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
            causes: Vec::default(),
            infos: Vec::default(),
            debugs: Vec::default(),
            traces: Vec::default(),
            drainable: false,

            #[cfg(feature = "diagnostics")]
            diagnostics: Vec::default(),
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

    /// Add diagnostic to given [`LogIdEntry`].
    #[cfg(feature = "diagnostics")]
    pub(crate) fn add_diagnostic(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.push(diagnostic);
    }
}
