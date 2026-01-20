# Install ProcessCpuAutoService as Windows Service
# Must be run as Administrator

param(
    [string]$BinaryPath = ".\target\release\process_cpu_auto.exe",
    [string]$ServiceName = "ProcessCpuAutoService",
    [string]$DisplayName = "Process CPU Affinity Auto Service",
    [string]$Description = "Automatically binds whitelisted processes to Performance cores"
)

# Check administrator privileges
$currentPrincipal = New-Object Security.Principal.WindowsPrincipal([Security.Principal.WindowsIdentity]::GetCurrent())
if (-not $currentPrincipal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)) {
    Write-Error "This script must be run as Administrator"
    exit 1
}

Write-Host "Installing ProcessCpuAutoService..." -ForegroundColor Green
Write-Host ""

# Get absolute path to binary
$BinaryPath = Resolve-Path $BinaryPath -ErrorAction Stop
Write-Host "Binary path: $BinaryPath"

# Create ProgramData directory
$configDir = "$env:ProgramData\ProcessCpuAuto"
if (-not (Test-Path $configDir)) {
    Write-Host "Creating configuration directory: $configDir"
    New-Item -Path $configDir -ItemType Directory -Force | Out-Null
}

# Copy config file if it doesn't exist
$configSource = ".\config.toml"
$configDest = "$configDir\config.toml"
if (-not (Test-Path $configDest)) {
    if (Test-Path $configSource) {
        Write-Host "Copying configuration file to: $configDest"
        Copy-Item $configSource $configDest
    } else {
        Write-Warning "config.toml not found in current directory. Please create one manually at: $configDest"
    }
}

# Check if service already exists
$existingService = Get-Service -Name $ServiceName -ErrorAction SilentlyContinue
if ($existingService) {
    Write-Host "Service already exists. Stopping and removing..." -ForegroundColor Yellow
    if ($existingService.Status -eq 'Running') {
        Stop-Service -Name $ServiceName -Force
    }
    sc.exe delete $ServiceName | Out-Null
    Start-Sleep -Seconds 2
}

# Create service
Write-Host "Creating Windows Service..."
$result = sc.exe create $ServiceName `
    binPath= "`"$BinaryPath`" --service" `
    start= auto `
    DisplayName= $DisplayName

if ($LASTEXITCODE -eq 0) {
    Write-Host "Service created successfully!" -ForegroundColor Green

    # Set service description
    sc.exe description $ServiceName $Description | Out-Null

    # Configure service recovery options
    sc.exe failure $ServiceName reset= 86400 actions= restart/60000/restart/60000/restart/60000 | Out-Null

    Write-Host ""
    Write-Host "Service Configuration:" -ForegroundColor Cyan
    Write-Host "  Name: $ServiceName"
    Write-Host "  Display Name: $DisplayName"
    Write-Host "  Binary: $BinaryPath"
    Write-Host "  Config: $configDest"
    Write-Host "  Log: $configDir\service.log"
    Write-Host "  Startup: Automatic"
    Write-Host "  Recovery: Restart on failure"
    Write-Host ""

    # Ask if user wants to start the service
    $start = Read-Host "Start the service now? (Y/N)"
    if ($start -eq 'Y' -or $start -eq 'y') {
        Write-Host "Starting service..."
        Start-Service -Name $ServiceName
        Start-Sleep -Seconds 2

        $service = Get-Service -Name $ServiceName
        if ($service.Status -eq 'Running') {
            Write-Host "Service started successfully!" -ForegroundColor Green
            Write-Host "Check log file at: $configDir\service.log"
        } else {
            Write-Warning "Service did not start. Check the log file for errors."
        }
    } else {
        Write-Host "Service installed but not started."
        Write-Host "To start manually, run: Start-Service $ServiceName"
    }

    Write-Host ""
    Write-Host "Installation complete!" -ForegroundColor Green
    Write-Host ""
    Write-Host "Useful commands:" -ForegroundColor Cyan
    Write-Host "  Start:   Start-Service $ServiceName"
    Write-Host "  Stop:    Stop-Service $ServiceName"
    Write-Host "  Status:  Get-Service $ServiceName"
    Write-Host "  Logs:    Get-Content '$configDir\service.log' -Tail 50"

} else {
    Write-Error "Failed to create service. Error code: $LASTEXITCODE"
    exit 1
}
