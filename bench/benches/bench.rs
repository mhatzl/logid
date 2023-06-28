// use criterion::{criterion_group, criterion_main};
use criterion::{black_box, Criterion};
use logid::{
    err, event_handler::LogEventHandlerBuilder, log, log_id::LogLevel,
    logging::event_entry::AddonKind, DbgLogId, ErrLogId, InfoLogId, TraceLogId, WarnLogId,
};

// criterion_main!(benches);
// criterion_group!(benches, bench_compare_advanced_logging);
// criterion_group!(
//     name = benches;
//     config = Criterion::default()
//         .sample_size(10)
//         .warm_up_time(std::time::Duration::from_millis(1))
//         .measurement_time(std::time::Duration::from_millis(700));
//     targets = bench_full_logid
// );

pub fn bench_error_tracing(c: &mut Criterion) {
    tracing_subscriber::fmt::init();

    c.bench_function("tracing errors", |b| {
        b.iter(|| tracing::error!("Trace an error."))
    });
}

pub fn bench_error_logid(c: &mut Criterion) {
    let _log_handler = LogEventHandlerBuilder::new()
        .to_stderr()
        .all_log_events()
        .build();

    let err_id = logid::new_log_id!("ErrorId", LogLevel::Error);

    c.bench_function("logid errors", |b| {
        b.iter(|| log!(err_id, "Trace an error."))
    });
}

pub fn bench_full_logid(c: &mut Criterion) {
    let _ = logid::set_filter!("trace(all)");

    let _log_handler = LogEventHandlerBuilder::new()
        .to_stderr()
        .all_log_events()
        .build();

    c.bench_function("full logid", |b| b.iter(|| {
        log!(BenchError::Test, black_box("Logid error in full bench."), add: AddonKind::Info(black_box("Added info in full bench related to error trace.").to_string())
        , add: AddonKind::Debug(black_box("Added debug info in full bench related to error trace.").to_string())
        , add: AddonKind::Trace(black_box("Added trace info in full bench related to error trace.").to_string()));

        let warn_event = log!(BenchWarn::Test, black_box("Logid warn in full bench."));
        let info_event = log!(BenchInfo::Test, black_box("Logid info in full bench."), add: AddonKind::Related(warn_event));
        let dbg_event = log!(BenchDbg::Test, black_box("Logid debug in full bench."), add: AddonKind::Related(info_event));
        log!(BenchTrace::Test, black_box("Logid trace in full bench."), add: AddonKind::Related(dbg_event));

        let _: Result<(), BenchError> = err!(BenchError::Test);
    }));
}

pub fn bench_compare(c: &mut Criterion) {
    tracing_subscriber::fmt::init();

    let _log_handler = LogEventHandlerBuilder::new()
        .to_stderr()
        .all_log_events()
        .build();

    let err_id = logid::new_log_id!("ErrorId", LogLevel::Error);

    let mut bench_group = c.benchmark_group("compare bench");

    bench_group.bench_function("tracing", |b| {
        b.iter(|| tracing::error!("{}", black_box("Trace error in bench group.")))
    });

    bench_group.bench_function("logid", |b| {
        b.iter(|| log!(black_box(err_id), black_box("Trace error in bench group.")))
    });

    bench_group.finish();
}

pub fn bench_compare_advanced_logging(c: &mut Criterion) {
    tracing_subscriber::fmt::init();

    let _ = logid::set_filter!("debug(infos)");

    let _log_handler = LogEventHandlerBuilder::new()
        .to_stderr()
        .all_log_events()
        .build();

    let mut bench_group = c.benchmark_group("compare advanced bench");

    bench_group.bench_function("tracing", |b| b.iter(|| advanced_tracing()));

    bench_group.bench_function("logid", |b| b.iter(|| advanced_logid()));

    bench_group.finish();
}

fn advanced_logid() -> Result<(), BenchError> {
    log!(BenchError::Test, black_box("Logid error in advanced bench."), add: AddonKind::Info(black_box("Trace info in advanced bench related to error trace.").to_string()));

    log!(BenchWarn::Test, black_box("Logid warn in advanced bench."));
    log!(BenchInfo::Test, black_box("Logid info in advanced bench."));
    log!(BenchDbg::Test, black_box("Logid debug in advanced bench."));

    err!(BenchError::Test)
}

fn advanced_tracing() -> Result<(), BenchError> {
    tracing::error!("{}", black_box("Trace error in advanced bench."));
    tracing::info!(
        "{}",
        black_box("|--> Info: Trace info in advanced bench related to error trace.")
    );

    tracing::warn!("{}", black_box("Trace warn in advanced bench."));
    tracing::info!("{}", black_box("Trace info in advanced bench."));
    tracing::debug!("{}", black_box("Trace debug in advanced bench."));

    Err({
        tracing::error!("{}", black_box(BenchError::Test.to_string()));
        BenchError::Test
    })
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
