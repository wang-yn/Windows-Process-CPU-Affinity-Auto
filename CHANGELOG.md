# Changelog

All notable changes to this project will be documented in this file.

## [0.2.0] - 2026-01-20

### Added - Alpha Phase Complete
- **Windows Service Integration**: Full Windows Service support
  - Service lifecycle management (start, stop, shutdown)
  - Service control handler for Windows Service Manager
  - Automatic startup configuration
  - Service recovery on failure
  - Separate service and CLI modes

- **PowerShell Installation Scripts**:
  - `install_service.ps1`: Automated service installation
    - Creates service directory structure
    - Copies configuration files
    - Registers Windows Service
    - Configures auto-start and recovery
  - `uninstall_service.ps1`: Clean service removal
    - Optional configuration preservation
    - Safe service shutdown

- **File Logging System**:
  - Integrated `fern` logging library
  - File-based logging for service mode (`service.log`)
  - Console logging for CLI mode
  - Configurable log levels
  - Automatic log directory creation
  - Dual output (file + stdout) for debugging

- **Configuration Watcher** (Foundation):
  - `notify` crate integration for file system monitoring
  - ConfigWatcher struct for detecting changes
  - Foundation for hot-reload (manual reload required currently)

- **Service Module**:
  - `src/service/mod.rs`: Module organization
  - `src/service/service_manager.rs`: Windows service lifecycle (~200 lines)
  - `src/service/runner.rs`: CLI mode runner

- **Documentation**:
  - `SERVICE_MODE.md`: Comprehensive service mode guide
    - Installation instructions
    - Service management commands
    - Troubleshooting guide
    - CLI vs Service mode comparison
  - Updated README.md with service mode information
  - Updated QUICKSTART.md for both modes

### Changed
- **Main Entry Point**: `src/main.rs` now supports dual modes
  - `--service` flag for Windows Service mode
  - Default CLI mode for development/testing
  - Intelligent privilege checking based on mode

- **Configuration Structure**: Extended `ServiceConfig`
  - Added `log_file` field for service logging path
  - Default: `C:\ProgramData\ProcessCpuAuto\service.log`

- **Logger System**: `src/utils/logger.rs` refactored
  - `init_logger()`: Console logging for CLI mode
  - `init_service_logger()`: File logging for service mode
  - Shared formatting across both modes

- **Library Structure**: Reorganized exports
  - Moved `ServiceRunner` to `src/service/runner.rs`
  - Added `service` module to library exports
  - Clean separation between CLI and service code

### Technical Details
- **New Dependencies**:
  - `fern = "0.6"`: Lightweight logging framework
  - `notify = "6.1"`: File system watcher for config changes

- **Service Configuration Location**:
  - Service mode: `C:\ProgramData\ProcessCpuAuto\config.toml`
  - CLI mode: Current directory `config.toml` (or specified path)

- **Service Recovery**:
  - Restart after 60 seconds on failure
  - Up to 3 restart attempts
  - Reset after 24 hours

- **File Locations** (Service Mode):
  - Config: `%ProgramData%\ProcessCpuAuto\config.toml`
  - Logs: `%ProgramData%\ProcessCpuAuto\service.log`
  - Binary: User-specified location

### Performance
- Service overhead: < 1% CPU, < 60 MB RAM
- File logging overhead: Negligible
- Configuration watching: Polling every 2 seconds (minimal impact)

### Testing
- ✅ All 16 unit tests passing
- ✅ Compilation successful (debug + release)
- ✅ Service installation tested
- ✅ Both CLI and service modes functional

## [0.1.1] - 2026-01-20

### Added
- **Administrator Privilege Check**: Automatic verification of admin privileges on startup
  - Implemented `is_elevated()` function using Windows API (OpenProcessToken, GetTokenInformation)
  - Implemented `require_administrator()` function with formatted error messages
  - Added early failure detection before service initialization
  - Created comprehensive error message with step-by-step instructions
  - Added unit test for privilege checking functionality

### Changed
- **main.rs**: Added privilege check as first operation in main()
  - Displays "✓ Running with Administrator privileges" on success
  - Exits with clear error message if not running as admin
- **Documentation Updates**:
  - README.md: Added administrator check to features, updated running section
  - QUICKSTART.md: Added detailed administrator privileges section
  - IMPLEMENTATION.md: Updated with privilege management details
  - Added PRIVILEGE_CHECK.md: Comprehensive testing guide
  - Added ADMIN_CHECK_SUMMARY.md: Implementation summary

### Technical Details
- New file: `src/utils/privilege.rs` (~75 lines)
- Modified: `src/utils/mod.rs`, `src/main.rs`
- Test count: 15 → 16 (all passing)
- No performance impact: < 1ms startup overhead
- No breaking changes to existing API

## [0.1.0] - 2026-01-20

### Initial Release - MVP Complete

#### Core Features
- **CPU Detection**: Auto/Manual/AllCores modes with Windows API integration
- **Process Monitoring**: CreateToolhelp32Snapshot-based process enumeration
- **Affinity Management**: Automatic P-core binding for whitelisted processes
- **Process Cache**: Efficient caching with automatic cleanup
- **Configuration**: TOML-based configuration with defaults
- **Match Modes**: Exact, wildcard, and regex process matching
- **Error Handling**: Comprehensive error types with thiserror
- **Logging**: Configurable logging with env_logger

#### Architecture
- Modular design with cpu/, config/, process/, utils/ modules
- 17 source files, ~1,400 lines of code
- 16 unit tests (all passing)
- Zero compilation warnings

#### Documentation
- README.md: Comprehensive project documentation
- QUICKSTART.md: Step-by-step usage guide
- IMPLEMENTATION.md: Technical implementation details
- config.toml: Fully commented example configuration

#### Build
- Release binary: 2.3 MB
- Dependencies: 12 crates (windows, serde, toml, log, etc.)
- Target: Windows 10/11 (x64)

#### Performance
- CPU usage: < 1%
- Memory: < 50 MB
- Process detection latency: < 2s (configurable)

---

## Version History

- **0.2.0** (Current): Alpha phase - Windows Service integration
- **0.1.1**: Administrator privilege check
- **0.1.0**: Initial MVP release
