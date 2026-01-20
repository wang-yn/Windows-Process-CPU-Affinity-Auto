use crate::config::settings::Config;
use crate::utils::ServiceError;
use std::fs;
use std::path::Path;

pub struct ConfigLoader;

impl ConfigLoader {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Config, ServiceError> {
        let path = path.as_ref();

        if !path.exists() {
            log::warn!("Configuration file not found at {:?}, creating default configuration", path);
            let config = Config::default();
            Self::save(path, &config)?;
            return Ok(config);
        }

        let content = fs::read_to_string(path)
            .map_err(|e| ServiceError::Config(format!("Failed to read config file: {}", e)))?;

        let config: Config = toml::from_str(&content)?;

        log::info!("Configuration loaded from {:?}", path);
        Ok(config)
    }

    pub fn save<P: AsRef<Path>>(path: P, config: &Config) -> Result<(), ServiceError> {
        let path = path.as_ref();

        // Create parent directory if it doesn't exist
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| ServiceError::Config(format!("Failed to create config directory: {}", e)))?;
        }

        let toml_string = toml::to_string_pretty(config)
            .map_err(|e| ServiceError::Config(format!("Failed to serialize config: {}", e)))?;

        fs::write(path, toml_string)
            .map_err(|e| ServiceError::Config(format!("Failed to write config file: {}", e)))?;

        log::info!("Configuration saved to {:?}", path);
        Ok(())
    }

    pub fn get_default_path() -> String {
        "config.toml".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.service.scan_interval_ms, 1000);
        assert_eq!(config.service.log_level, "info");
    }
}
