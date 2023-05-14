use evident::event::origin::Origin;

use crate::log_id::{LogId, LogLevel};

#[derive(Default, Clone)]
pub struct LogEventEntry {
    /// The event id of this entry
    pub(crate) event_id: LogId,
    /// The unique id of this entry
    pub(crate) entry_id: uuid::Uuid,
    /// The main message set when creating the log-id entry
    pub(crate) msg: String,
    /// List of additional warnings for this log-id entry
    pub(crate) warnings: Vec<String>,
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

    /// List of EventEntry ids that caused this log-id entry
    #[cfg(feature = "causes")]
    pub(crate) causes: Vec<uuid::Uuid>,

    /// List of diagnostics for this log-id entry
    #[cfg(feature = "diagnostics")]
    pub(crate) diagnostics: Vec<lsp_types::Diagnostic>,

    /// List of payloads for this log-id entry
    #[cfg(feature = "payloads")]
    pub(crate) payloads: Vec<serde_json::value::Value>,
}

impl evident::event::entry::EventEntry<LogId> for LogEventEntry {
    fn new(
        event_id: LogId,
        msg: &str,
        crate_name: &'static str,
        module_path: &'static str,
        filename: &'static str,
        line_nr: u32,
    ) -> Self {
        LogEventEntry {
            event_id,
            entry_id: uuid::Uuid::new_v4(),
            msg: msg.to_string(),
            warnings: Vec::new(),
            infos: Vec::new(),
            debugs: Vec::new(),
            traces: Vec::new(),
            origin: evident::event::origin::Origin::new(crate_name, module_path, filename, line_nr),

            #[cfg(feature = "spans")]
            span: if tracing::span::Span::current().is_disabled() {
                None
            } else {
                Some(tracing::span::Span::current())
            },

            #[cfg(feature = "causes")]
            causes: Vec::new(),

            #[cfg(feature = "diagnostics")]
            diagnostics: Vec::new(),

            #[cfg(feature = "payloads")]
            payloads: Vec::new(),
        }
    }

    fn get_event_id(&self) -> &LogId {
        &self.event_id
    }

    fn get_entry_id(&self) -> uuid::Uuid {
        self.entry_id
    }

    fn get_msg(&self) -> &str {
        &self.msg
    }

    fn get_crate_name(&self) -> &'static str {
        self.origin.crate_name
    }

    fn get_origin(&self) -> &evident::event::origin::Origin {
        &self.origin
    }
}

impl LogEventEntry {
    /// Get the level of the log-id of this entry
    pub fn get_level(&self) -> LogLevel {
        self.event_id.log_level
    }
    /// Get the main message set when creating the log-id entry
    pub fn get_msg(&self) -> &String {
        &self.msg
    }
    /// Get the list of additional warnings for this log-id entry
    pub fn get_warnings(&self) -> &Vec<String> {
        &self.warnings
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
    pub fn get_causes(&self) -> &Vec<uuid::Uuid> {
        &self.causes
    }

    #[cfg(feature = "diagnostics")]
    pub fn get_diagnostics(&self) -> &Vec<lsp_types::Diagnostic> {
        &self.diagnostics
    }

    #[cfg(feature = "payloads")]
    pub fn get_payloads(&self) -> &Vec<serde_json::value::Value> {
        &self.payloads
    }
}

/// [`EntryKind`] defines the information kind to be added to an [`EventEntry`].
pub(crate) enum EntryKind {
    Warning(String),
    Info(String),
    Debug(String),
    Trace(String),

    #[cfg(feature = "causes")]
    Cause(uuid::Uuid),

    #[cfg(feature = "diagnostics")]
    Diagnostic(lsp_types::Diagnostic),

    #[cfg(feature = "payloads")]
    Payload(serde_json::value::Value),
}
