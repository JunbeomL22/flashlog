use anyhow::Result;
use serde::{Deserialize, Serialize};
use flashlog::get_unix_nano;

use flashlog::{
    self,
    TIMEZONE,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LogStruct {
    pub data: [u64; 10],
}

const log_struct: LogStruct = LogStruct { data: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10] };

fn main() {
    let _ = Logger::initialize()
        .with_file("logs", "message")?
        .with_console_report(false)
        .with_max_log_level(LogLevel::Info)
        .with_timezone(TimeZone::Local)
        .with
        .launch();
}