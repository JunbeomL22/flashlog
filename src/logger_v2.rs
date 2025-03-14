#[macro_export]
macro_rules! flush {
    () => {{
        $crate::LOG_SENDER.try_send($crate::LogMessage::Flush).unwrap();
    }};
}

#[macro_export]
macro_rules! log_with_level {
    // Case 1: topic, format string argument arguments, and key-value pairs
    ($level:expr, $topic:expr; $fmt:expr, $($arg:expr),* ; $($key:ident = $value:expr),+ $(,)?) => {{
        $crate::log_fn_json_v2!($level, $topic; $fmt, $($arg),*; $($key = $value),+);
    }};

    // Case 2: topic and static string, and key-value pairs
    ($level:expr, $topic:expr; $msg:expr; $($key:ident = $value:expr),+ $(,)?) => {{
        $crate::log_fn_json_v2!($level, $topic; $msg; $($key = $value),+);
    }};

    // Case 3: topic and key-value pairs
    ($level:expr, $topic:expr; $($key:ident = $value:expr),+ $(,)?) => {{
        $crate::log_fn_json_v2!($level, $topic; ""; $($key = $value),+);
    }};
    
    // Case 4: topic and format string with arguments
    ($level:expr, $topic:expr; $fmt:expr, $($arg:expr),* $(,)?) => {{
        $crate::log_fn_json_v2!($level, $topic; $fmt, $($arg),*);
    }};

    // Case 5: topic and static string
    ($level:expr, $topic:expr; $msg:expr) => {{
        $crate::log_fn_json_v2!($level, $topic; $msg);
    }};

    // Case 6: Topic only
    ($level:expr, $topic:expr) => {{
        $crate::log_fn_json_v2!($level, $topic; "");
    }};

    // **Case 7: Single key-value pair without topic**
    ($level:expr, $key:ident = $value:expr $(,)?) => {{
        $crate::log_fn_json_v2!($level, $key = $value);
    }};
    
    // **Case 8: Multiple key-value pairs without topic**
    ($level:expr, $($key:ident = $value:expr),+ $(,)?) => {{
        $crate::log_fn_json_v2!($level, $($key = $value),+);
    }};
}

#[macro_export]
macro_rules! flash_trace {
    // Handle one or more key-value pairs without a topic
    ( $( $key:ident = $value:expr ),+ $(,)? ) => {
        #[cfg(all(
            debug_assertions,
            any(feature = "max_level_trace")
        ))]
        $crate::log_with_level!($crate::LogLevel::Trace, ""; $( $key = $value ),+ );

        #[cfg(all(
            not(debug_assertions),
            any(feature = "release_max_level_trace")
        ))]
        $crate::log_with_level!($crate::LogLevel::Trace, ""; $( $key = $value ),+ );
    };

    // Handle all other cases (e.g., with topic, message, etc.)
    ( $($args:tt)* ) => {
        #[cfg(all(
            debug_assertions,
            any(feature = "max_level_trace")
        ))]
        $crate::log_with_level!($crate::LogLevel::Trace, $($args)* );

        #[cfg(all(
            not(debug_assertions),
            any(feature = "release_max_level_trace")
        ))]
        $crate::log_with_level!($crate::LogLevel::Trace, $($args)* );
    };
}

