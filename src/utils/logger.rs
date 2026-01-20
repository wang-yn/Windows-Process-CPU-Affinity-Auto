use log::LevelFilter;
use std::fs;
use std::path::Path;
use crate::utils::ServiceError;

/// Initialize console logger for CLI mode
pub fn init_logger(log_level: &str) {
    let level = parse_log_level(log_level);

    env_logger::Builder::new()
        .filter_level(level)
        .format(|buf, record| {
            use std::io::Write;
            writeln!(
                buf,
                "[{} {} {}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.target(),
                record.args()
            )
        })
        .init();
}

/// Initialize file logger for service mode
pub fn init_service_logger(log_level: &str) -> Result<(), ServiceError> {
    let level = parse_log_level(log_level);
    let log_file = get_service_log_path()?;

    // Create log directory if it doesn't exist
    if let Some(parent) = Path::new(&log_file).parent() {
        fs::create_dir_all(parent).map_err(|e| {
            ServiceError::Service(format!("Failed to create log directory: {}", e))
        })?;
    }

    let file_output = fern::log_file(&log_file).map_err(|e| {
        ServiceError::Service(format!("Failed to open log file: {}", e))
    })?;

    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(level)
        .chain(file_output)
        .chain(std::io::stdout()) // Also log to stdout for debugging
        .apply()
        .map_err(|e| ServiceError::Service(format!("Failed to initialize logger: {}", e)))?;

    log::info!("Logger initialized with file: {}", log_file);
    Ok(())
}

fn parse_log_level(log_level: &str) -> LevelFilter {
    match log_level.to_lowercase().as_str() {
        "trace" => LevelFilter::Trace,
        "debug" => LevelFilter::Debug,
        "info" => LevelFilter::Info,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        _ => LevelFilter::Info,
    }
}

fn get_service_log_path() -> Result<String, ServiceError> {
    if let Ok(program_data) = std::env::var("ProgramData") {
        Ok(format!("{}\\ProcessCpuAuto\\service.log", program_data))
    } else {
        Ok("C:\\ProgramData\\ProcessCpuAuto\\service.log".to_string())
    }
}
