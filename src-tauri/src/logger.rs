use chrono::Local;
use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
    sync::{Mutex, OnceLock},
};

const MAX_LOG_FILE_SIZE: u64 = 5 * 1024 * 1024;
const MAX_LOG_FILES: usize = 5;

static LOGGER: OnceLock<Mutex<Logger>> = OnceLock::new();

struct Logger {
    log_dir: PathBuf,
    file: File,
    current_size: u64,
    sequence: u32,
}

pub fn init() -> Result<PathBuf, String> {
    let exe_dir = get_exe_dir()?;
    let log_dir = exe_dir.join("log");
    fs::create_dir_all(&log_dir)
        .map_err(|e| format!("Failed to create log directory: {}", e))?;

    cleanup_old_logs(&log_dir, MAX_LOG_FILES.saturating_sub(1));
    let (file_path, file) = create_log_file(&log_dir, 0)?;
    let logger = Logger {
        log_dir,
        file,
        current_size: 0,
        sequence: 0,
    };

    let _ = LOGGER.set(Mutex::new(logger));
    log("INFO", &format!("Logger initialized: {}", file_path.display()));
    Ok(file_path)
}

pub fn info(message: impl AsRef<str>) {
    log("INFO", message.as_ref());
}

pub fn warn(message: impl AsRef<str>) {
    log("WARN", message.as_ref());
}

pub fn error(message: impl AsRef<str>) {
    log("ERROR", message.as_ref());
}

pub fn log(level: &str, message: &str) {
    let Some(logger) = LOGGER.get() else {
        eprintln!("[{}] {}", level, message);
        return;
    };

    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
    let mut line = format!("[{}] [{}] {}\n", timestamp, level, message);
    if line.len() as u64 > MAX_LOG_FILE_SIZE {
        line = format!(
            "[{}] [{}] log entry too large; original size={} bytes\n",
            timestamp,
            level,
            message.len()
        );
    }

    if let Ok(mut logger) = logger.lock() {
        if logger.current_size + line.len() as u64 > MAX_LOG_FILE_SIZE {
            if let Err(e) = logger.rotate() {
                eprintln!("Failed to rotate log file: {}", e);
                return;
            }
        }

        if logger.file.write_all(line.as_bytes()).is_ok() {
            logger.current_size += line.len() as u64;
            let _ = logger.file.flush();
        }
    }
}

#[tauri::command]
pub fn write_log(level: String, message: String) {
    let level = match level.to_ascii_uppercase().as_str() {
        "ERROR" => "ERROR",
        "WARN" => "WARN",
        _ => "INFO",
    };
    log(level, &format!("frontend: {}", message));
}

impl Logger {
    fn rotate(&mut self) -> Result<(), String> {
        self.sequence += 1;
        cleanup_old_logs(&self.log_dir, MAX_LOG_FILES.saturating_sub(1));
        let (_file_path, file) = create_log_file(&self.log_dir, self.sequence)?;
        self.file = file;
        self.current_size = 0;
        cleanup_old_logs(&self.log_dir, MAX_LOG_FILES);
        Ok(())
    }
}

fn create_log_file(log_dir: &Path, sequence: u32) -> Result<(PathBuf, File), String> {
    let timestamp = Local::now().format("%Y%m%d_%H%M%S");
    let filename = if sequence == 0 {
        format!("itools_{}.log", timestamp)
    } else {
        format!("itools_{}_{}.log", timestamp, sequence)
    };
    let file_path = log_dir.join(filename);
    let file = File::create(&file_path)
        .map_err(|e| format!("Failed to create log file {}: {}", file_path.display(), e))?;
    Ok((file_path, file))
}

fn cleanup_old_logs(log_dir: &Path, keep_count: usize) {
    let Ok(entries) = fs::read_dir(log_dir) else {
        return;
    };

    let mut logs = entries
        .filter_map(Result::ok)
        .filter_map(|entry| {
            let path = entry.path();
            let is_log = path.extension().is_some_and(|ext| ext == "log");
            if !is_log {
                return None;
            }

            let modified = entry.metadata().and_then(|metadata| metadata.modified()).ok()?;
            Some((path, modified))
        })
        .collect::<Vec<_>>();

    logs.sort_by_key(|(_, modified)| *modified);
    while logs.len() > keep_count {
        if let Some((path, _)) = logs.first() {
            let _ = fs::remove_file(path);
        }
        logs.remove(0);
    }
}

fn get_exe_dir() -> Result<PathBuf, String> {
    let exe_path = std::env::current_exe()
        .map_err(|e| format!("Failed to get exe path: {}", e))?;
    exe_path
        .parent()
        .ok_or("Failed to get exe directory".to_string())
        .map(|p| p.to_path_buf())
}
