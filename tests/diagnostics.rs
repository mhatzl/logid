//! Test functionality enabled by the `diagnostics` feature.

#[cfg(feature = "diagnostics")]
use logid::{
    capturing::LogIdTracing,
    id_entry::{Diagnostic, DiagnosticTag, Position, Range},
    id_map::LogIdMap,
    log_id::{get_log_id, EventLevel},
};

#[cfg(feature = "diagnostics")]
#[test]
fn capture_single_logid_with_diagnostics() {
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

    let log_map = LogIdMap::new();
    log_id
        .set_event_with(&log_map, msg, file!(), line!())
        .add_diagnostic(diagnostics.clone())
        .finalize();

    let map = log_map.drain_map().unwrap();

    let entries = map.get(&log_id).unwrap();
    let entry = entries.last().unwrap();
    let act_diagnostics = entry.diagnostics.last().unwrap();
    assert_eq!(
        act_diagnostics, &diagnostics,
        "Set and stored diagnostics are not equal"
    );
}
