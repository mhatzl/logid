// //! Contains the [`LogIdEntry`] definition used to capture messages for a log-id.

// use std::collections::hash_map::DefaultHasher;
// use std::collections::HashSet;
// use std::hash::{Hash, Hasher};
// #[cfg(feature = "diagnostics")]
// use std::path::PathBuf;

// use crate::capturing::LogIdEvent;
// use crate::log_id::{LogLevel, LogId, LogIdParts};

// /// Structure representing the origin of a log-id.
// #[derive(Debug, Default, PartialEq, Eq, Clone)]
// pub struct Origin {
//     /// Filename where the log-id was set
//     pub filename: String,
//     /// Linenumber where the log-id was set
//     pub line_nr: u32,
// }

// impl Origin {
//     /// Create a new [`Origin`].
//     pub fn new(filename: &str, line_nr: u32) -> Self {
//         Origin {
//             filename: filename.to_string(),
//             line_nr,
//         }
//     }
// }

// impl From<&Origin> for String {
//     /// Outputs given [`Origin`] as `file="<filename>", line=<line number>`.
//     fn from(origin: &Origin) -> Self {
//         format!("file=\"{}\", line={}", origin.filename, origin.line_nr)
//     }
// }

// impl core::fmt::Display for Origin {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", String::from(self))
//     }
// }

// /// Structure to capture all messages set for a log-id.
// #[derive(Debug, Clone, Eq, Default)]
// pub struct LogIdEntry {
//     /// The hash uniquely identifying this entry.
//     ///
//     /// **Note:** The hash is computed using the current ThreadId and time when the entry is created, and the origin of the entry.
//     pub(crate) hash: u64,
//     /// The log-id of this entry
//     pub(crate) id: LogId,
//     /// The level of the log-id of this entry
//     pub(crate) level: LogLevel,
//     /// The main message set when creating the log-id entry
//     pub(crate) msg: String,
//     /// List of additional informations for this log-id entry
//     pub(crate) infos: Vec<String>,
//     /// List of additional debug informations for this log-id entry
//     pub(crate) debugs: Vec<String>,
//     /// List of additional trace information for this log-id entry
//     pub(crate) traces: Vec<String>,
//     /// Code position where the log-id entry was created
//     pub(crate) origin: Origin,
//     /// Name of the span that was current when the log-id event was set
//     pub(crate) span: &'static str,

//     /// List of causes for this log-id entry
//     #[cfg(feature = "causes")]
//     pub causes: Vec<String>,

//     /// List of diagnostics for this log-id entry
//     #[cfg(feature = "diagnostics")]
//     pub diagnostics: Vec<Diagnostic>,
// }

// /// [`EntryKind`] defines the message kind to be added to a [`LogIdEntry`].
// pub(crate) enum EntryKind {
//     Info(String),
//     Debug(String),
//     Trace(String),
//     #[cfg(feature = "causes")]
//     Cause(String),
//     #[cfg(feature = "diagnostics")]
//     Diagnostic(Diagnostic),
// }

// impl PartialEq for LogIdEntry {
//     fn eq(&self, other: &Self) -> bool {
//         self.hash == other.hash
//     }
// }

// impl Hash for LogIdEntry {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         self.hash.hash(state);
//     }
// }

// impl LogIdEntry {
//     pub(crate) fn shallow_new(hash: u64) -> Self {
//         LogIdEntry {
//             hash,
//             ..Default::default()
//         }
//     }
//     /// Get the log-id of this entry
//     pub fn get_id(&self) -> &LogId {
//         &self.id
//     }
//     /// Get the level of the log-id of this entry
//     pub fn get_level(&self) -> &LogLevel {
//         &self.level
//     }
//     /// Get the main message set when creating the log-id entry
//     pub fn get_msg(&self) -> &String {
//         &self.msg
//     }
//     /// Get the list of additional informations for this log-id entry
//     pub fn get_infos(&self) -> &Vec<String> {
//         &self.infos
//     }
//     /// Get the list of additional debug informations for this log-id entry
//     pub fn get_debugs(&self) -> &Vec<String> {
//         &self.debugs
//     }
//     /// Get the list of additional trace information for this log-id entry
//     pub fn get_traces(&self) -> &Vec<String> {
//         &self.traces
//     }
//     /// Get the code position where the log-id entry was created
//     pub fn get_origin(&self) -> &Origin {
//         &self.origin
//     }
//     /// Get the name of the span that was current when the log-id event was set
//     pub fn get_span(&self) -> &str {
//         self.span
//     }
// }

// /// Trait used to implement functions on the [`LogIdEntry`] HashSet.
// pub trait LogIdEntrySet {
//     /// Returns the [`LogId`] all entries in this [`LogIdEntrySet`] have, or `None` if the set is empty.
//     fn get_logid(&self) -> Option<LogId>;

