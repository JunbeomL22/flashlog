use crate::flash_trace;
use crate::timer::get_unix_nano;
use crate::rolling_file::{
    RollingFileWriter,
    RollingConfig,
    RollingPeriod,
};
//
//use anyhow::{anyhow, Ok, Result};
use chrono;
use core_affinity;
use crossbeam_channel::{unbounded, Sender};
use once_cell::sync::Lazy;
use std::path::PathBuf;
use std::{
    sync::{
        atomic::{AtomicBool, AtomicI32, AtomicUsize, Ordering, AtomicU64},
        Mutex,
    },
    thread,
};

static LOG_MESSAGE_BUFFER_SIZE: Lazy<AtomicUsize> = Lazy::new(|| AtomicUsize::new(1_000_000));
//2_000_000; // string length
static LOG_MESSAGE_FLUSH_INTERVAL: Lazy<AtomicU64> = Lazy::new(|| AtomicU64::new(500_000_000));
//2_000_000_000; // 2 second

pub static INCLUDE_UNIXNANO: Lazy<AtomicBool> = Lazy::new(|| AtomicBool::new(false));
pub static MAX_LOG_LEVEL: Lazy<AtomicUsize> = Lazy::new(|| AtomicUsize::new(LogLevel::NIL.as_usize()));
pub static TIMEZONE: Lazy<AtomicI32> = Lazy::new(|| AtomicI32::new(TimeZone::Local as i32));
pub static CONSOLE_REPORT: Lazy<AtomicBool> = Lazy::new(|| AtomicBool::new(false));
pub static FILE_REPORT: Lazy<AtomicBool> = Lazy::new(|| AtomicBool::new(false));
pub static LOGGER_HANDLER: Lazy<Mutex<Option<thread::JoinHandle<()>>>> =Lazy::new(|| Mutex::new(None));
pub static LOGGER_CORE: Lazy<AtomicI32> = Lazy::new(|| AtomicI32::new(-1)); // -1 means that setting affinity to any remaining core

