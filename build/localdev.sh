#!/bin/bash

set -e  # Exit on any error

# Function to display help
show_help() {
    cat << EOF
Test Optimization Native Library - Local Development Build Script

This script builds the native library for local development by:
1. Setting up the dd-trace-go repository
2. Copying build and native files
3. Building platform-specific libraries
4. Applying platform-specific optimizations

Usage:
    ./localdev.sh [OPTIONS]

Options:
    -h, --help      Show this help message and exit

Requirements:
    - Go 1.24 or later
    - Git
    - Platform-specific build tools (MinGW for Windows, etc.)

Output:
    All build artifacts will be placed in ../dev-output/

Examples:
    ./localdev.sh           # Build for current platform
    ./localdev.sh --help    # Show this help

EOF
}

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -h|--help|-\?)
            show_help
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            echo "Use --help for usage information."
            exit 1
            ;;
    esac
    shift
done

# Configuration
DEV_DIR="../dev"
OUTPUT_DIR="../dev-output"
TOOLS_DIR="../tools"
REPO_URL="https://github.com/DataDog/dd-trace-go.git"
BRANCH="main"
TARGET_DIR="$DEV_DIR/internal/civisibility/native"
UPX_VERSION="5.0.0"

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Platform detection
PLATFORM=$(uname -s)
ARCH=$(uname -m)

echo -e "${BLUE}=== Test Optimization Native Library - Local Development Build ===${NC}"
echo -e "${YELLOW}Detected platform: $PLATFORM ($ARCH)${NC}"

# Check if Go is installed
if ! command -v go &> /dev/null; then
    echo -e "${RED}Error: Go is not installed. Please install Go 1.24 or later.${NC}"
    exit 1
fi

# Check Go version
GO_VERSION=$(go version | awk '{print $3}' | sed 's/go//')
echo -e "${GREEN}✓ Found Go version: $GO_VERSION${NC}"

# Platform-specific setup
case "$PLATFORM" in
    "Darwin")
        echo -e "${GREEN}✓ Running on macOS${NC}"
        # Check if brew and coreutils are available
        if ! command -v brew &> /dev/null; then
            echo -e "${YELLOW}Warning: Homebrew not found. You may need to install coreutils manually.${NC}"
        elif ! brew list coreutils &> /dev/null; then
            echo -e "${YELLOW}Installing coreutils...${NC}"
            brew install coreutils
        fi
        ;;
    "Linux")
        echo -e "${GREEN}✓ Running on Linux${NC}"
        ;;
    "MINGW"*|"MSYS"*|"CYGWIN"*)
        echo -e "${GREEN}✓ Running on Windows${NC}"
        ;;
    *)
        echo -e "${YELLOW}Warning: Unknown platform $PLATFORM. Proceeding anyway...${NC}"
        ;;
esac

# Function to check if directory is a git repository
is_git_repo() {
    [ -d "$1/.git" ]
}

# Function to get current branch
get_current_branch() {
    git -C "$1" rev-parse --abbrev-ref HEAD 2>/dev/null || echo ""
}

