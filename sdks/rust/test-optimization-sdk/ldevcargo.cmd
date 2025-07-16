@echo off
setlocal enabledelayedexpansion

REM Check if this is a help request
if "%~1"=="-h" goto :show_help
if "%~1"=="--help" goto :show_help

REM Check if PowerShell is available
where powershell >nul 2>&1
if errorlevel 1 (
    echo Error: PowerShell is not available on this system
    echo Please install PowerShell or use a different method to run cargo commands
    exit /b 1
)

REM Run the PowerShell script with execution policy bypass
powershell -NoProfile -ExecutionPolicy Bypass -File "%~dp0ldevcargo.ps1" %*
exit /b %errorlevel%

:show_help
echo Rust SDK Local Development Build Script - CMD Wrapper
echo.
echo This wrapper runs the PowerShell version of ldevcargo with execution policy bypass.
echo.
echo Usage:
echo     ldevcargo.cmd [OPTIONS] [CARGO_ARGS...]
echo.
echo Options:
echo     -h, --help          Show this help message and exit
echo     -sn, --skip-native  Skip building native libraries (use existing builds)
echo.
echo Examples:
echo     ldevcargo.cmd build                    # Build native libs + cargo build
echo     ldevcargo.cmd test                     # Build native libs + cargo test
echo     ldevcargo.cmd -sn check                # Skip native build, run cargo check
echo     ldevcargo.cmd --skip-native clippy     # Skip native build, run cargo clippy
echo     ldevcargo.cmd build --release          # Build native libs + cargo build --release
echo.
echo Notes:
echo     - Requires PowerShell to be installed
echo     - Runs with -NoProfile -ExecutionPolicy Bypass for clean execution
echo     - All arguments are passed to the PowerShell script
echo.
exit /b 0 