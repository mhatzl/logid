use evident::event::finalized::FinalizedEvent;

use crate::evident::event::origin::Origin;
use crate::log_id::{LogId, LogLevel};

use super::msg::LogMsg;

#[derive(Default, Debug, Clone)]
pub struct LogEventEntry {
    /// The event id of this entry
    pub(crate) event_id: LogId,
    /// The unique id of this entry
    pub(crate) entry_id: crate::evident::uuid::Uuid,
    /// The main message set when creating the log-id entry
    pub(crate) msg: Option<LogMsg>,
    /// List of additional informations for this log-id entry
    pub(crate) infos: Vec<String>,
    /// List of additional debug informations for this log-id entry
    pub(crate) debugs: Vec<String>,
    /// List of additional trace information for this log-id entry
    pub(crate) traces: Vec<String>,
    /// List of related log-id event entries
    pub(crate) related: Vec<FinalizedEvent<LogId>>,
    /// Code position where the log-id entry was created
    pub(crate) origin: Origin,

    /// List of hints for this log-id entry
    #[cfg(feature = "hint_note")]
    pub(crate) hints: Vec<String>,
    /// List of notes for this log-id entry
    #[cfg(feature = "hint_note")]
    pub(crate) notes: Vec<String>,

    /// List of diagnostics for this log-id entry
    #[cfg(feature = "diagnostics")]
    pub(crate) diagnostics: Vec<crate::lsp_types::Diagnostic>,

    /// List of payloads for this log-id entry
    #[cfg(feature = "payloads")]
    pub(crate) payloads: Vec<serde_json::value::Value>,
}

impl crate::evident::event::entry::EventEntry<LogId, LogMsg> for LogEventEntry {
    fn new(event_id: LogId, msg: Option<impl Into<LogMsg>>, origin: Origin) -> Self {
        LogEventEntry {
            event_id,
            entry_id: crate::evident::uuid::Uuid::new_v4(),
            msg: msg.map(|m| m.into()),
            infos: Vec::new(),
            debugs: Vec::new(),
            traces: Vec::new(),
            related: Vec::new(),
            origin,

            #[cfg(feature = "hint_note")]
            hints: Vec::new(),
            #[cfg(feature = "hint_note")]
            notes: Vec::new(),

            #[cfg(feature = "diagnostics")]
            diagnostics: Vec::new(),

            #[cfg(feature = "payloads")]
            payloads: Vec::new(),
        }
    }

    fn get_event_id(&self) -> &LogId {
        &self.event_id
    }

    fn into_event_id(self) -> LogId {
        self.event_id
    }

    fn get_entry_id(&self) -> crate::evident::uuid::Uuid {
        self.entry_id
    }

    fn get_msg(&self) -> Option<&LogMsg> {
        self.msg.as_ref()
    }

    fn get_origin(&self) -> &crate::evident::event::origin::Origin {
        &self.origin
    }
}

impl LogEventEntry {
    /// Get the level of the log-id of this entry
    pub fn get_level(&self) -> LogLevel {
        self.event_id.log_level
    }

    /// Get the code position where the log-id entry was created
    pub fn get_origin(&self) -> &Origin {
        &self.origin
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
    /// Get the list of related log-id event entries
    pub fn get_related(&self) -> &Vec<FinalizedEvent<LogId>> {
        &self.related
    }

    #[cfg(feature = "hint_note")]
    pub fn get_hints(&self) -> &Vec<String> {
        &self.hints
    }

    #[cfg(feature = "hint_note")]
    pub fn get_notes(&self) -> &Vec<String> {
        &self.notes
    }

    #[cfg(feature = "diagnostics")]
    pub fn get_diagnostics(&self) -> &Vec<crate::lsp_types::Diagnostic> {
        &self.diagnostics
    }

    #[cfg(feature = "payloads")]
    pub fn get_payloads(&self) -> &Vec<serde_json::value::Value> {
        &self.payloads
    }
}

/// [`AddonKind`] defines the information kind to be added to an [`EventEntry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddonKind {
    Info(String),
    Debug(String),
    Trace(String),
    Related(FinalizedEvent<LogId>),

    #[cfg(feature = "hint_note")]
    Hint(String),
    #[cfg(feature = "hint_note")]
    Note(String),

    #[cfg(feature = "diagnostics")]
    Diagnostic(crate::lsp_types::Diagnostic),

    #[cfg(feature = "payloads")]
    Payload(serde_json::value::Value),
}
