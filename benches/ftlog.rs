use criterion::{criterion_group, criterion_main, Criterion};
use ftlog::appender::FileAppender;
use log::info;
use serde::{Deserialize, Serialize};
use flashlog::get_unix_nano;
use ftlog::{
    appender::file::Period,
    LoggerGuard,
    LevelFilter,
};
use time::Duration;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LogStruct {
    pub data: [u64; 5],
}

impl Default for LogStruct {
    fn default() -> Self {
        LogStruct { data: [1, 2, 3, 4, 5] }
    }
}

fn init() -> LoggerGuard {
    // Rotate every day, clean stale logs that were modified 7 days ago on each rotation
    let writer = FileAppender::builder()
        .path("./current.log")
        .rotate(Period::Minute)
        .expire(Duration::minutes(4))
        .build();
    ftlog::Builder::new()
        // global max log level
        .max_log_level(LevelFilter::Info)
        // define root appender, pass None would write to stderr
        .root(writer)
        .unbounded()
        // write logs in ftlog::appender to "./ftlog-appender.log" instead of "./current.log"
        .filter("ftlog::appender", "ftlog-appender", LevelFilter::Error)
        .appender("ftlog-appender", FileAppender::new("ftlog-appender.log"))
        .try_init()
        .expect("logger build or set failed")
}

fn bench_ftlog(c: &mut Criterion) {
    let mut group = c.benchmark_group("ftlog");
    group.nresamples(5);
    let _guard = init();

    let test_struct = LogStruct::default();
    
    info!("warm up");
    info!("warm up");
    info!("warm up");

    let iteration = 1_000_000;

    group.bench_function(format!("ftlog - log {} times", iteration).as_str(), |b| b.iter(|| {
        for _ in 0..iteration {
            info!("Log message struct: {:?}", test_struct.clone());
        }
    }));

    group.finish();
}

criterion_group!(benches, bench_ftlog);
criterion_main!(benches);