#[macro_export]
macro_rules! flash_debug {
    // Handle one or more key-value pairs without a topic
    ( $( $key:ident = $value:expr ),+ $(,)? ) => {
        #[cfg(all(
            debug_assertions,
            any(
                feature = "max_level_debug",
                feature = "max_level_trace",
            )
        ))]
        $crate::log_with_level!($crate::LogLevel::Debug, ""; $( $key = $value ),+ );

        #[cfg(all(
            not(debug_assertions),
            any(
                feature = "release_max_level_debug",
                feature = "release_max_level_trace",
            )
        ))]
        $crate::log_with_level!($crate::LogLevel::Debug, ""; $( $key = $value ),+ )
    };
    // Handle all other cases (e.g., with topic, message, etc.)
    ( $($args:tt)* ) => {
        #[cfg(all(
            debug_assertions,
            any(
                feature = "max_level_debug",
                feature = "max_level_trace",
            )
        ))]
        $crate::log_with_level!($crate::LogLevel::Debug, $($args)* );

        #[cfg(all(
            not(debug_assertions),
            any(
                feature = "release_max_level_debug",
                feature = "release_max_level_trace",
            )
        ))]
        $crate::log_with_level!($crate::LogLevel::Debug, $($args)* )
    };
}
#[macro_export]
macro_rules! flash_info {
    // Handle one or more key-value pairs without a topic
    ( $( $key:ident = $value:expr ),+ $(,)? ) => {
        #[cfg(all(
            debug_assertions,
            any(
                feature = "max_level_info",
                feature = "max_level_debug",
                feature = "max_level_trace",
            )
        ))]
        $crate::log_with_level!($crate::LogLevel::Info, ""; $( $key = $value ),+ );

        #[cfg(all(
            not(debug_assertions),
            any(
                feature = "release_max_level_info",
                feature = "release_max_level_debug",
                feature = "release_max_level_trace",
            )
        ))]
        $crate::log_with_level!($crate::LogLevel::Info, ""; $( $key = $value ),+ )
    };
    // Handle all other cases (e.g., with topic, message, etc.)
    ( $($args:tt)* ) => {
        #[cfg(all(
            debug_assertions,
            any(
                feature = "max_level_info",
                feature = "max_level_debug",
                feature = "max_level_trace",
            )
        ))]
        $crate::log_with_level!($crate::LogLevel::Info, $($args)* );

        #[cfg(all(
            not(debug_assertions),
            any(
                feature = "release_max_level_info",
                feature = "release_max_level_debug",
                feature = "release_max_level_trace",
            )
        ))]
        $crate::log_with_level!($crate::LogLevel::Info, $($args)* )
    };
}

#[macro_export]
macro_rules! flash_warn {
    // Handle one or more key-value pairs without a topic
    ( $( $key:ident = $value:expr ),+ $(,)? ) => {
        #[cfg(all(
            debug_assertions,
            any(
                feature = "max_level_warn",
                feature = "max_level_info",
                feature = "max_level_debug",
                feature = "max_level_trace",
            )
        ))]
        $crate::log_with_level!($crate::LogLevel::Warn, ""; $( $key = $value ),+ );

        #[cfg(all(
            not(debug_assertions),
            any(
                feature = "release_max_level_warn",
                feature = "release_max_level_info",
                feature = "release_max_level_debug",
                feature = "release_max_level_trace",
            )
        ))]
        $crate::log_with_level!($crate::LogLevel::Warn, ""; $( $key = $value ),+ )
    };
    // Handle all other cases (e.g., with topic, message, etc.)
    ( $($args:tt)* ) => {
        #[cfg(all(
            debug_assertions,
            any(
                feature = "max_level_warn",
                feature = "max_level_info",
                feature = "max_level_debug",
                feature = "max_level_trace",
            )
        ))]
        $crate::log_with_level!($crate::LogLevel::Warn, $($args)* );

        #[cfg(all(
            not(debug_assertions),
            any(
                feature = "release_max_level_warn",
                feature = "release_max_level_info",
                feature = "release_max_level_debug",
                feature = "release_max_level_trace",
            )
        ))]
        $crate::log_with_level!($crate::LogLevel::Warn, $($args)* )
    };
}

