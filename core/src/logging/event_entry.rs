use evident::event::finalized::FinalizedEvent;

use crate::evident::event::origin::Origin;
use crate::log_id::{LogId, LogLevel};

use super::msg::LogMsg;

#[cfg(feature = "fmt")]
use super::msg::FmtMsg;

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
    /// List of additional formatted information for this log-id entry
    #[cfg(feature = "fmt")]
    pub(crate) fmt_infos: Vec<FmtMsg>,
    /// List of additional debug informations for this log-id entry
    pub(crate) debugs: Vec<String>,
    /// List of additional formatted debug information for this log-id entry
    #[cfg(feature = "fmt")]
    pub(crate) fmt_debugs: Vec<FmtMsg>,
    /// List of additional trace information for this log-id entry
    pub(crate) traces: Vec<String>,
    /// List of additional formatted trace information for this log-id entry
    #[cfg(feature = "fmt")]
    pub(crate) fmt_traces: Vec<FmtMsg>,
    /// List of related log-id event entries
    pub(crate) related: Vec<FinalizedEvent<LogId>>,
    /// Code position where the log-id entry was created
    pub(crate) origin: Origin,

    /// List of hints for this log-id entry
    #[cfg(feature = "hint_note")]
    pub(crate) hints: Vec<String>,
    /// List of formatted hints for this log-id entry
    #[cfg(all(feature = "hint_note", feature = "fmt"))]
    pub(crate) fmt_hints: Vec<FmtMsg>,
    /// List of notes for this log-id entry
    #[cfg(feature = "hint_note")]
    pub(crate) notes: Vec<String>,
    /// List of formatted notes for this log-id entry
    #[cfg(all(feature = "hint_note", feature = "fmt"))]
    pub(crate) fmt_notes: Vec<FmtMsg>,

    /// List of diagnostics for this log-id entry
    #[cfg(feature = "diagnostics")]
    pub(crate) diagnostics: Vec<crate::lsp_types::Diagnostic>,
    /// List of formatted diagnostics for this log-id entry
    #[cfg(all(feature = "diagnostics", feature = "fmt"))]
    pub(crate) fmt_diagnostics: Vec<FmtDiagnostics>,

    /// List of payloads for this log-id entry
    #[cfg(feature = "payloads")]
    pub(crate) payloads: Vec<crate::serde_json::value::Value>,
    /// List of formatted payloads for this log-id entry
    #[cfg(all(feature = "payloads", feature = "fmt"))]
    pub(crate) fmt_payloads: Vec<FmtMsg>,
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

            #[cfg(feature = "fmt")]
            fmt_infos: Vec::new(),
            #[cfg(feature = "fmt")]
            fmt_debugs: Vec::new(),
            #[cfg(feature = "fmt")]
            fmt_traces: Vec::new(),

            #[cfg(feature = "hint_note")]
            hints: Vec::new(),
            #[cfg(all(feature = "hint_note", feature = "fmt"))]
            fmt_hints: Vec::new(),
            #[cfg(feature = "hint_note")]
            notes: Vec::new(),
            #[cfg(all(feature = "hint_note", feature = "fmt"))]
            fmt_notes: Vec::new(),

            #[cfg(feature = "diagnostics")]
            diagnostics: Vec::new(),
            #[cfg(all(feature = "diagnostics", feature = "fmt"))]
            fmt_diagnostics: Vec::new(),

            #[cfg(feature = "payloads")]
            payloads: Vec::new(),
            #[cfg(all(feature = "payloads", feature = "fmt"))]
            fmt_payloads: Vec::new(),
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
    /// Get the list of additional formatted informations for this log-id entry
    #[cfg(feature = "fmt")]
    pub fn get_fmt_infos(&self) -> &Vec<FmtMsg> {
        &self.fmt_infos
    }
    /// Get the list of additional debug informations for this log-id entry
    pub fn get_debugs(&self) -> &Vec<String> {
        &self.debugs
    }
    /// Get the list of additional formatted debug informations for this log-id entry
    #[cfg(feature = "fmt")]
    pub fn get_fmt_debugs(&self) -> &Vec<FmtMsg> {
        &self.fmt_debugs
    }
    /// Get the list of additional trace information for this log-id entry
    pub fn get_traces(&self) -> &Vec<String> {
        &self.traces
    }
    /// Get the list of additional formatted trace informations for this log-id entry
    #[cfg(feature = "fmt")]
    pub fn get_fmt_traces(&self) -> &Vec<FmtMsg> {
        &self.fmt_traces
    }
    /// Get the list of related log-id event entries
    pub fn get_related(&self) -> &Vec<FinalizedEvent<LogId>> {
        &self.related
    }

    #[cfg(feature = "hint_note")]
    pub fn get_hints(&self) -> &Vec<String> {
        &self.hints
    }
    #[cfg(all(feature = "hint_note", feature = "fmt"))]
    pub fn get_fmt_hints(&self) -> &Vec<FmtMsg> {
        &self.fmt_hints
    }

    #[cfg(feature = "hint_note")]
    pub fn get_notes(&self) -> &Vec<String> {
        &self.notes
    }
    #[cfg(all(feature = "hint_note", feature = "fmt"))]
    pub fn get_fmt_notes(&self) -> &Vec<FmtMsg> {
        &self.fmt_notes
    }

    #[cfg(feature = "diagnostics")]
    pub fn get_diagnostics(&self) -> &Vec<crate::lsp_types::Diagnostic> {
        &self.diagnostics
    }
    #[cfg(all(feature = "diagnostics", feature = "fmt"))]
    pub fn get_fmt_diagnostics(&self) -> &Vec<FmtDiagnostics> {
        &self.fmt_diagnostics
    }

    #[cfg(feature = "payloads")]
    pub fn get_payloads(&self) -> &Vec<crate::serde_json::value::Value> {
        &self.payloads
    }
    #[cfg(all(feature = "payloads", feature = "fmt"))]
    pub fn get_fmt_payloads(&self) -> &Vec<FmtMsg> {
        &self.fmt_payloads
    }
}

