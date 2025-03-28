pub const TRACE: usize = 5;
pub const DEBUG: usize = 4;
pub const INFO: usize = 3;
pub const WARN: usize = 2;
pub const ERROR: usize = 1;
pub const OFF: usize = 0;

pub const MAX_LEVEL: usize = if cfg!(feature = "max-level-trace") {
    TRACE
} else if cfg!(feature = "max-level-debug") {
    DEBUG
} else if cfg!(feature = "max-level-info") {
    INFO
} else if cfg!(feature = "max-level-warn") {
    WARN
} else if cfg!(feature = "max-level-error") {
    ERROR
} else if cfg!(feature = "max-level-off") {
    OFF
} else {
    TRACE
};

#[macro_export]
macro_rules! log_with_level_ct {
    // Case 1: topic, format string argument arguments, and key-value pairs
    ($level:expr, $topic:expr; $fmt:expr, $($arg:expr),* ; $($key:ident = $value:expr),+ $(,)?) => {{
        $crate::log_fn_json_v3!($level, $topic; $fmt, $($arg),*; $($key = $value),+);
    }};

    // Case 2: topic and static string, and key-value pairs
    ($level:expr, $topic:expr; $msg:expr; $($key:ident = $value:expr),+ $(,)?) => {{
        $crate::log_fn_json_v3!($level, $topic; $msg; $($key = $value),+);
    }};

    // Case 3: topic and key-value pairs
    ($level:expr, $topic:expr; $($key:ident = $value:expr),+ $(,)?) => {{
        $crate::log_fn_json_v3!($level, $topic; ""; $($key = $value),+);
    }};
    
    // Case 4: topic and format string with arguments
    ($level:expr, $topic:expr; $fmt:expr, $($arg:expr),* $(,)?) => {{
        $crate::log_fn_json_v3!($level, $topic; $fmt, $($arg),*);
    }};

    // Case 5: topic and static string
    ($level:expr, $topic:expr; $msg:expr) => {{
        $crate::log_fn_json_v3!($level, $topic; $msg);
    }};

    // Case 6: Topic only
    ($level:expr, $topic:expr) => {{
        $crate::log_fn_json_v3!($level, $topic; "");
    }};

    // **Case 7: Single key-value pair without topic**
    ($level:expr, $key:ident = $value:expr $(,)?) => {{
        $crate::log_fn_json_v3!($level, $key = $value);
    }};
    
    // **Case 8: Multiple key-value pairs without topic**
    ($level:expr, $($key:ident = $value:expr),+ $(,)?) => {{
        $crate::log_fn_json_v3!($level, $($key = $value),+);
    }};
}

#[macro_export]
macro_rules! flash_trace_ct {
    // Handle one or more key-value pairs without a topic
    ( $( $key:ident = $value:expr ),+ $(,)? ) => {
        $crate::log_with_level_ct!($crate::compile_time::TRACE, ""; $( $key = $value ),+ );
    };

    // Handle all other cases (e.g., with topic, message, etc.)
    ( $($args:tt)* ) => {
        $crate::log_with_level_ct!($crate::compile_time::TRACE, $($args)* );
    };
}

#[macro_export]
macro_rules! flash_debug_ct {
    // Handle one or more key-value pairs without a topic
    ( $( $key:ident = $value:expr ),+ $(,)? ) => {
        $crate::log_with_level_ct!($crate::compile_time::DEBUG, ""; $( $key = $value ),+ )
    };
    // Handle all other cases (e.g., with topic, message, etc.)
    ( $($args:tt)* ) => {
        $crate::log_with_level_ct!($crate::compile_time::DEBUG, $($args)* )
    };
}

#[macro_export]
macro_rules! flash_info_ct {
    // Handle one or more key-value pairs without a topic
    ( $( $key:ident = $value:expr ),+ $(,)? ) => {
        $crate::log_with_level_ct!($crate::compile_time::INFO, ""; $( $key = $value ),+ );
    };
    // Handle all other cases (e.g., with topic, message, etc.)
    ( $($args:tt)* ) => {
        $crate::log_with_level_ct!($crate::compile_time::INFO, $($args)* );
    };
}

