# Windows Service Mode - Installation and Usage Guide

## Overview

The Process CPU Affinity Auto Service can now run as a Windows Service, allowing it to:
- Start automatically with Windows
- Run in the background without a console window
- Log to file in `C:\ProgramData\ProcessCpuAuto\service.log`
- Be managed using Windows Service tools

## Prerequisites

- Windows 10/11
- Administrator privileges
- Release build of `process_cpu_auto.exe`

## Installation

### Step 1: Build the Release Version

```bash
cargo build --release
```

The binary will be at: `target\release\process_cpu_auto.exe`

### Step 2: Install as Windows Service

Open PowerShell as Administrator and run:

```powershell
.\install_service.ps1
```

The script will:
1. Create service directory: `C:\ProgramData\ProcessCpuAuto`
2. Copy configuration file
3. Register the Windows Service
4. Configure automatic startup
5. Set up service recovery (auto-restart on failure)
6. Optionally start the service

#### Installation Options

```powershell
# Install with custom binary path
.\install_service.ps1 -BinaryPath "C:\path\to\process_cpu_auto.exe"

# Install with custom service name
.\install_service.ps1 -ServiceName "MyCustomServiceName"
```

### Step 3: Configure the Service

Edit the configuration file at:
```
C:\ProgramData\ProcessCpuAuto\config.toml
```

After editing, restart the service for changes to take effect:
```powershell
Restart-Service ProcessCpuAutoService
```

## Service Management

### Start the Service

```powershell
Start-Service ProcessCpuAutoService
```

### Stop the Service

```powershell
Stop-Service ProcessCpuAutoService
```

### Check Service Status

```powershell
Get-Service ProcessCpuAutoService
```

### Restart the Service

```powershell
Restart-Service ProcessCpuAutoService
```

### View Service Details

```powershell
Get-Service ProcessCpuAutoService | Format-List *
```

## Viewing Logs

### View Last 50 Lines

```powershell
Get-Content "C:\ProgramData\ProcessCpuAuto\service.log" -Tail 50
```

### View Logs in Real-Time

```powershell
Get-Content "C:\ProgramData\ProcessCpuAuto\service.log" -Wait -Tail 20
```

### Open Log File in Notepad

```powershell
notepad "C:\ProgramData\ProcessCpuAuto\service.log"
```

## Uninstallation

### Uninstall (Keep Configuration)

```powershell
.\uninstall_service.ps1
```

This will:
- Stop the service
- Remove the service registration
- **Keep** configuration and log files

### Uninstall (Remove Everything)

```powershell
.\uninstall_service.ps1 -RemoveConfig
```

This will:
- Stop the service
- Remove the service registration
- **Delete** all configuration and log files

## Troubleshooting

### Service Won't Start

1. **Check the log file:**
   ```powershell
   Get-Content "C:\ProgramData\ProcessCpuAuto\service.log" -Tail 50
   ```

2. **Check Windows Event Viewer:**
   - Open Event Viewer
   - Go to: Windows Logs → Application
   - Look for errors from "ProcessCpuAutoService"

3. **Verify configuration:**
   ```powershell
   notepad "C:\ProgramData\ProcessCpuAuto\config.toml"
   ```

4. **Test in CLI mode first:**
   ```bash
   .\target\release\process_cpu_auto.exe "C:\ProgramData\ProcessCpuAuto\config.toml"
   ```

### Service Crashes

The service is configured to automatically restart on failure. Check logs to identify the issue:

```powershell
Get-Content "C:\ProgramData\ProcessCpuAuto\service.log" -Tail 100
```

### Permission Issues

If you see "Access Denied" errors in logs:
1. Verify the service is running as SYSTEM (default)
2. Check that the binary has proper permissions
3. Ensure configuration directory is accessible

## Service Configuration

The service uses configuration from:
```
C:\ProgramData\ProcessCpuAuto\config.toml
```

Important settings for service mode:

