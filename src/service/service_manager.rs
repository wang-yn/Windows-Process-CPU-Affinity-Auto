use std::ffi::OsString;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use windows_service::service::{
    ServiceControl as WinServiceControl, ServiceControlAccept, ServiceExitCode, ServiceState,
    ServiceStatus, ServiceType,
};
use windows_service::service_control_handler::{self, ServiceControlHandlerResult};
use windows_service::{define_windows_service, service_dispatcher};

use crate::config::ConfigLoader;
use crate::cpu::{AffinityManager, CpuDetector, DetectionMode};
use crate::process::ProcessManager;
use crate::utils::ServiceError;

const SERVICE_NAME: &str = "ProcessCpuAutoService";
const SERVICE_TYPE: ServiceType = ServiceType::OWN_PROCESS;

define_windows_service!(ffi_service_main, service_main);

/// Service control state
pub struct ServiceControl {
    shutdown: Arc<Mutex<bool>>,
}

impl ServiceControl {
    pub fn new() -> Self {
        Self {
            shutdown: Arc::new(Mutex::new(false)),
        }
    }

    pub fn should_shutdown(&self) -> bool {
        *self.shutdown.lock().unwrap()
    }

    pub fn request_shutdown(&self) {
        *self.shutdown.lock().unwrap() = true;
    }
}

/// Run the Windows service
pub fn run_service() -> Result<(), ServiceError> {
    service_dispatcher::start(SERVICE_NAME, ffi_service_main).map_err(|e| {
        ServiceError::Service(format!("Failed to start service dispatcher: {}", e))
    })?;
    Ok(())
}

/// Service main entry point
fn service_main(_arguments: Vec<OsString>) {
    if let Err(e) = run_service_logic() {
        log::error!("Service error: {}", e);
    }
}

fn run_service_logic() -> Result<(), ServiceError> {
    let service_control = Arc::new(ServiceControl::new());
    let service_control_clone = Arc::clone(&service_control);

    // Define service control handler
    let event_handler = move |control_event| -> ServiceControlHandlerResult {
        match control_event {
            WinServiceControl::Stop | WinServiceControl::Shutdown => {
                log::info!("Received stop/shutdown signal");
                service_control_clone.request_shutdown();
                ServiceControlHandlerResult::NoError
            }
            WinServiceControl::Interrogate => ServiceControlHandlerResult::NoError,
            _ => ServiceControlHandlerResult::NotImplemented,
        }
    };

    // Register service control handler
    let status_handle = service_control_handler::register(SERVICE_NAME, event_handler)
        .map_err(|e| ServiceError::Service(format!("Failed to register control handler: {}", e)))?;

    // Tell Windows we're starting
    status_handle
        .set_service_status(ServiceStatus {
            service_type: SERVICE_TYPE,
            current_state: ServiceState::StartPending,
            controls_accepted: ServiceControlAccept::empty(),
            exit_code: ServiceExitCode::Win32(0),
            checkpoint: 0,
            wait_hint: Duration::from_secs(5),
            process_id: None,
        })
        .map_err(|e| ServiceError::Service(format!("Failed to set service status: {}", e)))?;

    // Initialize service
    log::info!("=== Process CPU Auto Service Starting ===");

    // Load configuration
    let config_path = get_service_config_path();
    let config = ConfigLoader::load(&config_path)?;
    let config = Arc::new(config);

    // Initialize logger with file logging
    crate::utils::logger::init_service_logger(&config.service.log_level)?;

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
    let mut process_manager = ProcessManager::new(Arc::clone(&config), affinity_manager);

    // Tell Windows we're running
    status_handle
        .set_service_status(ServiceStatus {
            service_type: SERVICE_TYPE,
            current_state: ServiceState::Running,
            controls_accepted: ServiceControlAccept::STOP | ServiceControlAccept::SHUTDOWN,
            exit_code: ServiceExitCode::Win32(0),
            checkpoint: 0,
            wait_hint: Duration::default(),
            process_id: None,
        })
        .map_err(|e| ServiceError::Service(format!("Failed to set running status: {}", e)))?;

    log::info!("Service is now running");
    log::info!("Scan interval: {}ms", config.service.scan_interval_ms);
    log::info!("Whitelisted processes: {:?}", config.whitelist.processes);

    // Main service loop
    let scan_interval = Duration::from_millis(config.service.scan_interval_ms);
    let cleanup_interval = Duration::from_secs(config.advanced.cache_cleanup_interval_secs);
    let mut last_cleanup = std::time::Instant::now();

    while !service_control.should_shutdown() {
        // Scan and process
        match process_manager.scan_and_process() {
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
            let removed = process_manager.cleanup_cache();
            if removed > 0 {
                log::debug!("Cache cleanup: removed {} stale entries", removed);
            }
            log::debug!("{}", process_manager.get_cache_stats());
            last_cleanup = std::time::Instant::now();
        }

        // Sleep before next scan
        std::thread::sleep(scan_interval);
    }

    // Tell Windows we're stopping
    status_handle
        .set_service_status(ServiceStatus {
            service_type: SERVICE_TYPE,
            current_state: ServiceState::StopPending,
            controls_accepted: ServiceControlAccept::empty(),
            exit_code: ServiceExitCode::Win32(0),
            checkpoint: 0,
            wait_hint: Duration::from_secs(5),
            process_id: None,
        })
        .map_err(|e| ServiceError::Service(format!("Failed to set stop pending: {}", e)))?;

    log::info!("Service stopped");

    // Tell Windows we're stopped
    status_handle
        .set_service_status(ServiceStatus {
            service_type: SERVICE_TYPE,
            current_state: ServiceState::Stopped,
            controls_accepted: ServiceControlAccept::empty(),
            exit_code: ServiceExitCode::Win32(0),
            checkpoint: 0,
            wait_hint: Duration::default(),
            process_id: None,
        })
        .map_err(|e| ServiceError::Service(format!("Failed to set stopped status: {}", e)))?;

    Ok(())
}

fn get_service_config_path() -> String {
    // Use ProgramData directory for service configuration
    if let Ok(program_data) = std::env::var("ProgramData") {
        format!("{}\\ProcessCpuAuto\\config.toml", program_data)
    } else {
        "C:\\ProgramData\\ProcessCpuAuto\\config.toml".to_string()
    }
}
