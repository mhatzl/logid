
/// Structure representing the origin of a log-id.
#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Origin {
    /// Filename where the log-id was set
    pub filename: String,
    /// Linenumber where the log-id was set
    pub line_nr: u32,
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
