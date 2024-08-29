use flashlog::get_unix_nano;
use tracing::{info, span, Level};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LogStruct {
    data: [u64; 10],
}

impl Default for LogStruct {
    fn default() -> Self {
        LogStruct { data: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10] }
    }
}

fn tracing_i32() {
    // Set up a rolling file appender that rotates daily
    let file_appender = RollingFileAppender::new(Rotation::DAILY, "logs", "logs/i32.log");

    // Create a subscriber that writes to the file appender
    let subscriber = tracing_subscriber::registry()
        .with(fmt::Layer::new().with_writer(file_appender).with_ansi(false))
        .with(EnvFilter::from_default_env());

    // Set the subscriber as the global default
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set subscriber");

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
        for _ in 0..iteration {
            info!("Log message: {}", 42);
        }
        let elapsed = get_unix_nano() - start;
        res_vec.push(elapsed);
    }

    let ave_res: Vec<u64> = res_vec.iter().map(|x| x / iteration).collect();

    for (i, res) in ave_res.iter().enumerate() {
        println!("Test {}: average time: {} ns", i, res);
    }

    println!("Average time: {:.1} ns", ave_res.iter().sum::<f64>() / test_number as f64);
}

fn tracing_array_80bytes() {
    // Set up a rolling file appender that rotates daily
    let file_appender = RollingFileAppender::new(Rotation::DAILY, "logs", "logs/array_80bytes.log");

    // Create a subscriber that writes to the file appender
    let subscriber = tracing_subscriber::registry()
        .with(fmt::Layer::new().with_writer(file_appender).with_ansi(false))
        .with(EnvFilter::from_default_env());

    // Set the subscriber as the global default
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set subscriber");

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
}

fn main() {
    #[cfg(feature = "i32")]
    tracing_i32();
    #[cfg(feature = "arr")]
    tracing_array_80bytes();
}