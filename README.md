# FlashLog

A blazingly fast Rust logging library with lazy evaluation.

[![Crates.io](https://img.shields.io/crates/v/flashlog.svg)](https://crates.io/crates/flashlog)
[![Documentation](https://docs.rs/flashlog/badge.svg)](https://docs.rs/flashlog)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Important Changes in Version 0.3.0

1. **Cloning in New Macros**: In `flash_xxx_ct!` macros, cloning happens automatically inside the macro. Users don't need to clone data outside the macro, but be aware that implicit cloning still occurs in the worker thread.

2. **Timestamp Generation**: Timestamps are now generated in the logger thread rather than the worker thread.

3. **Compile-Time Feature Options**: New compile-time feature options have been added, compatible with the `flash_xxx_ct!` macros. The `flash_xxx!` macros are now deprecated in favor of the new compile-time filtered macros.

## Features

- **Lazy Evaluation**: Most evaluations are performed in the logger thread, resulting in exceptional performance.
- **Lazy String**: String interpolation in `flash_xxx!` macros is inherently lazy.
- **JSON Output**: Log messages are printed in `JSON` format for easy parsing and analysis.
- **Customizable**: Flexible configuration options for file output, console reporting, buffer size, and more.
- **Timezone Support**: Ability to set local or custom timezones for log timestamps.

## Quick Start

Add FlashLog to your `Cargo.toml`:

```toml
[dependencies]
flashlog = {version = "0.3", features = ["max-level-info"]}
```

The compile time feature `max-level-info` is optional and can be omitted. It sets the maximum log level to `Info` at compile time.
`flash_xxx_ct!` macros are filtered only by the compile-time feature not using the runtime configuration.

Available compile-time features are:
- `max-level-off`
- `max-level-error`
- `max-level-warn`
- `max-level-info`
- `max-level-debug`
- `max-level-trace`

Basic usage example:

```rust
use flashlog::{Logger, LogLevel, flash_info_ct};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let logger = Logger::initialize()
        .with_file("logs", "message")?
        .with_max_log_level(LogLevel::Info)
        .launch();

    flash_info_ct!("Hello, FlashLog!");

    Ok(())
}
```

## Advanced Usage

### Logging Structs

FlashLog can easily log custom structs:

```rust
use serde::{Deserialize, Serialize};
use flashlog::{Logger, LogLevel, flash_info_ct};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LogStruct {
    data: [u64; 10],
}

impl Default for LogStruct {
    fn default() -> Self {
        LogStruct { data: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10] }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let logger = Logger::initialize()
        .with_file("logs", "message")?
        .with_max_log_level(LogLevel::Info)
        .launch();

    let log_struct = LogStruct::default();
    flash_info_ct!("Log message"; log_struct = log_struct); 

    Ok(())
}
```

## Configuration and Log Interfaces

Topic and message are optional and separated by a semicolon. In addition, messages can be added with key-value pairs

```rust
use flashlog::{flash_info_ct, flush, Logger, LogLevel, TimeZone, RollingPeriod};

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _logger = Logger::initialize()
        .with_file("logs", "message")?               // Log to a file called "message" in the "logs" directory
        .with_roll_period(RollingPeriod::Daily)?     // Log file is rolled in daily basis
        .with_max_roll_files(10)?                    // Ten old file will remain. if compress is true, there will remain 10 gz file (older log) as well 
        .with_compress(false)?                       // compress old log file
        .with_console_report(true)                   // Enable logging to the console
        .with_msg_flush_interval(2_000_000_000)      // Flush every 2 seconds
        .with_msg_buffer_size(10)                    // Flush when there are 10 more log messages
        //.with_max_log_level(LogLevel::Debug)       // DEPRECATED! compile-time feature flags are recommended
        .with_timezone(TimeZone::Local)              // Use local timezone for timestamps
        .include_unixnano(true)                      // Include unixnano in the log message
        .launch();

    flash_info_ct!(Hello::FlashLog);
    // {"date":"20240915","level":"Info","message":"","offset":9,"src":"src\\logger_v2.rs:346","time":"20:34:30.684:921:877","topic":"World", "unixnano": 1741046422247135000}
    flash_info_ct!(Hello::World);
    // {"date":"20240915","level":"Info","message":"","offset":9,"src":"src\\logger_v2.rs:347","time":"20:34:30.684:922:238","topic":"FlashLog", "unixnano": 1741046422247135000}
    flash_info_ct!("Hello");
    // {"date":"20240915","level":"Info","message":"","offset":9,"src":"src\\logger_v2.rs:348","time":"20:34:30.684:922:488","topic":"Hello", "unixnano": 1741046422247135000}
    flash_info_ct!("Hello"; "FlashLog");
    // {"date":"20240915","level":"Info","message":"FlashLog","offset":9,"src":"src\\logger_v2.rs:349","time":"20:34:30.684:922:739","topic":"Hello", "unixnano": 1741046422247135000}
    flash_info_ct!("Hello"; "FlashLog"; version = "0.1.0");
    // {"data":{"version":"0.1.0"},"date":"20240915","level":"Info","message":"FlashLog","offset":9,"src":"src\\logger_v2.rs:350","time":"20:34:30.684:924:813","topic":"Hello", "unixnano": 1741046422247135000}
    flash_info_ct!("Hello"; "FlashLog"; version = "0.1.0", author = "John Doe");
    // {"data":{"author":"John Doe","version":"0.1.0"},"date":"20240915","level":"Info","message":"FlashLog","offset":9,"src":"src\\logger_v2.rs:351","time":"20:34:30.684:925:143","topic":"Hello", "unixnano": 1741046422247135000}
    flash_info_ct!(version = "0.1.0");
    // {"data":{"version":"0.1.0"},"date":"20240915","level":"Info","message":"","offset":9,"src":"src\\logger_v2.rs:352","time":"20:34:30.684:925:394","topic":"", "unixnano": 1741046422247135000}
    flash_info_ct!(version = "0.1.0", author = "John Doe");
    // {"data":{"author":"John Doe","version":"0.1.0"},"date":"20240915","level":"Info","message":"","offset":9,"src":"src\\logger_v2.rs:353","time":"20:34:30.684:925:654","topic":"", "unixnano": 1741046422247135000}
    flash_info_ct!("topic1"; "message {} {}", 1, 2);
    // {"data":"","date":"20240915","level":"Info","message":"message 1 2","offset":9,"src":"src\\logger_v2.rs:354","time":"20:34:30.684:925:955","topic":"topic1", "unixnano": 1741046422247135000}
    flash_info_ct!("topic2"; "message {} {}", 1, 2; struct_info = 1, struct_info2 = 2);
    // {"data":{"struct_info":1,"struct_info2":2},"date":"20240915","level":"Info","message":"message 1 2","offset":9,"src":"src\\logger_v2.rs:355","time":"20:34:30.684:926:847","topic":"topic2", "unixnano": 1741046422247135000}
    flush!(); // this flushes regardless of the buffer size and flush interval

    Ok(())
}
```

# Benchmark
## Test configurations
Print 500,000 logs. Perform the test 5 times. Before each test, sleep for 2 seconds, then print a warm-up message, and then continuously print 500,000 messages. Test has been done on two types: i32 and

```Rust
struct LogStruct {
    data: [u64; 10],
}
```
## message examples for the struct
### flashlog: [test-file](./examples/flashlog/src/main.rs)
```Json
{"data":{"log_struct":{"data":[1,2,3,4,5,6,7,8,9,10]}},"date":"20240915","level":"Info","message":"","offset":9,"src":"src/main.rs:52","time":"20:52:02.998:044:806","topic":"Bench"}
```

### ftlog: [test-file](./examples/ftlog/src/main.rs)
```
2024-08-29 09:39:13.503+09 0ms INFO main [src/main.rs:57] Log message: LogStruct { data: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10] }
```

### fast_log: [test-file](./examples/fast_log/src/main.rs)
```
2024-08-29 10:31:16.7598955 [INFO] Log message: LogStruct { data: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10] }
```

### slog: [test-file](./examples/slog/src/main.rs)
```
Aug 29 01:53:20.725 INFO Log message: LogStruct { data: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10] }
```

### fern: [test-file](./examples/fern/src/main.rs)
```
[2024-08-29T05:59:56.608510100Z INFO example_fern] Log message: LogStruct { data: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10] }
```

### tracing: [test-file](./examples/tracing/src/main.rs)
```
2024-08-30T01:17:18.997070Z  INFO example_tracing: Log message: LogStruct { data: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10] }
```
## Performance comparisons

### Test machine: Ryzen 7 7700, 3.8 Ghz
| Logger    | i32           | 80 byte struct  |
| --------- | ------------- | --------------- |
| flashlog  | 30 ns         | 40 ns           |
| ftlog     | 260 ns        | 480 ns          |
| fast_log  | 410 ns        | 358 ns          |
| slog      | 250 ns        | 452 ns          |
| fern      | 3,813 ns      | 3,962 ns        |
| tracing   | 4,003 ns      | 4,258 ns        | 

### Test machine: i5-14400F, 2.5Ghz

| Logger    | i32           | 80 byte struct  |
| --------- | ------------- | --------------- |
| flashlog  | 64 ns         | 89 ns           |
| ftlog     | 323 ns        | 581 ns          |
| fast_log  | 500 ns        | 500 ns          |
| slog      | 324 ns        | 604 ns          |
| fern      | 4,732 ns      | 5,714 ns        |
| tracing   | 5,177 ns      | 6,190 ns        |


## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
## Authors

- [Junbeom Lee](https://github.com/JunbeomL22)
- [Youngjin Park](https://github.com/youngjin-create)

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.