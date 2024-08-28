// heavily advised by youngjin park https://github.com/youngjin-create
use crate::timer::get_unix_nano;
//
use anyhow::{anyhow, Result};
use chrono;
use core_affinity;
use crossbeam_channel::{unbounded, Sender};
use once_cell::sync::Lazy;
use std::{
    fs::{File, OpenOptions},
    io::{BufWriter, Write},
    path::PathBuf,
};
use std::{
    sync::{
        atomic::{AtomicBool, AtomicI32, AtomicUsize, Ordering, AtomicU64},
        Mutex,
    },
    thread,
};

static LOG_MESSAGE_BUFFER_SIZE: Lazy<AtomicUsize> = Lazy::new(|| AtomicUsize::new(1_000_000));
//2_000_000; // string length
static LOG_MESSAGE_FLUSH_INTERVAL: Lazy<AtomicU64> = Lazy::new(|| AtomicU64::new(2_000_000));
//2_000_000; // 2 second

pub static MAX_LOG_LEVEL: Lazy<AtomicUsize> =
    Lazy::new(|| AtomicUsize::new(LogLevel::NIL.as_usize()));
pub static TIMEZONE: Lazy<AtomicI32> = Lazy::new(|| AtomicI32::new(TimeZone::Local as i32));
pub static CONSOLE_REPORT: Lazy<AtomicBool> = Lazy::new(|| AtomicBool::new(false));
pub static LOGGER_HANDLER: Lazy<Mutex<Option<thread::JoinHandle<()>>>> =
    Lazy::new(|| Mutex::new(None));

