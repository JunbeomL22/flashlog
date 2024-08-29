//! # My Crate
//!
//! This crate provides utilities for handling timers, lazy strings, and logging.
//!
//! ## Modules
//!
//! - `timer`: Functions for working with Unix timestamps.
//! - `lazy_string`: Utilities for lazy string evaluation.
//! - `logger`: A logging framework with various log levels and message formatting options.


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


