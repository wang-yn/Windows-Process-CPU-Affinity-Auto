use crate::cpu::types::{CoreInfo, DetectionMode};
use crate::utils::ServiceError;
use std::sync::Arc;

pub struct CpuDetector;

impl CpuDetector {
    pub fn detect(mode: DetectionMode, p_cores: Vec<u32>, e_cores: Vec<u32>) -> Result<Arc<CoreInfo>, ServiceError> {
        match mode {
            DetectionMode::Auto => Self::detect_auto(),
            DetectionMode::Manual => Self::detect_manual(p_cores, e_cores),
            DetectionMode::AllCores => Self::detect_all_cores(),
        }
    }

    fn detect_auto() -> Result<Arc<CoreInfo>, ServiceError> {
        log::info!("Attempting automatic CPU core detection...");

        // Try Windows API detection first
        match Self::detect_via_windows_api() {
            Ok(info) => {
                log::info!("Successfully detected CPU cores via Windows API");
                return Ok(info);
            }
            Err(e) => {
                log::warn!("Windows API detection failed: {}, falling back to all_cores mode", e);
            }
        }

        // Fall back to all cores mode
        Self::detect_all_cores()
    }

    fn detect_via_windows_api() -> Result<Arc<CoreInfo>, ServiceError> {
        use windows::Win32::System::SystemInformation::{
            GetLogicalProcessorInformationEx,
            RelationProcessorCore,
            SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX,
        };

        // Get buffer size needed
        let mut buffer_size: u32 = 0;
        unsafe {
            let _ = GetLogicalProcessorInformationEx(
                RelationProcessorCore,
                None,
                &mut buffer_size,
            );
        }

        if buffer_size == 0 {
            return Err(ServiceError::CpuDetection(
                "Failed to get processor information buffer size".to_string()
            ));
        }

        // Allocate buffer
        let mut buffer: Vec<u8> = vec![0; buffer_size as usize];
        let result = unsafe {
            GetLogicalProcessorInformationEx(
                RelationProcessorCore,
                Some(buffer.as_mut_ptr() as *mut SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX),
                &mut buffer_size,
            )
        };

        if !result.as_bool() {
            return Err(ServiceError::CpuDetection(
                "Failed to get processor information".to_string()
            ));
        }

        let mut p_cores = Vec::new();
        let mut e_cores = Vec::new();
        let mut offset = 0usize;
        let mut core_index = 0u32;

        while offset < buffer_size as usize {
            let info = unsafe {
                &*(buffer.as_ptr().add(offset) as *const SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX)
            };

            if info.Relationship == RelationProcessorCore {
                // Check efficiency class (Windows 11+)
                // EfficiencyClass: 0 = E-core, 1 = P-core
                let efficiency_class = unsafe { info.Anonymous.Processor.EfficiencyClass };

                let group_mask = unsafe { &info.Anonymous.Processor.GroupMask };
                if group_mask.len() > 0 {
                    let mask = group_mask[0].Mask as usize;

                    // Count logical processors in this core
                    for bit in 0..64 {
                        if mask & (1 << bit) != 0 {
                            if efficiency_class == 1 {
                                p_cores.push(core_index);
                            } else {
                                e_cores.push(core_index);
                            }
                            core_index += 1;
                        }
                    }
                }
            }

            offset += info.Size as usize;
        }

        if p_cores.is_empty() && e_cores.is_empty() {
            return Err(ServiceError::CpuDetection(
                "No cores detected".to_string()
            ));
        }

        // If only one type detected, treat as non-hybrid
        if p_cores.is_empty() {
            p_cores = e_cores.clone();
            e_cores.clear();
        }

        log::info!("Detected P-cores: {:?}, E-cores: {:?}", p_cores, e_cores);
        Ok(Arc::new(CoreInfo::new(p_cores, e_cores)))
    }

    fn detect_manual(p_cores: Vec<u32>, e_cores: Vec<u32>) -> Result<Arc<CoreInfo>, ServiceError> {
        if p_cores.is_empty() {
            return Err(ServiceError::CpuDetection(
                "Manual mode requires at least P-cores to be specified".to_string()
            ));
        }

        log::info!("Using manual CPU configuration: P-cores: {:?}, E-cores: {:?}", p_cores, e_cores);
        Ok(Arc::new(CoreInfo::new(p_cores, e_cores)))
    }

    fn detect_all_cores() -> Result<Arc<CoreInfo>, ServiceError> {
        use windows::Win32::System::SystemInformation::GetSystemInfo;
        use windows::Win32::System::SystemInformation::SYSTEM_INFO;

        let mut sys_info = SYSTEM_INFO::default();
        unsafe {
            GetSystemInfo(&mut sys_info);
        }

        let num_processors = sys_info.dwNumberOfProcessors;
        log::info!("Using all_cores mode with {} processors", num_processors);

        Ok(Arc::new(CoreInfo::all_cores(num_processors)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manual_detection() {
        let p_cores = vec![0, 1, 2, 3];
        let e_cores = vec![4, 5, 6, 7];
        let result = CpuDetector::detect(DetectionMode::Manual, p_cores, e_cores);
        assert!(result.is_ok());

        let info = result.unwrap();
        assert_eq!(info.p_cores.len(), 4);
        assert_eq!(info.e_cores.len(), 4);
    }

    #[test]
    fn test_all_cores_detection() {
        let result = CpuDetector::detect(DetectionMode::AllCores, Vec::new(), Vec::new());
        assert!(result.is_ok());

        let info = result.unwrap();
        assert!(info.total_cores > 0);
        assert!(!info.has_hybrid_architecture());
    }
}
