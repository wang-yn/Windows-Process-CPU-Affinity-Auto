# Windows Process CPU Affinity Auto Service

A Windows background service that automatically monitors new processes and binds whitelisted applications to Performance cores (P-cores), improving performance for CPU-intensive applications on hybrid architecture CPUs.

## Features

- **Automatic CPU Detection**: Detects P-cores and E-cores using Windows API
- **Process Monitoring**: Continuously monitors for new processes
- **Whitelist Management**: Configure processes via TOML file
- **Multiple Match Modes**: Exact, wildcard, and regex matching
- **Automatic Affinity Setting**: Binds whitelisted processes to P-cores
- **Flexible Configuration**: Easy TOML-based configuration
- **Low Overhead**: Minimal CPU and memory usage
- **Administrator Check**: Automatically verifies administrator privileges on startup
- **Windows Service Mode**: Run as background service with automatic startup
- **File Logging**: Comprehensive logging with file output for service mode
- **Configuration Hot-Reload**: Detect configuration changes (foundation implemented)

## Architecture

```
src/
├── main.rs                 # Entry point
├── lib.rs                  # Library interface
├── cpu/                    # CPU detection and affinity management
│   ├── types.rs           # Core data structures
│   ├── detector.rs        # P/E core detection
│   └── affinity.rs        # Affinity mask setting
├── process/               # Process monitoring
│   ├── monitor.rs         # Process enumeration
│   ├── manager.rs         # Process management
│   └── cache.rs           # Process cache
├── config/                # Configuration
│   ├── settings.rs        # Config structures
│   └── loader.rs          # TOML loading
└── utils/                 # Utilities
    ├── error.rs           # Error types
    └── logger.rs          # Logging
```

## Quick Start

### Prerequisites

- Windows 10/11 (Windows 11 recommended for automatic P/E core detection)
- Rust toolchain (for building from source)
- Administrator privileges (required for setting process affinity)

### Building

```bash
# Clone the repository
git clone <repository-url>
cd process_cpu_auto

# Build release version
cargo build --release

# The binary will be at: target/release/process_cpu_auto.exe
```

### Configuration

Edit `config.toml`:

```toml
[service]
scan_interval_ms = 1000
log_level = "info"

[cpu]
detection_mode = "auto"  # auto, manual, or all_cores

[whitelist]
match_mode = "wildcard"
processes = [
    "chrome.exe",
    "code.exe",
    "*.game.exe",
]
```

### Running

The service supports two modes:

#### CLI Mode (Testing/Development)

**Important: Must run as Administrator!**

The service will automatically check for administrator privileges on startup. If not running as Administrator, it will display a helpful error message and exit.

```bash
# Run in command-line mode (for testing)
# Right-click Command Prompt → "Run as administrator"
.\target\release\process_cpu_auto.exe

# Or specify a custom config path
.\target\release\process_cpu_auto.exe path\to\config.toml
```

#### Service Mode (Production)

Install and run as a Windows Service for production use:

```powershell
# Install service (run as Administrator)
.\install_service.ps1

# Manage service
Start-Service ProcessCpuAutoService
Stop-Service ProcessCpuAutoService
Get-Service ProcessCpuAutoService

# View logs
Get-Content "C:\ProgramData\ProcessCpuAuto\service.log" -Tail 50

# Uninstall service
.\uninstall_service.ps1
```

**Service Benefits:**
- Automatic startup with Windows
- Runs in background (no console window)
- Automatic recovery on failure
- File-based logging
- Managed by Windows Service Manager

For detailed service mode instructions, see [`SERVICE_MODE.md`](SERVICE_MODE.md).

## Configuration Guide

### CPU Detection Modes

1. **auto** (Recommended for Windows 11)
   - Automatically detects P-cores and E-cores using Windows API
   - Falls back to `all_cores` if detection fails

2. **manual**
   - Manually specify P-cores and E-cores
   ```toml
   [cpu]
   detection_mode = "manual"
   p_cores = [0, 1, 2, 3, 4, 5, 6, 7]
   e_cores = [8, 9, 10, 11]
   ```

3. **all_cores**
   - Use all available CPU cores
   - No P/E distinction

### Match Modes

1. **exact**: Case-insensitive exact matching
   ```toml
   processes = ["chrome.exe", "firefox.exe"]
   ```

2. **wildcard**: Support `*` and `?` wildcards
   ```toml
   processes = ["*.game.exe", "my?.exe"]
   ```

3. **regex**: Regular expression matching
   ```toml
   processes = ["^chrome.*\\.exe$"]
   ```

## How It Works

1. **Startup**
   - Load configuration from `config.toml`
   - Detect CPU cores (P-cores and E-cores)
   - Initialize process monitor and cache

2. **Monitoring Loop**
   - Scan all running processes every `scan_interval_ms`
   - Identify new processes not in cache
   - Check against whitelist and exclusion list
   - Set CPU affinity to P-cores for matched processes
   - Cache processed processes to avoid redundant operations

3. **Cache Management**
   - Periodically clean up stale process entries
   - Remove entries for processes that have exited

## Testing

```bash
# Run unit tests
cargo test

# Run with verbose logging
cargo run -- config.toml

# Test with a specific process
# 1. Start the service
# 2. Launch a whitelisted process (e.g., notepad.exe)
# 3. Check logs for affinity setting confirmation
```

## Performance

- **CPU Usage**: < 1% (typical)
- **Memory Usage**: < 50 MB
- **Process Detection Latency**: < 2 seconds (depends on scan_interval_ms)

## Limitations

- Requires Administrator privileges
- Cannot modify system processes (svchost.exe, system, etc.)
- Windows API CPU detection requires Windows 11 for best results
- Some protected processes may be inaccessible

## Troubleshooting

### "Permission denied" errors
- Ensure the program is run as Administrator
- Some system processes cannot be modified

### P-cores not detected correctly
- Switch to `manual` mode and specify cores explicitly
- Check Windows version (Windows 11 has better API support)

### Process not being caught
- Check process name matches whitelist exactly
- Try `wildcard` or `regex` match mode
- Ensure process is not in `exclude_processes` list
- Check logs for detailed information

## Project Status

**Current Phase**: Alpha - Windows Service Integration Complete

Implemented:
- ✅ Configuration management (TOML)
- ✅ CPU detection (auto, manual, all_cores modes)
- ✅ Process monitoring (CreateToolhelp32Snapshot)
- ✅ Affinity setting
- ✅ Process cache management
- ✅ Multiple match modes (exact, wildcard, regex)
- ✅ Command-line interface
- ✅ Administrator privilege check
- ✅ **Windows Service mode**
- ✅ **File logging with rotation**
- ✅ **Configuration watcher (foundation)**
- ✅ **Service install/uninstall scripts**

Planned (Beta Phase):
- ⏳ Active configuration hot-reload
- ⏳ Windows Event Log integration
- ⏳ Performance metrics
- ⏳ GUI management interface (optional)

## License

[Specify your license here]

## Contributing

Contributions are welcome! Please feel free to submit issues and pull requests.

## Credits

Developed as a solution for improving application performance on Intel hybrid architecture CPUs (12th gen and later).