```toml
[service]
scan_interval_ms = 1000      # How often to check for new processes
log_level = "info"            # Log verbosity
log_file = "C:\\ProgramData\\ProcessCpuAuto\\service.log"

[whitelist]
processes = [
    "chrome.exe",
    "msedge.exe",
    # Add your processes here
]
```

## CLI Mode vs Service Mode

### CLI Mode (Development/Testing)
```bash
.\target\release\process_cpu_auto.exe [config_path]
```
- Runs in console window
- Logs to console
- Stops when console is closed
- Requires manual start
- Good for testing

### Service Mode (Production)
```powershell
# Install once
.\install_service.ps1

# Runs automatically
# No console window
# Logs to file
# Starts with Windows
# Managed by Windows Service Manager
```

## Differences from CLI Mode

| Feature | CLI Mode | Service Mode |
|---------|----------|--------------|
| Runs at startup | No | Yes (Automatic) |
| Console window | Yes | No (Background) |
| Logging | Console | File (`service.log`) |
| Management | Manual | Windows Services |
| Shutdown | Ctrl+C | Service Stop |
| Recovery | None | Auto-restart |

## Service Recovery Settings

The service is configured to automatically restart on failure:
- First failure: Restart after 1 minute
- Second failure: Restart after 1 minute
- Subsequent failures: Restart after 1 minute
- Reset failure count: After 24 hours

## File Locations

| Item | Location |
|------|----------|
| Service Binary | User-specified (usually `target\release\process_cpu_auto.exe`) |
| Configuration | `C:\ProgramData\ProcessCpuAuto\config.toml` |
| Log File | `C:\ProgramData\ProcessCpuAuto\service.log` |
| Install Script | `install_service.ps1` |
| Uninstall Script | `uninstall_service.ps1` |

## Advanced: Manual Service Installation

If you prefer manual installation:

```cmd
sc create ProcessCpuAutoService ^
    binPath= "C:\path\to\process_cpu_auto.exe --service" ^
    start= auto ^
    DisplayName= "Process CPU Affinity Auto Service"

sc description ProcessCpuAutoService "Automatically binds whitelisted processes to Performance cores"

sc failure ProcessCpuAutoService reset= 86400 actions= restart/60000/restart/60000/restart/60000
```

## Advanced: Service Account

By default, the service runs as SYSTEM. To change:

```powershell
sc.exe config ProcessCpuAutoService obj= ".\LocalService"
```

Note: The service requires administrator privileges to set process affinity.

## Verification

After installation, verify the service:

1. **Check service status:**
   ```powershell
   Get-Service ProcessCpuAutoService
   ```
   Should show: Status = Running

2. **Check log file:**
   ```powershell
   Get-Content "C:\ProgramData\ProcessCpuAuto\service.log" -Tail 20
   ```
   Should show initialization messages

3. **Test functionality:**
   - Launch a whitelisted process (e.g., `chrome.exe`)
   - Check log for affinity setting confirmation
   - Verify in Task Manager → Details → Set Affinity

## FAQ

**Q: Can I run both CLI and Service mode?**
A: No, only one instance should run at a time to avoid conflicts.

**Q: How do I update the service?**
A: Stop the service, replace the binary, then start again:
```powershell
Stop-Service ProcessCpuAutoService
# Replace binary
Start-Service ProcessCpuAutoService
```

**Q: Can I change the configuration without restarting?**
A: Currently no, you must restart the service after configuration changes.

**Q: Where are crash dumps stored?**
A: Check Windows Event Viewer (Application log) for service errors.

**Q: How do I enable debug logging?**
A: Edit config.toml, set `log_level = "debug"`, then restart service.

## Next Steps

After installation:
1. Configure whitelisted processes in `config.toml`
2. Restart the service
3. Monitor the log file
4. Test with your applications

---

For more information, see:
- `README.md` - General documentation
- `QUICKSTART.md` - Getting started guide
- `config.toml` - Configuration reference