pub static LOG_SENDER: Lazy<Sender<LogMessage>> = Lazy::new(|| {
    let (sender, receiver) = unbounded();

    let mut message_queue: Vec<String> = Vec::with_capacity(LOG_MESSAGE_BUFFER_SIZE.load(Ordering::SeqCst));
    let msg_buffer_size = LOG_MESSAGE_BUFFER_SIZE.load(Ordering::SeqCst);
    let msg_flush_interval = LOG_MESSAGE_FLUSH_INTERVAL.load(Ordering::SeqCst);

    let mut last_flush_time = get_unix_nano();

    *LOGGER_HANDLER.lock().unwrap() = Some(thread::spawn(move || {
        let mut writer: Option<BufWriter<File>> = None;
        while let Ok(msg) = receiver.recv() {
            match msg {
                LogMessage::LazyMessage(lazy_message) => {
                    let message = lazy_message.eval();
                    let new_msg_length = message.len();
                    let buffer_size = message_queue.len();
                    let timestamp = get_unix_nano();
                    message_queue.push(message);

                    if (buffer_size + new_msg_length > msg_buffer_size)
                        || (timestamp - last_flush_time > msg_flush_interval)
                    {
                        if let Some(ref mut writer) = writer {
                            let output = message_queue.join("");
                            writer.write_all(output.as_bytes()).unwrap();
                            if CONSOLE_REPORT.load(Ordering::Relaxed) {
                                println!("{}", output);
                            }

                            message_queue.clear();
                            last_flush_time = get_unix_nano();
                        }
                    }
                }
                LogMessage::FlushingMessage(lazy_message) => {
                    let message = lazy_message.eval();
                    message_queue.push(message);

                    if let Some(ref mut writer) = writer {
                        let output = message_queue.join("");
                        writer.write_all(output.as_bytes()).unwrap();
                        if CONSOLE_REPORT.load(Ordering::Relaxed) {
                            println!("{}", output);
                        }
                        message_queue.clear();
                        last_flush_time = get_unix_nano();
                    }
                }
                LogMessage::StaticString(message) => {
                    let buffer_size = message_queue.len();
                    let timestamp = get_unix_nano();
                    message_queue.push(message.to_string());

                    if (buffer_size + message.len() > msg_buffer_size)
                        || (timestamp - last_flush_time > msg_flush_interval)
                    {
                        if let Some(ref mut writer) = writer {
                            let output = message_queue.join("");
                            writer.write_all(output.as_bytes()).unwrap();
                            if CONSOLE_REPORT.load(Ordering::Relaxed) {
                                println!("{}", output);
                            }

                            message_queue.clear();
                            last_flush_time = get_unix_nano();
                        }
                    }
                }
                LogMessage::SetFile(file_name) => {
                    if let Some(ref mut writer) = writer {
                        writer.flush().unwrap();
                        let _ = writer.get_mut().sync_all();
                        *writer = OpenOptions::new()
                            .create(true)
                            .append(true)
                            .open(file_name)
                            .map(BufWriter::new)
                            .unwrap();
                    } else {
                        writer = Some(BufWriter::new(
                            OpenOptions::new()
                                .create(true)
                                .append(true)
                                .open(&file_name)
                                .map_err(|e| {
                                    anyhow!("Failed to open file: {} [{}]", file_name.display(), e)
                                })
                                .unwrap(),
                        ));
                    }
                }
                LogMessage::SetCore => {
                    let core_ids = core_affinity::get_core_ids().unwrap();
                    if let Some(last_core_id) = core_ids.first() {
                        core_affinity::set_for_current(*last_core_id);
                    } else {
                        panic!("No core available for logger thread")
                    }
                }
                LogMessage::Close => {
                    if let Some(ref mut writer) = writer {
                        writer.write_all(message_queue.join("").as_bytes()).unwrap();
                        writer.flush().unwrap();
                        let _ = writer.get_mut().sync_all();
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

    pub fn from_usize(level: usize) -> Result<LogLevel> {
        match level {
            0 => Ok(LogLevel::NIL),
            1 => Ok(LogLevel::Error),
            2 => Ok(LogLevel::Warn),
            3 => Ok(LogLevel::Info),
            4 => Ok(LogLevel::Debug),
            5 => Ok(LogLevel::Trace),
            _ => {
                let error = || anyhow!("Invalid log level: {}", level);
                Err(error())
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

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {{
        let msg = format!($($arg)*);
        $crate::log_fn_json!($crate::LogLevel::Error, "not given", text = msg);
    }};
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {{
        let msg = format!($($arg)*);
        $crate::log_fn_json!($crate::LogLevel::Warn, "not given", text = msg);
    }};
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {{
        let msg = format!($($arg)*);
        $crate::log_fn_json!($crate::LogLevel::Info, "not given", text = msg);
    }};
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {{
        let msg = format!($($arg)*);
        $crate::log_fn_json!($crate::LogLevel::Debug, "not given", text = msg);
    }};
}

#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {{
        let msg = format!($($arg)*);
        $crate::log_fn_json!($crate::LogLevel::Trace, "not given", text = msg);
    }};
}

#[macro_export]
macro_rules! log_warn {
    ($topic:expr, $($key:ident=$value:expr),+ $(,)?) => {{
        $crate::log_fn_json!($crate::LogLevel::Warn, $topic, $($key=$value),+);
    }};
    ($topic:expr, $struct:expr) => {{
        $crate::log_fn_json!($crate::LogLevel::Warn, $topic, $struct);
    }};
}

#[macro_export]
macro_rules! log_info {
    ($topic:expr, $($key:ident=$value:expr),+ $(,)?) => {{
        $crate::log_fn_json!($crate::LogLevel::Info, $topic, $($key=$value),+);
    }};
    ($topic:expr, $struct:expr) => {{
        $crate::log_fn_json!($crate::LogLevel::Info, $topic, $struct);
    }};
}

#[macro_export]
macro_rules! log_debug {
    ($topic:expr, $($key:ident=$value:expr),+ $(,)?) => {{
        $crate::log_fn_json!($crate::LogLevel::Debug, $topic, $($key=$value),+);
    }};
    ($topic:expr, $struct:expr) => {{
        $crate::log_fn_json!($crate::LogLevel::Debug, $topic, $struct);
    }};
}

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
macro_rules! log_trace {
    ($topic:expr, $($key:ident=$value:expr),+ $(,)?) => {{
        $crate::log_fn_json!($crate::LogLevel::Trace, $topic, $($key=$value),+);
    }};
    ($topic:expr, $struct:expr) => {{
        $crate::log_fn_json!($crate::LogLevel::Trace, $topic, $struct);
    }};
}

#[macro_export]
macro_rules! log_fn_json {
    ($level:expr, $topic:expr, $($key:ident=$value:expr),+ $(,)?) => {{
        let max_log_level = $crate::LogLevel::from_usize($crate::MAX_LOG_LEVEL.load(std::sync::atomic::Ordering::Relaxed)).unwrap();
        if $level <= max_log_level {
            let timestamp = $crate::get_unix_nano();
            let func = move || {
                let json_obj = $crate::serde_json::json!({
                    $(
                        stringify!($key): $value,
                    )+
                });
                let timezone = $crate::TIMEZONE.load(std::sync::atomic::Ordering::Relaxed);
                let (date, time) = $crate::convert_unix_nano_to_date_and_time(timestamp, timezone);
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
            };

            $crate::LOG_SENDER.try_send($crate::LogMessage::LazyMessage($crate::LazyMessage::new(func))).unwrap();
        }
    }};

    // In case of structs
    ($level:expr, $topic:expr, $struct:expr) => {{
        let current_level = $crate::LogLevel::from_usize($crate::LOG_LEVEL.load(std::sync::atomic::Ordering::Relaxed)).unwrap();
        if $level <= current_level {
            let timestamp = $crate::get_unix_nano();
            let func = move || {
                let json_obj = $crate::serde_json::to_value($struct).unwrap_or_else(|e| {
                    $crate::serde_json::json!({ "error": format!("serialization error: {}", e) })
                });
                let timezone = $crate::TIMEZONE.load(std::sync::atomic::Ordering::Relaxed);
                let (date, time) = $crate::convert_unix_nano_to_date_and_time(timestamp, timezone);
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
            };

            $crate::LOG_SENDER.try_send($crate::LogMessage::LazyMessage($crate::LazyMessage::new(func))).unwrap();
        }
    }};
}

#[macro_export]
macro_rules! flushing_log_info {
    ($topic:expr, $($key:ident=$value:expr),+ $(,)?) => {{
        $crate::flushing_log_fn_json!($crate::LogLevel::Info, $topic, $($key=$value),+);
    }};
    ($topic:expr, $struct:expr) => {{
        $crate::flushing_log_fn_json!($crate::LogLevel::Info, $topic, $struct);
    }};
}

#[macro_export]
macro_rules! flushing_log_debug {
    ($topic:expr, $($key:ident=$value:expr),+ $(,)?) => {{
        $crate::flushing_log_fn_json!($crate::LogLevel::Debug, $topic, $($key=$value),+);
    }};
    ($topic:expr, $struct:expr) => {{
        $crate::flushing_log_fn_json!($crate::LogLevel::Debug, $topic, $struct);
    }};
}

#[macro_export]
macro_rules! flushing_log_error {
    ($topic:expr, $($key:ident=$value:expr),+ $(,)?) => {{
        $crate::flushing_log_fn_json!($crate::LogLevel::Error, $topic, $($key=$value),+);
    }};
    ($topic:expr, $struct:expr) => {{
        $crate::flushing_log_fn_json!($crate::LogLevel::Error, $topic, $struct);
    }};
}

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
        let max_log_level = $crate::LogLevel::from_usize($crate::MAX_LOG_LEVEL.load(std::sync::atomic::Ordering::Relaxed)).unwrap();
        if $level <= max_log_level {
            let timestamp = $crate::get_unix_nano();
            let func = move || {
                let json_obj = $crate::serde_json::json!({
                    $(
                        stringify!($key): $value,
                    )+
                });
                let timezone = $crate::TIMEZONE.load(std::sync::atomic::Ordering::Relaxed);
                let (date, time) = $crate::convert_unix_nano_to_date_and_time(timestamp, timezone);
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
            };

            $crate::LOG_SENDER.try_send($crate::LogMessage::FlushingMessage($crate::LazyMessage::new(func))).unwrap();
        }
    }};

    // In case of structs
    ($level:expr, $topic:expr, $struct:expr) => {{
        let current_level = $crate::LogLevel::from_usize($crate::LOG_LEVEL.load(std::sync::atomic::Ordering::Relaxed)).unwrap();
        if $level <= current_level {
            let timestamp = $crate::get_unix_nano();
            let func = move || {
                let json_obj = $crate::serde_json::to_value($struct).unwrap_or_else(|e| {
                    $crate::serde_json::json!({ "error": format!("serialization error: {}", e) })
                });
                let timezone = $crate::TIMEZONE.load(std::sync::atomic::Ordering::Relaxed);
                let (date, time) = $crate::convert_unix_nano_to_date_and_time(timestamp, timezone);
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
            };

            $crate::LOG_SENDER.try_send($crate::LogMessage::FlushingMessage($crate::LazyMessage::new(func))).unwrap();
        }
    }};
}

pub struct LoggerGuard;

impl Drop for LoggerGuard {
    fn drop(&mut self) {
        log_trace!("logger", message = "LoggerGuard is dropped");
        Logger::finalize();
    }
}

pub struct Logger {
    file_name: Option<String>,
}

impl Logger {
    pub fn finalize() {
        let _ = LOG_SENDER.try_send(LogMessage::Close);
        if let Some(handler) = LOGGER_HANDLER.lock().unwrap().take() {
            let _ = handler.join();
        }
    }

    pub fn initialize() -> Logger {
        let _ = get_unix_nano();
        LOG_MESSAGE_BUFFER_SIZE.store(1_000_000, Ordering::Relaxed);
        LOG_MESSAGE_FLUSH_INTERVAL.store(1_000_000, Ordering::Relaxed);
        Logger { file_name: None }
    }

    pub fn with_file(mut self, file_path: &str, file_name: &str) -> Result<Logger> {
        std::fs::create_dir_all(file_path)?;

        let current_time = chrono::Local::now();
        let file_name = format!(
            "{}/{}-{}.log",
            file_path,
            file_name,
            current_time.format("%Y%m%d-%H%M%S")
        );
        self.file_name = Some(file_name);
        Ok(self)
    }

    pub fn with_console_report(self, console_report: bool) -> Logger {
        CONSOLE_REPORT.store(console_report, Ordering::Relaxed);
        self
    }

    pub fn with_buffer_size(self, size: usize) -> Logger {
        LOG_MESSAGE_BUFFER_SIZE.store(size, Ordering::Relaxed);
        self
    }

    pub fn with_flush_interval(self, interval: u64) -> Logger {
        LOG_MESSAGE_FLUSH_INTERVAL.store(interval, Ordering::Relaxed);
        self
    }

    pub fn with_max_log_level(self, level: LogLevel) -> Logger {
        MAX_LOG_LEVEL.store(level.as_usize(), Ordering::Relaxed);
        self
    }

    pub fn with_timezone(self, timezone: TimeZone) -> Logger {
        TIMEZONE.store(timezone.as_offset_hour(), Ordering::Relaxed);
        self
    }

    pub fn launch(self) -> LoggerGuard {
        let file_name = self.file_name.clone();
        let _ = LOG_SENDER.send(LogMessage::SetCore);
        if let Some(file_name) = file_name {
            let _ = LOG_SENDER.send(LogMessage::SetFile(PathBuf::from(file_name)));
        }
        LoggerGuard {}
    }
}

pub enum LogMessage {
    LazyMessage(LazyMessage),
    FlushingMessage(LazyMessage),
    StaticString(&'static str),
    SetFile(PathBuf),
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;

    #[derive(Clone, Serialize)]
    struct TestStruct {
        a: i32,
        b: f64,
        c: String,
    }

    #[test]
    fn test_logger() -> Result<()> {
        let _guard = Logger::initialize()
            .with_file("logs", "test")?
            .with_console_report(false)
            .with_max_log_level(LogLevel::Info)
            .with_timezone(TimeZone::Local)
            .launch();

        info!("warm up");
        info!("warm up");
        info!("warm up");

        let iteration = 100;

        let start = crate::get_unix_nano();

        for _ in 0..iteration {
            //let test_clone = test_struct.clone();
            info!("test");
        }

        let end = crate::get_unix_nano();

        let elapsed = end - start;
        let elapsed_as_seconds = elapsed as f64 / 1_000_000_000.0;
        let elapsed_average = elapsed as f64 / iteration as f64;

        let message = format!(
            "elapsed: {}s, average: {}ns",
            elapsed_as_seconds, elapsed_average,
        );

        flushing_log_info!(
            "TestDone",
            message = message,  
        );

        println!("elapsed: {}s, average: {}ns", elapsed_as_seconds, elapsed_average);

        assert!(true);
        drop(_guard);
        Ok(())
    }
}