#[macro_export]
macro_rules! flash_warn_ct {
    // Handle one or more key-value pairs without a topic
    ( $( $key:ident = $value:expr ),+ $(,)? ) => {
        $crate::log_with_level_ct!($crate::compile_time::WARN, ""; $( $key = $value ),+ )
    };
    // Handle all other cases (e.g., with topic, message, etc.)
    ( $($args:tt)* ) => {
        $crate::log_with_level_ct!($crate::compile_time::WARN, $($args)* )
    };
}

#[macro_export]
macro_rules! flash_error_ct {
    // Handle one or more key-value pairs without a topic
    ( $( $key:ident = $value:expr ),+ $(,)? ) => {
        $crate::log_with_level_ct!($crate::compile_time::ERROR, ""; $( $key = $value ),+ )
    };
    // Handle all other cases (e.g., with topic, message, etc.)
    ( $($args:tt)* ) => {
        $crate::log_with_level_ct!($crate::compile_time::ERROR, $($args)* )
    };
}

#[inline]
pub fn usize_to_level(level: usize) -> &'static str {
    match level {
        5 => "Trace",
        4 => "Debug",
        3 => "Info",
        2 => "Warn",
        1 => "Error",
        0 => "Off",
        _ => "Unknown",
    }
}

#[macro_export]
macro_rules! log_fn_json_v3 {
    // Case 1: topic, format sring, kv
    ($level:expr, $topic:expr; $fmt:expr, $($arg:expr),*; $($key:ident = $value:expr),+ $(,)?) => {{
        if $level <= $crate::compile_time::MAX_LEVEL {
            $(
                #[allow(non_snake_case)]
                let $key = $value.clone();
            )+

            let func = move || {
                let json_obj = $crate::serde_json::json!({
                    $(
                        stringify!($key): $key,
                    )+
                });
                let unixnano = $crate::get_unix_nano();
                let include_unixnano = $crate::logger::INCLUDE_UNIXNANO.load(std::sync::atomic::Ordering::Relaxed);
                let timezone = $crate::TIMEZONE.load(std::sync::atomic::Ordering::Relaxed);
                let (date, time) = $crate::convert_unix_nano_to_date_and_time(unixnano, timezone);
                
                let json_msg = match include_unixnano {
                    false => $crate::serde_json::json!({
                        "date": date,
                        "time": time,
                        "offset": timezone,
                        "level": $crate::compile_time::usize_to_level($level),
                        "src": format!("{}:{}", file!(), line!()),
                        "topic": $topic,
                        "data": json_obj,
                        "message": format!($fmt, $($arg),*),
                    }),
                    true => $crate::serde_json::json!({
                        "date": date,
                        "time": time,
                        "offset": timezone,
                        "level": $crate::compile_time::usize_to_level($level),
                        "src": format!("{}:{}", file!(), line!()),
                        "topic": $topic,
                        "data": json_obj,
                        "message": format!($fmt, $($arg),*),
                        "unixnano": unixnano,
                    }),
                };

                json_msg.to_string() + "\n"
            };

            $crate::LOG_SENDER.try_send($crate::LogMessage::LazyMessage($crate::LazyMessage::new(func))).unwrap();
        }
    }};
    
    // Case 2: topic, static string, kv
    ($level:expr, $topic:expr; $msg:expr; $($key:ident = $value:expr),+ $(,)?) => {{
        if $level <= $crate::compile_time::MAX_LEVEL {
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
                let unixnano = $crate::get_unix_nano();
                let include_unixnano = $crate::logger::INCLUDE_UNIXNANO.load(std::sync::atomic::Ordering::Relaxed);
                let timezone = $crate::TIMEZONE.load(std::sync::atomic::Ordering::Relaxed);
                let (date, time) = $crate::convert_unix_nano_to_date_and_time(unixnano, timezone);
                let json_msg = match include_unixnano {
                    false => $crate::serde_json::json!({
                        "date": date,
                        "time": time,
                        "offset": timezone,
                        "level": $crate::compile_time::usize_to_level($level),
                        "src": format!("{}:{}", file!(), line!()),
                        "topic": $topic,
                        "data": json_obj,
                        "message": $msg,
                    }),
                    true => $crate::serde_json::json!({
                        "date": date,
                        "time": time,
                        "offset": timezone,
                        "level": $crate::compile_time::usize_to_level($level),
                        "src": format!("{}:{}", file!(), line!()),
                        "topic": $topic,
                        "data": json_obj,
                        "message": $msg,
                        "unixnano": unixnano,
                    }),
                };

                json_msg.to_string() + "\n"
            };

            $crate::LOG_SENDER.try_send($crate::LogMessage::LazyMessage($crate::LazyMessage::new(func))).unwrap();
        }
    }};
    
    // Case 3: topic and formated string
    ($level:expr, $topic:expr; $fmt:expr, $($arg:expr),* $(,)?) => {{
        if $level <= $crate::compile_time::MAX_LEVEL {
            let func = move || {
                let timezone = $crate::TIMEZONE.load(std::sync::atomic::Ordering::Relaxed);
                let unixnano = $crate::get_unix_nano();
                let (date, time) = $crate::convert_unix_nano_to_date_and_time(unixnano, timezone);
                let include_unixnano = $crate::logger::INCLUDE_UNIXNANO.load(std::sync::atomic::Ordering::Relaxed);
                let json_msg = match include_unixnano {
                    false => $crate::serde_json::json!({
                        "date": date,
                        "time": time,
                        "offset": timezone,
                        "level": $crate::compile_time::usize_to_level($level),
                        "src": format!("{}:{}", file!(), line!()),
                        "topic": $topic,
                        "message": format!($fmt, $($arg),*),
                        "data": "",
                    }),
                    true => $crate::serde_json::json!({
                        "date": date,
                        "time": time,
                        "offset": timezone,
                        "level": $crate::compile_time::usize_to_level($level),
                        "src": format!("{}:{}", file!(), line!()),
                        "topic": $topic,
                        "message": format!($fmt, $($arg),*),
                        "data": "",
                        "unixnano": unixnano,
                    }),
                };
                json_msg.to_string() + "\n"
            };
            $crate::LOG_SENDER.try_send($crate::LogMessage::LazyMessage($crate::LazyMessage::new(func))).unwrap();
        }
    }};
    
    // Case 4: topic and static string
    ($level:expr, $topic:expr; $msg:expr $(,)?) => {{
        if $level <= $crate::compile_time::MAX_LEVEL {
            let func = move || {
                let unixnano = $crate::get_unix_nano();
                let include_unixnano = $crate::logger::INCLUDE_UNIXNANO.load(std::sync::atomic::Ordering::Relaxed);
                let timezone = $crate::TIMEZONE.load(std::sync::atomic::Ordering::Relaxed);
                let (date, time) = $crate::convert_unix_nano_to_date_and_time(unixnano, timezone);
                let json_msg = match include_unixnano {
                    false => $crate::serde_json::json!({
                        "date": date,
                        "time": time,
                        "offset": timezone,
                        "level": $crate::compile_time::usize_to_level($level),
                        "src": format!("{}:{}", file!(), line!()),
                        "topic": $topic.to_string(),
                        "message": $msg
                    }),
                    true => $crate::serde_json::json!({
                        "date": date,
                        "time": time,
                        "offset": timezone,
                        "level": $crate::compile_time::usize_to_level($level),
                        "src": format!("{}:{}", file!(), line!()),
                        "topic": $topic.to_string(),
                        "message": $msg,
                        "unixnano": unixnano,
                    }),
                };

                json_msg.to_string() + "\n"
            };

            $crate::LOG_SENDER.try_send($crate::LogMessage::LazyMessage($crate::LazyMessage::new(func))).unwrap();
        }
    }};

    // **Case 7: Single key-value pair without topic**
    ($level:expr, $key:ident = $value:expr) => {{
        if $level <= $crate::compile_time::MAX_LEVEL {
            $(
                #[allow(non_snake_case)]
                let $key = $value.clone();
            )*
            let func = move || {
                let unixnano = $crate::get_unix_nano();
                let include_unixnano = $crate::logger::INCLUDE_UNIXNANO.load(std::sync::atomic::Ordering::Relaxed);
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
                        "level": $crate::compile_time::usize_to_level($level),
                        "src": format!("{}:{}", file!(), line!()),
                        "topic": "",
                        "data": json_obj,
                        "message": "",
                    }),
                    true => $crate::serde_json::json!({
                        "date": date,
                        "time": time,
                        "offset": timezone,
                        "level": $crate::compile_time::usize_to_level($level),
                        "src": format!("{}:{}", file!(), line!()),
                        "topic": "",
                        "data": json_obj,
                        "message": "",
                        "unixnano": unixnano,
                    }),
                };
                json_msg.to_string() + "\n"
            };

            $crate::LOG_SENDER.try_send($crate::LogMessage::LazyMessage($crate::LazyMessage::new(func))).unwrap();
        }
    }};
    
    // **Case 8: Multiple key-value pairs without topic**
    ($level:expr, $($key:ident = $value:expr),+ $(,)?) => {{
        if $level <= $crate::compile_time::MAX_LEVEL {
            $(
                #[allow(non_snake_case)]
                let $key = $value.clone();
            )*
            let func = move || {
                let unixnano = $crate::get_unix_nano();
                let include_unixnano = $crate::logger::INCLUDE_UNIXNANO.load(std::sync::atomic::Ordering::Relaxed);
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
                        "level": $crate::compile_time::usize_to_level($level),
                        "src": format!("{}:{}", file!(), line!()),
                        "data": json_obj,
                        "topic": "",
                        "message": "",
                    }),
                    true => $crate::serde_json::json!({
                        "date": date,
                        "time": time,
                        "offset": timezone,
                        "level": $crate::compile_time::usize_to_level($level),
                        "src": format!("{}:{}", file!(), line!()),
                        "data": json_obj,
                        "topic": "",
                        "message": "",
                        "unixnano": unixnano,
                    }),
                };
                json_msg.to_string() + "\n"
            };

            $crate::LOG_SENDER.try_send($crate::LogMessage::LazyMessage($crate::LazyMessage::new(func))).unwrap();
        }
    }};
}

