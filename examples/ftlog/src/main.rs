use ftlog::{
    appender::{file::Period, FileAppender},
    info, LoggerGuard,
};
use log::LevelFilter;
use time::Duration;
use flashlog::get_unix_nano;
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

fn init() -> LoggerGuard {
    // Rotate every day, clean stale logs that were modified 7 days ago on each rotation
    let writer = FileAppender::builder()
        .path("logs/current.log")
        .rotate(Period::Minute)
        .expire(Duration::minutes(4))
        .build();
    ftlog::Builder::new()
        // global max log level
        .max_log_level(LevelFilter::Info)
        // define root appender, pass None would write to stderr
        .root(writer)
        .unbounded()
        // write logs in ftlog::appender to "./ftlog-appender.log" instead of "./current.log"
        .filter("ftlog::appender", "ftlog-appender", LevelFilter::Error)
        .appender("ftlog-appender", FileAppender::new("logs/ftlog-appender.log"))
        .try_init()
        .expect("logger build or set failed")
}

fn ftlog_arr_80byte() {
    let logger = init();
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

fn ftlog_i32() {
    let logger = init();
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
}

fn main() {
    #[cfg(feature = "i32")]
    ftlog_i32();
    #[cfg(feature = "arr")]
    ftlog_arr_80byte();
}