pub static LOG_SENDER: Lazy<Sender<LogMessage>> = Lazy::new(|| {
    let (sender, receiver) = unbounded();

    let mut message_queue: Vec<String> = Vec::with_capacity(LOG_MESSAGE_BUFFER_SIZE.load(Ordering::SeqCst));
    let msg_buffer_size = LOG_MESSAGE_BUFFER_SIZE.load(Ordering::SeqCst);
    let msg_flush_interval = LOG_MESSAGE_FLUSH_INTERVAL.load(Ordering::SeqCst);
    let affinity_core = LOGGER_CORE.load(Ordering::SeqCst);
    let mut last_flush_time = get_unix_nano();

    *LOGGER_HANDLER.lock().expect("Logger hander lock") = Some(thread::spawn(move || {
        let mut rolling_writer: Option<RollingFileWriter> = None;
        let file_report = FILE_REPORT.load(Ordering::Relaxed);
        let console_report = CONSOLE_REPORT.load(Ordering::Relaxed);
        while let Ok(msg) = receiver.recv() {
            match msg {
                LogMessage::LazyMessage(lazy_message) => {
                    let message = lazy_message.eval();
                    let new_msg_length = message.len();
                    let buffer_size = message_queue.len();
                    let current_timestamp = get_unix_nano();
                    message_queue.push(message);

                    if (buffer_size + new_msg_length >= msg_buffer_size)
                        || (current_timestamp >= msg_flush_interval + last_flush_time)
                        // flush if the buffer is full or the time interval is passed
                    {
                        let output = message_queue.join("");

                        if file_report {
                            if let Some(ref mut writer) = rolling_writer {    
                                writer.write_all(output.as_bytes()).unwrap();
                            }
                        }
                        
                        if console_report {
                            println!("{}", output);
                        }

                        message_queue.clear();
                        last_flush_time = current_timestamp;
                    }
                }
                LogMessage::FlushingMessage(lazy_message) => {
                    let message = lazy_message.eval();
                    message_queue.push(message);

                    let output = message_queue.join("");
                    if file_report {
                        if let Some(ref mut writer) = rolling_writer {
                            writer.write_all(output.as_bytes()).unwrap();
                        }
                    }
                    if console_report {
                        println!("{}", output);
                    }

                    message_queue.clear();
                    last_flush_time = get_unix_nano();
                }
                LogMessage::StaticString(message) => {
                    let buffer_size = message_queue.len();
                    let timestamp = get_unix_nano();
                    message_queue.push(message.to_string());

                    if (buffer_size + message.len() >= msg_buffer_size)
                        || (timestamp >= msg_flush_interval + last_flush_time)
                    {
                        let output = message_queue.join("");
                        if file_report {
                            if let Some(ref mut writer) = rolling_writer {
                                writer.write_all(output.as_bytes()).unwrap();
                            }
                        }

                        if console_report {
                            println!("{}", output);
                        }
                    }
                }
                LogMessage::SetFile(config) => {
                    if let Some(ref mut writer) = rolling_writer {
                        writer.flush().expect("Failed to flush log file writer");
                        let _ = writer.sync_all();
                    } else {
                        let writer = RollingFileWriter::new(config).expect("Failed to create RollingFileWriter");
                        rolling_writer = Some(writer);
                    }
                }
                LogMessage::Flush => {
                    let output = message_queue.join("");
                    if file_report {
                        if let Some(ref mut writer) = rolling_writer {
                            writer.write_all(output.as_bytes()).unwrap();
                            writer.flush().expect("Failed to flush log file writer");
                            let _ = writer.sync_all();
                        }
                    }
                    if console_report {
                        println!("{}", output);
                    }
                    message_queue.clear();
                    last_flush_time = get_unix_nano();
                }
                LogMessage::SetCore => {
                    let available_core_ids = core_affinity::get_core_ids().expect("Failed to get available core IDs");
                    let core_id = if affinity_core == -1 {
                        available_core_ids.first().cloned()
                    } else {
                        let core_id = core_affinity::CoreId { id: affinity_core as usize };
                        if available_core_ids.contains(&core_id) {
                            Some(core_id)
                        } else {
                            available_core_ids.first().cloned()
                        }
                    };

                    if let Some(core_id) = core_id {
                        core_affinity::set_for_current(core_id);
                    }
                }
                LogMessage::Close => {
                    let output = message_queue.join("");
                    if file_report {
                        if let Some(ref mut writer) = rolling_writer {
                            writer.write_all(output.as_bytes()).unwrap();
                            writer.flush().expect("Failed to flush log file writer in Close");
                            let _ = writer.sync_all();
                        }
                    }
                    if console_report {
                        println!("{}", output);
                    }
                    break;
                }
            }
        }
    }));
    sender
});

pub enum TimeZone {
    Local,
    Seoul,
    Japan,
    NewYork,
}

impl TimeZone {
    #[inline]
    pub fn as_offset_hour(&self) -> i32 {
        match self {
            TimeZone::Local => {
                let local = chrono::Local::now();
                let offset = local.offset().local_minus_utc() / 3600;
                offset
            }
            TimeZone::Seoul => 9,
            TimeZone::Japan => 9,
            TimeZone::NewYork => -4,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LogLevel {
    NIL = 0,
    Error = 1,
    Warn = 2,
    Info = 3,
    Debug = 4,
    Trace = 5,
}

impl LogLevel {
    #[inline]
    pub fn as_usize(&self) -> usize {
        match self {
            LogLevel::NIL => 0,
            LogLevel::Error => 1,
            LogLevel::Warn => 2,
            LogLevel::Info => 3,
            LogLevel::Debug => 4,
            LogLevel::Trace => 5,
        }
    }

    #[inline]
    pub fn from_usize(level: usize) -> Result<LogLevel, &'static str> {
        match level {
            0 => Ok(LogLevel::NIL),
            1 => Ok(LogLevel::Error),
            2 => Ok(LogLevel::Warn),
            3 => Ok(LogLevel::Info),
            4 => Ok(LogLevel::Debug),
            5 => Ok(LogLevel::Trace),
            _ => {
                Err("Invalid log level")
            }
        }
    }
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LogLevel::NIL => write!(f, "Nil"),
            LogLevel::Trace => write!(f, "Trace"),
            LogLevel::Debug => write!(f, "Debug"),
            LogLevel::Info => write!(f, "Info"),
            LogLevel::Error => write!(f, "Error"),
            LogLevel::Warn => write!(f, "Warn"),
        }
    }
}

#[deprecated(since = "0.2.0", note = "Use flashlog::flash_trace! instead")]
#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {{
        $crate::log_fn_json!($crate::LogLevel::Trace, "not given", text = format!($($arg)*));
    }};
}


