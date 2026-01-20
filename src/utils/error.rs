use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("CPU detection error: {0}")]
    CpuDetection(String),

    #[error("Process monitoring error: {0}")]
    ProcessMonitoring(String),

    #[error("Affinity setting error: {0}")]
    AffinitySetting(String),

    #[error("Windows API error: {0}")]
    WindowsApi(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("TOML parsing error: {0}")]
    TomlParse(#[from] toml::de::Error),

    #[error("Service error: {0}")]
    Service(String),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),
}

pub type Result<T> = std::result::Result<T, ServiceError>;
