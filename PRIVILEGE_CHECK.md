# Administrator Privilege Check - Testing Guide

## Overview

The service now includes automatic administrator privilege verification on startup. If the program is not running with administrator privileges, it will display a clear error message and exit gracefully.

## Implementation Details

### Files Modified/Added

1. **src/utils/privilege.rs** (NEW)
   - `is_elevated()`: Checks if current process has administrator privileges
   - `require_administrator()`: Checks privileges and displays error if not elevated

2. **src/utils/mod.rs** (MODIFIED)
   - Exports privilege checking functions

3. **src/main.rs** (MODIFIED)
   - Calls `require_administrator()` before initializing the service

### Windows API Used

- `OpenProcessToken`: Opens the access token of the current process
- `GetTokenInformation`: Retrieves token elevation information
- `TOKEN_ELEVATION`: Structure containing elevation status

## Testing the Privilege Check

### Test 1: Running WITHOUT Administrator (Expected Failure)

1. Open a regular Command Prompt (not as Administrator)
2. Navigate to project directory:
   ```bash
   cd C:\code0\projects\process_cpu_auto
   ```
3. Run the program:
   ```bash
   .\target\release\process_cpu_auto.exe
   ```

**Expected Output:**
```
Windows Process CPU Affinity Auto Service
==========================================

╔════════════════════════════════════════════════════════════════╗
║                    Administrator Privileges Required           ║
╚════════════════════════════════════════════════════════════════╝

This service requires Administrator privileges to set process
CPU affinity. Please run this program as Administrator.

To run as Administrator:
  1. Right-click on Command Prompt or PowerShell
  2. Select 'Run as administrator'
  3. Navigate to the program directory
  4. Run the program again

Or use this command:
  powershell -Command "Start-Process cmd -Verb RunAs"

Error: PermissionDenied("Administrator privileges required to set process affinity")
```

**Exit Code:** Non-zero (indicates error)

### Test 2: Running WITH Administrator (Expected Success)

1. Right-click on Command Prompt
2. Select "Run as administrator"
3. Navigate to project directory:
   ```bash
   cd C:\code0\projects\process_cpu_auto
   ```
4. Run the program:
   ```bash
   .\target\release\process_cpu_auto.exe
   ```

**Expected Output:**
```
Windows Process CPU Affinity Auto Service
==========================================

✓ Running with Administrator privileges

[INFO] Configuration loaded from: config.toml
[INFO] CPU Detection: CoreInfo { ... }
[INFO] Service runner started. Press Ctrl+C to stop.
...
```

**Exit Code:** 0 (when stopped with Ctrl+C)

## Error Message Design

The error message is designed to be:
- **Clear**: Immediately explains the problem
- **Actionable**: Provides step-by-step instructions
- **Helpful**: Includes a PowerShell command for quick elevation
- **Professional**: Formatted with box drawing characters

## Code Example

The privilege check is performed at the very start of `main()`:

```rust
fn main() -> Result<(), ServiceError> {
    println!("Windows Process CPU Affinity Auto Service");
    println!("==========================================");
    println!();

    // Check administrator privileges first
    process_cpu_auto::utils::privilege::require_administrator()?;

    println!("✓ Running with Administrator privileges");
    println!();

    // Rest of the initialization...
}
```

## Integration with Service Error Handling

The privilege check integrates with the existing error handling system:

```rust
pub enum ServiceError {
    // ... other variants ...

    #[error("Permission denied: {0}")]
    PermissionDenied(String),
}
```

This allows the privilege check to be part of the normal error flow.

## Benefits

1. **Early Detection**: Fails fast before any initialization
2. **Clear Messaging**: Users immediately know what's wrong
3. **Helpful Guidance**: Provides instructions on how to fix
4. **No Partial Initialization**: Avoids confusing errors later
5. **Professional UX**: Clean, formatted error messages

## Testing Checklist

- [x] Compile successfully
- [x] All unit tests pass (16/16)
- [x] Error message displays correctly when run without admin
- [x] Service starts normally when run with admin
- [x] Error message formatting is clean and readable
- [x] Documentation updated (README, QUICKSTART)

## Known Limitations

1. The privilege check only verifies elevation at startup
2. If privileges are somehow revoked while running, operations will fail with different errors
3. The check uses Windows-specific APIs and won't work on other platforms

## Future Enhancements

Potential improvements for future versions:
- Add a command-line flag to skip the check (for testing)
- Add automatic elevation request (UAC prompt)
- Add privilege check before each sensitive operation
- Log privilege status to file

## Conclusion

The administrator privilege check provides a much better user experience by:
- Failing fast with a clear message
- Providing actionable guidance
- Preventing confusing errors later in execution
- Maintaining professional appearance

Users no longer need to guess why the program fails - they get immediate, helpful feedback.
