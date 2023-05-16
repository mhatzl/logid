pub trait LogEventAddons {
    /// Add an info message to this log-id event
    fn add_info(self, msg: &str) -> Self;

    /// Add a debug message to this log-id event
    fn add_debug(self, msg: &str) -> Self;

    /// Add a trace message to this log-id event
    fn add_trace(self, msg: &str) -> Self;

    /// Add diagnostic info to this log-id event
    #[cfg(feature = "diagnostics")]
    fn add_diagnostic(self, diagnostic: lsp_types::Diagnostic) -> Self;

    /// Add a payload to this log-id event
    #[cfg(feature = "payloads")]
    fn add_payload(self, payload: serde_json::value::Value) -> Self;
}
