# Uninstall ProcessCpuAutoService
# Must be run as Administrator

param(
    [string]$ServiceName = "ProcessCpuAutoService",
    [switch]$RemoveConfig = $false
)

# Check administrator privileges
$currentPrincipal = New-Object Security.Principal.WindowsPrincipal([Security.Principal.WindowsIdentity]::GetCurrent())
if (-not $currentPrincipal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)) {
    Write-Error "This script must be run as Administrator"
    exit 1
}

Write-Host "Uninstalling ProcessCpuAutoService..." -ForegroundColor Yellow
Write-Host ""

# Check if service exists
$service = Get-Service -Name $ServiceName -ErrorAction SilentlyContinue
if (-not $service) {
    Write-Warning "Service '$ServiceName' not found. Nothing to uninstall."
    exit 0
}

# Stop service if running
if ($service.Status -eq 'Running') {
    Write-Host "Stopping service..."
    try {
        Stop-Service -Name $ServiceName -Force -ErrorAction Stop
        Write-Host "Service stopped." -ForegroundColor Green
    } catch {
        Write-Error "Failed to stop service: $_"
        exit 1
    }
}

# Delete service
Write-Host "Removing service..."
$result = sc.exe delete $ServiceName

if ($LASTEXITCODE -eq 0) {
    Write-Host "Service removed successfully!" -ForegroundColor Green

    # Optionally remove configuration
    if ($RemoveConfig) {
        $configDir = "$env:ProgramData\ProcessCpuAuto"
        if (Test-Path $configDir) {
            Write-Host "Removing configuration directory: $configDir"
            $confirm = Read-Host "This will delete all configuration and log files. Continue? (Y/N)"
            if ($confirm -eq 'Y' -or $confirm -eq 'y') {
                Remove-Item -Path $configDir -Recurse -Force
                Write-Host "Configuration removed." -ForegroundColor Green
            } else {
                Write-Host "Configuration preserved at: $configDir" -ForegroundColor Cyan
            }
        }
    } else {
        Write-Host "Configuration preserved at: $env:ProgramData\ProcessCpuAuto" -ForegroundColor Cyan
        Write-Host "To remove configuration, run: .\uninstall_service.ps1 -RemoveConfig"
    }

    Write-Host ""
    Write-Host "Uninstallation complete!" -ForegroundColor Green

} else {
    Write-Error "Failed to remove service. Error code: $LASTEXITCODE"
    exit 1
}