/// [`AddonKind`] defines the information kind to be added to an [`EventEntry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddonKind {
    Info(String),
    Debug(String),
    Trace(String),
    Related(FinalizedEvent<LogId>),

    #[cfg(feature = "fmt")]
    FmtInfo(FmtMsg),
    #[cfg(feature = "fmt")]
    FmtDebug(FmtMsg),
    #[cfg(feature = "fmt")]
    FmtTrace(FmtMsg),

    #[cfg(feature = "hint_note")]
    Hint(String),
    #[cfg(all(feature = "hint_note", feature = "fmt"))]
    FmtHint(FmtMsg),
    #[cfg(feature = "hint_note")]
    Note(String),
    #[cfg(all(feature = "hint_note", feature = "fmt"))]
    FmtNote(FmtMsg),

    #[cfg(feature = "diagnostics")]
    Diagnostic(crate::lsp_types::Diagnostic),
    #[cfg(all(feature = "diagnostics", feature = "fmt"))]
    FmtDiagnostic(FmtDiagnostics),

    #[cfg(feature = "payloads")]
    Payload(crate::serde_json::value::Value),
    #[cfg(all(feature = "payloads", feature = "fmt"))]
    FmtPayload(FmtMsg),
}

#[cfg(all(feature = "diagnostics", feature = "fmt"))]
#[derive(Clone)]
pub struct FmtDiagnostics {
    func: for<'a> fn(&'a crate::lsp_types::Diagnostic) -> String,
    data: crate::lsp_types::Diagnostic,
}

#[cfg(all(feature = "diagnostics", feature = "fmt"))]
impl FmtDiagnostics {
    pub fn new(
        func: for<'a> fn(&'a crate::lsp_types::Diagnostic) -> String,
        data: crate::lsp_types::Diagnostic,
    ) -> Self {
        FmtDiagnostics { func, data }
    }

    pub fn get_data(&self) -> &crate::lsp_types::Diagnostic {
        &self.data
    }
}

