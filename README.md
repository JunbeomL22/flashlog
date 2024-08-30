# FlashLog

A blazingly fast Rust logging library with lazy evaluation.

[![Crates.io](https://img.shields.io/crates/v/flashlog.svg)](https://crates.io/crates/flashlog)
[![Documentation](https://docs.rs/flashlog/badge.svg)](https://docs.rs/flashlog)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Features

- **Lazy Evaluation**: Most evaluations are performed in the logger thread, resulting in exceptional performance.
- **JSON Output**: Log messages are printed in `JSON` format for easy parsing and analysis.
- **LazyString**: Provides `LazyString` for optimized string interpolation.
- **Customizable**: Flexible configuration options for file output, console reporting, buffer size, and more.
- **Timezone Support**: Ability to set local or custom timezones for log timestamps.

## Quick Start

Add FlashLog to your `Cargo.toml`:

```toml
[dependencies]
flashlog = "0.1"
```

Basic usage example:

```rust
use flashlog::{Logger, LogLevel, info};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let logger = Logger::initialize()
        .with_file("logs", "message")?
        .with_max_log_level(LogLevel::Info)
        .launch();

    info!("Hello, FlashLog!");

    Ok(())
}
```

## Advanced Usage

### Logging Structs

FlashLog can easily log custom structs:

```rust
use serde::{Deserialize, Serialize};
use flashlog::{Logger, LogLevel, log_info};

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
    log_info!("Log message", log_struct = log_struct);

    Ok(())
}
```

### Using LazyString for Optimization

```rust
use flashlog::{lazy_string::LazyString, log_info};

let lazy_msg = LazyString::new(|| format!("{} {} {}", 1, 2, 3)); // evaluated in the logger thread
log_info!("LazyOne", msg = lazy_msg);
```

## Configuration Options

FlashLog offers various configuration options:

```rust
let logger = Logger::initialize()
    .with_file("logs", "message")?
    .with_console_report(false)
    .with_msg_buffer_size(1_000_000)
    .with_msg_flush_interval(1_000_000)
    .with_max_log_level(LogLevel::Info)
    .with_timezone(TimeZone::Local)
    .launch();
```

## Output Format

Logs are outputted in JSON format for easy parsing:

```json
{
  "data": {"text": "Warm up"},
  "date": "20240829",
  "level": "Info",
  "offset": 9,
  "src": "src/main.rs:135",
  "time": "20:08:21.071:409:907",
  "topic": "not given"
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
{"data": {"test_struct":{"data":[1,2,3,4,5,6,7,8,9,10]}},
"date":"20240829","level":"Info",
"offset":9,"src":"src/main.rs:48","time":"09:22:14.328:084:318","topic":"Log message"}
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
### Test machine: i5-14400F, 2.5Ghz

| Logger    | i32           | 80 byte struct  |
| --------- | ------------- | --------------- |
| flashlog  | 80 ns         | 90 ns           |
| ftlog     | 323 ns        | 581 ns          |
| fast_log  | 500 ns        | 500 ns          |
| slog      | 324 ns        | 604 ns          |
| fern      | 4,732 ns      | 5,714 ns        |
| tracing   | 5,177 ns      | 6,190 ns        |


### Test machine: Ryzen 7 7700, 3.8 Ghz
| Logger    | i32           | 80 byte struct  |
| --------- | ------------- | --------------- |
| flashlog  | 48 ns         | 60 ns           |
| ftlog     | 260 ns        | 480 ns          |
| fast_log  | 410 ns        | 358 ns          |
| slog      | 250 ns        | 452 ns          |
| fern      | 3,813 ns      | 3,962 ns        |

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
## Authors

- [Junbeom Lee](https://github.com/JunbeomL22)
- [Youngjin Park](https://github.com/youngjin-create)

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.