//! # FlashLog
//! 
//! A blazingly fast Rust logging library with lazy evaluation.
//! 
//! [![Crates.io](https://img.shields.io/crates/v/flashlog.svg)](https://crates.io/crates/flashlog)
//! [![Documentation](https://docs.rs/flashlog/badge.svg)](https://docs.rs/flashlog)
//! [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
//! 
//! ## Features
//! 
//! - **Lazy Evaluation**: Most evaluations are performed in the logger thread, resulting in exceptional performance.
//! - **Lazy String**: String interpolation in `flash_xxx!` macros is inherently lazy.
//! - **JSON Output**: Log messages are printed in `JSON` format for easy parsing and analysis.
//! - **Customizable**: Flexible configuration options for file output, console reporting, buffer size, and more.
//! - **Timezone Support**: Ability to set local or custom timezones for log timestamps.
//! 
//! ## Quick Start
//! 
//! Add FlashLog to your `Cargo.toml`:
//! 
//! ```toml
//! [dependencies]
//! flashlog = "0.2"
//! ```
//! 
//! Basic usage example:
//! 
//! Topic and message are optional and separated by a semicolon. In addition, messages can be added with key-value pairs.
//! 
//! ```rust
//! use flashlog::{flash_info, flash_debug, flush, Logger, LogLevel, TimeZone, RollingPeriod};
//! 
//! pub enum Hello {
//!    FlashLog,
//!    World,
//! }
//! 
//! impl std::fmt::Display for Hello {
//!     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//!         match self {
//!             Hello::FlashLog => write!(f, "FlashLog"),
//!             Hello::World => write!(f, "World"),
//!         }
//!     }
//! }
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let _logger = Logger::initialize()
//!         .with_file("logs", "message")? // without this the logger does not report a file
//!         .with_roll_period(RollingPeriod::Daily)? // Log file is rolled in daily basis
//!         .with_max_roll_files(10)? // Ten old file will remain. if compress is true, there will remain 10 gz file (older log) as well 
//!         .with_compress(true)? // compress old log file
//!         .with_console_report(true) // true means it reports to console too
//!         .with_msg_flush_interval(2_000_000_000) // flushing interval is 2 billion nanoseconds = 2 seconds
//!         .with_msg_buffer_size(100) // messages are flushed when there are more than 100 messages
//!         .with_max_log_level(LogLevel::Debug)
//!         .with_timezone(TimeZone::Local)
//!         .launch();
//! 
//!     flash_info!(Hello::FlashLog);
//!     // {"date":"20240915","level":"Info","message":"","offset":9,"src":"src\\logger_v2.rs:346","time":"20:34:30.684:921:877","topic":"World"}
//!     flash_info!(Hello::World);
//!     // {"date":"20240915","level":"Info","message":"","offset":9,"src":"src\\logger_v2.rs:347","time":"20:34:30.684:922:238","topic":"FlashLog"}
//!     flash_info!("Hello");
//!     // {"date":"20240915","level":"Info","message":"","offset":9,"src":"src\\logger_v2.rs:348","time":"20:34:30.684:922:488","topic":"Hello"}
//!     flash_info!("Hello"; "FlashLog");
//!     // {"date":"20240915","level":"Info","message":"FlashLog","offset":9,"src":"src\\logger_v2.rs:349","time":"20:34:30.684:922:739","topic":"Hello"}
//!     flash_info!("Hello"; "FlashLog"; version = "0.1.0");
//!     // {"data":{"version":"0.1.0"},"date":"20240915","level":"Info","message":"FlashLog","offset":9,"src":"src\\logger_v2.rs:350","time":"20:34:30.684:924:813","topic":"Hello"}
//!     flash_info!("Hello"; "FlashLog"; version = "0.1.0", author = "John Doe");
//!     // {"data":{"author":"John Doe","version":"0.1.0"},"date":"20240915","level":"Info","message":"FlashLog","offset":9,"src":"src\\logger_v2.rs:351","time":"20:34:30.684:925:143","topic":"Hello"}
//!     flash_info!(version = "0.1.0");
//!     // {"data":{"version":"0.1.0"},"date":"20240915","level":"Info","message":"","offset":9,"src":"src\\logger_v2.rs:352","time":"20:34:30.684:925:394","topic":""}
//!     flash_info!(version = "0.1.0", author = "John Doe");
//!     // {"data":{"author":"John Doe","version":"0.1.0"},"date":"20240915","level":"Info","message":"","offset":9,"src":"src\\logger_v2.rs:353","time":"20:34:30.684:925:654","topic":""}
//!     flash_info!("topic1"; "message {} {}", 1, 2);
//!     // {"data":"","date":"20240915","level":"Info","message":"message 1 2","offset":9,"src":"src\\logger_v2.rs:354","time":"20:34:30.684:925:955","topic":"topic1"}
//!     flash_info!("topic2"; "message {} {}", 1, 2; struct_info = 1, struct_info2 = 2);
//!     // {"data":{"struct_info":1,"struct_info2":2},"date":"20240915","level":"Info","message":"message 1 2","offset":9,"src":"src\\logger_v2.rs:355","time":"20:34:30.684:926:847","topic":"topic2"}
//!     flush!(); // this flushes regardless of the buffer size and flush interval
//! 
//!     Ok(())
//! }
//! ```
//! 
//! For more detailed information about each module and function, please refer to the individual documentation pages.


pub mod timer;
pub mod lazy_string;
pub mod logger_v2;
pub mod logger;
pub mod rolling_file;
pub mod compile_time;

pub use crate::timer::{
    get_unix_nano,
    convert_unix_nano_to_date_and_time,
};
pub type UnixNano = u64;
pub use crate::logger::{
    LazyMessage, 
    LogLevel, 
    LogMessage, 
    TimeZone, 
    Logger,
    LOG_SENDER,
    TIMEZONE,
    MAX_LOG_LEVEL,
};
pub use rolling_file::{
    RollingConfig,
    RollingFileWriter,
    RollingPeriod,
};
pub use serde_json;


