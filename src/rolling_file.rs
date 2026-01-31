use crate::{get_unix_nano, UnixNano};
use std::path::{Path, PathBuf};
use std::fs::{File, OpenOptions, remove_file};
use std::io::{self, BufWriter, Write};
use std::sync::OnceLock;
use chrono::Local;

static INITIAL_LOG_FILE_PATH: OnceLock<PathBuf> = OnceLock::new();

/// Returns the initial log file path if file logging is enabled.
/// Returns `None` if file logging is not configured.
pub fn get_initial_log_file_path() -> Option<PathBuf> {
    INITIAL_LOG_FILE_PATH.get().cloned()
}

/// Generates and stores the initial log file path. Called by Logger::launch().
pub(crate) fn set_initial_log_file_path(base_path: &Path, prefix: &str) -> PathBuf {
    let file_path = RollingFileWriter::generate_file_path(base_path, prefix);
    let _ = INITIAL_LOG_FILE_PATH.set(file_path.clone());
    file_path
}

const SECOND_IN_NANOS: u64 = 1_000_000_000;
const MINUATE_IN_NANOS: u64 = 60_000_000_000;
const HOUR_IN_NANOS: u64 = 3_600_000_000_000;
const DAY_IN_NANOS: u64 = 86_400_000_000_000;
const WEEK_IN_NANOS: u64 = 604_800_000_000_000;

#[derive(Clone, Debug)]
pub enum RollingPeriod {
    None,
    Secondly,
    Minutely,
    Hourly,
    Daily,
    Weekly,
}

#[derive(Clone, Debug)]
pub struct RollingConfig {
    pub base_path: PathBuf,
    pub file_name_prefix: String,
    //
    pub roll_period: Option<RollingPeriod>,
    pub max_roll_files: Option<usize>,
    //
    pub compress: bool,
    /// Pre-generated file path (set by Logger::launch)
    pub(crate) initial_file_path: Option<PathBuf>,
}

impl Default for RollingConfig {
    fn default() -> Self {
        Self {
            base_path: PathBuf::from("./"),
            file_name_prefix: "log".to_string(),
            roll_period: None,
            max_roll_files: None,
            compress: false,
            initial_file_path: None,
        }
    }
}

pub struct RollingFileWriter {
    config: RollingConfig,
    current_file: Option<BufWriter<File>>,
    rolling_nanos: Option<UnixNano>,
    max_roll_files: usize,
    last_roll_time: UnixNano,
}

impl RollingFileWriter {
    pub fn new(config: RollingConfig) -> io::Result<Self> {
        let file_path = config.initial_file_path.clone()
            .unwrap_or_else(|| Self::generate_file_path(&config.base_path, &config.file_name_prefix));
        let _ = INITIAL_LOG_FILE_PATH.set(file_path.clone());
        let current_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&file_path)?;