#[deprecated(since = "0.2.0", note = "Use flashlog::flash_debug! instead")]
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {{
        $crate::log_fn_json!($crate::LogLevel::Debug, "not given", text = format!($($arg)*));
    }};
}

#[deprecated(since = "0.2.0", note = "Use flashlog::flash_info! instead")]
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {{
        $crate::log_fn_json!($crate::LogLevel::Info, "not given", text = format!($($arg)*));
    }};
}

#[deprecated(since = "0.2.0", note = "Use flashlog::flash_warn! instead")]
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {{
        $crate::log_fn_json!($crate::LogLevel::Warn, "not given", text = format!($($arg)*));
    }};
}

#[deprecated(since = "0.2.0", note = "Use flashlog::flash_error! instead")]
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {{
        $crate::log_fn_json!($crate::LogLevel::Error, "not given", text = format!($($arg)*));
    }};
}

//---------
#[deprecated(since = "0.2.0", note = "Use flashlog::flash_trace! instead")]
#[macro_export]
macro_rules! log_trace {
    ($topic:expr, $($key:ident=$value:expr),+ $(,)?) => {{
        $crate::log_fn_json!($crate::LogLevel::Trace, $topic, $($key=$value),+);
    }};

    ($topic:expr, $struct:expr) => {{
        $crate::log_fn_json!($crate::LogLevel::Trace, $topic, $struct);
    }};
}

#[deprecated(since = "0.2.0", note = "Use flashlog::flash_debug! instead")]
#[macro_export]
macro_rules! log_debug {
    ($topic:expr, $($key:ident=$value:expr),+ $(,)?) => {{
        $crate::log_fn_json!($crate::LogLevel::Debug, $topic, $($key=$value),+);
    }};
    ($topic:expr, $struct:expr) => {{
        $crate::log_fn_json!($crate::LogLevel::Debug, $topic, $struct);
    }};
}

#[deprecated(since = "0.2.0", note = "Use flashlog::flash_info! instead")]
#[macro_export]
macro_rules! log_info {
    ($topic:expr, $($key:ident=$value:expr),+ $(,)?) => {{
        $crate::log_fn_json!($crate::LogLevel::Info, $topic, $($key=$value),+);
    }};
    ($topic:expr, $struct:expr) => {{
        $crate::log_fn_json!($crate::LogLevel::Info, $topic, $struct);
    }};
}


#[deprecated(since = "0.2.0", note = "Use flashlog::flash_warn! instead")]
#[macro_export]
macro_rules! log_warn {
    ($topic:expr, $($key:ident=$value:expr),+ $(,)?) => {{
        $crate::log_fn_json!($crate::LogLevel::Warn, $topic, $($key=$value),+);
    }};
    ($topic:expr, $struct:expr) => {{
        $crate::log_fn_json!($crate::LogLevel::Warn, $topic, $struct);
    }};
}


#[deprecated(since = "0.2.0", note = "Use flashlog::flash_error! instead")]
#[macro_export]
macro_rules! log_error {
    ($topic:expr, $($key:ident=$value:expr),+ $(,)?) => {{
        $crate::log_fn_json!($crate::LogLevel::Error, $topic, $($key=$value),+);
    }};
    ($topic:expr, $struct:expr) => {{
        $crate::log_fn_json!($crate::LogLevel::Error, $topic, $struct);
    }};
}

