use flashlog::{Logger, LogLevel, TimeZone};

fn main() {
    // Create a logger with immediate flushing (0 interval) and minimal buffer size
    let _logger = Logger::initialize()
        .with_file("logs", "immediate") // Log to logs/immediate_*.log
        .unwrap() // Unwrap the Result
        .with_console_report(true) // Also output to console
        .with_msg_buffer_size(100) // Minimal buffer
        .with_msg_flush_interval(100) // Immediate flush
        .with_max_log_level(LogLevel::Info)
        .with_timezone(TimeZone::Seoul)
        .include_unixnano(true)
        .launch();

    // Log some immediate messages using compile-time filtered macros
    flashlog::flash_info_ct!("Application started");
    flashlog::flash_info_ct!(message = "Processing data", step = 1, total_steps = 10);
    flashlog::flash_warn_ct!("This is a warning message");
    flashlog::flash_error_ct!(topic = "database_error", error_code = 500, component = "database");

    // Force flush to ensure all messages are written
    flashlog::flush!();

    println!("Immediate log example completed. Check logs/immediate_*.log");
}
