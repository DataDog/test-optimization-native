# PowerShell Local Development Build Script for Test Optimization Native Library
param(
    [switch]$Help
)

if ($Help) {
    Write-Host @"
Test Optimization Native Library - Local Development Build Script

This script builds the native library for local development by:
1. Setting up the dd-trace-go repository
2. Copying build and native files
3. Building platform-specific libraries
4. Applying platform-specific optimizations

Usage:
    .\localdev.ps1

Requirements:
    - Go 1.24 or later
    - Git
    - Platform-specific build tools (MinGW for Windows, etc.)

Output:
    All build artifacts will be placed in ../dev-output/
"@
    exit 0
}

# Enable strict mode and stop on errors
$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

# Configuration
$DEV_DIR = "../dev"
$OUTPUT_DIR = "../dev-output"
$TOOLS_DIR = "../tools"
$REPO_URL = "https://github.com/DataDog/dd-trace-go.git"
$BRANCH = "main"
$TARGET_DIR = "$DEV_DIR/internal/civisibility/native"
$UPX_VERSION = "5.0.0"

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

# Platform detection
$PLATFORM = [System.Environment]::OSVersion.Platform
$ARCH = [System.Environment]::GetEnvironmentVariable("PROCESSOR_ARCHITECTURE")
if (-not $ARCH) {
    $ARCH = $env:PROCESSOR_ARCHITECTURE
    if (-not $ARCH) {
        $ARCH = "AMD64"  # Default assumption
    }
}

# Normalize platform names
$PlatformName = switch ($PLATFORM) {
    "Win32NT" { "Windows" }
    "Unix" { 
        if ($IsLinux) { "Linux" }
        elseif ($IsMacOS) { "Darwin" }
        else { "Unix" }
    }
    default { "Windows" }  # Default to Windows for PowerShell
}

Write-Blue "=== Test Optimization Native Library - Local Development Build ==="
Write-Yellow "Detected platform: $PlatformName ($ARCH)"

# Check if Go is installed
try {
    $goVersion = (go version 2>$null)
    if (-not $goVersion) {
        throw "Go not found"
    }
    $goVersionNumber = ($goVersion -split '\s+')[2] -replace '^go', ''
    Write-Green "✓ Found Go version: $goVersionNumber"
} catch {
    Write-Red "Error: Go is not installed. Please install Go 1.24 or later."
    exit 1
}

# Platform-specific setup
switch ($PlatformName) {
    "Windows" {
        Write-Green "✓ Running on Windows"
    }
    "Darwin" {
        Write-Green "✓ Running on macOS"
        # Check if brew and coreutils are available
        try {
            $null = Get-Command brew -ErrorAction Stop
            try {
                & brew list coreutils 2>$null | Out-Null
            } catch {
                Write-Yellow "Installing coreutils..."
                & brew install coreutils
            }
        } catch {
            Write-Yellow "Warning: Homebrew not found. You may need to install coreutils manually."
        }
    }
    "Linux" {
        Write-Green "✓ Running on Linux"
    }
    default {
        Write-Yellow "Warning: Unknown platform $PlatformName. Proceeding anyway..."
    }
}

# Function to check if directory is a git repository
function Test-GitRepository {
    param([string]$Path)
    return Test-Path (Join-Path $Path ".git")
}

# Function to get current branch
function Get-CurrentBranch {
    param([string]$Path)
    try {
        $currentDir = Get-Location
        Set-Location $Path
        $branch = & git rev-parse --abbrev-ref HEAD 2>$null
        Set-Location $currentDir
        return $branch
    } catch {
        Set-Location $currentDir
        return ""
    }
}

