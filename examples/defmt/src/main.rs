use defmt_rtt as _;
use panic_probe as _;

fn main() {
    defmt::println!("Hello, world!");
    defmt::info!("This is an info message");
    defmt::debug!("This is a debug message");
    defmt::warn!("This is a warning message");
    defmt::error!("This is an error message");
}