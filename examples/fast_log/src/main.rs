use log::{error, info, warn};
use fast_log::Config;
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

fn fast_log_array_80bytes() {
    fast_log::init(Config::new().file("logs/arr.log").chan_len(Some(100000))).unwrap();
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

fn fast_log_i32() {
    fast_log::init(Config::new().file("logs/i32.log").chan_len(Some(100000))).unwrap();
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
        log::logger().flush();
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
    fast_log_i32();   

    #[cfg(feature = "arr")]
    fast_log_array_80bytes();
}