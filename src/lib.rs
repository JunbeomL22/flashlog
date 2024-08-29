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
//! - **JSON Output**: Log messages are printed in `JSON` format for easy parsing and analysis.
//! - **LazyString**: Provides `LazyString` for optimized string interpolation.
//! - **Customizable**: Flexible configuration options for file output, console reporting, buffer size, and more.
//! - **Timezone Support**: Ability to set local or custom timezones for log timestamps.
//! 
//! ## Quick Start
//! 
//! Add FlashLog to your `Cargo.toml`:
//! 
//! ```toml
//! [dependencies]
//! flashlog = "0.1"
//! ```
//! 
//! Basic usage example:
//! 
//! ```rust
//! use flashlog::{Logger, LogLevel, log_info};
//! 
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let logger = Logger::initialize()
//!         .with_file("logs", "message")?
//!         .with_max_log_level(LogLevel::Info)
//!         .launch();
//! 
//!     log_info!("Hello, FlashLog!");
//! 
//!     Ok(())
//! }
//! ```
//! 
//! ## Advanced Usage
//! 
//! ### Logging Structs
//! 
//! FlashLog can easily log custom structs:
//! 
//! ```rust
//! use serde::{Deserialize, Serialize};
//! use flashlog::{Logger, LogLevel, log_info};
//! 
//! #[derive(Debug, Serialize, Deserialize, Clone)]
//! pub struct LogStruct {
//!     data: [u64; 10],
//! }
//! 
//! impl Default for LogStruct {
//!     fn default() -> Self {
//!         LogStruct { data: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10] }
//!     }
//! }
//! 
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let logger = Logger::initialize()
//!         .with_file("logs", "message")?
//!         .with_max_log_level(LogLevel::Info)
//!         .launch();
//! 
//!     let log_struct = LogStruct::default();
//!     log_info!("Log message", log_struct = log_struct);
//! 
//!     Ok(())
//! }
//! ```
//! 
//! ### Using LazyString for Optimization
//! 
//! ```rust
//! use flashlog::{lazy_string::LazyString, log_info};
//! 
//! let lazy_msg = LazyString::new(|| format!("{} {} {}", 1, 2, 3)); // Evaluated in logger thread
//! log_info!("LazyOne", msg = lazy_msg);
//! ```
//! 
//! ## Configuration Options
//! 
//! FlashLog offers various configuration options:
//! 
//! ```rust
//! let logger = Logger::initialize()
//!     .with_file("logs", "message")?
//!     .with_console_report(false)
//!     .with_msg_buffer_size(1_000_000)
//!     .with_msg_flush_interval(1_000_000)
//!     .with_max_log_level(LogLevel::Info)
//!     .with_timezone(TimeZone::Local)
//!     .launch();
//! ```
//! 
//! ## Output Format
//! 
//! Logs are outputted in JSON format for easy parsing:
//! 
//! ```json
//! {
//!   "data": {"text": "Warm up"},
//!   "date": "20240829",
//!   "level": "Info",
//!   "offset": 9,
//!   "src": "src/main.rs:135",
//!   "time": "20:08:21.071:409:907",
//!   "topic": "not given"
//! }
//! ```
//! 
//! For more detailed information about each module and function, please refer to the individual documentation pages.


pub mod timer;
pub mod lazy_string;
pub mod logger;

pub use crate::timer::{
    get_unix_nano,
    convert_unix_nano_to_date_and_time,
};
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
pub use serde_json;


