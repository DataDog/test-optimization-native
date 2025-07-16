# PowerShell Local Development Cargo Script for Rust SDK
param(
    [Parameter(ValueFromRemainingArguments = $true)]
    [string[]]$RemainingArgs = @()
)

# Enable strict mode and stop on errors
$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

# Colors for output (using Write-Host with colors)
function Write-ColorOutput {
    param(
        [string]$Message,
        [string]$Color = "White"
    )
    Write-Host $Message -ForegroundColor $Color
}

function Write-Blue { param([string]$Message) Write-ColorOutput $Message "Blue" }
function Write-Green { param([string]$Message) Write-ColorOutput $Message "Green" }
function Write-Yellow { param([string]$Message) Write-ColorOutput $Message "Yellow" }
function Write-Red { param([string]$Message) Write-ColorOutput $Message "Red" }

# Function to display help
function Show-Help {
    Write-Host @"
Rust SDK Local Development Build Script

This script builds native libraries and runs cargo commands with dev mode enabled.

Usage:
    .\ldevcargo.ps1 [OPTIONS] [CARGO_ARGS...]
    pwsh -NoProfile .\ldevcargo.ps1 [OPTIONS] [CARGO_ARGS...]  (recommended)

Options:
    -h, --help          Show this help message and exit
    -sn, --skip-native  Skip building native libraries (use existing builds)

Examples:
    .\ldevcargo.ps1 build                    # Build native libs + cargo build
    .\ldevcargo.ps1 test                     # Build native libs + cargo test
    .\ldevcargo.ps1 -sn check                # Skip native build, run cargo check
    .\ldevcargo.ps1 --skip-native clippy     # Skip native build, run cargo clippy
    .\ldevcargo.ps1 build --release          # Build native libs + cargo build --release

Notes:
    - Native libraries are built using ../../../build/localdev.ps1
    - TEST_OPTIMIZATION_DEV_MODE=1 is automatically set
    - The --skip-native/-sn flag is filtered out before passing args to cargo
    - Use 'pwsh -NoProfile' to avoid PowerShell profile interference

"@
}

# Parse arguments to check for --skip-native and help
$SkipNative = $false
$CargoArgs = @()

foreach ($arg in $RemainingArgs) {
    switch ($arg) {
        { $_ -in @("-h", "--help") } {
            Show-Help
            exit 0
        }
        { $_ -in @("-sn", "--skip-native") } {
            $SkipNative = $true
        }
        default {
            $CargoArgs += $arg
        }
    }
}

Write-Blue "=== Rust SDK Local Development Build ==="

# Run localdev.ps1 first to build native libraries (unless skipped)
if ($SkipNative) {
    Write-Yellow "Skipping native library build (--skip-native specified)"
} else {
    $LocaldevScript = "../../../build/localdev.ps1"

    if (Test-Path $LocaldevScript) {
        Write-Blue "Building native libraries first..."
        $currentLocation = Get-Location
        Set-Location "../../../build"
        try {
            & .\localdev.ps1
            Write-Green "✓ Native libraries built"
        } finally {
            Set-Location $currentLocation
        }
    } else {
        Write-Red "Error: localdev.ps1 not found at $LocaldevScript"
        exit 1
    }
}

Write-Blue "Running cargo with dev mode..."
$env:TEST_OPTIMIZATION_DEV_MODE = "1"

# Run cargo with filtered arguments
if ($CargoArgs.Count -gt 0) {
    & cargo @CargoArgs
} else {
    & cargo
} 