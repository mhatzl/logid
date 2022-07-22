//! Contains the [`IdEntry`] definition used to capture messages for a log-id.

use crate::log_id::{EventLevel, LogId};

/// Structure representing the origin of a log-id.
pub struct Origin {
    /// Filename where the log-id was set
    filename: String,
    /// Linenumber where the log-id was set
    line: u32,
}

/// Structure to capture all messages set for a log-id.
pub struct LogIdEntry {
    /// The log-id
    id: LogId,
    /// The level of the log-id
    level: EventLevel,
    /// The main message set when creating the log-id
    msg: String,
    /// List of causes for this log-id
    causes: Option<Vec<String>>,
    /// List of additional informations for this log-id
    infos: Option<Vec<String>>,
    /// List of additional debug informations for this log-id
    debugs: Option<Vec<String>>,
    /// List of additional trace information for this log-id
    traces: Option<Vec<String>>,
    /// Code position where the log-id was created
    origin: Origin,
}
