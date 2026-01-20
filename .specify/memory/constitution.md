# Process CPU Auto Constitution

<!--
Sync Impact Report:
- Version: 1.0.0 (Initial ratification)
- Constitution Type: Initial creation for Windows system utility
- Principles Defined: 7 core principles
- Sections Added: Core Principles, System Constraints, Quality Standards, Governance
- Templates Status:
  ✅ plan-template.md: Reviewed - constitution check section compatible
  ✅ spec-template.md: Reviewed - user story format aligns with principles
  ✅ tasks-template.md: Reviewed - task organization supports testing discipline
- Follow-up TODOs: None
- Ratification Date: 2026-01-20
-->

## Core Principles

### I. System Reliability First

The service runs as a Windows background service with minimal user intervention. Every feature MUST prioritize stability and graceful degradation over functionality.

**Non-negotiable rules:**
- All Windows API calls MUST have proper error handling with graceful fallbacks
- Service MUST NOT crash on permission denied errors (expected for system processes)
- Configuration errors MUST result in safe defaults, not service failure
- Resource leaks (handles, memory) are treated as critical bugs

**Rationale:** As a system service that auto-starts with Windows, reliability outweighs features. Users depend on "set and forget" behavior.

### II. Security and Privilege Handling

The service requires Administrator privileges by design. All privilege-sensitive operations MUST be explicit and validated.

**Non-negotiable rules:**
- Administrator privilege check MUST occur before any system operations
- All process handle operations MUST use minimum required access rights
- Process enumeration MUST handle protected/system processes gracefully
- Configuration file MUST NOT be writable by non-admin users in production
- No hardcoded credentials or sensitive data in configuration files

**Rationale:** Operating with elevated privileges requires explicit security discipline to prevent privilege escalation or system instability.

### III. Configuration-Driven Behavior

All runtime behavior MUST be controllable via TOML configuration. Code changes should only be needed for new features, not behavior tuning.

**Non-negotiable rules:**
- Every configurable parameter MUST have a documented default value
- Invalid configuration MUST produce clear validation errors with suggestions
- Configuration changes MUST be hot-reloadable where safe (future: via file watcher)
- Minimal configuration MUST produce working defaults for common use cases
- Configuration schema changes MUST maintain backward compatibility or provide migration path

**Rationale:** Users have diverse CPU architectures and application needs. Configuration flexibility enables broad applicability without code changes.

### IV. Test-First for Core Logic (NON-NEGOTIABLE)

Core CPU detection, process matching, and affinity calculation logic MUST have passing unit tests BEFORE implementation is merged.

**Non-negotiable rules:**
- Write failing tests for CPU mask calculation FIRST
- Write failing tests for process matching (exact/wildcard/regex) FIRST
- Write failing tests for whitelist/exclude logic FIRST
- Integration tests for Windows API wrappers are OPTIONAL but encouraged
- Tests MUST NOT require Administrator privileges to run (use mocks for privilege-gated code)

**Rationale:** Core logic bugs can cause incorrect CPU binding, degrading performance instead of improving it. Tests prevent regressions.

### V. Platform-Specific Observability

As a Windows-only service, logging and diagnostics MUST align with Windows ecosystem conventions.

**Non-negotiable rules:**
- Console mode (CLI): Use structured console logging (env_logger/fern)
- Service mode: File logging with rotation (fern) + Windows Event Log for critical events
- All affinity operations MUST log: process name, PID, core mask, success/failure
- Configuration loading MUST log: file path, detection mode, core counts
- Performance metrics (cache size, processing rate) MUST be logged at INFO level every N iterations

**Rationale:** Headless service operation requires comprehensive logging for debugging. Windows Event Log integration aids enterprise monitoring.

### VI. Windows API Abstraction

All Windows API usage MUST be wrapped in Rust modules with safety guarantees and error mapping.

**Non-negotiable rules:**
- Direct `unsafe` Windows API calls MUST be confined to dedicated wrapper functions
- Windows API errors MUST be mapped to typed Rust errors (ServiceError enum)
- Handle lifecycle MUST be managed via RAII (Drop trait)
- API wrappers MUST be unit-testable via traits or cfg(test) mocks
- API documentation MUST reference the Windows API function name for traceability

**Rationale:** Windows APIs are inherently unsafe. Localized abstraction boundaries improve safety and testability.

