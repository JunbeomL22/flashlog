use anyhow::Result;
use serde::{Deserialize, Serialize};

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
    let _logger = flashlog::Logger::initialize()
        .with_file("logs", "message")?
        .with_console_report(false)
        .with_msg_buffer_size(500)
        .with_msg_flush_interval(500_000_000)
        .with_max_log_level(flashlog::LogLevel::Error)
        .with_timezone(flashlog::TimeZone::Local)
        .with_logger_core(0)
        .include_unixnano(true)
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
        flashlog::flash_info!("Warm up");
        let start = flashlog::get_unix_nano();
        for _ in 0..iteration {
            flashlog::flash_error_ct!(LogStruct = log_struct);
            flashlog::flash_warn_ct!(LogStruct = log_struct);
            flashlog::flash_info_ct!(LogStruct = log_struct);
            flashlog::flash_debug_ct!(LogStruct = log_struct);
            flashlog::flash_trace_ct!(LogStruct = log_struct);
            
            /*
            flashlog::flash_error!(LogStruct = log_struct);
            flashlog::flash_warn!(LogStruct = log_struct);
            flashlog::flash_info!(LogStruct = log_struct);
            flashlog::flash_debug!(LogStruct = log_struct);
            flashlog::flash_trace!(LogStruct = log_struct);
            */
            
        }    
        let elapsed = flashlog::get_unix_nano() - start;
        res_vec.push(elapsed);
    }

    flashlog::flush!(); 

    let ave_res: Vec<f64> = res_vec.iter().map(|x| *x as f64 / iteration as f64).collect();

    for (i, res) in ave_res.iter().enumerate() {
        println!("Test number: {}, Elapsed time: {:.1} ns", i, res);
    }

    println!("Average time: {:.1} ns", ave_res.iter().sum::<f64>() / test_number as f64);

    
    Ok(())
}

fn flashlog_i32() -> Result<()> {
    let _logger = flashlog::Logger::initialize()
        .with_file("logs", "message")?
        .with_console_report(false)
        .with_msg_buffer_size(100)
        .with_msg_flush_interval(500_000_000)
        .with_max_log_level(flashlog::LogLevel::Error)
        .with_timezone(flashlog::TimeZone::Local)
        .with_logger_core(1)
        .include_unixnano(true)
        .launch();

    let iteration = 500_000;
    let test_number = 5;
    
    let mut res_vec = Vec::new();

    println!("Start test: i32 ");
    println!("Iteration: {}, Test number: {}", iteration, test_number);
    println!("At each test, sleep for 2 seconds and log warm up msg");
    for _ in 0..test_number {
        std::thread::sleep(std::time::Duration::from_secs(1));
        flashlog::flash_info!("Warm up");
        let start = flashlog::get_unix_nano();
        for i in 0..iteration {
            flashlog::flash_error_ct!(log_int = i);
        }
        let elapsed = flashlog::get_unix_nano() - start;
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
    let _logger = flashlog::Logger::initialize()
        .with_file("logs", "message")?
        .with_console_report(false)
        .with_msg_buffer_size(100)
        .with_msg_flush_interval(500_000_000)
        .with_max_log_level(flashlog::LogLevel::Error)
        .with_timezone(flashlog::TimeZone::Local)
        .with_logger_core(0)
        .include_unixnano(true)
        .launch();

    let iteration = 500_000;
    let test_number = 5;
    let log_struct = LogStruct::default();
    let mut res_vec = Vec::new();

    println!("Start test");
    println!("Iteration: {}, Test number: {}", iteration, test_number);
    println!("At each test, sleep for 3 seconds and log warm up msg");
    for _ in 0..test_number {
        std::thread::sleep(std::time::Duration::from_secs(3));
        flashlog::flash_info!("Warm up");
        let start = flashlog::get_unix_nano();
        for i in 0..iteration {
            flashlog::flash_trace_ct!(LogStruct = log_struct);
            flashlog::flash_debug_ct!(LogStruct = log_struct);
            flashlog::flash_info_ct!(LogStruct = log_struct);
            flashlog::flash_warn_ct!(LogStruct = log_struct);
            flashlog::flash_error_ct!(LogStruct = log_struct);
        }
        let elapsed = flashlog::get_unix_nano() - start;
        res_vec.push(elapsed);
    }
    //flush!();

    let ave_res: Vec<f64> = res_vec.iter().map(|x| *x as f64 / iteration as f64).collect();

    for (i, res) in ave_res.iter().enumerate() {
        println!("Test number: {}, Elapsed time: {:.1} ns", i, res);
    }

    println!("Average time: {:.1} ns", ave_res.iter().sum::<f64>() / test_number as f64);
    //
    //
    let iteration = 500_000;
    let test_number = 5;
    
    let mut res_vec = Vec::new();
    let log_struct = LogStruct::default();
    println!("Start test");
    println!("Iteration: {}, Test number: {}", iteration, test_number);
    println!("At each test, sleep for 3 seconds and log warm up msg");
    for _ in 0..test_number {
        std::thread::sleep(std::time::Duration::from_secs(3));
        flashlog::flash_info!("Warm up");
        let start = flashlog::get_unix_nano();
        for i in 0..iteration {
            //let log_struct = LogStruct::default();
            //flashlog::flash_error!(LogStruct = log_struct);
            flashlog::flash_warn!("Hello");
            //flashlog::flash_info!(LogStruct = log_struct);
            //flashlog::flash_debug!(LogStruct = log_struct);
            //flashlog::flash_trace!(LogStruct = log_struct);
        }
        let elapsed = flashlog::get_unix_nano() - start;
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

fn main() -> Result<()> {
    #[cfg(feature = "i32")]
    flashlog_i32();
    
    #[cfg(feature = "arr")]
    flashlog_array_80bytes();
    
    #[cfg(feature = "test")]
    test_logger();

    Ok(())
}