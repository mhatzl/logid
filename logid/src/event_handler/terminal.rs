use std::{
    io::{BufWriter, Write},
    str::Lines,
    sync::Arc,
};

use colored::*;
use logid_core::{
    evident::event::Event,
    log_id::{LogId, LogLevel},
    logging::{event_entry::LogEventEntry, LOGGER},
};

pub(super) fn stderr_writer(log_event: Arc<Event<LogId, LogEventEntry>>) {
    terminal_writer(log_event, true);
}

pub(super) fn stdout_writer(log_event: Arc<Event<LogId, LogEventEntry>>) {
    terminal_writer(log_event, false);
}

fn terminal_writer(log_event: Arc<Event<LogId, LogEventEntry>>, to_stderr: bool) {
    let id = log_event.get_event_id();
    let level = id.get_log_level();

    let colored_vbar = get_colored_vbar(level);
    let colored_lcross = get_colored_lcross(level);
    let colored_arrow = get_colored_arrow(level);
    let colored_lbot = get_colored_lbot(level);
    let colored_mbot = get_colored_mbot(level);

    let mut content_builder = ContentBuilder::new();
    content_builder.add_header(level, log_event.get_msg(), &colored_vbar);

    if let Some(filter) = LOGGER.get_filter() {
        let origin = log_event.get_origin();

        if filter.show_id(*id, origin) {
            let event_line = format!(
                "{}{} {}: {}",
                colored_lcross,
                colored_arrow,
                "Event".bold(),
                get_event_string(id, &log_event.get_entry_id().to_string())
            );
            content_builder.add_line(event_line);
        }

        if filter.show_origin_info(*id, origin) {
            let origin_line = format!(
                "{}{} {}: {}",
                colored_lcross,
                colored_arrow,
                "Origin".bold(),
                origin
            );
            content_builder.add_line(origin_line);
        }
    }

    let entry = log_event.get_entry();

    // Note: Addon filter is already applied on capture side, so printing what is captured is fine here

    for related in entry.get_related() {
        let related_id = related.get_event_id();
        let related_line = format!(
            "{}{} {}: lvl='{}', {}",
            colored_lcross,
            colored_arrow,
            "Related".bold(),
            get_colored_level(related_id.get_log_level()),
            get_event_string(related_id, &related.get_entry_id().to_string()),
        );
        content_builder.add_line(related_line);
    }

    for info in entry.get_infos() {
        content_builder.add_multiline_addon(
            "Info",
            info.lines(),
            Some(get_level_color(LogLevel::Info)),
            &colored_lcross,
            &colored_arrow,
            &colored_vbar,
        );
    }

    for debug in entry.get_debugs() {
        content_builder.add_multiline_addon(
            "Debug",
            debug.lines(),
            Some(get_level_color(LogLevel::Debug)),
            &colored_lcross,
            &colored_arrow,
            &colored_vbar,
        );
    }

    for trace in entry.get_traces() {
        content_builder.add_multiline_addon(
            "Trace",
            trace.lines(),
            Some(get_level_color(LogLevel::Trace)),
            &colored_lcross,
            &colored_arrow,
            &colored_vbar,
        );
    }

    #[cfg(feature = "hint_note")]
    for hint in entry.get_hints() {
        content_builder.add_multiline_addon(
            "Hint",
            hint.lines(),
            Some(Color::Cyan),
            &colored_lcross,
            &colored_arrow,
            &colored_vbar,
        );
    }

    #[cfg(feature = "hint_note")]
    for note in entry.get_notes() {
        content_builder.add_multiline_addon(
            "Note",
            note.lines(),
            Some(Color::Cyan),
            &colored_lcross,
            &colored_arrow,
            &colored_vbar,
        );
    }

    #[cfg(feature = "diagnostics")]
    for diag in entry.get_diagnostics() {
        // TODO: make diag output prettier
        content_builder.add_multiline_addon(
            "Diagnostics",
            diag.message.lines(),
            None,
            &colored_lcross,
            &colored_arrow,
            &colored_vbar,
        );
    }

    #[cfg(feature = "payloads")]
    for payload in entry.get_payloads() {
        content_builder.add_multiline_addon(
            "Payload",
            payload.to_string().lines(),
            None,
            &colored_lcross,
            &colored_arrow,
            &colored_vbar,
        );
    }

    if content_builder.lines.len() > 1 {
        let last_line = content_builder
            .lines
            .remove(content_builder.lines.len() - 1);
        content_builder.lines.push(
            last_line
                .replacen(&colored_lcross, &colored_lbot, 1)
                .replacen(&colored_vbar, &colored_mbot, 1),
        );
    }

    let content_len = content_builder.content_len + content_builder.lines.len(); // + line-len for newline char
    if to_stderr {
        content_builder.write(BufWriter::with_capacity(
            content_len,
            std::io::stderr().lock(),
        ));
    } else {
        content_builder.write(BufWriter::with_capacity(
            content_len,
            std::io::stdout().lock(),
        ));
    };
}

