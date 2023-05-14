#[cfg(feature = "diagnostics")]
mod diagnostic_tests {
    use logid::{logging::LOGGER, set_event};
    use logid_derive::WarnLogId;
    use lsp_types::{Diagnostic, DiagnosticSeverity, Position, Range};

    #[derive(Debug, Default, WarnLogId)]
    enum TestWarnId {
        #[default]
        One,
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

        let recv = LOGGER.subscribe(TestWarnId::One).unwrap();

        set_event!(TestWarnId::One, msg)
            .add_diagnostic(diagnostics.clone())
            .finalize();

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
