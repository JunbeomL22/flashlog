use flashlog::{Logger, RollingPeriod, flush};
use std::fs;

#[test]
fn test_rolling_period_none_does_not_roll() {
    let temp_dir = std::env::temp_dir().join("flashlog_test_no_roll");
    // Clean up before test
    let _ = fs::remove_dir_all(&temp_dir);
    let _ = fs::create_dir_all(&temp_dir);

    {
        let _logger = Logger::initialize()
            .with_file(temp_dir.to_str().unwrap(), "test_no_roll")
            .expect("Failed to set file")
            .with_roll_period(RollingPeriod::None)
            .expect("Failed to set roll period")
            .with_console_report(false)
            .launch();

        // Log some messages
        flashlog::flash_info_ct!("test"; "message 1");
        flashlog::flash_info_ct!("test"; "message 2");
        flashlog::flash_info_ct!("test"; "message 3");
        flush!();
    } // Logger is dropped here and finalized

    // Small delay to ensure file operations complete
    std::thread::sleep(std::time::Duration::from_millis(100));

    // Count log files - should be exactly 1 since rolling is disabled
    let log_files: Vec<_> = fs::read_dir(&temp_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .file_name()
                .map(|n| n.to_string_lossy().starts_with("test_no_roll") && n.to_string_lossy().ends_with(".log"))
                .unwrap_or(false)
        })
        .collect();

    assert_eq!(log_files.len(), 1, "Should have exactly 1 log file when rolling is disabled");

    // Clean up
    let _ = fs::remove_dir_all(&temp_dir);
}
