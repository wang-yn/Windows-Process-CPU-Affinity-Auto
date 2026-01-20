# Quick Deployment Guide - Version 0.2.0

## For Production Use (Windows Service)

### 1. Build
```bash
cargo build --release
```

### 2. Install Service
```powershell
# Run as Administrator
.\install_service.ps1
```

### 3. Configure
Edit: `C:\ProgramData\ProcessCpuAuto\config.toml`
```toml
[whitelist]
processes = [
    "chrome.exe",
    "msedge.exe",
    "code.exe",
    # Add your processes here
]
```

### 4. Start
```powershell
Start-Service ProcessCpuAutoService
```

### 5. Verify
```powershell
# Check status
Get-Service ProcessCpuAutoService

# View logs
Get-Content "C:\ProgramData\ProcessCpuAuto\service.log" -Tail 20
```

---

## For Development/Testing (CLI Mode)

```bash
# Run as Administrator
.\target\release\process_cpu_auto.exe

# Or with custom config
.\target\release\process_cpu_auto.exe path\to\config.toml
```

---

## Service Management

```powershell
# Start
Start-Service ProcessCpuAutoService

# Stop
Stop-Service ProcessCpuAutoService

# Restart
Restart-Service ProcessCpuAutoService

# Status
Get-Service ProcessCpuAutoService

# Logs (tail)
Get-Content "C:\ProgramData\ProcessCpuAuto\service.log" -Tail 50

# Logs (follow)
Get-Content "C:\ProgramData\ProcessCpuAuto\service.log" -Wait -Tail 20
```

---

## Uninstallation

```powershell
# Keep configuration
.\uninstall_service.ps1

# Remove everything
.\uninstall_service.ps1 -RemoveConfig
```

---

## File Locations

| Item | Path |
|------|------|
| Binary | `target\release\process_cpu_auto.exe` |
| Config | `C:\ProgramData\ProcessCpuAuto\config.toml` |
| Logs | `C:\ProgramData\ProcessCpuAuto\service.log` |

---

## Quick Troubleshooting

### Service won't start
1. Check logs: `Get-Content "C:\ProgramData\ProcessCpuAuto\service.log" -Tail 50`
2. Verify config: `notepad "C:\ProgramData\ProcessCpuAuto\config.toml"`
3. Test in CLI mode first

### Process not being caught
1. Enable debug logging: Set `log_level = "debug"` in config
2. Restart service
3. Check log for process detection messages

### Permission errors
- Service runs as SYSTEM (has full privileges)
- CLI mode requires Administrator

---

For complete documentation, see:
- **SERVICE_MODE.md**: Full service guide
- **README.md**: General documentation
- **QUICKSTART.md**: Getting started