# Function to download and setup UPX
function Set-UpUPX {
    $upxDir = "$TOOLS_DIR/upx-$UPX_VERSION"
    $upxBinary = ""
    
    # Determine platform-specific UPX details
    switch ($PlatformName) {
        "Darwin" {
            $upxArchive = "upx-$UPX_VERSION-macos.tar.xz"
            $upxUrl = "https://github.com/upx/upx/releases/download/v$UPX_VERSION/$upxArchive"
            $upxBinary = "$upxDir/upx"
        }
        "Linux" {
            $upxArchive = "upx-$UPX_VERSION-amd64_linux.tar.xz"
            $upxUrl = "https://github.com/upx/upx/releases/download/v$UPX_VERSION/$upxArchive"
            $upxBinary = "$upxDir/upx"
        }
        "Windows" {
            $upxArchive = "upx-$UPX_VERSION-win64.zip"
            $upxUrl = "https://github.com/upx/upx/releases/download/v$UPX_VERSION/$upxArchive"
            $upxBinary = "$upxDir/upx-$UPX_VERSION-win64/upx.exe"
        }
        default {
            Write-Yellow "UPX not available for platform $PlatformName"
            return $null
        }
    }
    
    # Check if UPX is already available
    if (Test-Path $upxBinary) {
        Write-Green "✓ Found cached UPX at $upxBinary"
        return $upxBinary
    }
    
    Write-Yellow "Downloading UPX $UPX_VERSION for $PlatformName..."
    
    # Create tools directory
    if (-not (Test-Path $TOOLS_DIR)) {
        New-Item -ItemType Directory -Path $TOOLS_DIR -Force | Out-Null
    }
    
    # Download UPX
    $tempFile = "$TOOLS_DIR/$upxArchive"
    try {
        Invoke-WebRequest -Uri $upxUrl -OutFile $tempFile -UseBasicParsing
    } catch {
        Write-Red "Error: Failed to download UPX"
        return $null
    }
    
    if (-not (Test-Path $tempFile)) {
        Write-Red "Error: Failed to download UPX"
        return $null
    }
    
    # Extract UPX
    Write-Yellow "Extracting UPX..."
    try {
        if ($upxArchive -like "*.zip") {
            Expand-Archive -Path $tempFile -DestinationPath $upxDir -Force
        } elseif ($upxArchive -like "*.tar.xz") {
            # For tar.xz files, we need to use external tools or handle differently
            if (-not (Test-Path $upxDir)) {
                New-Item -ItemType Directory -Path $upxDir -Force | Out-Null
            }
            # Try using tar if available
            try {
                & tar -xf $tempFile -C $upxDir --strip-components=1
            } catch {
                Write-Red "Error: tar not found or failed to extract"
                return $null
            }
        }
    } catch {
        Write-Red "Error: UPX extraction failed"
        return $null
    }
    
    # Clean up download
    Remove-Item $tempFile -Force
    
    # Verify extraction
    if (Test-Path $upxBinary) {
        Write-Green "✓ UPX $UPX_VERSION installed to $upxBinary"
        return $upxBinary
    } else {
        Write-Red "Error: UPX extraction failed"
        return $null
    }
}

Write-Blue "--- Setting up dd-trace-go repository ---"

if (Test-Path $DEV_DIR) {
    if (Test-GitRepository $DEV_DIR) {
        Write-Green "✓ Found existing dd-trace-go repository in $DEV_DIR"
        
        # Check current branch
        $currentBranch = Get-CurrentBranch $DEV_DIR
        if ($currentBranch -ne $BRANCH) {
            Write-Yellow "Current branch is '$currentBranch', switching to '$BRANCH'..."
            $currentDir = Get-Location
            Set-Location $DEV_DIR
            & git fetch origin
            & git checkout $BRANCH
            Set-Location $currentDir
        }
        
        # Pull latest changes
        Write-Yellow "Pulling latest changes from $BRANCH..."
        $currentDir = Get-Location
        Set-Location $DEV_DIR
        & git pull origin $BRANCH
        Set-Location $currentDir
    } else {
        Write-Red "Error: $DEV_DIR exists but is not a git repository"
        Write-Yellow "Please remove $DEV_DIR directory and run this script again"
        exit 1
    }
} else {
    Write-Yellow "Cloning dd-trace-go repository..."
    & git clone --branch $BRANCH $REPO_URL $DEV_DIR
    Write-Green "✓ Repository cloned successfully"
}

Write-Blue "--- Preparing build environment ---"

# Create target directory
Write-Yellow "Creating target directory: $TARGET_DIR"
if (-not (Test-Path $TARGET_DIR)) {
    New-Item -ItemType Directory -Path $TARGET_DIR -Force | Out-Null
}

# Copy build files
Write-Yellow "Copying build files..."
Copy-Item -Path "./*" -Destination $TARGET_DIR -Recurse -Force
Write-Green "✓ Build files copied"

# Copy native files
Write-Yellow "Copying native files..."
Copy-Item -Path "../native/*" -Destination $TARGET_DIR -Recurse -Force
Write-Green "✓ Native files copied"

