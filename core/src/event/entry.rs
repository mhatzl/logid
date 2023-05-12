use std::{
    collections::hash_map::DefaultHasher,
    fmt::Display,
    hash::{Hash, Hasher},
};

use crate::log_id::{LogId, LogIdParts, LogLevel};

use super::origin::Origin;

#[cfg(feature = "causes")]
use super::Event;

#[cfg(feature = "diagnostics")]
use lsp_types::Diagnostic;

/// Structure to capture all messages set for a log-id.
#[derive(Debug, Clone, Default)]
pub struct EventEntry {
    /// The hash uniquely identifying this entry.
    ///
    /// **Note:** The hash is computed using the current ThreadId and time when the entry is created, and the origin of the entry.
    pub(crate) hash: u64,
    /// The log-id of this entry
    pub(crate) id: LogId,
    /// The level of the log-id of this entry
    pub(crate) level: LogLevel,
    /// The main message set when creating the log-id entry
    pub(crate) msg: String,
    /// List of additional informations for this log-id entry
    pub(crate) infos: Vec<String>,
    /// List of additional debug informations for this log-id entry
    pub(crate) debugs: Vec<String>,
    /// List of additional trace information for this log-id entry
    pub(crate) traces: Vec<String>,
    /// Code position where the log-id entry was created
    pub(crate) origin: Origin,

    /// Name of the span that was current when the log-id event was set
    #[cfg(feature = "spans")]
    pub(crate) span: Option<tracing::span::Span>,

    /// List of other log-id events that caused this log-id entry
    #[cfg(feature = "causes")]
    pub(crate) causes: Vec<Event>,

    /// List of diagnostics for this log-id entry
    #[cfg(feature = "diagnostics")]
    pub(crate) diagnostics: Vec<Diagnostic>,

    /// List of payloads for this log-id entry
    #[cfg(feature = "payloads")]
    pub(crate) payloads: Vec<serde_json::value::Value>,
}

impl EventEntry {
    /// Create a new [`LogIdEntry`].
    pub(crate) fn new(
        id: LogId,
        msg: &str,
        filename: &str,
        line_nr: u32,
        module_path: &str,
    ) -> Self {
        EventEntry {
            hash: compute_hash(filename, line_nr),
            id,
            level: id.get_level(),
            msg: msg.to_string(),
            origin: Origin::new(filename, line_nr, module_path),
            infos: Vec::default(),
            debugs: Vec::default(),
            traces: Vec::default(),

            #[cfg(feature = "spans")]
            span: if tracing::span::Span::current().is_disabled() {
                None
            } else {
                Some(tracing::span::Span::current())
            },

            #[cfg(feature = "causes")]
            causes: Vec::default(),

            #[cfg(feature = "diagnostics")]
            diagnostics: Vec::default(),

            #[cfg(feature = "payloads")]
            payloads: Vec::default(),
        }
    }

    /// Get the log-id of this entry
    pub fn get_id(&self) -> &LogId {
        &self.id
    }
    /// Get the level of the log-id of this entry
    pub fn get_level(&self) -> &LogLevel {
        &self.level
    }
    /// Get the main message set when creating the log-id entry
    pub fn get_msg(&self) -> &String {
        &self.msg
    }
    /// Get the list of additional informations for this log-id entry
    pub fn get_infos(&self) -> &Vec<String> {
        &self.infos
    }
    /// Get the list of additional debug informations for this log-id entry
    pub fn get_debugs(&self) -> &Vec<String> {
        &self.debugs
    }
    /// Get the list of additional trace information for this log-id entry
    pub fn get_traces(&self) -> &Vec<String> {
        &self.traces
    }
    /// Get the code position where the log-id entry was created
    pub fn get_origin(&self) -> &Origin {
        &self.origin
    }

    /// Get the name of the span that was current when the log-id event was set
    #[cfg(feature = "spans")]
    pub fn get_span(&self) -> &Option<tracing::span::Span> {
        &self.span
    }

    #[cfg(feature = "causes")]
    pub fn get_causes(&self) -> &Vec<Event> {
        &self.causes
    }

    #[cfg(feature = "diagnostics")]
    pub fn get_diagnostics(&self) -> &Vec<Diagnostic> {
        &self.diagnostics
    }

    #[cfg(feature = "payloads")]
    pub fn get_payloads(&self) -> &Vec<serde_json::value::Value> {
        &self.payloads
    }
}

impl PartialEq for EventEntry {
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash
    }
}

impl Eq for EventEntry {}

impl Hash for EventEntry {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.hash.hash(state);
    }
}

impl Display for EventEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LogId-Entry")
            .field("id", &self.id)
            .field("origin", &self.origin)
            .field("hash", &self.hash)
            .finish()
    }
}

/// [`EntryKind`] defines the message kind to be added to a [`LogIdEntry`].
pub(crate) enum EntryKind {
    Info(String),
    Debug(String),
    Trace(String),

    #[cfg(feature = "causes")]
    Cause(Event),

    #[cfg(feature = "diagnostics")]
    Diagnostic(Diagnostic),

    #[cfg(feature = "payloads")]
    Payload(serde_json::value::Value),
}

/// This function computes the hash for a [`LogIdEntry`].
///
/// The hash is computed over filename, line_nr, the current ThreadId,
/// and the current UTC time in nanoseconds.
///
/// # Arguments
///
/// - `filename` - the filename to use when calculating the hash
/// - `line_nr` - the line number to use when calculating the hash
///
/// **Note:** The hash function is not cryptographically secure,
/// but that is ok since we only need the hash to identify the entry in a map.
fn compute_hash(filename: &str, line_nr: u32) -> u64 {
    let mut hasher = DefaultHasher::new();
    hasher.write(filename.as_bytes());
    hasher.write_u32(line_nr);
    std::thread::current().id().hash(&mut hasher);
    hasher.write_i64(chrono::Utc::now().timestamp_nanos());
    hasher.finish()
}
