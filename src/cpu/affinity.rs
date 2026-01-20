use crate::cpu::types::CoreInfo;
use crate::utils::ServiceError;
use std::sync::Arc;
use windows::Win32::Foundation::{CloseHandle, HANDLE};
use windows::Win32::System::Threading::{
    OpenProcess, SetProcessAffinityMask, PROCESS_ACCESS_RIGHTS, PROCESS_SET_INFORMATION,
    PROCESS_QUERY_INFORMATION,
};

pub struct AffinityManager {
    core_info: Arc<CoreInfo>,
}

impl AffinityManager {
    pub fn new(core_info: Arc<CoreInfo>) -> Self {
        Self { core_info }
    }

    pub fn set_affinity_to_p_cores(&self, pid: u32, process_name: &str) -> Result<(), ServiceError> {
        self.set_affinity(pid, self.core_info.p_core_mask, process_name)
    }

    pub fn set_affinity(&self, pid: u32, affinity_mask: usize, process_name: &str) -> Result<(), ServiceError> {
        let handle = self.open_process(pid)?;

        let result = unsafe {
            SetProcessAffinityMask(handle, affinity_mask)
        };

        // Close handle
        unsafe {
            let _ = CloseHandle(handle);
        }

        if !result.as_bool() {
            return Err(ServiceError::AffinitySetting(format!(
                "Failed to set affinity mask 0x{:X} for process {} (PID: {})",
                affinity_mask, process_name, pid
            )));
        }

        log::debug!(
            "Set CPU affinity mask 0x{:X} for process {} (PID: {})",
            affinity_mask,
            process_name,
            pid
        );

        Ok(())
    }

    fn open_process(&self, pid: u32) -> Result<HANDLE, ServiceError> {
        let access_rights = PROCESS_ACCESS_RIGHTS(
            PROCESS_SET_INFORMATION.0 | PROCESS_QUERY_INFORMATION.0
        );

        let handle = unsafe {
            OpenProcess(access_rights, false, pid)
        };

        match handle {
            Ok(h) if !h.is_invalid() => Ok(h),
            _ => Err(ServiceError::AffinitySetting(format!(
                "Failed to open process with PID: {} (insufficient permissions or process doesn't exist)",
                pid
            ))),
        }
    }

    pub fn get_p_core_mask(&self) -> usize {
        self.core_info.p_core_mask
    }

    pub fn get_core_info(&self) -> Arc<CoreInfo> {
        Arc::clone(&self.core_info)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_affinity_manager_creation() {
        let p_cores = vec![0, 1, 2, 3];
        let e_cores = vec![4, 5, 6, 7];
        let core_info = Arc::new(CoreInfo::new(p_cores, e_cores));
        let manager = AffinityManager::new(core_info);

        assert_eq!(manager.get_p_core_mask(), 0x0F);
    }
}