#[macro_export]
macro_rules! flash_error {
    // Handle one or more key-value pairs without a topic
    ( $( $key:ident = $value:expr ),+ $(,)? ) => {
        #[cfg(all(
            debug_assertions,
            any(
                feature = "max_level_error",
                feature = "max_level_warn",
                feature = "max_level_info",
                feature = "max_level_debug",
                feature = "max_level_trace",
            )
        ))]
        $crate::log_with_level!($crate::LogLevel::Error, ""; $( $key = $value ),+ );

        #[cfg(all(
            not(debug_assertions),
            any(
                feature = "release_max_level_error",
                feature = "release_max_level_warn",
                feature = "release_max_level_info",
                feature = "release_max_level_debug",
                feature = "release_max_level_trace",
            )
        ))]
        $crate::log_with_level!($crate::LogLevel::Error, ""; $( $key = $value ),+ )
    };
    // Handle all other cases (e.g., with topic, message, etc.)
    ( $($args:tt)* ) => {
        #[cfg(all(
            debug_assertions,
            any(
                feature = "max_level_error",
                feature = "max_level_warn",
                feature = "max_level_info",
                feature = "max_level_debug",
                feature = "max_level_trace",
            )
        ))]
        $crate::log_with_level!($crate::LogLevel::Error, $($args)* );

        #[cfg(all(
            not(debug_assertions),
            any(
                feature = "release_max_level_error",
                feature = "release_max_level_warn",
                feature = "release_max_level_info",
                feature = "release_max_level_debug",
                feature = "release_max_level_trace",
            )
        ))]
        $crate::log_with_level!($crate::LogLevel::Error, $($args)* )
    };
}