#[macro_export]
macro_rules! log_fn_json {
    ($level:expr, $topic:expr, $($key:ident=$value:expr),+ $(,)?) => {{
        if $level <= $crate::LogLevel::from_usize($crate::MAX_LOG_LEVEL.load(std::sync::atomic::Ordering::Relaxed)).expect("Invalid log level") {
            let unixnano = $crate::get_unix_nano();
            let include_unixnano = $crate::logger::INCLUDE_UNIXNANO.load(std::sync::atomic::Ordering::Relaxed);
            //
            $(
                #[allow(non_snake_case)]
                let $key = $value.clone();
            )*
            let func = move || {
                let json_obj = $crate::serde_json::json!({
                    $(
                        stringify!($key): $key,
                    )+
                });
                let timezone = $crate::TIMEZONE.load(std::sync::atomic::Ordering::Relaxed);
                let (date, time) = $crate::convert_unix_nano_to_date_and_time(unixnano, timezone);
                let json_msg = match include_unixnano {
                    false => $crate::serde_json::json!({
                        "date": date,
                        "time": time,
                        "offset": timezone,
                        "level": $level.to_string(),
                        "src": format!("{}:{}", file!(), line!()),
                        "topic": $topic,
                        "data": json_obj,
                    }),
                    true => $crate::serde_json::json!({
                        "date": date,
                        "time": time,
                        "offset": timezone,
                        "level": $level.to_string(),
                        "src": format!("{}:{}", file!(), line!()),
                        "topic": $topic,
                        "data": json_obj,
                        "unixnano": unixnano,
                    }),
                };

                json_msg.to_string() + "\n"
            };

            $crate::LOG_SENDER.try_send($crate::LogMessage::LazyMessage($crate::LazyMessage::new(func))).unwrap();
        }
    }};

    // In case of structs
    ($level:expr, $topic:expr, $struct:expr) => {{
        if $level <= $crate::LogLevel::from_usize($crate::LOG_LEVEL.load(std::sync::atomic::Ordering::Relaxed)).expect("Invalid log level") {
            let unixnano = $crate::get_unix_nano();
            let include_unixnano = $crate::logger::INCLUDE_UNIXNANO.load(std::sync::atomic::Ordering::Relaxed);
            #[allow(non_snake_case)]
            let struct_clone = $struct.clone();
            let func = move || {
                let json_obj = $crate::serde_json::to_value(struct_clone).unwrap_or_else(|e| {
                    $crate::serde_json::json!({ "error": format!("serialization error: {}", e) })
                });
                let timezone = $crate::TIMEZONE.load(std::sync::atomic::Ordering::Relaxed);
                let (date, time) = $crate::convert_unix_nano_to_date_and_time(unixnano, timezone);
                let json_msg = match include_unixnano {
                    false => $crate::serde_json::json!({
                        "date": date,
                        "time": time,
                        "offset": timezone,
                        "level": $level.to_string(),
                        "src": format!("{}:{}", file!(), line!()),
                        "topic": $topic,
                        "data": json_obj,
                    }),
                    true => $crate::serde_json::json!({
                        "date": date,
                        "time": time,
                        "offset": timezone,
                        "level": $level.to_string(),
                        "src": format!("{}:{}", file!(), line!()),
                        "topic": $topic,
                        "data": json_obj,
                        "unixnano": unixnano,
                    }),
                };

                json_msg.to_string() + "\n"
            };

            $crate::LOG_SENDER.try_send($crate::LogMessage::LazyMessage($crate::LazyMessage::new(func))).unwrap();
        }
    }};
}

#[deprecated(since = "0.2.0", note = "Use flashlog::flush! instead")]
#[macro_export]
macro_rules! flushing_log_info {
    ($topic:expr, $($key:ident=$value:expr),+ $(,)?) => {{
        $crate::flushing_log_fn_json!($crate::LogLevel::Info, $topic, $($key=$value),+);
    }};
    ($topic:expr, $struct:expr) => {{
        $crate::flushing_log_fn_json!($crate::LogLevel::Info, $topic, $struct);
    }};
}

#[deprecated(since = "0.2.0", note = "Use flashlog::flush! instead")]
#[macro_export]
macro_rules! flushing_log_debug {
    ($topic:expr, $($key:ident=$value:expr),+ $(,)?) => {{
        $crate::flushing_log_fn_json!($crate::LogLevel::Debug, $topic, $($key=$value),+);
    }};
    ($topic:expr, $struct:expr) => {{
        $crate::flushing_log_fn_json!($crate::LogLevel::Debug, $topic, $struct);
    }};
}

#[deprecated(since = "0.2.0", note = "Use flashlog::flush! instead")]
#[macro_export]
macro_rules! flushing_log_error {
    ($topic:expr, $($key:ident=$value:expr),+ $(,)?) => {{
        $crate::flushing_log_fn_json!($crate::LogLevel::Error, $topic, $($key=$value),+);
    }};
    ($topic:expr, $struct:expr) => {{
        $crate::flushing_log_fn_json!($crate::LogLevel::Error, $topic, $struct);
    }};
}