### VII. Incremental Complexity

Start with simplest viable implementation. Justify complexity before adding it.

**Non-negotiable rules:**
- MVP: CLI mode with manual intervention (COMPLETED)
- Alpha: Windows Service integration with file logging (COMPLETED)
- Beta: Configuration hot-reload, metrics, event log integration (PLANNED)
- Advanced features (GUI, remote management) require documented use cases
- No speculative features without user requests or clear roadmap placement

**Rationale:** System utilities benefit from stability over feature bloat. Each complexity tier must prove value before advancing.

## System Constraints

### Platform Requirements
- **Target Platform**: Windows 10/11 x64 only
- **Privilege Level**: Administrator (SYSTEM account for service mode)
- **CPU Architecture**: Intel 12th gen+ or AMD Ryzen with hybrid cores (graceful fallback for uniform CPUs)
- **Rust Version**: Stable channel, MSRV 1.70+

### Performance Standards
- **CPU Usage**: < 1% average during monitoring (measured via Task Manager)
- **Memory Usage**: < 50 MB working set
- **Process Detection Latency**: < 2 seconds (limited by scan_interval_ms configuration)
- **Cache Cleanup**: Automated cleanup every 5 minutes (configurable)

### Compatibility Constraints
- **Configuration Format**: TOML only (no YAML/JSON to minimize dependencies)
- **Windows API**: Use windows-rs crate exclusively (no winapi crate mixing)
- **Service Framework**: windows-service crate for service mode
- **Logging**: env_logger (CLI) + fern (service mode) for structured logs

## Quality Standards

### Code Quality Gates
- **Compilation**: MUST compile with zero warnings on stable Rust
- **Tests**: Core logic MUST have ≥80% coverage (measured via tarpaulin or similar)
- **Clippy**: MUST pass `cargo clippy --all-targets --all-features` with no warnings
- **Format**: MUST pass `cargo fmt --check` (enforce via CI)
- **Documentation**: Public APIs MUST have doc comments with examples

### Error Handling Policy
- **Windows API Errors**: Map to ServiceError enum with context
- **Configuration Errors**: Provide specific validation messages (e.g., "Invalid core ID: 99, max available: 12")
- **Permission Errors**: Log as INFO (expected for system processes), not ERROR
- **Critical Errors**: Service MUST NOT panic; use graceful shutdown with error code

### Logging Discipline
- **ERROR**: Service-level failures (config load failure, service crash)
- **WARN**: Expected failures (permission denied for system process, CPU detection fallback)
- **INFO**: Normal operations (affinity set, cache cleanup, config reload)
- **DEBUG**: Detailed diagnostics (each process scanned, cache hits)
- **TRACE**: Full API call tracing (disabled by default)

## Governance

### Constitution Authority
This constitution supersedes all other development practices for the Process CPU Auto project. When in conflict, constitution principles take precedence.

### Amendment Procedure
1. **Proposal**: Document proposed change with rationale in GitHub issue
2. **Discussion**: Minimum 48-hour community review period
3. **Approval**: Requires maintainer approval + demonstration that existing features still align
4. **Migration**: If principles change, update affected code within 2 release cycles
5. **Versioning**: Bump constitution version per semantic versioning (see below)

### Version Numbering
- **MAJOR**: Principle removed or redefined (breaking governance change)
- **MINOR**: New principle added or existing principle expanded
- **PATCH**: Clarifications, wording improvements, typo fixes

### Compliance Review
- **Pull Requests**: Reviewers MUST verify alignment with Core Principles (use checklist in PR template)
- **Feature Proposals**: MUST map to existing principles or propose principle amendment
- **Complexity Justification**: Features violating Principle VII (Incremental Complexity) MUST document justification
- **Quarterly Audits**: Review constitution adherence in codebase every quarter

### Development Guidance
- Runtime development guidance is documented in [README.md](../../README.md) and [QUICKSTART.md](../../QUICKSTART.md)
- Implementation details are tracked in [IMPLEMENTATION.md](../../IMPLEMENTATION.md)
- Service mode specifics are in [SERVICE_MODE.md](../../SERVICE_MODE.md)

**Version**: 1.0.0 | **Ratified**: 2026-01-20 | **Last Amended**: 2026-01-20