# Create output directory at repo root
Write-Yellow "Creating output directory: $OUTPUT_DIR"
if (-not (Test-Path $OUTPUT_DIR)) {
    New-Item -ItemType Directory -Path $OUTPUT_DIR -Force | Out-Null
}

# Create tools directory for cached tools
Write-Yellow "Creating tools directory: $TOOLS_DIR"
if (-not (Test-Path $TOOLS_DIR)) {
    New-Item -ItemType Directory -Path $TOOLS_DIR -Force | Out-Null
}

# Get absolute path to output directory before changing directories
$ABS_OUTPUT_DIR = (Resolve-Path $OUTPUT_DIR).Path

# Change to target directory
$originalLocation = Get-Location
Set-Location $TARGET_DIR

# Set common environment variables
$env:CGO_ENABLED = "1"

Write-Blue "--- Building Native Libraries ---"

# Function to build static library
function Build-StaticLibrary {
    param(
        [string]$GOOS,
        [string]$GOARCH,
        [string]$Extension,
        [string]$OutputName,
        [string]$CFlags,
        [string]$LDFlags
    )
    
    Write-Yellow "Building $GOOS-$GOARCH static library..."
    
    $env:GOOS = $GOOS
    $env:GOARCH = $GOARCH
    $env:CGO_CFLAGS = $CFlags
    $env:CGO_LDFLAGS = $LDFlags
    
    $outputPath = "$ABS_OUTPUT_DIR/$OutputName"
    if (-not (Test-Path $outputPath)) {
        New-Item -ItemType Directory -Path $outputPath -Force | Out-Null
    }
    
    & go build -tags civisibility_native -buildmode=c-archive -ldflags="-s -w" -gcflags="all=-l" -o "$outputPath/libtestoptimization.$Extension" (Get-ChildItem "*.go")
    
    Write-Green "✓ $GOOS-$GOARCH static library built"
}

# Function to build dynamic library
function Build-DynamicLibrary {
    param(
        [string]$GOOS,
        [string]$GOARCH,
        [string]$Extension,
        [string]$OutputName,
        [string]$CFlags,
        [string]$LDFlags,
        [string]$StripCommand = ""
    )
    
    Write-Yellow "Building $GOOS-$GOARCH dynamic library..."
    
    $env:GOOS = $GOOS
    $env:GOARCH = $GOARCH
    $env:CGO_CFLAGS = $CFlags
    $env:CGO_LDFLAGS = $LDFlags
    
    $outputPath = "$ABS_OUTPUT_DIR/$OutputName"
    if (-not (Test-Path $outputPath)) {
        New-Item -ItemType Directory -Path $outputPath -Force | Out-Null
    }
    
    & go build -tags civisibility_native -buildmode=c-shared -ldflags="-s -w" -gcflags="all=-l" -o "$outputPath/libtestoptimization.$Extension" (Get-ChildItem "*.go")
    
    if ($StripCommand -and (Get-Command ($StripCommand -split '\s+')[0] -ErrorAction SilentlyContinue)) {
        $stripArgs = ($StripCommand -split '\s+')[1..($StripCommand -split '\s+').Length]
        & ($StripCommand -split '\s+')[0] @stripArgs "$outputPath/libtestoptimization.$Extension"
    }
    
    Write-Green "✓ $GOOS-$GOARCH dynamic library built"
}

