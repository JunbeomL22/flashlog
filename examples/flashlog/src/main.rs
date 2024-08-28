use anyhow::Result;
use serde::{Deserialize, Serialize};

use flashlog::{
    LogLevel, Logger, TimeZone,
    get_unix_nano,
    log_info,
    info,
    flushing_log_info as flush,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LogStruct {
    pub data: [u64; 10],
}

impl Default for LogStruct {
    fn default() -> Self {
        LogStruct { data: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10] }
    }
}

fn main() -> Result<()> {
    let logger = Logger::initialize()
        .with_file("logs", "message")?
        .with_console_report(false)
        .with_msg_buffer_size(2_000_000)
        .with_msg_flush_interval(2_000_000)
        .with_max_log_level(LogLevel::Info)
        .with_timezone(TimeZone::Local)
        .launch();

    info!("Warm up");

    let iteration = 300_000;
    let start = get_unix_nano();

    let log_struct = LogStruct::default();
    for i in 0..iteration {
        let test_clone = log_struct.clone();
        log_info!("Log message", test_struct = test_clone);
        //log_info!("Log message", test_struct = i);
    }
    flush!("flushing", data = "");

    let elapsed = get_unix_nano() - start;
    let elapsed_as_seconds = elapsed as f64 / 1_000_000_000.0;
    let elapsed_average = elapsed as f64 / iteration as f64;

    
    println!(
        "elapsed: {:.3}s, average: {:.0}ns",
        elapsed_as_seconds, elapsed_average
    );

    Ok(())
}