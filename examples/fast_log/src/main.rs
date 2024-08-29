use log::{error, info, warn};
fn main() {
    fast_log::init(Config::new().file("target/test.log").chan_len(Some(100000))).unwrap();
    log::info!("Commencing yak shaving{}", 0);
}