use logid::{
    err,
    event_handler::builder::LogEventHandlerBuilder,
    log,
    log_id::LogLevel,
    logging::{event_entry::AddonKind, filter::AddonFilter, LOGGER},
    payload_addon, DbgLogId, ErrLogId, InfoLogId, TraceLogId, WarnLogId,
};
use serde::{Deserialize, Serialize};

fn main() {
    let _ = logid::logging::filter::set_filter((LogLevel::Trace, AddonFilter::AllAllowed));
    let _ = logid::logging::filter::set_filter(logid::filter!(Trace(AllAllowed)));

    let handler = LogEventHandlerBuilder::new()
        .to_stderr()
        .all_log_events()
        .build()
        .unwrap();

    let handler2 = LogEventHandlerBuilder::new()
        // .to_stderr()
        .all_log_events()
        .build()
        .unwrap();

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

    let payload = DummyPayload {
        vals: vec!["First", "Second", "Third", "Fourth"]
            .iter_mut()
            .map(|f| f.to_string())
            .collect(),
    };

    log!(
        BenchInfo::Test,
        "Event logged again.",
        add: payload_addon!(fmt_payload, logid::serde_json::to_value(payload).unwrap())
    );

    handler2.stop();

    log!(
        BenchError::Test,
        "Event logged => handler2.stop() does not affect handler."
    );

    LOGGER.stop();

    log!(
        BenchError::Test,
        "Global logging stopped => Event not logged."
    );

    LOGGER.start();

    log!(BenchInfo::Test, "Event logged again globally.");

    // Use Display impl for "BenchWarn::Test"
    log!(BenchWarn::Test);

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
        add: AddonKind::Related(warn_event),
        add: AddonKind::Hint("Some hint".to_string()),
        add: AddonKind::Note("Some note".to_string())
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

impl std::fmt::Display for BenchWarn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BenchWarn Display impl.")
    }
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

#[derive(Serialize, Deserialize)]
struct DummyPayload {
    vals: Vec<String>,
}

fn fmt_payload(data: &logid::serde_json::Value) -> String {
    match logid::serde_json::from_value::<DummyPayload>(data.clone()) {
        Ok(payload) => {
            let mut s = String::new();
            for val in payload.vals {
                s.push_str(&val);
                s.push('\n');
            }
            s
        }
        Err(_) => "Could not deserialize back to payload!".to_string(),
    }
}
