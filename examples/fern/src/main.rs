use fern::Dispatch;
use log::info;
use serde::{Serialize, Deserialize};
use flashlog::get_unix_nano;
use std::fs::File;
use std::time::SystemTime;
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

fn fern_array_80bytes() -> Result<()> {
    // Configure logger at runtime
    Dispatch::new()
    // Perform allocation-free log formatting
    .format(|out, message, record| {
        out.finish(format_args!(
            "[{} {} {}] {}",
            humantime::format_rfc3339(std::time::SystemTime::now()),
            record.level(),
            record.target(),
            message
        ))
    })
    // Add blanket level filter -
    .level(log::LevelFilter::Debug)
    // - and per-module overrides
    .level_for("hyper", log::LevelFilter::Info)
    // Output to stdout, files, and other Dispatch configurations
    //.chain(std::io::stdout())
    .chain(fern::log_file("logs/struct.log")?)
    // Apply globally
    .apply()?;

    let iteration = 500_000;
    let test_number = 5;
    let log_struct = LogStruct::default();
    let mut res_vec = Vec::new();

    println!("Start test: struct containing 80 bytes array ");
    println!("Iteration: {}, Test number: {}", iteration, test_number);
    println!("At each test, sleep for 2 seconds and log warm up msg");
    for _ in 0..test_number {
        std::thread::sleep(std::time::Duration::from_secs(2));
        info!("Warm up");
        let start = get_unix_nano();
        for _ in 0..iteration {
            info!("Log message: {:?}", &log_struct);
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

fn fern_i32() -> Result<()> {
    // Configure logger at runtime
    Dispatch::new()
    // Perform allocation-free log formatting
    .format(|out, message, record| {
        out.finish(format_args!(
            "[{} {} {}] {}",
            humantime::format_rfc3339(std::time::SystemTime::now()),
            record.level(),
            record.target(),
            message
        ))
    })
    // Add blanket level filter -
    .level(log::LevelFilter::Debug)
    // - and per-module overrides
    .level_for("hyper", log::LevelFilter::Info)
    // Output to stdout, files, and other Dispatch configurations
    //.chain(std::io::stdout())
    .chain(fern::log_file("logs/i32.log")?)
    // Apply globally
    .apply()?;

    let iteration = 500_000;
    let test_number = 5;
    let mut res_vec = Vec::new();

    println!("Start test: i32");
    println!("Iteration: {}, Test number: {}", iteration, test_number);
    println!("At each test, sleep for 2 seconds and log warm up msg");
    for _ in 0..test_number {
        std::thread::sleep(std::time::Duration::from_secs(2));
        info!("Warm up");
        let start = get_unix_nano();
        for i in 0..iteration {
            info!("Log message: {}", i);
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
fn main() -> Result<()> {
    #[cfg(feature = "i32")]
    fern_i32();
    #[cfg(feature = "arr")]
    fern_array_80bytes();

    Ok(())
}