//     /// Tries to get a [`LogIdEntry`] that is identified by the given [`MappedLogId`].
//     ///
//     /// # Arguments
//     ///
//     /// - `mapped_id` - [`MappedLogId`] used to identify the [`LogIdEntry`]
//     fn get_entry(&self, mapped_id: &LogIdEvent) -> Option<&LogIdEntry>;

//     /// Tries to retrieve a [`LogIdEntry`] that is identified by the given [`MappedLogId`].
//     ///
//     /// # Arguments
//     ///
//     /// - `mapped_id` - [`MappedLogId`] used to identify the [`LogIdEntry`]
//     fn take_entry(&mut self, mapped_id: &LogIdEvent) -> Option<LogIdEntry>;
// }

// impl LogIdEntrySet for HashSet<LogIdEntry> {
//     fn get_logid(&self) -> Option<LogId> {
//         if self.is_empty() {
//             return None;
//         }
//         Some(self.iter().last()?.id)
//     }

//     fn get_entry(&self, mapped_id: &LogIdEvent) -> Option<&LogIdEntry> {
//         self.get(&LogIdEntry::shallow_new(mapped_id.entry.hash))
//     }

//     fn take_entry(&mut self, mapped_id: &LogIdEvent) -> Option<LogIdEntry> {
//         self.take(&LogIdEntry::shallow_new(mapped_id.entry.hash))
//     }
// }

// /// Diagnostic struct offering information about the original input
// /// that may be used to create detailed diagnostics (e.g. for language server diagnostics).
// #[cfg(feature = "diagnostics")]
// #[derive(Debug, Clone, PartialEq, Eq)]
// pub struct Diagnostic {
//     /// Original input that caused this log-id entry
//     ///
//     /// **Note:** If `filepath` is set instead, the original input may be read directly from the file.
//     pub input: Option<String>,
//     /// Path to the file holding the original input that caused this log-id entry
//     pub filepath: Option<PathBuf>,
//     /// The range inside the original input that caused this log-id entry
//     pub range: Range,
//     /// Diagnostic tags that apply to this log-id entry
//     pub tags: Vec<DiagnosticTag>,
// }

// /// Specifies a position inside a text-based 2D-structure.
// #[cfg(feature = "diagnostics")]
// #[derive(Debug, Clone, PartialEq, Eq)]
// pub struct Position {
//     /// The line number of the position.
//     pub line: usize,
//     /// The column number of the position
//     pub column: usize,
// }

// /// Specifies a range inside a text-based 2D-structure.
// #[cfg(feature = "diagnostics")]
// #[derive(Debug, Clone, PartialEq, Eq)]
// pub struct Range {
//     /// The start position of the range.
//     pub start: Position,
//     /// The end position of the range.
//     pub end: Position,
// }

// /// Diagnostic tags inspired from the language server protocol.
// #[cfg(feature = "diagnostics")]
// #[derive(Debug, Clone, PartialEq, Eq)]
// pub enum DiagnosticTag {
//     /// Tag to mark unused or unnecessary input
//     Unnecessary = 1,
//     /// Tag to mark the usage of a certain input as deprecated.
//     Deprecated = 2,
// }

// /// This function computes the hash for a [`LogIdEntry`].
// ///
// /// The hash is computed over filename, line_nr, the current ThreadId,
// /// and the current UTC time in nanoseconds.
// ///
// /// # Arguments
// ///
// /// - `filename` - the filename to use when calculating the hash
// /// - `line_nr` - the line number to use when calculating the hash
// ///
// /// **Note:** The hash function is not cryptographically secure,
// /// but that is ok since we only need the hash to identify the entry in a map.
// fn compute_hash(filename: &str, line_nr: u32) -> u64 {
//     let mut hasher = DefaultHasher::new();
//     hasher.write(filename.as_bytes());
//     hasher.write_u32(line_nr);
//     std::thread::current().id().hash(&mut hasher);
//     hasher.write_i64(chrono::Utc::now().timestamp_nanos());
//     hasher.finish()
// }

// impl LogIdEntry {
//     /// Create a new [`LogIdEntry`].
//     pub(crate) fn new(id: LogId, msg: &str, filename: &str, line_nr: u32) -> Self {
//         LogIdEntry {
//             hash: compute_hash(filename, line_nr),
//             id,
//             level: id.get_level(),
//             msg: msg.to_string(),
//             origin: Origin::new(filename, line_nr),
//             span: if let Some(span) = tracing::span::Span::current().metadata() {
//                 span.name()
//             } else {
//                 "event not in span"
//             },
//             infos: Vec::default(),
//             debugs: Vec::default(),
//             traces: Vec::default(),

//             #[cfg(feature = "causes")]
//             causes: Vec::default(),

//             #[cfg(feature = "diagnostics")]
//             diagnostics: Vec::default(),
//         }
//     }
// }