#[cfg(test)]
mod tests {
    use crate::logger::Logger;
    use crate::logger::LogLevel;
    use crate::logger::TimeZone;
    use anyhow::Result;
    use serde::Serialize;

    #[derive(Debug, Clone, Serialize)]
    pub enum Hello {
        FlashLog,
        World,
    }

    impl std::fmt::Display for Hello {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Hello::FlashLog => write!(f, "FlashLog"),
                Hello::World => write!(f, "World"),
            }
        }
    }

    #[derive(Debug, Clone, Serialize)]
    pub struct TestStruct {
        test: i32,
    }

    #[derive(Debug, Clone, Serialize)]
    pub struct TestStruct2 {
        test: i32,
    }

    #[test]
    fn test_ct() -> Result<()> {
        let _guard = Logger::initialize()
            //.with_file("logs", "message")?
            .with_console_report(true)
            .with_msg_buffer_size(1_000_000)
            .with_msg_flush_interval(1_000_000_000)
            .with_max_log_level(LogLevel::Trace)
            .with_timezone(TimeZone::Local)
            .include_unixnano(false)
            .launch();

        flash_error_ct!(Hello::FlashLog);
        flash_error_ct!(Hello::World);
        flash_error_ct!("Hello");
        flash_error_ct!("Hello"; "FlashLog");
        flash_error_ct!("Hello"; "FlashLog"; version = "0.1.0");
        flash_error_ct!("Hello"; "FlashLog"; version = "0.1.0", author = "John Doe");
        flash_error_ct!(version = "0.1.0");
        flash_error_ct!(version = "0.1.0", author = "John Doe");
        flash_error_ct!("topic1"; "message {} {}", 1, 2);
        flash_error_ct!("topic2"; "message {} {}", 1, 2; struct_info = 1, struct_info2 = 2);
        flash_error_ct!("topic2"; "message {} {}", 1, 2; Hello = Hello::FlashLog);
        let test_info = TestStruct { test: 1 };
        let test_info2 = TestStruct2 { test: 2 };
        flash_error_ct!("topic2"; "message {} {}", 1, 2; TestStruct = test_info, TestStruct2 = test_info2);
        println!("{:?}", test_info); // still alive
        crate::flush!(); // this flushes regardless of the buffer size and flush interval

        flash_warn_ct!(Hello::World);
        flash_warn_ct!("Hello");
        flash_warn_ct!("Hello"; "FlashLog");
        flash_warn_ct!("Hello"; "FlashLog"; version = "0.1.0");
        flash_warn_ct!("Hello"; "FlashLog"; version = "0.1.0", author = "John Doe");
        flash_warn_ct!(version = "0.1.0");
        flash_warn_ct!(version = "0.1.0", author = "John Doe");
        flash_warn_ct!("topic1"; "message {} {}", 1, 2);
        flash_warn_ct!("topic2"; "message {} {}", 1, 2; struct_info = 1, struct_info2 = 2);
        flash_warn_ct!("topic2"; "message {} {}", 1, 2; Hello = Hello::FlashLog);
        let test_info = TestStruct { test: 1 };
        flash_warn_ct!("topic2"; "message {} {}", 1, 2; TestStruct = test_info);
        println!("{:?}", test_info);

        crate::flush!(); // this flushes regardless of the buffer size and flush interval

        flash_info_ct!(Hello::World);
        flash_info_ct!(Hello::FlashLog);
        flash_info_ct!("Hello"); 
        flash_info_ct!("Hello"; "FlashLog");
        flash_info_ct!("Hello"; "FlashLog"; version = "0.1.0");
        flash_info_ct!("Hello"; "FlashLog"; version = "0.1.0", author = "John Doe");
        flash_info_ct!(version = "0.1.0");
        flash_info_ct!(version = "0.1.0", author = "John Doe");
        flash_info_ct!("topic1"; "message {} {}", 1, 2);
        flash_info_ct!("topic2"; "message {} {}", 1, 2; struct_info = 1, struct_info2 = 2);
        flash_info_ct!("topic2"; "message {} {}", 1, 2; Hello = Hello::FlashLog);
        let test_info = TestStruct { test: 1 };
        flash_info_ct!("topic2"; "message {} {}", 1, 2; TestStruct = test_info);
        println!("{:?}", test_info);

        crate::flush!(); // this flushes regardless of the buffer size and flush interval

        flash_debug_ct!(Hello::World);
        flash_debug_ct!("Hello");
        flash_debug_ct!("Hello"; "FlashLog");
        flash_debug_ct!("Hello"; "FlashLog"; version = "0.1.0");
        flash_debug_ct!("Hello"; "FlashLog"; version = "0.1.0", author = "John Doe");
        flash_debug_ct!(version = "0.1.0");
        flash_debug_ct!(version = "0.1.0", author = "John Doe");
        flash_debug_ct!("topic1"; "message {} {}", 1, 2);
        flash_debug_ct!("topic2"; "message {} {}", 1, 2; struct_info = 1, struct_info2 = 2);
        flash_debug_ct!("topic2"; "message {} {}", 1, 2; Hello = Hello::FlashLog);
        let test_info = TestStruct { test: 1 };
        flash_debug_ct!("topic2"; "message {} {}", 1, 2; TestStruct = test_info);
        println!("{:?}", test_info);

        crate::flush!(); // this flushes regardless of the buffer size and flush interval

        flash_trace_ct!(Hello::World);
        flash_trace_ct!("Hello");
        flash_trace_ct!("Hello"; "FlashLog");
        flash_trace_ct!("Hello"; "FlashLog"; version = "0.1.0");
        flash_trace_ct!("Hello"; "FlashLog"; version = "0.1.0", author = "John Doe");
        flash_trace_ct!(version = "0.1.0");
        flash_trace_ct!(version = "0.1.0", author = "John Doe");
        flash_trace_ct!("topic1"; "message {} {}", 1, 2);
        flash_trace_ct!("topic2"; "message {} {}", 1, 2; struct_info = 1, struct_info2 = 2);
        flash_trace_ct!("topic2"; "message {} {}", 1, 2; Hello = Hello::FlashLog);
        let test_info = TestStruct { test: 1 };
        flash_trace_ct!("topic2"; "message {} {}", 1, 2; TestStruct = test_info);
        println!("{:?}", test_info);

        crate::flush!();

        assert!(true);

        Ok(())
    }
}