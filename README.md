# FlashLog
A fast Rust logging library. This logging system is lazy. As a result, it is blazingly fast.
To explain more specifically, aside from struct cloning and timestamp (unix nano) marking, 
most evaluations are performed in the logger thread. To the best of my knowledge, it is the fastest among most logging systems designed for Rust. Log messages are printed in JSON format. Additionally, it provides LazyString for optimization.

# Usage
```Cargo.toml```:
```toml
[dependencies]
flashlog = "0.1"
```
The following example is logging a struct that contains array of ```u64```
```Rust
use serde::{Deserialize, Serialize};
use flashlog::{
    LogLevel, Logger, TimeZone,
    get_unix_nano,
    log_info,
    info,
    flushing_log_info as flush,
};
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LogStruct {
    data: [u64; 10],
}

impl Default for LogStruct {
    fn default() -> Self {
        LogStruct { data: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10] }
    }
}

fn main() -> Result<()> {
    let logger = Logger::initialize()
            // folder and file name
            .with_file("logs", "message")?
            .with_console_report(false)
            // In the logger thread, the messages are filled in a buffer
            // It flushes the messages where the length is more than 1,000,000
            .with_msg_buffer_size(1_000_000)
            // The messages are flushed if it has been passed 1,000,000 ns from the last flush
            .with_msg_flush_interval(1_000_000)
            .with_max_log_level(LogLevel::Info)
            .with_timezone(TimeZone::Local)
            .launch();

    let log_struct = LogStruct::default();
    info!("Warm up");
    log_info!("Log message", log_struct = log_struct);
    // this macro flushes message regardless of the options
    flush!("flushing", data = "");

    Ok(())
}
```
The following ```Json``` is the result of the above: 
```Json
{"data":{"text":"Warm up"}, "date":"20240829", "level":"Info","offset":9,
"src":"src/main.rs:135","time":"20:08:21.071:409:907","topic":"not given"}
{"data":{"log_struct":{"data":[1,2,3,4,5,6,7,8,9,10]}},"date":"20240829","level":"Info",
"offset":9,"src":"src/main.rs:136","time":"20:08:21.071:410:277","topic":"Log message"}
{"data":{"data":""},"date":"20240829","level":"Info","offset":9,"src":"src/main.rs:138",
"time":"20:08:21.071:410:408","topic":"flushing"}
```
For optimization, this crate supports a lazy string which can defer string interpolation to the logger thread
```Rust
use flashlog::lazy_string::LazyString;
let lazy_msg = LazyString::new(|| format!("{} {} {}", 1, 2, 3)); // This will be evaluated in the logger thread
log_info!("LazyOne", msg = lazy_msg);   
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
## Performance comparisons
### Test machine: i5-14400F, 2.5Ghz

| Logger    | i32           | 80 byte struct  |
| --------- | ------------- | --------------- |
| flashlog  | 80 ns         | 90 ns           |
| ftlog     | 323 ns        | 581 ns          |
| fast_log  | 500 ns        | 500 ns          |
| slog      | 324 ns        | 604 ns          |
| fern      | 4,732 ns      | 5,714 ns         |

### Test machine: Ryzen 7 7700, 3.8 Ghz
| Logger    | i32           | 80 byte struct  |
| --------- | ------------- | --------------- |
| flashlog  | 48 ns         | 60 ns           |
| ftlog     | 260 ns        | 480 ns          |
| fast_log  | 410 ns        | 358 ns          |
| slog      | 250 ns        | 452 ns          |
| fern      | 3,813 ns       | 3,962 ns         |