# Platform-specific builds
switch ($PlatformName) {
    "Darwin" {
        Write-Blue "--- Building for macOS and iOS ---"
        
        # macOS builds
        $MACOS_CFLAGS = "-mmacosx-version-min=11.0 -O2 -Os -DNDEBUG -fdata-sections -ffunction-sections"
        $MACOS_LDFLAGS = "-Wl,-x"
        
        Build-StaticLibrary "darwin" "arm64" "a" "macos-arm64-libtestoptimization-static" $MACOS_CFLAGS $MACOS_LDFLAGS
        Build-DynamicLibrary "darwin" "arm64" "dylib" "macos-arm64-libtestoptimization-dynamic" $MACOS_CFLAGS $MACOS_LDFLAGS "strip -x"
        
        Build-StaticLibrary "darwin" "amd64" "a" "macos-x64-libtestoptimization-static" $MACOS_CFLAGS $MACOS_LDFLAGS
        Build-DynamicLibrary "darwin" "amd64" "dylib" "macos-x64-libtestoptimization-dynamic" $MACOS_CFLAGS $MACOS_LDFLAGS "strip -x"
        
        # iOS build
        Write-Yellow "Building iOS library..."
        $env:GOOS = "ios"
        $env:GOARCH = "arm64"
        $env:CGO_ENABLED = "1"
        
        $iosPath = "$ABS_OUTPUT_DIR/ios-libtestoptimization-static"
        if (-not (Test-Path $iosPath)) {
            New-Item -ItemType Directory -Path $iosPath -Force | Out-Null
        }
        
        & go build -tags civisibility_native -buildmode=c-archive -ldflags="-s -w" -gcflags="all=-l" -o "$iosPath/libtestoptimization.a" (Get-ChildItem "*.go")
        Write-Green "✓ iOS library built"
        
        # Create universal binaries
        Write-Yellow "Creating universal binaries..."
        
        # Universal static library
        $universalStaticPath = "$ABS_OUTPUT_DIR/macos-libtestoptimization-static"
        if (-not (Test-Path $universalStaticPath)) {
            New-Item -ItemType Directory -Path $universalStaticPath -Force | Out-Null
        }
        
        & lipo -create "$ABS_OUTPUT_DIR/macos-arm64-libtestoptimization-static/libtestoptimization.a" "$ABS_OUTPUT_DIR/macos-x64-libtestoptimization-static/libtestoptimization.a" -output "$universalStaticPath/libtestoptimization.a"
        Copy-Item "$ABS_OUTPUT_DIR/macos-arm64-libtestoptimization-static/libtestoptimization.h" "$universalStaticPath/libtestoptimization.h"
        
        # Universal dynamic library
        $universalDynamicPath = "$ABS_OUTPUT_DIR/macos-libtestoptimization-dynamic"
        if (-not (Test-Path $universalDynamicPath)) {
            New-Item -ItemType Directory -Path $universalDynamicPath -Force | Out-Null
        }
        
        & lipo -create "$ABS_OUTPUT_DIR/macos-arm64-libtestoptimization-dynamic/libtestoptimization.dylib" "$ABS_OUTPUT_DIR/macos-x64-libtestoptimization-dynamic/libtestoptimization.dylib" -output "$universalDynamicPath/libtestoptimization.dylib"
        Copy-Item "$ABS_OUTPUT_DIR/macos-arm64-libtestoptimization-dynamic/libtestoptimization.h" "$universalDynamicPath/libtestoptimization.h"
        
        Write-Green "✓ Universal binaries created"
        
        # Clean up individual architecture folders
        Write-Yellow "Cleaning up individual architecture folders..."
        Remove-Item "$ABS_OUTPUT_DIR/macos-arm64-libtestoptimization-static" -Recurse -Force
        Remove-Item "$ABS_OUTPUT_DIR/macos-arm64-libtestoptimization-dynamic" -Recurse -Force
        Remove-Item "$ABS_OUTPUT_DIR/macos-x64-libtestoptimization-static" -Recurse -Force
        Remove-Item "$ABS_OUTPUT_DIR/macos-x64-libtestoptimization-dynamic" -Recurse -Force
        Write-Green "✓ Individual architecture folders cleaned up"
    }
    
    "Linux" {
        Write-Blue "--- Building for Linux and Android ---"
        
        # Linux builds
        $LINUX_CFLAGS = "-O2 -Os -s -DNDEBUG -fdata-sections -ffunction-sections"
        $LINUX_LDFLAGS = "-s -Wl,--gc-sections"
        
        Build-StaticLibrary "linux" "arm64" "a" "linux-arm64-libtestoptimization-static" $LINUX_CFLAGS $LINUX_LDFLAGS
        Build-DynamicLibrary "linux" "arm64" "so" "linux-arm64-libtestoptimization-dynamic" $LINUX_CFLAGS $LINUX_LDFLAGS "strip --strip-unneeded"
        
        Build-StaticLibrary "linux" "amd64" "a" "linux-x64-libtestoptimization-static" $LINUX_CFLAGS $LINUX_LDFLAGS
        Build-DynamicLibrary "linux" "amd64" "so" "linux-x64-libtestoptimization-dynamic" $LINUX_CFLAGS $LINUX_LDFLAGS "strip --strip-unneeded"
        
        # Android build (simplified version, full Android NDK setup would be complex)
        Write-Yellow "Building Android library (simplified - no NDK)..."
        Write-Yellow "Note: For production Android builds, use the Docker workflow"
        
        try {
            $env:GOOS = "android"
            $env:GOARCH = "arm64"
            $env:CGO_ENABLED = "1"
            
            $androidPath = "$ABS_OUTPUT_DIR/android-arm64-libtestoptimization-dynamic"
            if (-not (Test-Path $androidPath)) {
                New-Item -ItemType Directory -Path $androidPath -Force | Out-Null
            }
            
            & go build -tags civisibility_native -buildmode=c-shared -ldflags="-s -w" -gcflags="all=-l" -o "$androidPath/libtestoptimization.so" (Get-ChildItem "*.go") 2>$null
        } catch {
            Write-Yellow "Warning: Android build failed (expected without NDK setup)"
            Write-Yellow "Use the Docker workflow for proper Android builds"
        }
    }
    
    "Windows" {
        Write-Blue "--- Building for Windows ---"
        
        # Windows builds
        $WINDOWS_CFLAGS = "-O2 -fno-unwind-tables -fno-asynchronous-unwind-tables"
        $WINDOWS_LDFLAGS = "-Wl,--no-seh"
        
        # Check for MinGW GCC
        $CC = $null
        try {
            $null = Get-Command "x86_64-w64-mingw32-gcc" -ErrorAction Stop
            $CC = "x86_64-w64-mingw32-gcc"
        } catch {
            try {
                $null = Get-Command "gcc" -ErrorAction Stop
                $CC = "gcc"
            } catch {
                Write-Red "Error: No suitable C compiler found for Windows builds"
                exit 1
            }
        }
        $env:CC = $CC
        
        Build-StaticLibrary "windows" "amd64" "lib" "windows-x64-libtestoptimization-static" $WINDOWS_CFLAGS $WINDOWS_LDFLAGS
        Build-DynamicLibrary "windows" "amd64" "dll" "windows-x64-libtestoptimization-dynamic" $WINDOWS_CFLAGS $WINDOWS_LDFLAGS
        
        # Windows-specific post-processing
        Write-Yellow "Post-processing Windows libraries..."
        
        # Strip .pdata/.xdata sections from .lib file (fixes MSVC 14.44 linker issues)
        $libDir = "$ABS_OUTPUT_DIR/windows-x64-libtestoptimization-static"
        $libFile = "$libDir/libtestoptimization.lib"
        
        if (Test-Path $libFile) {
            Write-Yellow "Removing .pdata/.xdata sections from static library..."
            
            # Try to find llvm-ar and llvm-objcopy
            $LLVM_AR = $null
            $LLVM_OBJCOPY = $null
            
            # Check common locations for LLVM tools
            $searchPaths = @("/usr/bin", "/mingw64/bin", "/c/mingw64/bin", "$env:MINGW_PATH/mingw64/bin")
            
            foreach ($path in $searchPaths) {
                if (Test-Path "$path/llvm-ar.exe") {
                    $LLVM_AR = "$path/llvm-ar.exe"
                } elseif (Test-Path "$path/llvm-ar") {
                    $LLVM_AR = "$path/llvm-ar"
                }
                
                if (Test-Path "$path/llvm-objcopy.exe") {
                    $LLVM_OBJCOPY = "$path/llvm-objcopy.exe"
                } elseif (Test-Path "$path/llvm-objcopy") {
                    $LLVM_OBJCOPY = "$path/llvm-objcopy"
                }
                
                if ($LLVM_AR -and $LLVM_OBJCOPY) {
                    break
                }
            }
            
            # Fallback to system ar/objcopy if llvm tools not found
            if (-not $LLVM_AR) {
                try {
                    $null = Get-Command "ar" -ErrorAction Stop
                    $LLVM_AR = "ar"
                } catch {
                    Write-Yellow "Warning: ar tool not found, skipping .lib post-processing"
                }
            }
            
            if (-not $LLVM_OBJCOPY) {
                try {
                    $null = Get-Command "objcopy" -ErrorAction Stop
                    $LLVM_OBJCOPY = "objcopy"
                } catch {
                    Write-Yellow "Warning: objcopy tool not found, skipping .lib post-processing"
                }
            }
            
            if ($LLVM_AR -and $LLVM_OBJCOPY) {
                Write-Yellow "Using $LLVM_AR and $LLVM_OBJCOPY for post-processing"
                
                # Create temporary directory for processing
                $tempDir = New-TemporaryFile | ForEach-Object { Remove-Item $_; New-Item -ItemType Directory -Path $_ }
                Copy-Item $libFile $tempDir
                
                Push-Location $tempDir
                
                try {
                    # Extract all object files
                    & $LLVM_AR x "libtestoptimization.lib"
                    
                    # Strip problematic sections from each object file
                    Get-ChildItem "*.o" | ForEach-Object {
                        Write-Yellow "  • cleaning $($_.Name)"
                        & $LLVM_OBJCOPY --remove-section=.pdata --remove-section=.xdata $_.FullName 2>$null
                    }
                    
                    # Rebuild the static library
                    Remove-Item "libtestoptimization.lib" -Force
                    & $LLVM_AR rc "libtestoptimization.lib" (Get-ChildItem "*.o")
                    & $LLVM_AR s "libtestoptimization.lib"  # rebuild symbol index
                    
                    # Copy back the processed library
                    Copy-Item "libtestoptimization.lib" $libFile -Force
                } finally {
                    Pop-Location
                    Remove-Item $tempDir -Recurse -Force
                }
                
                Write-Green "✓ Static library post-processed"
            }
        }
        
        # Compress DLL with UPX
        $dllFile = "$ABS_OUTPUT_DIR/windows-x64-libtestoptimization-dynamic/libtestoptimization.dll"
        if (Test-Path $dllFile) {
            $upxBinary = Set-UpUPX
            if ($upxBinary) {
                Write-Yellow "Compressing DLL with UPX..."
                try {
                    & $upxBinary --best --lzma $dllFile 2>$null
                    Write-Green "✓ DLL compressed"
                } catch {
                    Write-Yellow "Warning: UPX compression failed, continuing..."
                }
            } else {
                Write-Yellow "Note: UPX setup failed, skipping DLL compression"
            }
        }
    }
    
    default {
        Write-Yellow "Building for current platform ($PlatformName)..."
        
        # Generic builds for current platform
        $CURRENT_GOARCH = ""
        switch ($ARCH) {
            { $_ -in @("x86_64", "amd64", "AMD64") } {
                $CURRENT_GOARCH = "amd64"
            }
            { $_ -in @("arm64", "aarch64", "ARM64") } {
                $CURRENT_GOARCH = "arm64"
            }
            default {
                Write-Red "Error: Unsupported architecture $ARCH"
                exit 1
            }
        }
        
        $GENERIC_CFLAGS = "-O2 -DNDEBUG"
        $GENERIC_LDFLAGS = ""
        
        # Determine file extensions and build
        if ($PlatformName -like "*Windows*") {
            Build-StaticLibrary $PlatformName $CURRENT_GOARCH "lib" "generic-libtestoptimization-static" $GENERIC_CFLAGS $GENERIC_LDFLAGS
            Build-DynamicLibrary $PlatformName $CURRENT_GOARCH "dll" "generic-libtestoptimization-dynamic" $GENERIC_CFLAGS $GENERIC_LDFLAGS
            
            # Compress DLL with UPX
            $dllFile = "$ABS_OUTPUT_DIR/generic-libtestoptimization-dynamic/libtestoptimization.dll"
            if (Test-Path $dllFile) {
                $upxBinary = Set-UpUPX
                if ($upxBinary) {
                    Write-Yellow "Compressing DLL with UPX..."
                    try {
                        & $upxBinary --best --lzma $dllFile 2>$null
                        Write-Green "✓ DLL compressed"
                    } catch {
                        Write-Yellow "Warning: UPX compression failed, continuing..."
                    }
                } else {
                    Write-Yellow "Note: UPX setup failed, skipping DLL compression"
                }
            }
        } else {
            Build-StaticLibrary $PlatformName $CURRENT_GOARCH "a" "generic-libtestoptimization-static" $GENERIC_CFLAGS $GENERIC_LDFLAGS
            Build-DynamicLibrary $PlatformName $CURRENT_GOARCH "so" "generic-libtestoptimization-dynamic" $GENERIC_CFLAGS $GENERIC_LDFLAGS "strip"
        }
    }
}

# Return to original location
Set-Location $originalLocation

Write-Blue "--- Build Summary ---"
Write-Green "✓ All libraries built successfully!"
Write-Yellow "Output directory: $ABS_OUTPUT_DIR"
Write-Yellow "Available builds:"

# List output directories
Get-ChildItem $ABS_OUTPUT_DIR -Directory | ForEach-Object {
    Write-Host "  • $($_.Name): $($_.FullName)/"
}

Write-Blue "=== Build completed successfully! ===" 