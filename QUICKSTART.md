# Quick Start Guide

## Prerequisites

- Windows 10/11 (**Administrator privileges required**)
- Rust toolchain installed (if building from source)

## Important: Administrator Privileges

**This service MUST be run as Administrator** to modify process CPU affinity.

If you try to run without Administrator privileges, you will see:

```
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
```

The program will automatically check for administrator privileges on startup and exit with a clear error message if not running as Administrator.

## Building

```bash
cargo build --release
```

Binary location: `target/release/process_cpu_auto.exe`

## Configuration

Edit `config.toml`:

```toml
[service]
scan_interval_ms = 1000    # Check for new processes every second
log_level = "info"          # info, debug, warn, error

[cpu]
detection_mode = "auto"     # Let Windows API detect P/E cores
# For manual mode:
# detection_mode = "manual"
# p_cores = [0, 1, 2, 3, 4, 5, 6, 7]
# e_cores = [8, 9, 10, 11]

[whitelist]
match_mode = "wildcard"     # exact, wildcard, or regex
processes = [
    "chrome.exe",           # Exact match
    "msedge.exe",
    "code.exe",
    "*.game.exe",           # Wildcard: any file ending with .game.exe
]
exclude_processes = [
    "system",
    "svchost.exe",
]
```

## Running

### Step 1: Open Administrator Command Prompt

Press `Win+X`, select "Terminal (Admin)" or "Command Prompt (Admin)"

### Step 2: Navigate to Project Directory

```bash
cd C:\code0\projects\process_cpu_auto
```

### Step 3: Run the Service

```bash
.\target\release\process_cpu_auto.exe
```

Or specify a custom config path:

```bash
.\target\release\process_cpu_auto.exe path\to\custom_config.toml
```

## Verifying It Works

### Test 1: Basic Functionality

1. Add `notepad.exe` to your whitelist in `config.toml`:
   ```toml
   [whitelist]
   processes = ["notepad.exe"]
   ```

2. Start the service:
   ```bash
   .\target\release\process_cpu_auto.exe
   ```

3. In another terminal, launch Notepad:
   ```bash
   notepad.exe
   ```

4. Check the service logs. You should see:
   ```
   [INFO] Successfully set P-core affinity for process notepad.exe (PID: xxxxx)
   ```

### Test 2: Verify CPU Affinity

1. Launch the service and a whitelisted process

2. Open Task Manager (Ctrl+Shift+Esc)

3. Go to "Details" tab

4. Right-click your process → "Set affinity"

5. Verify only P-cores are selected

## Understanding the Output

### Startup Messages
```
[INFO] Configuration loaded from: config.toml
[INFO] CPU Detection: CoreInfo { ... }
[INFO] Service runner started. Press Ctrl+C to stop.
[INFO] Whitelisted processes: ["chrome.exe", "code.exe"]
```

### Process Detection
```
[INFO] Successfully set P-core affinity for process chrome.exe (PID: 12345)
[DEBUG] Set CPU affinity mask 0xFF for process chrome.exe (PID: 12345)
```

### Cache Management
```
[DEBUG] Cleaned up 5 stale process entries from cache
[DEBUG] Cache stats - Total: 10, Processed: 8, Unprocessed: 2
```

### Errors
```
[WARN] Failed to set affinity for process system (PID: 4): Permission denied
```
This is normal for system processes.

## Common Issues

### "Permission denied" for all processes
**Solution**: Run as Administrator

### Process not being caught
**Solutions**:
- Check process name matches exactly (use Task Manager to verify)
- Try wildcard mode: `*.exe`
- Check exclude list
- Increase log level to "debug" to see all processes

### CPU cores not detected
**Solution**: Use manual mode:
```toml
[cpu]
detection_mode = "manual"
p_cores = [0, 1, 2, 3]  # Adjust for your CPU
e_cores = []
```

### Service crashes on startup
**Check**:
- config.toml exists and is valid TOML
- Running as Administrator
- Windows version (Windows 10/11 required)

## Stopping the Service

Press `Ctrl+C` in the terminal window

## Performance Tips

### Lower CPU Usage
Increase scan interval:
```toml
scan_interval_ms = 2000  # Check every 2 seconds instead of 1
```

### Faster Detection
Decrease scan interval:
```toml
scan_interval_ms = 500   # Check every 0.5 seconds
```

### Larger Cache
Increase cleanup interval:
```toml
cache_cleanup_interval_secs = 600  # Clean every 10 minutes
```

## Next Steps

### Testing with Real Applications

1. **Browsers**: Add `chrome.exe`, `firefox.exe`, `msedge.exe`
2. **IDEs**: Add `code.exe`, `devenv.exe`, `idea64.exe`
3. **Games**: Use wildcards like `*.game.exe` or `*Game*.exe`

### Monitoring

Set log level to "debug" to see all operations:
```toml
log_level = "debug"
```

### Troubleshooting

Enable detailed logging and reproduce the issue:
```bash
RUST_LOG=trace .\target\release\process_cpu_auto.exe
```

## Example Configurations

### Gaming Setup
```toml
[whitelist]
processes = [
    "*.game.exe",
    "*Game*.exe",
    "*.Game.exe",
    "steam.exe",
    "EpicGamesLauncher.exe",
]
```

### Development Setup
```toml
[whitelist]
processes = [
    "code.exe",
    "devenv.exe",
    "rider64.exe",
    "idea64.exe",
    "chrome.exe",
    "msedge.exe",
]
```

### Everything Setup
```toml
[whitelist]
match_mode = "wildcard"
processes = ["*.exe"]
exclude_processes = [
    "system",
    "svchost.exe",
    "dwm.exe",
    "csrss.exe",
]
```

## FAQ

**Q: Do I need to run this at startup?**
A: Not in MVP version. Future versions will support Windows Service installation.

**Q: Will this work on non-hybrid CPUs?**
A: Yes! Use `detection_mode = "all_cores"` or `manual` mode.

**Q: Can I whitelist processes by path?**
A: Not yet. Currently only process names (e.g., `chrome.exe`) are supported.

**Q: Does this affect already-running processes?**
A: By default, no. Set `process_existing_on_startup = true` to process them.

**Q: How much CPU/RAM does this use?**
A: Very little - typically < 1% CPU and < 50 MB RAM.

## Getting Help

1. Check logs with `log_level = "debug"`
2. Review README.md for detailed information
3. Check IMPLEMENTATION.md for technical details
4. Review issue tracker or create a new issue

---

**Remember**: This tool requires Administrator privileges to modify process affinity!
