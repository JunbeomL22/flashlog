use anyhow::Result;
use serde::{Deserialize, Serialize};

use flashlog::{
    LogLevel, Logger, TimeZone,
    get_unix_nano,
    flush,
    flash_info,
    log_info,
    flushing_log_info,
    info,

};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LogStruct {
    data: [u64; 10],
}

impl Default for LogStruct {
    fn default() -> Self {
        LogStruct { data: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10] }
    }
}

impl std::fmt::Display for LogStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.data)
    }
}

fn flashlog_array_80bytes() -> Result<()> {
    let _logger = Logger::initialize()
        .with_file("logs", "message")?
        .with_console_report(false)
        .with_msg_buffer_size(1_000_000_000)
        .with_msg_flush_interval(1_000_000)
        .with_max_log_level(LogLevel::Info)
        .with_timezone(TimeZone::Local)
        .launch();

    let iteration = 500_000;
    let test_number = 5;
    
    let log_struct = LogStruct::default();
    let mut res_vec = Vec::new();

    println!("Start test: struct containing 80 bytes array ");
    println!("Iteration: {}, Test number: {}", iteration, test_number);
    println!("At each test, sleep for 2 seconds and log warm up msg");
    for _ in 0..test_number {
        std::thread::sleep(std::time::Duration::from_secs(2));
        flash_info!("Warm up");
        let start = get_unix_nano();
        for _ in 0..iteration {
            let test_clone = log_struct.clone();
            flash_info!(struct_info = test_clone);
        }    
        let elapsed = get_unix_nano() - start;
        res_vec.push(elapsed);
    }

    let ave_res: Vec<f64> = res_vec.iter().map(|x| *x as f64 / iteration as f64).collect();

    for (i, res) in ave_res.iter().enumerate() {
        println!("Test number: {}, Elapsed time: {:.1} ns", i, res);
    }

    println!("Average time: {:.1} ns", ave_res.iter().sum::<f64>() / test_number as f64);

    
    Ok(())
}

fn flashlog_i32() -> Result<()> {
    let _logger = Logger::initialize()
        .with_file("logs", "message")?
        .with_console_report(false)
        .with_msg_buffer_size(1000_000_000)
        .with_msg_flush_interval(1000_000)
        .with_max_log_level(LogLevel::Info)
        .with_timezone(TimeZone::Local)
        .launch();

    let iteration = 500_000;
    let test_number = 5;
    
    let log_struct = LogStruct::default();
    let mut res_vec = Vec::new();

    println!("Start test: i32 ");
    println!("Iteration: {}, Test number: {}", iteration, test_number);
    println!("At each test, sleep for 2 seconds and log warm up msg");
    for _ in 0..test_number {
        std::thread::sleep(std::time::Duration::from_secs(2));
        flash_info!("Warm up");
        let start = get_unix_nano();
        for i in 0..iteration {
            flash_info!(log_int = i);
        }    
        let elapsed = get_unix_nano() - start;
        res_vec.push(elapsed);
    }
    //flush!();

    let ave_res: Vec<f64> = res_vec.iter().map(|x| *x as f64 / iteration as f64).collect();

    for (i, res) in ave_res.iter().enumerate() {
        println!("Test number: {}, Elapsed time: {:.1} ns", i, res);
    }

    println!("Average time: {:.1} ns", ave_res.iter().sum::<f64>() / test_number as f64);
    
    Ok(())
}

fn test_logger() -> Result<()> {
    let _logger = Logger::initialize()
        .with_file("logs", "message")? // without this the the logger dose not report a file
        .with_console_report(true) // true means it reports to console too
        .with_msg_flush_interval(2_000_000_000) // flushing interval is 2 bil nano seconds = 2 seconds
        .with_msg_buffer_size(1_000_000) // but messages are flushed the 
        .with_max_log_level(LogLevel::Debug)
        .with_timezone(TimeZone::Local)
        .launch();

    info!("Warm up"); // pushed in message queue but not reported, 
    // this will be reported when another log comes in after 2 seconds (with_msg_flush_interval = 2_000_000_000)
    flush!(); // without this the logger does not report any message 
    // since the time and msg length conditions are not met
    flushing_log_info!("Log message", data = "data"); // This logs and flushes together
    Ok(())
}

fn main() -> Result<()> {
    #[cfg(feature = "i32")]
    flashlog_i32();
    
    #[cfg(feature = "arr")]
    flashlog_array_80bytes();
    
    #[cfg(feature = "test")]
    test_logger();

    Ok(())
}
    
/*
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

    let lazy_msg = LazyString::new(|| format!("{} {} {}", 1, 2, 3));
    log_info!("LazyOne", msg = lazy_msg);   
    // this macro flushes message regardless of the options
    flush!("flushing", data = "");

    Ok(())
} */