        let last_roll_time = get_unix_nano();
        let max_roll_files = config.max_roll_files.unwrap_or(10);
        let rolling_nanos = match config.roll_period {
            Some(RollingPeriod::None) => None,
            Some(RollingPeriod::Secondly) => Some(SECOND_IN_NANOS),
            Some(RollingPeriod::Minutely) => Some(MINUATE_IN_NANOS),
            Some(RollingPeriod::Hourly) => Some(HOUR_IN_NANOS),
            Some(RollingPeriod::Daily) => Some(DAY_IN_NANOS),
            Some(RollingPeriod::Weekly) => Some(WEEK_IN_NANOS),
            None => None,
        };
        Ok(Self {
            config,
            current_file: Some(BufWriter::new(current_file)),
            rolling_nanos,
            max_roll_files,
            last_roll_time,
        })
    }

    pub(crate) fn generate_file_path(base_path: &Path, prefix: &str) -> PathBuf {
        let now = Local::now();
        let timestamp = now.format("%Y%m%d-%H%M%S");
        let file_name = format!("{}-{}.log", prefix, timestamp);
        base_path.join(file_name)
    }

    pub fn write_all(&mut self, data: &[u8]) -> io::Result<()> {
        if self.should_roll(None) {
            self.roll_file()?;
        }

        if let Some(ref mut current_file) = self.current_file {
            current_file.write_all(data)?;
        }

        Ok(())
    }

    pub fn flush(&mut self) -> io::Result<()> {
        if let Some(ref mut current_file) = self.current_file {
            current_file.flush()?;
        }
        Ok(())
    }

    pub fn sync_all(&mut self) -> io::Result<()> {
        if let Some(ref mut current_file) = self.current_file {
            current_file.get_ref().sync_all()?;
        }
        Ok(())
    }

    fn should_roll(&mut self, now: Option<UnixNano>) -> bool {
        if self.rolling_nanos.is_none() {
            return false;
        }

        let now = now.unwrap_or(get_unix_nano());
        let diff = now - self.last_roll_time;

        if diff >= self.rolling_nanos.unwrap() {
            self.last_roll_time = now;
            true
        } else {
            false
        }
    }

    fn roll_file(&mut self) -> io::Result<()> {
        // Flush and close current file
        if let Some(ref mut current_file) = self.current_file.take() {
            current_file.flush()?;
        }
        // Generate new file path
        let new_file_path = Self::generate_file_path(&self.config.base_path, &self.config.file_name_prefix);

        // Rotate old files if needed
        self.rotate_old_files()?;

        // open new file
        let new_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&new_file_path)?;

        self.current_file = Some(BufWriter::new(new_file));
        self.last_roll_time = get_unix_nano();


        Ok(())
    }

    fn compress_file(&self, file_path: &Path) -> io::Result<()> {
        let gz_path = file_path.with_extension("gz");
        let input = std::fs::read(file_path)?;
        let output = File::create(&gz_path)?;
        let mut encoder = flate2::write::GzEncoder::new(output, flate2::Compression::fast());
        encoder.write_all(&input)?;
        encoder.finish()?;
        remove_file(file_path)?;
        Ok(())
    }

    fn collect_log_files(&self) -> io::Result<Vec<PathBuf>> {
        let mut files = Vec::new();
        for entry in std::fs::read_dir(&self.config.base_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "log" {
                        if let Some(file_name) = path.file_name() {
                            if file_name.to_string_lossy().starts_with(&self.config.file_name_prefix) {
                                files.push(path);
                            }
                        }
                    }
                }
            }
        }
        Ok(files)
    }

    fn collect_compressed_files(&self) -> io::Result<Vec<PathBuf>> {
        let mut files = Vec::new();
        for entry in std::fs::read_dir(&self.config.base_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "gz" {
                        if let Some(file_name) = path.file_name() {
                            if file_name.to_string_lossy().starts_with(&self.config.file_name_prefix) {
                                files.push(path);
                            }
                        }
                    }
                }
            }
        }
        Ok(files)
    }
    
    fn rotate_old_files(&self) -> io::Result<()> {
        let mut log_files = self.collect_log_files()?;
        log_files.sort_by(|a, b| b.cmp(a)); // Sort in reverse order
        
        // Remove oldest files if we exceed max_roll_files
        while log_files.len() >= self.max_roll_files {
            if let Some(oldest_file) = log_files.pop() {
                if self.config.compress {
                    self.compress_file(&oldest_file)?;
                } else {
                    remove_file(oldest_file)?;
                }
            }
        }

        let mut gz_files = self.collect_compressed_files()?;
        gz_files.sort_by(|a, b| b.cmp(a)); // Sort in reverse order

        // Remove oldest files if we exceed max_roll_files
        while gz_files.len() >= self.max_roll_files {
            if let Some(oldest_file) = gz_files.pop() {
                remove_file(oldest_file)?;
            }
        }

        Ok(())
    }
}