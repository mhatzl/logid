#[cfg(feature = "diagnostics")]
mod diagnostic_tests {
    use logid::{
        log,
        logging::{event_entry::EntryKind, LOGGER},
        lsp_types::{Diagnostic, DiagnosticSeverity, Position, Range},
    };
    use logid_derive::WarnLogId;

    #[derive(Debug, Default, WarnLogId, Clone)]
    enum TestWarnId {
        One,
        #[default]
        Two,
    }

    #[test]
    fn capture_single_logid_with_diagnostics() {
        let msg = "Set first log message";
        let diagnostics = Diagnostic {
            range: Range {
                start: Position {
                    line: 0,
                    character: 4,
                },
                end: Position {
                    line: 0,
                    character: 10,
                },
            },
            severity: Some(DiagnosticSeverity::INFORMATION),
            message: "Some diagnostic information useful for lsp implementations.".to_owned(),
            ..Default::default()
        };

        let recv = LOGGER.subscribe(TestWarnId::One.into()).unwrap();

        log!(
            TestWarnId::One,
            msg,
            addon: EntryKind::Diagnostic(diagnostics.clone())
        );

        let event = recv
            .get_receiver()
            .recv_timeout(std::time::Duration::from_millis(10))
            .unwrap();

        let entry = event.get_entry();
        let act_diagnostics = entry.get_diagnostics().last().unwrap();
        assert_eq!(
            act_diagnostics, &diagnostics,
            "Set and stored diagnostics are not equal"
        );
    }
}
