use crate::utils::ServiceError;
use windows::Win32::Foundation::{CloseHandle, HANDLE};
use windows::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, Process32FirstW, Process32NextW, PROCESSENTRY32W, TH32CS_SNAPPROCESS,
};

#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub parent_pid: u32,
}

pub struct ProcessMonitor;

impl ProcessMonitor {
    pub fn get_all_processes() -> Result<Vec<ProcessInfo>, ServiceError> {
        let snapshot = Self::create_snapshot()?;
        let processes = Self::enumerate_processes(snapshot)?;

        unsafe {
            let _ = CloseHandle(snapshot);
        }

        Ok(processes)
    }

    fn create_snapshot() -> Result<HANDLE, ServiceError> {
        let snapshot = unsafe {
            CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0)
        };

        match snapshot {
            Ok(h) if !h.is_invalid() => Ok(h),
            _ => Err(ServiceError::ProcessMonitoring(
                "Failed to create process snapshot".to_string()
            )),
        }
    }

    fn enumerate_processes(snapshot: HANDLE) -> Result<Vec<ProcessInfo>, ServiceError> {
        let mut processes = Vec::new();
        let mut entry = PROCESSENTRY32W {
            dwSize: std::mem::size_of::<PROCESSENTRY32W>() as u32,
            ..Default::default()
        };

        // Get first process
        let result = unsafe { Process32FirstW(snapshot, &mut entry) };
        if !result.as_bool() {
            return Err(ServiceError::ProcessMonitoring(
                "Failed to get first process".to_string()
            ));
        }

        // Process first entry
        if let Some(info) = Self::parse_process_entry(&entry) {
            processes.push(info);
        }

        // Enumerate remaining processes
        loop {
            entry.dwSize = std::mem::size_of::<PROCESSENTRY32W>() as u32;
            let result = unsafe { Process32NextW(snapshot, &mut entry) };

            if !result.as_bool() {
                break;
            }

            if let Some(info) = Self::parse_process_entry(&entry) {
                processes.push(info);
            }
        }

        Ok(processes)
    }

    fn parse_process_entry(entry: &PROCESSENTRY32W) -> Option<ProcessInfo> {
        let pid = entry.th32ProcessID;

        // Skip system idle process (PID 0)
        if pid == 0 {
            return None;
        }

        // Extract process name from szExeFile
        let name = Self::extract_process_name(&entry.szExeFile);

        Some(ProcessInfo {
            pid,
            name,
            parent_pid: entry.th32ParentProcessID,
        })
    }

    fn extract_process_name(sz_exe_file: &[u16; 260]) -> String {
        // Find the null terminator
        let len = sz_exe_file.iter().position(|&c| c == 0).unwrap_or(260);

        // Convert to String
        String::from_utf16_lossy(&sz_exe_file[..len])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_processes() {
        let result = ProcessMonitor::get_all_processes();
        assert!(result.is_ok());

        let processes = result.unwrap();
        assert!(!processes.is_empty());

        // Should contain at least current process
        let current_pid = std::process::id();
        let found = processes.iter().any(|p| p.pid == current_pid);
        assert!(found, "Current process not found in process list");
    }

    #[test]
    fn test_process_names() {
        let processes = ProcessMonitor::get_all_processes().unwrap();

        for process in &processes {
            assert!(!process.name.is_empty());
            assert!(process.pid > 0);
        }

        // Print some processes for manual inspection
        println!("Sample processes:");
        for process in processes.iter().take(5) {
            println!("  PID: {}, Name: {}", process.pid, process.name);
        }
    }
}
