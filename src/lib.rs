//! # FlashLog
//! 
//! A blazingly fast Rust logging library with lazy evaluation and compile-time filtering.
//! 
//! [![Crates.io](https://img.shields.io/crates/v/flashlog.svg)](https://crates.io/crates/flashlog)
//! [![Documentation](https://docs.rs/flashlog/badge.svg)](https://docs.rs/flashlog)
//! [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
//! 
//! ## Important Changes in Version 0.3.0
//! 
//! 1. **Cloning in Compile-Time Macros**: In `flash_xxx_ct!` macros, cloning happens automatically inside the macro. Users don't need to clone data outside the macro, but be aware that implicit cloning still occurs in the worker thread.
//! 
//! 2. **Timestamp Generation**: Timestamps are now generated in the logger thread rather than the worker thread.
//! 
//! 3. **Compile-Time Feature Options**: New compile-time feature options have been added, compatible with the `flash_xxx_ct!` macros. The `flash_xxx!` macros are now deprecated in favor of the new compile-time filtered macros.
//! 
//! ## Features
//! 
//! - **Lazy Evaluation**: Most evaluations are performed in the logger thread, resulting in exceptional performance.
//! - **Lazy String**: String interpolation in `flash_xxx!` macros is inherently lazy.
//! - **Compile-Time Filtering**: Using `flash_xxx_ct!` macros and feature flags for efficient filtering at compile time.
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
//! flashlog = {version = "0.3", features = ["max-level-info"]}
//! ```
//! 
//! The compile time feature `max-level-info` is optional and can be omitted. It sets the maximum log level to `Info` at compile time.
//! Users can still use runtime log level filtering, but the compile-time option takes precedence over runtime conditions.
//! 
//! Available compile-time features are:
//! - `max-level-off`
//! - `max-level-error`
//! - `max-level-warn`
//! - `max-level-info`
//! - `max-level-debug`
//! - `max-level-trace`
//! 
//! Basic usage example:
//! 
//! ```rust
//! use flashlog::{flash_info_ct, flash_debug_ct, flush, Logger, LogLevel, TimeZone, RollingPeriod};
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
//!         .include_unixnano(true) // include unixnano (u64) in the log as well as date and time
//!         .with_logger_core(1) // core for logger affinity
//!         .launch();
//! 
//!     flash_info_ct!(Hello::FlashLog);
//!     flash_info_ct!(Hello::World);
//!     flash_info_ct!("Hello");
//!     flash_info_ct!("Hello"; "FlashLog");
//!     flash_info_ct!("Hello"; "FlashLog"; version = "0.1.0");
//!     flash_info_ct!("Hello"; "FlashLog"; version = "0.1.0", author = "John Doe");
//!     flash_info_ct!(version = "0.1.0");
//!     flash_info_ct!(version = "0.1.0", author = "John Doe");
//!     flash_info_ct!("topic1"; "message {} {}", 1, 2);
//!     flash_info_ct!("topic2"; "message {} {}", 1, 2; struct_info = 1, struct_info2 = 2);
//!     flush!(); // this flushes regardless of the buffer size and flush interval
//! 
//!     Ok(())
//! }
//! ```
//! 
//! For more detailed information about each module and function, please refer to the individual documentation pages.


pub mod timer;
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