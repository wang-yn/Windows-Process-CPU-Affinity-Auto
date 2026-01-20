use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    #[serde(default)]
    pub service: ServiceConfig,
    #[serde(default)]
    pub cpu: CpuConfig,
    #[serde(default)]
    pub whitelist: WhitelistConfig,
    #[serde(default)]
    pub advanced: AdvancedConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServiceConfig {
    #[serde(default = "default_scan_interval")]
    pub scan_interval_ms: u64,
    #[serde(default = "default_log_level")]
    pub log_level: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CpuConfig {
    #[serde(default = "default_detection_mode")]
    pub detection_mode: String,
    #[serde(default)]
    pub p_cores: Vec<u32>,
    #[serde(default)]
    pub e_cores: Vec<u32>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WhitelistConfig {
    #[serde(default = "default_match_mode")]
    pub match_mode: String,
    #[serde(default)]
    pub processes: Vec<String>,
    #[serde(default)]
    pub exclude_processes: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AdvancedConfig {
    #[serde(default)]
    pub process_existing_on_startup: bool,
    #[serde(default = "default_cache_cleanup_interval")]
    pub cache_cleanup_interval_secs: u64,
    #[serde(default = "default_retry_attempts")]
    pub retry_attempts: u32,
    #[serde(default = "default_retry_delay")]
    pub retry_delay_ms: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MatchMode {
    Exact,
    Wildcard,
    Regex,
}

impl MatchMode {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "exact" => MatchMode::Exact,
            "regex" => MatchMode::Regex,
            _ => MatchMode::Wildcard,
        }
    }
}

// Default values
fn default_scan_interval() -> u64 {
    1000
}

fn default_log_level() -> String {
    "info".to_string()
}

fn default_detection_mode() -> String {
    "auto".to_string()
}

fn default_match_mode() -> String {
    "wildcard".to_string()
}

fn default_cache_cleanup_interval() -> u64 {
    300
}

fn default_retry_attempts() -> u32 {
    3
}

fn default_retry_delay() -> u64 {
    100
}

impl Default for ServiceConfig {
    fn default() -> Self {
        Self {
            scan_interval_ms: default_scan_interval(),
            log_level: default_log_level(),
        }
    }
}

impl Default for CpuConfig {
    fn default() -> Self {
        Self {
            detection_mode: default_detection_mode(),
            p_cores: Vec::new(),
            e_cores: Vec::new(),
        }
    }
}

impl Default for WhitelistConfig {
    fn default() -> Self {
        Self {
            match_mode: default_match_mode(),
            processes: Vec::new(),
            exclude_processes: vec![
                "system".to_string(),
                "svchost.exe".to_string(),
            ],
        }
    }
}

impl Default for AdvancedConfig {
    fn default() -> Self {
        Self {
            process_existing_on_startup: false,
            cache_cleanup_interval_secs: default_cache_cleanup_interval(),
            retry_attempts: default_retry_attempts(),
            retry_delay_ms: default_retry_delay(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            service: ServiceConfig::default(),
            cpu: CpuConfig::default(),
            whitelist: WhitelistConfig::default(),
            advanced: AdvancedConfig::default(),
        }
    }
}