#[cfg(all(feature = "diagnostics", feature = "fmt"))]
impl std::fmt::Debug for FmtDiagnostics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", (self.func)(&self.data))
    }
}

#[cfg(all(feature = "diagnostics", feature = "fmt"))]
impl std::fmt::Display for FmtDiagnostics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", (self.func)(&self.data))
    }
}

#[cfg(all(feature = "diagnostics", feature = "fmt"))]
impl PartialEq<FmtDiagnostics> for FmtDiagnostics {
    fn eq(&self, other: &FmtDiagnostics) -> bool {
        (self.func)(&self.data) == (other.func)(&other.data)
    }
}

#[cfg(all(feature = "diagnostics", feature = "fmt"))]
impl Eq for FmtDiagnostics {}

#[macro_export]
macro_rules! info_addon {
    ($msg:expr) => {
        $crate::logging::event_entry::AddonKind::Info($msg)
    };
    ($fmt_fn:expr, $fmt_data:expr) => {
        $crate::logging::event_entry::AddonKind::FmtInfo($crate::logging::msg::FmtMsg::new(
            $fmt_fn, $fmt_data,
        ))
    };
}

#[macro_export]
macro_rules! debug_addon {
    ($msg:expr) => {
        $crate::logging::event_entry::AddonKind::Debug($msg)
    };
    ($fmt_fn:expr, $fmt_data:expr) => {
        $crate::logging::event_entry::AddonKind::FmtDebug($crate::logging::msg::FmtMsg::new(
            $fmt_fn, $fmt_data,
        ))
    };
}

#[macro_export]
macro_rules! trace_addon {
    ($msg:expr) => {
        $crate::logging::event_entry::AddonKind::Trace($msg)
    };
    ($fmt_fn:expr, $fmt_data:expr) => {
        $crate::logging::event_entry::AddonKind::FmtTrace($crate::logging::msg::FmtMsg::new(
            $fmt_fn, $fmt_data,
        ))
    };
}

#[cfg(feature = "hint_note")]
#[macro_export]
macro_rules! hint_addon {
    ($msg:expr) => {
        $crate::logging::event_entry::AddonKind::Hint($msg)
    };
    ($fmt_fn:expr, $fmt_data:expr) => {
        $crate::logging::event_entry::AddonKind::FmtHint($crate::logging::msg::FmtMsg::new(
            $fmt_fn, $fmt_data,
        ))
    };
}

#[cfg(feature = "hint_note")]
#[macro_export]
macro_rules! note_addon {
    ($msg:expr) => {
        $crate::logging::event_entry::AddonKind::Note($msg)
    };
    ($fmt_fn:expr, $fmt_data:expr) => {
        $crate::logging::event_entry::AddonKind::FmtNote($crate::logging::msg::FmtMsg::new(
            $fmt_fn, $fmt_data,
        ))
    };
}

#[cfg(feature = "diagnostics")]
#[macro_export]
macro_rules! diagnostic_addon {
    ($msg:expr) => {
        $crate::logging::event_entry::AddonKind::Diagnostic($msg)
    };
    ($fmt_fn:expr, $fmt_data:expr) => {
        $crate::logging::event_entry::AddonKind::FmtDiagnostic(
            $crate::logging::event_entry::FmtDiagnostics::new($fmt_fn, $fmt_data),
        )
    };
}

#[cfg(feature = "payloads")]
#[macro_export]
macro_rules! payload_addon {
    ($msg:expr) => {
        $crate::logging::event_entry::AddonKind::Payload($msg)
    };
    ($fmt_fn:expr, $fmt_data:expr) => {
        $crate::logging::event_entry::AddonKind::FmtPayload($crate::logging::msg::FmtMsg::new(
            $fmt_fn, $fmt_data,
        ))
    };
}
