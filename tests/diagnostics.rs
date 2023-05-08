//! Test functionality enabled by the `diagnostics` feature.

#[cfg(feature = "diagnostics")]
use logid::{
    drain_map,
    id_entry::{Diagnostic, DiagnosticTag, LogIdEntry, Position, Range},
    log_id::{get_log_id, EventLevel},
    set_event,
};

#[cfg(feature = "diagnostics")]
#[test]
fn capture_single_logid_with_diagnostics() {
    drain_map!();

    let log_id = get_log_id(1, 1, EventLevel::Debug, 0);
    let msg = "Set first log message";

    let diagnostics = Diagnostic {
        input: Some("Some input text that caused this log-id entry".to_string()),
        filepath: None,
        range: Range {
            start: Position { line: 0, column: 4 },
            end: Position {
                line: 0,
                column: 10,
            },
        },
        tags: [DiagnosticTag::Deprecated].into(),
    };

    let event = set_event!(log_id, msg).add_diagnostic(diagnostics.clone());
    event.finalize();

    let map = drain_map!().unwrap();

    let entries = map.get(&log_id).unwrap();
    let entries = entries.iter().collect::<Vec<&LogIdEntry>>();
    let entry = entries.last().unwrap();
    let act_diagnostics = entry.diagnostics.last().unwrap();
    assert_eq!(
        act_diagnostics, &diagnostics,
        "Set and stored diagnostics are not equal"
    );
}
