use anyhow::Result;
use flashlog::{Logger, get_initial_log_file_path, flash_info_ct, flush};

fn main() -> Result<()> {
    let _logger = Logger::initialize()
        .with_file("logs", "message")?
        .with_console_report(true)
        .with_msg_buffer_size(10)
        .with_msg_flush_interval(1_000_000_000)
        .with_timezone(flashlog::TimeZone::Local)
        .launch();

    // Get the log file path
    if let Some(path) = get_initial_log_file_path() {
        println!("Log file: {}", path.display());
    } else {
        println!("File logging not enabled");
    }

    flash_info_ct!("test"; "hello world");
    flush!();

    Ok(())
}
