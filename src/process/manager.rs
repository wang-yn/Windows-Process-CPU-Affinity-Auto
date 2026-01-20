use crate::config::settings::{Config, MatchMode};
use crate::cpu::AffinityManager;
use crate::process::{ProcessCache, ProcessMonitor};
use crate::utils::ServiceError;
use std::sync::Arc;

pub struct ProcessManager {
    config: Arc<Config>,
    affinity_manager: Arc<AffinityManager>,
    cache: ProcessCache,
    match_mode: MatchMode,
}

impl ProcessManager {
    pub fn new(config: Arc<Config>, affinity_manager: Arc<AffinityManager>) -> Self {
        let match_mode = MatchMode::from_str(&config.whitelist.match_mode);
        let cache = ProcessCache::new(config.advanced.cache_cleanup_interval_secs);

        Self {
            config,
            affinity_manager,
            cache,
            match_mode,
        }
    }

    pub fn scan_and_process(&mut self) -> Result<usize, ServiceError> {
        // Get all running processes
        let processes = ProcessMonitor::get_all_processes()?;

        let mut processed_count = 0;

        for process in processes {
            // Skip if already processed
            if self.cache.is_processed(process.pid) {
                continue;
            }

            // Check if this is a new process
            let is_new = self.cache.is_new_process(process.pid);
            if is_new {
                self.cache.mark_seen(process.pid, process.name.clone());
            }

            // Check if process matches whitelist
            if !self.is_whitelisted(&process.name) {
                continue;
            }

            // Check if process is excluded
            if self.is_excluded(&process.name) {
                log::debug!("Process {} is in exclude list, skipping", process.name);
                continue;
            }

            // Try to set affinity with retries
            match self.set_affinity_with_retry(process.pid, &process.name) {
                Ok(_) => {
                    self.cache.mark_processed(process.pid);
                    processed_count += 1;
                    log::info!(
                        "Successfully set P-core affinity for process {} (PID: {})",
                        process.name,
                        process.pid
                    );
                }
                Err(e) => {
                    log::warn!(
                        "Failed to set affinity for process {} (PID: {}): {}",
                        process.name,
                        process.pid,
                        e
                    );
                    // Mark as processed to avoid repeated attempts
                    self.cache.mark_processed(process.pid);
                }
            }
        }

        Ok(processed_count)
    }

    pub fn cleanup_cache(&mut self) -> usize {
        self.cache.cleanup()
    }

    pub fn get_cache_stats(&self) -> String {
        let stats = self.cache.stats();
        format!(
            "Cache stats - Total: {}, Processed: {}, Unprocessed: {}",
            stats.total_entries, stats.processed_count, stats.unprocessed_count
        )
    }

    fn is_whitelisted(&self, process_name: &str) -> bool {
        if self.config.whitelist.processes.is_empty() {
            return false;
        }

        match self.match_mode {
            MatchMode::Exact => self.exact_match(process_name),
            MatchMode::Wildcard => self.wildcard_match(process_name),
            MatchMode::Regex => self.regex_match(process_name),
        }
    }

    fn is_excluded(&self, process_name: &str) -> bool {
        let name_lower = process_name.to_lowercase();
        self.config
            .whitelist
            .exclude_processes
            .iter()
            .any(|excluded| name_lower.contains(&excluded.to_lowercase()))
    }

    fn exact_match(&self, process_name: &str) -> bool {
        let name_lower = process_name.to_lowercase();
        self.config
            .whitelist
            .processes
            .iter()
            .any(|pattern| pattern.to_lowercase() == name_lower)
    }

    fn wildcard_match(&self, process_name: &str) -> bool {
        use wildmatch::WildMatch;

        let name_lower = process_name.to_lowercase();
        self.config
            .whitelist
            .processes
            .iter()
            .any(|pattern| {
                let pattern_lower = pattern.to_lowercase();
                WildMatch::new(&pattern_lower).matches(&name_lower)
            })
    }

    fn regex_match(&self, process_name: &str) -> bool {
        use regex::Regex;

        self.config
            .whitelist
            .processes
            .iter()
            .any(|pattern| {
                if let Ok(re) = Regex::new(pattern) {
                    re.is_match(process_name)
                } else {
                    false
                }
            })
    }

    fn set_affinity_with_retry(&self, pid: u32, process_name: &str) -> Result<(), ServiceError> {
        let mut last_error = None;

        for attempt in 1..=self.config.advanced.retry_attempts {
            match self.affinity_manager.set_affinity_to_p_cores(pid, process_name) {
                Ok(_) => return Ok(()),
                Err(e) => {
                    last_error = Some(e);
                    if attempt < self.config.advanced.retry_attempts {
                        log::debug!(
                            "Retry {}/{} for process {} (PID: {})",
                            attempt,
                            self.config.advanced.retry_attempts,
                            process_name,
                            pid
                        );
                        std::thread::sleep(std::time::Duration::from_millis(
                            self.config.advanced.retry_delay_ms,
                        ));
                    }
                }
            }
        }

        Err(last_error.unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::types::CoreInfo;

    fn create_test_config() -> Arc<Config> {
        let mut config = Config::default();
        config.whitelist.processes = vec!["test.exe".to_string(), "*.game.exe".to_string()];
        Arc::new(config)
    }

    fn create_test_manager() -> ProcessManager {
        let config = create_test_config();
        let core_info = Arc::new(CoreInfo::all_cores(8));
        let affinity_manager = Arc::new(AffinityManager::new(core_info));
        ProcessManager::new(config, affinity_manager)
    }

    #[test]
    fn test_exact_match() {
        let manager = create_test_manager();
        assert!(manager.exact_match("test.exe"));
        assert!(manager.exact_match("TEST.EXE"));
        assert!(!manager.exact_match("other.exe"));
    }

    #[test]
    fn test_wildcard_match() {
        let manager = create_test_manager();
        assert!(manager.wildcard_match("my.game.exe"));
        assert!(manager.wildcard_match("best.game.exe"));
        assert!(!manager.wildcard_match("game.txt"));
    }

    #[test]
    fn test_exclude() {
        let manager = create_test_manager();
        assert!(manager.is_excluded("svchost.exe"));
        assert!(manager.is_excluded("SYSTEM"));
        assert!(!manager.is_excluded("chrome.exe"));
    }
}