# Function to download and setup UPX
setup_upx() {
    local upx_dir="$TOOLS_DIR/upx-$UPX_VERSION"
    local upx_binary=""
    
    # Determine platform-specific UPX details
    case "$PLATFORM" in
        "Darwin")
            local upx_archive="upx-${UPX_VERSION}-macos.tar.xz"
            local upx_url="https://github.com/upx/upx/releases/download/v${UPX_VERSION}/${upx_archive}"
            upx_binary="$upx_dir/upx"
            ;;
        "Linux")
            local upx_archive="upx-${UPX_VERSION}-amd64_linux.tar.xz"
            local upx_url="https://github.com/upx/upx/releases/download/v${UPX_VERSION}/${upx_archive}"
            upx_binary="$upx_dir/upx"
            ;;
        "MINGW"*|"MSYS"*|"CYGWIN"*)
            local upx_archive="upx-${UPX_VERSION}-win64.zip"
            local upx_url="https://github.com/upx/upx/releases/download/v${UPX_VERSION}/${upx_archive}"
            upx_binary="$upx_dir/upx-${UPX_VERSION}-win64/upx.exe"
            ;;
        *)
            echo -e "${YELLOW}UPX not available for platform $PLATFORM${NC}"
            return 1
            ;;
    esac
    
    # Check if UPX is already available
    if [ -f "$upx_binary" ]; then
        echo -e "${GREEN}✓ Found cached UPX at $upx_binary${NC}"
        echo "$upx_binary"
        return 0
    fi
    
    echo -e "${YELLOW}Downloading UPX $UPX_VERSION for $PLATFORM...${NC}"
    
    # Create tools directory
    mkdir -p "$TOOLS_DIR"
    
    # Download UPX
    local temp_file="$TOOLS_DIR/$upx_archive"
    if command -v wget &> /dev/null; then
        wget -q "$upx_url" -O "$temp_file"
    elif command -v curl &> /dev/null; then
        curl -L -s "$upx_url" -o "$temp_file"
    else
        echo -e "${RED}Error: No download tool found (wget or curl)${NC}"
        return 1
    fi
    
    if [ ! -f "$temp_file" ]; then
        echo -e "${RED}Error: Failed to download UPX${NC}"
        return 1
    fi
    
    # Extract UPX
    echo -e "${YELLOW}Extracting UPX...${NC}"
    case "$upx_archive" in
        *.zip)
            if command -v unzip &> /dev/null; then
                unzip -q "$temp_file" -d "$upx_dir"
            else
                echo -e "${RED}Error: unzip not found${NC}"
                return 1
            fi
            ;;
        *.tar.xz)
            if command -v tar &> /dev/null; then
                mkdir -p "$upx_dir"
                tar -xf "$temp_file" -C "$upx_dir" --strip-components=1
            else
                echo -e "${RED}Error: tar not found${NC}"
                return 1
            fi
            ;;
    esac
    
    # Clean up download
    rm -f "$temp_file"
    
    # Verify extraction
    if [ -f "$upx_binary" ]; then
        chmod +x "$upx_binary" 2>/dev/null || true
        echo -e "${GREEN}✓ UPX $UPX_VERSION installed to $upx_binary${NC}"
        echo "$upx_binary"
        return 0
    else
        echo -e "${RED}Error: UPX extraction failed${NC}"
        return 1
    fi
}

echo -e "${BLUE}--- Setting up dd-trace-go repository ---${NC}"

if [ -d "$DEV_DIR" ]; then
    if is_git_repo "$DEV_DIR"; then
        echo -e "${GREEN}✓ Found existing dd-trace-go repository in $DEV_DIR${NC}"
        
        # Check current branch
        CURRENT_BRANCH=$(get_current_branch "$DEV_DIR")
        if [ "$CURRENT_BRANCH" != "$BRANCH" ]; then
            echo -e "${YELLOW}Current branch is '$CURRENT_BRANCH', switching to '$BRANCH'...${NC}"
            git -C "$DEV_DIR" fetch origin
            git -C "$DEV_DIR" checkout "$BRANCH"
        fi
        
        # Pull latest changes
        echo -e "${YELLOW}Pulling latest changes from $BRANCH...${NC}"
        git -C "$DEV_DIR" pull origin "$BRANCH"
    else
        echo -e "${RED}Error: $DEV_DIR exists but is not a git repository${NC}"
        echo -e "${YELLOW}Please remove $DEV_DIR directory and run this script again${NC}"
        exit 1
    fi
else
    echo -e "${YELLOW}Cloning dd-trace-go repository...${NC}"
    git clone --branch "$BRANCH" "$REPO_URL" "$DEV_DIR"
    echo -e "${GREEN}✓ Repository cloned successfully${NC}"
fi

echo -e "${BLUE}--- Preparing build environment ---${NC}"

# Create target directory
echo -e "${YELLOW}Creating target directory: $TARGET_DIR${NC}"
mkdir -p "$TARGET_DIR"

