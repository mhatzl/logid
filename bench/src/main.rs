use logid::{
    err,
    event_handler::LogEventHandlerBuilder,
    log,
    logging::{event_entry::AddonKind, LOGGER},
    DbgLogId, ErrLogId, InfoLogId, TraceLogId, WarnLogId,
};

fn main() {
    let _ = logid::set_filter!("trace(all)");

    let handler = LogEventHandlerBuilder::new()
        .to_stderr()
        .all_log_events()
        .build();

    let handler2 = LogEventHandlerBuilder::new()
        // .to_stderr()
        .all_log_events()
        .build();

    let start_time = std::time::Instant::now();

    let _ = bench_full_logid();

    log!(BenchError::Test, "Logid error 1 in full bench\nspanning two lines", add: AddonKind::Info("Added info in full bench\nrelated to error trace.".to_string())
    , add: AddonKind::Debug("Added debug info in full bench\nrelated to error trace.".to_string())
    , add: AddonKind::Trace("Added trace info in full bench related to error trace.".to_string()));

    let end_time = std::time::Instant::now();

    handler.stop();

    log!(
        BenchWarn::Test,
        "Event not logged by handler, but captured."
    );

    handler.start();

    log!(BenchWarn::Test, "Event logged again.");

    handler2.stop();

    log!(
        BenchError::Test,
        "Event logged => handler2.stop() does not affect handler."
    );

    LOGGER.stop_capturing();

    log!(
        BenchWarn::Test,
        "Global logging stopped => Event not logged."
    );

    LOGGER.start_capturing();

    log!(BenchWarn::Test, "Event logged again globally.");

    println!(
        "Duration: {}us\n-----------------------------\n",
        end_time
            .checked_duration_since(start_time)
            .unwrap()
            .as_micros()
    );
}

fn bench_full_logid() -> Result<(), BenchError> {
    log!(BenchError::Test, "Logid error 2 in full bench", add: AddonKind::Info("Added info in full bench related to error trace.".to_string())
    , add: AddonKind::Debug("Added debug info in full bench related to error trace.".to_string())
    , add: AddonKind::Trace("Added trace info in full bench related to error trace.".to_string()));

    let warn_event = log!(BenchWarn::Test, "Logid warn in full bench.");
    let info_event = log!(
        BenchInfo::Test,
        "Logid info in full bench.",
        add: AddonKind::Related(warn_event)
    );
    let dbg_event = log!(
        BenchDbg::Test,
        "Logid debug in full bench.",
        add: AddonKind::Related(info_event)
    );
    log!(
        BenchTrace::Test,
        "Logid trace in full bench.",
        add: AddonKind::Related(dbg_event)
    );

    err!(BenchError::Test)
}

#[derive(Debug, Clone, thiserror::Error, ErrLogId)]
enum BenchError {
    #[error("Some benchmark test error")]
    Test,
}

#[derive(Debug, Clone, WarnLogId)]
enum BenchWarn {
    Test,
}

#[derive(Debug, Clone, InfoLogId)]
enum BenchInfo {
    Test,
}

#[derive(Debug, Clone, DbgLogId)]
enum BenchDbg {
    Test,
}

#[derive(Debug, Clone, TraceLogId)]
enum BenchTrace {
    Test,
}