const HEADER_PREFIX_LEN: usize = 6;

/// Returns number of spaces to align printed levels.
const fn get_level_space_alignment(level: LogLevel) -> usize {
    match level {
        LogLevel::Error => HEADER_PREFIX_LEN - "ERR".len(),
        LogLevel::Warn => HEADER_PREFIX_LEN - "WARN".len(),
        LogLevel::Info => HEADER_PREFIX_LEN - "INFO".len(),
        LogLevel::Debug => HEADER_PREFIX_LEN - "DEBUG".len(),
        LogLevel::Trace => HEADER_PREFIX_LEN - "TRACE".len(),
    }
}

fn get_colored_level(level: LogLevel) -> String {
    level
        .to_string()
        .bold()
        .color(get_level_color(level))
        .to_string()
}

const fn get_level_color(level: LogLevel) -> colored::Color {
    match level {
        LogLevel::Error => Color::Red,
        LogLevel::Warn => Color::Yellow,
        LogLevel::Info => Color::Green,
        LogLevel::Debug => Color::Blue,
        LogLevel::Trace => Color::Cyan,
    }
}

const fn get_addon_prefix_len(kind: &str) -> usize {
    // Note: Using '|--->' instead of Unicode arrow-combi, since len() is Utf8, and one arrow-combi char != one Utf8 code point.
    "|---> : ".len() + kind.len()
}

fn get_colored_arrow(level: LogLevel) -> String {
    "───>".color(get_level_color(level)).to_string()
}

fn get_colored_lcross(level: LogLevel) -> String {
    "├".color(get_level_color(level)).to_string()
}

fn get_colored_vbar(level: LogLevel) -> String {
    "│".color(get_level_color(level)).to_string()
}

fn get_colored_mbot(level: LogLevel) -> String {
    "┴".color(get_level_color(level)).to_string()
}

fn get_colored_lbot(level: LogLevel) -> String {
    "╰".color(get_level_color(level)).to_string()
}

fn get_event_string(id: &LogId, entry_nr: &str) -> String {
    let module = id.get_module_path();
    let identifier = id.get_identifier();
    format!("id='{module}::{identifier}', entry='{entry_nr}'")
}

struct ContentBuilder {
    lines: Vec<String>,
    content_len: usize,
}

impl ContentBuilder {
    fn new() -> Self {
        ContentBuilder {
            lines: Vec::new(),
            content_len: 0,
        }
    }

    fn write<W: Write>(self, mut writer: BufWriter<W>) {
        for line in self.lines {
            let _ = writeln!(writer, "{}", line);
        }
        let _ = writer.flush();
    }

    fn add_header(&mut self, level: LogLevel, msg: &str, colored_bar: &str) {
        let colored_level = get_colored_level(level);
        let space_offset = " ".repeat(get_level_space_alignment(level));
        let prefix = format!("{}{}", colored_level, space_offset);

        self.add_lines(prefix, HEADER_PREFIX_LEN, msg.lines(), colored_bar);
    }

    fn add_multiline_addon(
        &mut self,
        kind: &str,
        content: Lines,
        addon_color: Option<Color>,
        colored_lcross: &str,
        colored_arrow: &str,
        colored_vbar: &str,
    ) {
        let fmt_kind = if let Some(color) = addon_color {
            kind.bold().color(color)
        } else {
            kind.bold()
        };

        let prefix = format!("{}{} {}: ", colored_lcross, colored_arrow, fmt_kind);
        self.add_lines(prefix, get_addon_prefix_len(kind), content, colored_vbar);
    }

    fn add_line(&mut self, line: String) {
        self.content_len += line.len();
        self.lines.push(line);
    }

    /// Adds multiple lines, with the given prefix set before the content of the first line.
    ///
    /// Note: prefix_len needed, because colored_first_line_prefix with colors most likely validates "grapheme_count = length".
    fn add_lines(
        &mut self,
        colored_first_line_prefix: String,
        prefix_len: usize,
        mut lines: Lines,
        colored_bar: &str,
    ) {
        let indent = " ".repeat(prefix_len.saturating_sub(1)); // -1 for "colored_bar"

        if let Some(first_line) = lines.next() {
            let line = format!("{colored_first_line_prefix}{first_line}");
            self.content_len += line.len();
            self.lines.push(line);
        }

        for line in lines {
            let line = format!("{colored_bar}{indent}{line}");
            self.content_len += line.len();
            self.lines.push(line);
        }
    }
}