#[macro_export]
macro_rules! log_fn_json_v2 {
    // Case 1: topic, format sring, kv
    ($level:expr, $topic:expr; $fmt:expr, $($arg:expr),*; $($key:ident = $value:expr),+ $(,)?) => {{
        let current_level = $crate::LogLevel::from_usize($crate::MAX_LOG_LEVEL.load(std::sync::atomic::Ordering::Relaxed)).unwrap();
        if $level <= current_level {
            let unixnano = $crate::get_unix_nano();
            let include_unixnano = $crate::logger::INCLUDE_UNIXNANO.load(std::sync::atomic::Ordering::Relaxed);
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
                        "message": format!($fmt, $($arg),*),
                    }),
                    true => $crate::serde_json::json!({
                        "date": date,
                        "time": time,
                        "offset": timezone,
                        "level": $level.to_string(),
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
        let current_level = $crate::LogLevel::from_usize($crate::MAX_LOG_LEVEL.load(std::sync::atomic::Ordering::Relaxed)).unwrap();
        if $level <= current_level {
            let unixnano = $crate::get_unix_nano();
            let include_unixnano = $crate::logger::INCLUDE_UNIXNANO.load(std::sync::atomic::Ordering::Relaxed);
            
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
                        "message": $msg,
                    }),
                    true => $crate::serde_json::json!({
                        "date": date,
                        "time": time,
                        "offset": timezone,
                        "level": $level.to_string(),
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
        let current_level = $crate::LogLevel::from_usize($crate::MAX_LOG_LEVEL.load(std::sync::atomic::Ordering::Relaxed)).unwrap();
        if $level <= current_level {
            let unixnano = $crate::get_unix_nano();
            let include_unixnano = $crate::logger::INCLUDE_UNIXNANO.load(std::sync::atomic::Ordering::Relaxed);
            let func = move || {
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
                        "message": format!($fmt, $($arg),*),
                        "data": "",
                    }),
                    true => $crate::serde_json::json!({
                        "date": date,
                        "time": time,
                        "offset": timezone,
                        "level": $level.to_string(),
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
        let current_level = $crate::LogLevel::from_usize($crate::MAX_LOG_LEVEL.load(std::sync::atomic::Ordering::Relaxed)).unwrap();
        if $level <= current_level {
            let unixnano = $crate::get_unix_nano();
            let include_unixnano = $crate::logger::INCLUDE_UNIXNANO.load(std::sync::atomic::Ordering::Relaxed);
            let func = move || {
                let timezone = $crate::TIMEZONE.load(std::sync::atomic::Ordering::Relaxed);
                let (date, time) = $crate::convert_unix_nano_to_date_and_time(unixnano, timezone);
                let json_msg = match include_unixnano {
                    false => $crate::serde_json::json!({
                        "date": date,
                        "time": time,
                        "offset": timezone,
                        "level": $level.to_string(),
                        "src": format!("{}:{}", file!(), line!()),
                        "topic": $topic.to_string(),
                        "message": $msg
                    }),
                    true => $crate::serde_json::json!({
                        "date": date,
                        "time": time,
                        "offset": timezone,
                        "level": $level.to_string(),
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
        let current_level = $crate::LogLevel::from_usize($crate::MAX_LOG_LEVEL.load(std::sync::atomic::Ordering::Relaxed)).unwrap();
        if $level <= current_level {
            let unixnano = $crate::get_unix_nano();
            let include_unixnano = $crate::logger::INCLUDE_UNIXNANO.load(std::sync::atomic::Ordering::Relaxed);
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
                let (date, time) = $crate::convert_unix_nano_to_date_and_time(timestamp, timezone);
                let json_msg = match include_unixnano {
                    false => $crate::serde_json::json!({
                        "date": date,
                        "time": time,
                        "offset": timezone,
                        "level": $level.to_string(),
                        "src": format!("{}:{}", file!(), line!()),
                        "topic": "",
                        "data": json_obj,
                        "message": "",
                    }),
                    true => $crate::serde_json::json!({
                        "date": date,
                        "time": time,
                        "offset": timezone,
                        "level": $level.to_string(),
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
        let current_level = $crate::LogLevel::from_usize($crate::MAX_LOG_LEVEL.load(std::sync::atomic::Ordering::Relaxed)).unwrap();
        if $level <= current_level {
            let unixnano = $crate::get_unix_nano();
            let include_unixnano = $crate::logger::INCLUDE_UNIXNANO.load(std::sync::atomic::Ordering::Relaxed);
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
                let (date, time) = $crate::convert_unix_nano_to_date_and_time(timestamp, timezone);
                let json_msg = match unixnano {
                    false => $crate::serde_json::json!({
                        "date": date,
                        "time": time,
                        "offset": timezone,
                        "level": $level.to_string(),
                        "src": format!("{}:{}", file!(), line!()),
                        "data": json_obj,
                        "topic": "",
                        "message": "",
                    }),
                    true => $crate::serde_json::json!({
                        "date": date,
                        "time": time,
                        "offset": timezone,
                        "level": $level.to_string(),
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
    use anyhow::Result;
    use serde::Serialize;
    use crate::{Logger, LogLevel};
    use crate::TimeZone;

    #[derive(Debug, Clone, Serialize)]
    pub enum Hello {
        World,
        FlashLog,
    }

    impl std::fmt::Display for Hello {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Hello::World => write!(f, "World"),
                Hello::FlashLog => write!(f, "FlashLog"),
            }
        }
    }

    #[derive(Debug, Serialize, Clone)]
    pub struct TestStruct {
        pub test: i32,
    }

    #[derive(Debug, Serialize, Clone)]
    pub struct TestStruct2 {
        pub test: i32,
    }

    #[test]
    fn test_logger() -> Result<()> {
        let _guard = Logger::initialize()
            //.with_file("logs", "message")?
            .with_console_report(true)
            .with_msg_buffer_size(1_000_000)
            .with_msg_flush_interval(1_000_000_000)
            .with_max_log_level(LogLevel::Trace)
            .with_timezone(TimeZone::Local)
            .include_unixnano(false)
            .launch();

        flash_error!(Hello::FlashLog);
        flash_error!(Hello::World);
        flash_error!("Hello");
        flash_error!("Hello"; "FlashLog");
        flash_error!("Hello"; "FlashLog"; version = "0.1.0");
        flash_error!("Hello"; "FlashLog"; version = "0.1.0", author = "John Doe");
        flash_error!(version = "0.1.0");
        flash_error!(version = "0.1.0", author = "John Doe");
        flash_error!("topic1"; "message {} {}", 1, 2);
        flash_error!("topic2"; "message {} {}", 1, 2; struct_info = 1, struct_info2 = 2);
        flash_error!("topic2"; "message {} {}", 1, 2; Hello = Hello::FlashLog);
        let test_info = TestStruct { test: 1 };
        let test_info2 = TestStruct2 { test: 2 };
        flash_error!("topic2"; "message {} {}", 1, 2; TestStruct = test_info, TestStruct2 = test_info2);
        println!("{:?}", test_info); // still alive
        flush!(); // this flushes regardless of the buffer size and flush interval

        flash_warn!(Hello::World);
        flash_warn!("Hello");
        flash_warn!("Hello"; "FlashLog");
        flash_warn!("Hello"; "FlashLog"; version = "0.1.0");
        flash_warn!("Hello"; "FlashLog"; version = "0.1.0", author = "John Doe");
        flash_warn!(version = "0.1.0");
        flash_warn!(version = "0.1.0", author = "John Doe");
        flash_warn!("topic1"; "message {} {}", 1, 2);
        flash_warn!("topic2"; "message {} {}", 1, 2; struct_info = 1, struct_info2 = 2);
        flash_warn!("topic2"; "message {} {}", 1, 2; Hello = Hello::FlashLog);
        let test_info = TestStruct { test: 1 };
        flash_warn!("topic2"; "message {} {}", 1, 2; TestStruct = test_info);
        println!("{:?}", test_info);

        flush!(); // this flushes regardless of the buffer size and flush interval

        flash_info!(Hello::World);
        flash_info!(Hello::FlashLog);
        flash_info!("Hello"); 
        flash_info!("Hello"; "FlashLog");
        flash_info!("Hello"; "FlashLog"; version = "0.1.0");
        flash_info!("Hello"; "FlashLog"; version = "0.1.0", author = "John Doe");
        flash_info!(version = "0.1.0");
        flash_info!(version = "0.1.0", author = "John Doe");
        flash_info!("topic1"; "message {} {}", 1, 2);
        flash_info!("topic2"; "message {} {}", 1, 2; struct_info = 1, struct_info2 = 2);
        flash_info!("topic2"; "message {} {}", 1, 2; Hello = Hello::FlashLog);
        let test_info = TestStruct { test: 1 };
        flash_info!("topic2"; "message {} {}", 1, 2; TestStruct = test_info);
        println!("{:?}", test_info);

        flush!(); // this flushes regardless of the buffer size and flush interval

        flash_debug!(Hello::World);
        flash_debug!("Hello");
        flash_debug!("Hello"; "FlashLog");
        flash_debug!("Hello"; "FlashLog"; version = "0.1.0");
        flash_debug!("Hello"; "FlashLog"; version = "0.1.0", author = "John Doe");
        flash_debug!(version = "0.1.0");
        flash_debug!(version = "0.1.0", author = "John Doe");
        flash_debug!("topic1"; "message {} {}", 1, 2);
        flash_debug!("topic2"; "message {} {}", 1, 2; struct_info = 1, struct_info2 = 2);
        flash_debug!("topic2"; "message {} {}", 1, 2; Hello = Hello::FlashLog);
        let test_info = TestStruct { test: 1 };
        flash_debug!("topic2"; "message {} {}", 1, 2; TestStruct = test_info);
        println!("{:?}", test_info);

        flush!(); // this flushes regardless of the buffer size and flush interval

        flash_trace!(Hello::World);
        flash_trace!("Hello");
        flash_trace!("Hello"; "FlashLog");
        flash_trace!("Hello"; "FlashLog"; version = "0.1.0");
        flash_trace!("Hello"; "FlashLog"; version = "0.1.0", author = "John Doe");
        flash_trace!(version = "0.1.0");
        flash_trace!(version = "0.1.0", author = "John Doe");
        flash_trace!("topic1"; "message {} {}", 1, 2);
        flash_trace!("topic2"; "message {} {}", 1, 2; struct_info = 1, struct_info2 = 2);
        flash_trace!("topic2"; "message {} {}", 1, 2; Hello = Hello::FlashLog);
        let test_info = TestStruct { test: 1 };
        flash_trace!("topic2"; "message {} {}", 1, 2; TestStruct = test_info);
        println!("{:?}", test_info);

        crate::flush!();

        assert!(true);

        Ok(())
    }
}