# Copy build files
echo -e "${YELLOW}Copying build files...${NC}"
cp -rf ./* "$TARGET_DIR/"
echo -e "${GREEN}✓ Build files copied${NC}"

# Copy native files
echo -e "${YELLOW}Copying native files...${NC}"
cp -rf ../native/* "$TARGET_DIR/"
echo -e "${GREEN}✓ Native files copied${NC}"

# Create output directory at repo root
echo -e "${YELLOW}Creating output directory: $OUTPUT_DIR${NC}"
mkdir -p "$OUTPUT_DIR"

# Create tools directory for cached tools
echo -e "${YELLOW}Creating tools directory: $TOOLS_DIR${NC}"
mkdir -p "$TOOLS_DIR"

# Get absolute path to output directory before changing directories
ABS_OUTPUT_DIR=$(realpath "$OUTPUT_DIR")

# Change to target directory
cd "$TARGET_DIR"

# Set common environment variables
export CGO_ENABLED=1

echo -e "${BLUE}--- Building Native Libraries ---${NC}"

# Function to build static library
build_static() {
    local goos=$1
    local goarch=$2
    local ext=$3
    local output_name=$4
    local cflags=$5
    local ldflags=$6
    
    echo -e "${YELLOW}Building $goos-$goarch static library...${NC}"
    GOOS="$goos" GOARCH="$goarch" CGO_CFLAGS="$cflags" CGO_LDFLAGS="$ldflags" \
        go build -tags civisibility_native -buildmode=c-archive \
        -ldflags="-s -w" -gcflags="all=-l" \
        -o "$ABS_OUTPUT_DIR/$output_name/libtestoptimization.$ext" *.go
    echo -e "${GREEN}✓ $goos-$goarch static library built${NC}"
}

# Function to build dynamic library
build_dynamic() {
    local goos=$1
    local goarch=$2
    local ext=$3
    local output_name=$4
    local cflags=$5
    local ldflags=$6
    local strip_cmd=$7
    
    echo -e "${YELLOW}Building $goos-$goarch dynamic library...${NC}"
    GOOS="$goos" GOARCH="$goarch" CGO_CFLAGS="$cflags" CGO_LDFLAGS="$ldflags" \
        go build -tags civisibility_native -buildmode=c-shared \
        -ldflags="-s -w" -gcflags="all=-l" \
        -o "$ABS_OUTPUT_DIR/$output_name/libtestoptimization.$ext" *.go
    
    if [ -n "$strip_cmd" ] && command -v "$strip_cmd" &> /dev/null; then
        $strip_cmd "$ABS_OUTPUT_DIR/$output_name/libtestoptimization.$ext"
    fi
    echo -e "${GREEN}✓ $goos-$goarch dynamic library built${NC}"
}

# Platform-specific builds
case "$PLATFORM" in
    "Darwin")
        echo -e "${BLUE}--- Building for macOS and iOS ---${NC}"
        
        # macOS builds
        MACOS_CFLAGS="-mmacosx-version-min=11.0 -O2 -Os -DNDEBUG -fdata-sections -ffunction-sections"
        MACOS_LDFLAGS="-Wl,-x"
        
        build_static "darwin" "arm64" "a" "macos-arm64-libtestoptimization-static" "$MACOS_CFLAGS" "$MACOS_LDFLAGS"
        build_dynamic "darwin" "arm64" "dylib" "macos-arm64-libtestoptimization-dynamic" "$MACOS_CFLAGS" "$MACOS_LDFLAGS" "strip -x"
        
        build_static "darwin" "amd64" "a" "macos-x64-libtestoptimization-static" "$MACOS_CFLAGS" "$MACOS_LDFLAGS"
        build_dynamic "darwin" "amd64" "dylib" "macos-x64-libtestoptimization-dynamic" "$MACOS_CFLAGS" "$MACOS_LDFLAGS" "strip -x"
        
        # iOS build
        echo -e "${YELLOW}Building iOS library...${NC}"
        GOOS=ios GOARCH=arm64 CGO_ENABLED=1 \
            go build -tags civisibility_native -buildmode=c-archive \
            -ldflags="-s -w" -gcflags="all=-l" \
            -o "$ABS_OUTPUT_DIR/ios-libtestoptimization-static/libtestoptimization.a" *.go
        echo -e "${GREEN}✓ iOS library built${NC}"
        
        # Create universal binaries
        echo -e "${YELLOW}Creating universal binaries...${NC}"
        
        # Universal static library
        mkdir -p "$ABS_OUTPUT_DIR/macos-libtestoptimization-static"
        lipo -create \
            "$ABS_OUTPUT_DIR/macos-arm64-libtestoptimization-static/libtestoptimization.a" \
            "$ABS_OUTPUT_DIR/macos-x64-libtestoptimization-static/libtestoptimization.a" \
            -output "$ABS_OUTPUT_DIR/macos-libtestoptimization-static/libtestoptimization.a"
        cp "$ABS_OUTPUT_DIR/macos-arm64-libtestoptimization-static/libtestoptimization.h" \
           "$ABS_OUTPUT_DIR/macos-libtestoptimization-static/libtestoptimization.h"
        
        # Universal dynamic library
        mkdir -p "$ABS_OUTPUT_DIR/macos-libtestoptimization-dynamic"
        lipo -create \
            "$ABS_OUTPUT_DIR/macos-arm64-libtestoptimization-dynamic/libtestoptimization.dylib" \
            "$ABS_OUTPUT_DIR/macos-x64-libtestoptimization-dynamic/libtestoptimization.dylib" \
            -output "$ABS_OUTPUT_DIR/macos-libtestoptimization-dynamic/libtestoptimization.dylib"
        cp "$ABS_OUTPUT_DIR/macos-arm64-libtestoptimization-dynamic/libtestoptimization.h" \
           "$ABS_OUTPUT_DIR/macos-libtestoptimization-dynamic/libtestoptimization.h"
        
        echo -e "${GREEN}✓ Universal binaries created${NC}"
        
        # Clean up individual architecture folders
        echo -e "${YELLOW}Cleaning up individual architecture folders...${NC}"
        rm -rf "$ABS_OUTPUT_DIR/macos-arm64-libtestoptimization-static"
        rm -rf "$ABS_OUTPUT_DIR/macos-arm64-libtestoptimization-dynamic"
        rm -rf "$ABS_OUTPUT_DIR/macos-x64-libtestoptimization-static"
        rm -rf "$ABS_OUTPUT_DIR/macos-x64-libtestoptimization-dynamic"
        echo -e "${GREEN}✓ Individual architecture folders cleaned up${NC}"
        ;;
        
    "Linux")
        echo -e "${BLUE}--- Building for Linux and Android ---${NC}"
        
        # Linux builds
        LINUX_CFLAGS="-O2 -Os -s -DNDEBUG -fdata-sections -ffunction-sections"
        LINUX_LDFLAGS="-s -Wl,--gc-sections"
        
        build_static "linux" "arm64" "a" "linux-arm64-libtestoptimization-static" "$LINUX_CFLAGS" "$LINUX_LDFLAGS"
        build_dynamic "linux" "arm64" "so" "linux-arm64-libtestoptimization-dynamic" "$LINUX_CFLAGS" "$LINUX_LDFLAGS" "strip --strip-unneeded"
        
        build_static "linux" "amd64" "a" "linux-x64-libtestoptimization-static" "$LINUX_CFLAGS" "$LINUX_LDFLAGS"
        build_dynamic "linux" "amd64" "so" "linux-x64-libtestoptimization-dynamic" "$LINUX_CFLAGS" "$LINUX_LDFLAGS" "strip --strip-unneeded"
        
        # Android build (note: this is a simplified version, full Android NDK setup would be complex)
        echo -e "${YELLOW}Building Android library (simplified - no NDK)...${NC}"
        echo -e "${YELLOW}Note: For production Android builds, use the Docker workflow${NC}"
        GOOS=android GOARCH=arm64 CGO_ENABLED=1 \
            go build -tags civisibility_native -buildmode=c-shared \
            -ldflags="-s -w" -gcflags="all=-l" \
            -o "$ABS_OUTPUT_DIR/android-arm64-libtestoptimization-dynamic/libtestoptimization.so" *.go 2>/dev/null || {
            echo -e "${YELLOW}Warning: Android build failed (expected without NDK setup)${NC}"
            echo -e "${YELLOW}Use the Docker workflow for proper Android builds${NC}"
        }
        ;;
        
    "MINGW"*|"MSYS"*|"CYGWIN"*)
        echo -e "${BLUE}--- Building for Windows ---${NC}"
        
        # Windows builds
        WINDOWS_CFLAGS="-O2 -fno-unwind-tables -fno-asynchronous-unwind-tables"
        WINDOWS_LDFLAGS="-Wl,--no-seh"
        
        # Check for MinGW GCC
        if command -v x86_64-w64-mingw32-gcc &> /dev/null; then
            export CC=x86_64-w64-mingw32-gcc
        elif command -v gcc &> /dev/null; then
            export CC=gcc
        else
            echo -e "${RED}Error: No suitable C compiler found for Windows builds${NC}"
            exit 1
        fi
        
        build_static "windows" "amd64" "lib" "windows-x64-libtestoptimization-static" "$WINDOWS_CFLAGS" "$WINDOWS_LDFLAGS"
        build_dynamic "windows" "amd64" "dll" "windows-x64-libtestoptimization-dynamic" "$WINDOWS_CFLAGS" "$WINDOWS_LDFLAGS" ""
        
        # Windows-specific post-processing
        echo -e "${YELLOW}Post-processing Windows libraries...${NC}"
        
        # Strip .pdata/.xdata sections from .lib file (fixes MSVC 14.44 linker issues)
        LIB_DIR="$ABS_OUTPUT_DIR/windows-x64-libtestoptimization-static"
        LIB_FILE="$LIB_DIR/libtestoptimization.lib"
        
        if [ -f "$LIB_FILE" ]; then
            echo -e "${YELLOW}Removing .pdata/.xdata sections from static library...${NC}"
            
            # Try to find llvm-ar and llvm-objcopy
            LLVM_AR=""
            LLVM_OBJCOPY=""
            
            # Check common locations for LLVM tools
            for path in "/usr/bin" "/mingw64/bin" "/c/mingw64/bin" "$MINGW_PATH/mingw64/bin"; do
                if [ -f "$path/llvm-ar.exe" ] || [ -f "$path/llvm-ar" ]; then
                    LLVM_AR="$path/llvm-ar"
                    [ -f "$path/llvm-ar.exe" ] && LLVM_AR="$path/llvm-ar.exe"
                fi
                if [ -f "$path/llvm-objcopy.exe" ] || [ -f "$path/llvm-objcopy" ]; then
                    LLVM_OBJCOPY="$path/llvm-objcopy"
                    [ -f "$path/llvm-objcopy.exe" ] && LLVM_OBJCOPY="$path/llvm-objcopy.exe"
                fi
                if [ -n "$LLVM_AR" ] && [ -n "$LLVM_OBJCOPY" ]; then
                    break
                fi
            done
            
            # Fallback to system ar/objcopy if llvm tools not found
            if [ -z "$LLVM_AR" ]; then
                if command -v ar &> /dev/null; then
                    LLVM_AR="ar"
                else
                    echo -e "${YELLOW}Warning: ar tool not found, skipping .lib post-processing${NC}"
                fi
            fi
            
            if [ -z "$LLVM_OBJCOPY" ]; then
                if command -v objcopy &> /dev/null; then
                    LLVM_OBJCOPY="objcopy"
                else
                    echo -e "${YELLOW}Warning: objcopy tool not found, skipping .lib post-processing${NC}"
                fi
            fi
            
            if [ -n "$LLVM_AR" ] && [ -n "$LLVM_OBJCOPY" ]; then
                echo -e "${YELLOW}Using $LLVM_AR and $LLVM_OBJCOPY for post-processing${NC}"
                
                # Create temporary directory for processing
                TEMP_DIR=$(mktemp -d)
                cp "$LIB_FILE" "$TEMP_DIR/"
                
                pushd "$TEMP_DIR" > /dev/null
                
                # Extract all object files
                "$LLVM_AR" x libtestoptimization.lib
                
                # Strip problematic sections from each object file
                for obj_file in *.o; do
                    if [ -f "$obj_file" ]; then
                        echo -e "${YELLOW}  • cleaning $obj_file${NC}"
                        "$LLVM_OBJCOPY" --remove-section=.pdata --remove-section=.xdata "$obj_file" 2>/dev/null || true
                    fi
                done
                
                # Rebuild the static library
                rm -f libtestoptimization.lib
                "$LLVM_AR" rc libtestoptimization.lib *.o
                "$LLVM_AR" s libtestoptimization.lib  # rebuild symbol index
                
                # Copy back the processed library
                cp libtestoptimization.lib "$LIB_FILE"
                
                popd > /dev/null
                rm -rf "$TEMP_DIR"
                
                echo -e "${GREEN}✓ Static library post-processed${NC}"
            fi
        fi
        
        # Compress DLL with UPX
        DLL_FILE="$ABS_OUTPUT_DIR/windows-x64-libtestoptimization-dynamic/libtestoptimization.dll"
        if [ -f "$DLL_FILE" ]; then
            UPX_BINARY=$(setup_upx)
            if [ $? -eq 0 ] && [ -n "$UPX_BINARY" ]; then
                echo -e "${YELLOW}Compressing DLL with UPX...${NC}"
                "$UPX_BINARY" --best --lzma "$DLL_FILE" 2>/dev/null || {
                    echo -e "${YELLOW}Warning: UPX compression failed, continuing...${NC}"
                }
                echo -e "${GREEN}✓ DLL compressed${NC}"
            else
                echo -e "${YELLOW}Note: UPX setup failed, skipping DLL compression${NC}"
            fi
        fi
        ;;
        
    *)
        echo -e "${YELLOW}Building for current platform ($PLATFORM)...${NC}"
        
        # Generic builds for current platform
        CURRENT_GOARCH=""
        case "$ARCH" in
            "x86_64"|"amd64")
                CURRENT_GOARCH="amd64"
                ;;
            "arm64"|"aarch64")
                CURRENT_GOARCH="arm64"
                ;;
            *)
                echo -e "${RED}Error: Unsupported architecture $ARCH${NC}"
                exit 1
                ;;
        esac
        
        GENERIC_CFLAGS="-O2 -DNDEBUG"
        GENERIC_LDFLAGS=""
        
        # Determine file extensions and build
        if [[ "$PLATFORM" == *"MINGW"* ]] || [[ "$PLATFORM" == *"MSYS"* ]] || [[ "$PLATFORM" == *"CYGWIN"* ]]; then
            build_static "$PLATFORM" "$CURRENT_GOARCH" "lib" "generic-libtestoptimization-static" "$GENERIC_CFLAGS" "$GENERIC_LDFLAGS"
            build_dynamic "$PLATFORM" "$CURRENT_GOARCH" "dll" "generic-libtestoptimization-dynamic" "$GENERIC_CFLAGS" "$GENERIC_LDFLAGS" ""
            
            # Compress DLL with UPX
            DLL_FILE="$ABS_OUTPUT_DIR/generic-libtestoptimization-dynamic/libtestoptimization.dll"
            if [ -f "$DLL_FILE" ]; then
                UPX_BINARY=$(setup_upx)
                if [ $? -eq 0 ] && [ -n "$UPX_BINARY" ]; then
                    echo -e "${YELLOW}Compressing DLL with UPX...${NC}"
                    "$UPX_BINARY" --best --lzma "$DLL_FILE" 2>/dev/null || {
                        echo -e "${YELLOW}Warning: UPX compression failed, continuing...${NC}"
                    }
                    echo -e "${GREEN}✓ DLL compressed${NC}"
                else
                    echo -e "${YELLOW}Note: UPX setup failed, skipping DLL compression${NC}"
                fi
            fi
        else
            build_static "$PLATFORM" "$CURRENT_GOARCH" "a" "generic-libtestoptimization-static" "$GENERIC_CFLAGS" "$GENERIC_LDFLAGS"
            build_dynamic "$PLATFORM" "$CURRENT_GOARCH" "so" "generic-libtestoptimization-dynamic" "$GENERIC_CFLAGS" "$GENERIC_LDFLAGS" "strip"
        fi
        ;;
esac

echo -e "${BLUE}--- Build Summary ---${NC}"
echo -e "${GREEN}✓ All libraries built successfully!${NC}"
echo -e "${YELLOW}Output directory: $ABS_OUTPUT_DIR${NC}"
echo -e "${YELLOW}Available builds:${NC}"

# List output directories
for dir in "$ABS_OUTPUT_DIR"/*; do
    if [ -d "$dir" ]; then
        echo "  • $(basename "$dir"): $dir/"
    fi
done

echo -e "${BLUE}=== Build completed successfully! ===${NC}" 