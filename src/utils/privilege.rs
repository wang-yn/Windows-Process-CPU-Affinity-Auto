use windows::Win32::Foundation::{CloseHandle, HANDLE};
use windows::Win32::Security::{
    GetTokenInformation, TokenElevation, TOKEN_ELEVATION, TOKEN_QUERY,
};
use windows::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};

/// Check if the current process is running with administrator privileges
pub fn is_elevated() -> bool {
    unsafe {
        let mut token: HANDLE = HANDLE::default();

        // Open the current process token
        let result = OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token);
        if !result.as_bool() {
            return false;
        }

        // Query token elevation information
        let mut elevation = TOKEN_ELEVATION { TokenIsElevated: 0 };
        let mut return_length: u32 = 0;

        let result = GetTokenInformation(
            token,
            TokenElevation,
            Some(&mut elevation as *mut _ as *mut _),
            std::mem::size_of::<TOKEN_ELEVATION>() as u32,
            &mut return_length,
        );

        // Close the token handle
        let _ = CloseHandle(token);

        if !result.as_bool() {
            return false;
        }

        elevation.TokenIsElevated != 0
    }
}

/// Check administrator privileges and exit with error message if not elevated
pub fn require_administrator() -> Result<(), crate::utils::ServiceError> {
    if !is_elevated() {
        eprintln!("\n╔════════════════════════════════════════════════════════════════╗");
        eprintln!("║                    Administrator Privileges Required           ║");
        eprintln!("╚════════════════════════════════════════════════════════════════╝");
        eprintln!();
        eprintln!("This service requires Administrator privileges to set process");
        eprintln!("CPU affinity. Please run this program as Administrator.");
        eprintln!();
        eprintln!("To run as Administrator:");
        eprintln!("  1. Right-click on Command Prompt or PowerShell");
        eprintln!("  2. Select 'Run as administrator'");
        eprintln!("  3. Navigate to the program directory");
        eprintln!("  4. Run the program again");
        eprintln!();
        eprintln!("Or use this command:");
        eprintln!("  powershell -Command \"Start-Process cmd -Verb RunAs\"");
        eprintln!();

        return Err(crate::utils::ServiceError::PermissionDenied(
            "Administrator privileges required to set process affinity".to_string()
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_elevation_check() {
        // This test will pass or fail depending on how tests are run
        // Just verify the function doesn't panic
        let _ = is_elevated();
    }
}
