pub mod config;
pub mod cpu;
pub mod process;
pub mod utils;

pub use config::{Config, ConfigLoader};
pub use cpu::{AffinityManager, CoreInfo, CpuDetector, DetectionMode};
pub use process::{ProcessCache, ProcessManager, ProcessMonitor};
pub use utils::ServiceError;

use std::sync::Arc;
use std::time::Duration;

/// Main service runner
pub struct ServiceRunner {
    config: Arc<Config>,
    process_manager: ProcessManager,
}

impl ServiceRunner {
    pub fn new(config_path: &str) -> Result<Self, ServiceError> {
        // Load configuration
        let config = ConfigLoader::load(config_path)?;
        let config = Arc::new(config);

        // Initialize logger
        utils::logger::init_logger(&config.service.log_level);

        log::info!("=== Process CPU Auto Service Starting ===");
        log::info!("Configuration loaded from: {}", config_path);

        // Detect CPU cores
        let detection_mode = DetectionMode::from_str(&config.cpu.detection_mode);
        let core_info = CpuDetector::detect(
            detection_mode,
            config.cpu.p_cores.clone(),
            config.cpu.e_cores.clone(),
        )?;

        log::info!("CPU Detection: {}", core_info);

        // Create affinity manager
        let affinity_manager = Arc::new(AffinityManager::new(core_info));

        // Create process manager
        let process_manager = ProcessManager::new(Arc::clone(&config), affinity_manager);

        Ok(Self {
            config,
            process_manager,
        })
    }

    pub fn run(&mut self) -> Result<(), ServiceError> {
        log::info!("Service runner started. Press Ctrl+C to stop.");
        log::info!("Scan interval: {}ms", self.config.service.scan_interval_ms);
        log::info!("Whitelisted processes: {:?}", self.config.whitelist.processes);

        let scan_interval = Duration::from_millis(self.config.service.scan_interval_ms);
        let cleanup_interval = Duration::from_secs(self.config.advanced.cache_cleanup_interval_secs);
        let mut last_cleanup = std::time::Instant::now();

        loop {
            // Scan and process
            match self.process_manager.scan_and_process() {
                Ok(count) => {
                    if count > 0 {
                        log::info!("Processed {} new processes", count);
                    }
                }
                Err(e) => {
                    log::error!("Error during scan: {}", e);
                }
            }

            // Periodic cache cleanup
            if last_cleanup.elapsed() >= cleanup_interval {
                let removed = self.process_manager.cleanup_cache();
                if removed > 0 {
                    log::debug!("Cache cleanup: removed {} stale entries", removed);
                }
                log::debug!("{}", self.process_manager.get_cache_stats());
                last_cleanup = std::time::Instant::now();
            }

            // Sleep before next scan
            std::thread::sleep(scan_interval);
        }
    }

    pub fn run_once(&mut self) -> Result<usize, ServiceError> {
        self.process_manager.scan_and_process()
    }
}
