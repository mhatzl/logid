use std::{str::Lines, sync::Arc};

use colored::*;
use logid_core::{
    evident::event::{finalized::FinalizedEvent, Event},
    log_id::{LogId, LogLevel},
    logging::{event_entry::LogEventEntry, LOGGER},
};

pub(super) fn stderr_writer(log_event: Arc<Event<LogId, LogEventEntry>>) {
    console_writer(log_event, true);
}

pub(super) fn stdout_writer(log_event: Arc<Event<LogId, LogEventEntry>>) {
    console_writer(log_event, false);
}

fn console_writer(log_event: Arc<Event<LogId, LogEventEntry>>, to_stderr: bool) {
    let id = log_event.get_event_id();
    let level = id.get_log_level();
    let msg = log_event.get_msg();
    let mut content = format!(
        "{}: {}\n",
        colored_level(level),
        format_lines(
            msg.lines(),
            msg.len(),
            level.to_string().len() + 2, // +2 for ': '
            get_level_color(level)
        )
    );

    if let Some(filter) = LOGGER.get_filter() {
        let origin = log_event.get_origin();

        if filter.show_id(*id, origin) {
            content.push_str(&format!(
                "{} {}: id='{}::{}::{}', entry='{}'\n",
                colored_addon_start(level),
                "Event".bold(),
                id.get_crate_name(),
                id.get_module_path(),
                id.get_identifier(),
                log_event.get_entry_id(),
            ));
        }

        if filter.show_origin_info(*id, origin) {
            content.push_str(&format!(
                "{} {}: {}\n",
                colored_addon_start(level),
                "Origin".bold(),
                origin
            ));
        }
    }

    let entry = log_event.get_entry();

    // Note: Addon filter is already applied on capture side, so printing what is captured is fine here

    for related in entry.get_related() {
        content.push_str(&format!(
            "{} {}: {}\n",
            colored_addon_start(level),
            "Related".bold(),
            colored_related(related)
        ));
    }

    for info in entry.get_infos() {
        content.push_str(&format!(
            "{} {}: {}\n",
            colored_addon_start(level),
            "Info".bold().color(get_level_color(LogLevel::Info)),
            format_lines(
                info.lines(),
                info.len(),
                get_addon_indent("Info"),
                get_level_color(level)
            )
        ));
    }

    for debug in entry.get_debugs() {
        content.push_str(&format!(
            "{} {}: {}\n",
            colored_addon_start(level),
            "Debug".bold().color(get_level_color(LogLevel::Debug)),
            format_lines(
                debug.lines(),
                debug.len(),
                get_addon_indent("Debug"),
                get_level_color(level)
            )
        ));
    }

    for trace in entry.get_traces() {
        content.push_str(&format!(
            "{} {}: {}\n",
            colored_addon_start(level),
            "Trace".bold().color(get_level_color(LogLevel::Trace)),
            format_lines(
                trace.lines(),
                trace.len(),
                get_addon_indent("Trace"),
                get_level_color(level)
            )
        ));
    }

    #[cfg(feature = "hint_note")]
    for hint in entry.get_hints() {
        content.push_str(&format!(
            "{} {}: {}\n",
            colored_addon_start(level),
            "Hint".bold(),
            format_lines(
                hint.lines(),
                hint.len(),
                get_addon_indent("Hint"),
                get_level_color(level)
            )
        ));
    }

    #[cfg(feature = "hint_note")]
    for notes in entry.get_notes() {
        content.push_str(&format!(
            "{} {}: {}\n",
            colored_addon_start(level),
            "Note".bold(),
            format_lines(
                notes.lines(),
                notes.len(),
                get_addon_indent("Note"),
                get_level_color(level)
            )
        ));
    }

    #[cfg(feature = "diagnostics")]
    for diag in entry.get_diagnostics() {
        // TODO: make diag output prettier
        content.push_str(&format!(
            "{} {}: {}\n",
            colored_addon_start(level),
            "Diagnostics".bold(),
            format_lines(
                diag.message.lines(),
                diag.message.len(),
                get_addon_indent("Diagnostics"),
                get_level_color(level)
            )
        ));
    }

    #[cfg(feature = "payloads")]
    for payload in entry.get_payloads() {
        let s = payload.to_string();
        content.push_str(&format!(
            "{} {}: {}\n",
            colored_addon_start(level),
            "Payload".bold(),
            format_lines(
                s.lines(),
                s.len(),
                get_addon_indent("Payload"),
                get_level_color(level)
            )
        ));
    }

    if let Some((pre_last_lcross, post_last_lcross)) = content.rsplit_once('├') {
        content = format!("{}╰{}", pre_last_lcross, post_last_lcross);
    }

    if to_stderr {
        eprint!("{}", content);
    } else {
        print!("{}", content);
    };
}

fn colored_level(level: LogLevel) -> String {
    level
        .to_string()
        .bold()
        .color(get_level_color(level))
        .to_string()
}

fn get_level_color(level: LogLevel) -> colored::Color {
    match level {
        LogLevel::Error => Color::Red,
        LogLevel::Warn => Color::Yellow,
        LogLevel::Info => Color::Green,
        LogLevel::Debug => Color::Blue,
        LogLevel::Trace => Color::Cyan,
    }
}

fn format_lines(mut lines: Lines, capacity: usize, indent: usize, color: Color) -> String {
    let mut s = String::with_capacity(capacity);
    if let Some(first_line) = lines.next() {
        s.push_str(first_line);
    }

    for line in lines {
        s.push('\n');
        s.push_str("│".color(color).to_string().as_str());
        s.push_str(&" ".repeat(indent.saturating_sub(1))); // -1 for '│'
        s.push_str(line);
    }

    s
}

fn get_addon_indent(kind: &str) -> usize {
    // Note: Using '|-->' instead of Unicode arrow-combi, since len() is Utf8, and one arrow-combi char != one Utf8 code point.
    format!("|--> {}: ", kind).len()
}

fn colored_addon_start(level: LogLevel) -> String {
    "├──>".color(get_level_color(level)).to_string()
}

fn colored_related(related: &FinalizedEvent<LogId>) -> String {
    let id = related.event_id;
    format!(
        "id='{}: {}::{}::{}', entry='{}'",
        colored_level(id.get_log_level()),
        id.get_crate_name(),
        id.get_module_path(),
        id.get_identifier(),
        related.entry_id
    )
}