#[deprecated(since = "0.2.0", note = "Use flashlog::flush! instead")]
#[macro_export]
macro_rules! flushing_log_trace {
    ($topic:expr, $($key:ident=$value:expr),+ $(,)?) => {{
        $crate::flushing_log_fn_json!($crate::LogLevel::Trace, $topic, $($key=$value),+);
    }};
    ($topic:expr, $struct:expr) => {{
        $crate::flushing_log_fn_json!($crate::LogLevel::Trace, $topic, $struct);
    }};
}

#[macro_export]
macro_rules! flushing_log_fn_json {
    ($level:expr, $topic:expr, $($key:ident=$value:expr),+ $(,)?) => {{
        if $level <= $crate::LogLevel::from_usize($crate::MAX_LOG_LEVEL.load(std::sync::atomic::Ordering::Relaxed)).expect("Invalid log level") {
            let unixnano = $crate::get_unix_nano();
            let include_unixnano = $crate::logger::INCLUDE_UNIXNANO.load(std::sync::atomic::Ordering::Relaxed);
            let func = move || {
                let json_obj = $crate::serde_json::json!({
                    $(
                        stringify!($key): $value,
                    )+
                });
                let timezone = $crate::TIMEZONE.load(std::sync::atomic::Ordering::Relaxed);
                let (date, time) = $crate::convert_unix_nano_to_date_and_time(unixnano, timezone);
                let json_msg = match include_unixnano {
                    true => $crate::serde_json::json!({
                        "date": date,
                        "time": time,
                        "offset": timezone,
                        "level": $level.to_string(),
                        "src": format!("{}:{}", file!(), line!()),
                        "topic": $topic,
                        "data": json_obj,
                        "unixnano": unixnano,
                    }),
                    false => $crate::serde_json::json!({
                        "date": date,
                        "time": time,
                        "offset": timezone,
                        "level": $level.to_string(),
                        "src": format!("{}:{}", file!(), line!()),
                        "topic": $topic,
                        "data": json_obj,
                    }),
                };
                json_msg.to_string() + "\n"
            };

            $crate::LOG_SENDER.try_send($crate::LogMessage::FlushingMessage($crate::LazyMessage::new(func))).unwrap();
        }
    }};

    // In case of structs
    ($level:expr, $topic:expr, $struct:expr) => {{
        if $level <= $crate::LogLevel::from_usize($crate::LOG_LEVEL.load(std::sync::atomic::Ordering::Relaxed)).unwrap() {
            let unixnano = $crate::get_unix_nano();
            let include_unixnano = $crate::logger::INCLUDE_UNIXNANO.load(std::sync::atomic::Ordering::Relaxed);
            let func = move || {
                let json_obj = $crate::serde_json::to_value($struct).unwrap_or_else(|e| {
                    $crate::serde_json::json!({ "error": format!("serialization error: {}", e) })
                });
                let timezone = $crate::TIMEZONE.load(std::sync::atomic::Ordering::Relaxed);
                let (date, time) = $crate::convert_unix_nano_to_date_and_time(timestamp, timezone);
                match include_unixnano {
                    true => {
                        let json_msg = $crate::serde_json::json!({
                            "date": date,
                            "time": time,
                            "offset": timezone,
                            "level": $level.to_string(),
                            "src": format!("{}:{}", file!(), line!()),
                            "topic": $topic,
                            "data": json_obj,
                            "unixnano": unixnano,
                        });
                        json_msg.to_string() + "\n"
                    }
                    false => {
                        let json_msg = $crate::serde_json::json!({
                            "date": date,
                            "time": time,
                            "offset": timezone,
                            "level": $level.to_string(),
                            "src": format!("{}:{}", file!(), line!()),
                            "topic": $topic,
                            "data": json_obj,
                        });
                        json_msg.to_string() + "\n"
                    }
                }
            };
            $crate::LOG_SENDER.try_send($crate::LogMessage::FlushingMessage($crate::LazyMessage::new(func))).unwrap();
        }
    }};
}

pub struct LoggerGuard;

impl Drop for LoggerGuard {
    fn drop(&mut self) {
        flash_trace!("LoggerGuard"; "LoggerGuard is dropped");
        Logger::finalize();
    }
}

pub struct Logger {
    file_config: Option<RollingConfig>,
}


#[derive(Debug)]
pub enum LoggerError {
    UnsetFile,
}

impl std::fmt::Display for LoggerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LoggerError::UnsetFile => write!(f, "File config is not set. Use with_file first"),
        }
    }
}

