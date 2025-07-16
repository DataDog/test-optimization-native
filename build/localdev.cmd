@echo off
setlocal enabledelayedexpansion

REM Test Optimization Native Library - Local Development Build Script (CMD Wrapper)
REM This script calls the PowerShell version of localdev.ps1

echo Test Optimization Native Library - Local Development Build
echo.

REM Check if PowerShell is available
powershell -Command "exit 0" >nul 2>&1
if !errorlevel! neq 0 (
    echo Error: PowerShell is not available or not in PATH
    echo Please ensure PowerShell is installed and accessible
    exit /b 1
)

REM Get the directory where this script is located
set "SCRIPT_DIR=%~dp0"

REM Check if localdev.ps1 exists
if not exist "%SCRIPT_DIR%localdev.ps1" (
    echo Error: localdev.ps1 not found in %SCRIPT_DIR%
    echo Please ensure localdev.ps1 is in the same directory as this script
    exit /b 1
)

REM Show help if requested
if /i "%1"=="--help" goto :show_help
if /i "%1"=="-h" goto :show_help
if /i "%1"=="/?" goto :show_help

echo Executing PowerShell build script...
echo.

REM Execute the PowerShell script with bypass execution policy
REM This allows the script to run without requiring signed scripts
powershell -ExecutionPolicy Bypass -File "%SCRIPT_DIR%localdev.ps1" %*

REM Capture the exit code from PowerShell
set "PS_EXIT_CODE=!errorlevel!"

REM Check if PowerShell script executed successfully
if !PS_EXIT_CODE! equ 0 (
    echo.
    echo Build script completed successfully!
) else (
    echo.
    echo Build script failed with exit code !PS_EXIT_CODE!
    echo Please check the output above for error details
)

REM Exit with the same code as PowerShell
exit /b !PS_EXIT_CODE!

:show_help
echo.
echo Test Optimization Native Library - Local Development Build Script
echo.
echo This script builds the native library for local development by:
echo 1. Setting up the dd-trace-go repository
echo 2. Copying build and native files  
echo 3. Building platform-specific libraries
echo 4. Applying platform-specific optimizations
echo.
echo Usage:
echo     localdev.cmd [options]
echo.
echo Options:
echo     --help, -h, /?    Show this help message
echo.
echo Requirements:
echo     - Go 1.24 or later
echo     - Git
echo     - PowerShell
echo     - Platform-specific build tools (MinGW for Windows, etc.)
echo.
echo Output:
echo     All build artifacts will be placed in ../dev-output/
echo.
exit /b 0 