use criterion::{black_box, criterion_group, criterion_main, Criterion};
use logid::{event_handler::LogEventHandlerBuilder, log, log_id::LogLevel};

pub fn bench_error_tracing(c: &mut Criterion) {
    tracing_subscriber::fmt::init();

    c.bench_function("tracing errors", |b| {
        b.iter(|| tracing::error!("Trace an error."))
    });
}

pub fn bench_error_logid(c: &mut Criterion) {
    let log_handler = LogEventHandlerBuilder::new()
        .to_stderr()
        .all_log_events()
        .build();

    let err_id = logid::new_log_id!("ErrorId", LogLevel::Error);

    c.bench_function("logid errors", |b| {
        b.iter(|| log!(err_id, "Trace an error."))
    });

    log_handler.shutdown();
}

pub fn bench_compare(c: &mut Criterion) {
    tracing_subscriber::fmt::init();

    let log_handler = LogEventHandlerBuilder::new()
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

    log_handler.shutdown();
}

criterion_group!(benches, bench_compare);
// criterion_group!(benches, bench_error_tracing, bench_error_logid);

criterion_main!(benches);