impl std::error::Error for LoggerError {}

impl Logger {
    pub fn finalize() {
        let _ = LOG_SENDER.try_send(LogMessage::Close);
        if let Some(handler) = LOGGER_HANDLER.lock().expect("Failed to lock LOGGER_HANDLER").take() {
            let _ = handler.join();
        }
    }

    pub fn initialize() -> Logger {
        let _ = get_unix_nano();
        LOG_MESSAGE_BUFFER_SIZE.store(1_000_000, Ordering::Relaxed);
        LOG_MESSAGE_FLUSH_INTERVAL.store(1_000_000, Ordering::Relaxed);
        Logger { file_config: None }
    }

    pub fn with_file(mut self, file_path: &str, file_name: &str) -> Result<Logger, std::io::Error> {
        std::fs::create_dir_all(file_path)?;

        let config = RollingConfig {
            base_path: PathBuf::from(file_path),
            file_name_prefix: file_name.to_string(),
            roll_period: Some(RollingPeriod::Daily),
            max_roll_files: Some(10),
            compress: false,
        };

        self.file_config = Some(config);
        FILE_REPORT.store(true, Ordering::SeqCst);

        Ok(self)
    }

    pub fn with_compress(mut self, compress: bool) -> Result<Logger, LoggerError> {
        if let Some(ref mut config) = self.file_config {
            config.compress = compress;
            Ok(self)
        } else {
            Err(LoggerError::UnsetFile)
        }
    }

    pub fn with_logger_core(self, core: i32) -> Logger {
        LOGGER_CORE.store(core, Ordering::SeqCst);
        self
    }

    pub fn with_roll_period(mut self, period: RollingPeriod) -> Result<Logger, LoggerError> {
        if let Some(ref mut config) = self.file_config {
            config.roll_period = Some(period);
            Ok(self)
        } else {
            Err(LoggerError::UnsetFile)
        }
    }

    pub fn include_unixnano(self, include: bool) -> Logger {
        INCLUDE_UNIXNANO.store(include, Ordering::Relaxed);
        self
    }

    pub fn with_max_roll_files(mut self, max_roll_files: usize) -> Result<Logger, LoggerError> {
        if let Some(ref mut config) = self.file_config {
            config.max_roll_files = Some(max_roll_files);
            Ok(self)
        } else {
            Err(LoggerError::UnsetFile)
        }
    }

    pub fn with_console_report(self, console_report: bool) -> Logger {
        CONSOLE_REPORT.store(console_report, Ordering::Relaxed);
        self
    }

    pub fn with_msg_buffer_size(self, size: usize) -> Logger {
        LOG_MESSAGE_BUFFER_SIZE.store(size, Ordering::Relaxed);
        self
    }

    pub fn with_msg_flush_interval(self, interval: u64) -> Logger {
        LOG_MESSAGE_FLUSH_INTERVAL.store(interval, Ordering::Relaxed);
        self
    }

    #[deprecated(since = "0.3.0", note = "it is recommended to use compile time filter options and use flash_xxxx_ct! instead")]
    pub fn with_max_log_level(self, level: LogLevel) -> Logger {
        MAX_LOG_LEVEL.store(level.as_usize(), Ordering::Relaxed);
        self
    }

    pub fn with_timezone(self, timezone: TimeZone) -> Logger {
        TIMEZONE.store(timezone.as_offset_hour(), Ordering::Relaxed);
        self
    }

    pub fn launch(self) -> LoggerGuard {
        let rolling_config = self.file_config.clone();
        let _ = LOG_SENDER.send(LogMessage::SetCore);
        if let Some(config) = rolling_config {
            let _ = LOG_SENDER.send(LogMessage::SetFile(config));
        }
        LoggerGuard {}
    }
}

pub enum LogMessage {
    LazyMessage(LazyMessage),
    FlushingMessage(LazyMessage),
    StaticString(&'static str),
    SetFile(RollingConfig),
    Flush,
    SetCore,
    Close,
}

pub struct LazyMessage {
    data: Box<dyn (FnOnce() -> String) + Send + 'static>,
}

impl LazyMessage {
    pub fn new<F>(data: F) -> LazyMessage
    where
        F: (FnOnce() -> String) + Send + 'static,
    {
        LazyMessage {
            data: Box::new(data),
        }
    }

    pub fn eval(self) -> String {
        (self.data)()
    }
}