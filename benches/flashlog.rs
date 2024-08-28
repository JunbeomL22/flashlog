use std::iter;

use criterion::{criterion_group, criterion_main, Criterion};

use flashlog::{
    LogLevel, Logger, TimeZone,
    get_unix_nano,
    log_info,
    info,
    flushing_log_info as flushing,
};

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct LogStruct {
    pub data: [u64; 5],
}

impl Default for LogStruct {
    fn default() -> Self {
        LogStruct { data: [1, 2, 3, 4, 5] }
    }
}

fn bench_flashlog_40bytes(c: &mut Criterion) {
    let mut group = c.benchmark_group("flashlog");
    let _logger = Logger::initialize()
        .with_file("logs", "message")
        .unwrap()
        .with_console_report(false)
        .with_msg_buffer_size(3_000_000)
        .with_msg_flush_interval(3_000_000)
        .with_max_log_level(LogLevel::Info)
        .with_timezone(TimeZone::Local)
        .launch();

    info!("Warm up");
    info!("Warm up");
    info!("Warm up");

    let iteration = 200_000;
    //let test_number = 10;

    let log_struct = LogStruct::default();
    //let mut res = vec![];

    group.bench_function(format!("flashlog - log 40bytes struct {} times", iteration).as_str(), |b| b.iter(|| {
        for _ in 0..iteration {
            let test_clone = log_struct.clone();
            log_info!("Log message", test_struct = test_clone);
        }
        flushing!("flushing", data = "");
        //res.push(get_unix_nano() - timer);
    }));

    group.finish();
}

fn bench_flashlog_4bytes(c: &mut Criterion) {
    let mut group = c.benchmark_group("flashlog");
    let _logger = Logger::initialize()
        .with_file("logs", "message")
        .unwrap()
        .with_console_report(false)
        .with_msg_buffer_size(3_000_000)
        .with_msg_flush_interval(3_000_000)
        .with_max_log_level(LogLevel::Info)
        .with_timezone(TimeZone::Local)
        .launch();

    info!("Warm up");
    info!("Warm up");
    info!("Warm up");

    let iteration = 200_000;
    
    group.bench_function(format!("flashlog - log 4bytes struct {} times", iteration).as_str(), |b| b.iter(|| {
        for i in 0..iteration {
            log_info!("Log msg", test_struct = i);
        }
        flushing!("flushing", data = "");
    }));

    group.finish();
}

criterion_group!(benches, bench_flashlog_40bytes, bench_flashlog_4bytes);
criterion_main!(benches);