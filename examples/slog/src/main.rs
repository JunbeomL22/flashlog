use slog;
use slog_term;
use slog_async;
use std::fs::OpenOptions;
use slog::Drain;
use slog::{
    o,
    info,
};
use serde::{Deserialize, Serialize};
use flashlog::get_unix_nano;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LogStruct {
    data: [u64; 10],
}

impl Default for LogStruct {
    fn default() -> Self {
        LogStruct { data: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10] }
    }
}

fn slog_i32() {
   let log_path = "logs/i32.log";
   let file = OpenOptions::new()
      .create(true)
      .write(true)
      .truncate(true)
      .open(log_path)
      .unwrap();

    let decorator = slog_term::PlainDecorator::new(file);
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    let _log = slog::Logger::root(drain, o!());

    let iteration = 500_000;
    let test_number = 5;
    let mut res_vec = Vec::new();

    println!("Start test: i32");
    println!("Iteration: {}, Test number: {}", iteration, test_number);
    println!("At each test, sleep for 2 seconds and log warm up msg");

    for _ in 0..test_number {
        std::thread::sleep(std::time::Duration::from_secs(2));
        info!(_log, "Warm up");
        let start = get_unix_nano();
        for i in 0..iteration {
            info!(_log, "Log message: {}", i);
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

fn slog_array_80bytes() {
    let log_path = "logs/arr.log";
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(log_path)
        .unwrap();

    let decorator = slog_term::PlainDecorator::new(file);
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    let _log = slog::Logger::root(drain, o!());

    let iteration = 500_000;
    let test_number = 5;
    let log_struct = LogStruct::default();
    let mut res_vec = Vec::new();

    println!("Start test: struct containing 80 bytes array ");
    println!("Iteration: {}, Test number: {}", iteration, test_number);
    println!("At each test, sleep for 2 seconds and log warm up msg");

    for _ in 0..test_number {
        std::thread::sleep(std::time::Duration::from_secs(2));
        info!(_log, "Warm up");
        let start = get_unix_nano();
        for _ in 0..iteration {
            info!(_log, "Log message: {:?}", &log_struct);
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
    slog_i32();
    #[cfg(feature = "arr")]
    slog_array_80bytes();
}