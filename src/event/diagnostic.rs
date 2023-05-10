use std::path::PathBuf;

/// Diagnostic struct offering information about the original input
/// that may be used to create detailed diagnostics (e.g. for language server diagnostics).
#[cfg(feature = "diagnostics")]
#[derive(Debug, Clone, PartialEq, Eq)]
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Position {
    /// The line number of the position.
    pub line: usize,
    /// The column number of the position
    pub column: usize,
}

/// Specifies a range inside a text-based 2D-structure.
#[cfg(feature = "diagnostics")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Range {
    /// The start position of the range.
    pub start: Position,
    /// The end position of the range.
    pub end: Position,
}

/// Diagnostic tags inspired from the language server protocol.
#[cfg(feature = "diagnostics")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiagnosticTag {
    /// Tag to mark unused or unnecessary input
    Unnecessary = 1,
    /// Tag to mark the usage of a certain input as deprecated.
    Deprecated = 2,
}
