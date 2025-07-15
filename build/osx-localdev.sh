#!/bin/bash

set -e  # Exit on any error

# Configuration
DEV_DIR="../dev"
OUTPUT_DIR="../dev-output"
REPO_URL="https://github.com/DataDog/dd-trace-go.git"
BRANCH="main"
TARGET_DIR="$DEV_DIR/internal/civisibility/native"

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${BLUE}=== Test Optimization Native Library - macOS Local Development Build ===${NC}"

# Check if Go is installed
if ! command -v go &> /dev/null; then
    echo -e "${RED}Error: Go is not installed. Please install Go 1.24 or later.${NC}"
    exit 1
fi

# Check Go version
GO_VERSION=$(go version | awk '{print $3}' | sed 's/go//')
echo -e "${GREEN}✓ Found Go version: $GO_VERSION${NC}"

# Check if brew and coreutils are available
if ! command -v brew &> /dev/null; then
    echo -e "${YELLOW}Warning: Homebrew not found. You may need to install coreutils manually.${NC}"
elif ! brew list coreutils &> /dev/null; then
    echo -e "${YELLOW}Installing coreutils...${NC}"
    brew install coreutils
fi

# Function to check if directory is a git repository
is_git_repo() {
    [ -d "$1/.git" ]
}

# Function to get current branch
get_current_branch() {
    git -C "$1" rev-parse --abbrev-ref HEAD 2>/dev/null || echo ""
}

# Function to check if repo is clean
is_repo_clean() {
    [ -z "$(git -C "$1" status --porcelain)" ]
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

echo -e "${BLUE}--- Building for macOS ---${NC}"

# Create output directory at repo root
echo -e "${YELLOW}Creating output directory: $OUTPUT_DIR${NC}"
mkdir -p "$OUTPUT_DIR"

# Get absolute path to output directory before changing directories
ABS_OUTPUT_DIR=$(realpath "$OUTPUT_DIR")

# Change to target directory
cd "$TARGET_DIR"

# Set build environment variables
export CGO_CFLAGS="-mmacosx-version-min=11.0 -O2 -Os -DNDEBUG -fdata-sections -ffunction-sections"
export CGO_LDFLAGS="-Wl,-x"
export GOOS=darwin
export CGO_ENABLED=1

echo -e "${YELLOW}Building ARM64 static library...${NC}"
GOARCH=arm64 go build -tags civisibility_native -buildmode=c-archive -ldflags="-s -w" -gcflags="all=-l" -o "$ABS_OUTPUT_DIR/macos-arm64-libtestoptimization-static/libtestoptimization.a" *.go
echo -e "${GREEN}✓ ARM64 static library built${NC}"

echo -e "${YELLOW}Building ARM64 dynamic library...${NC}"
GOARCH=arm64 go build -tags civisibility_native -buildmode=c-shared -ldflags="-s -w" -gcflags="all=-l" -o "$ABS_OUTPUT_DIR/macos-arm64-libtestoptimization-dynamic/libtestoptimization.dylib" *.go
strip -x "$ABS_OUTPUT_DIR/macos-arm64-libtestoptimization-dynamic/libtestoptimization.dylib"
echo -e "${GREEN}✓ ARM64 dynamic library built${NC}"

echo -e "${YELLOW}Building AMD64 static library...${NC}"
GOARCH=amd64 go build -tags civisibility_native -buildmode=c-archive -ldflags="-s -w" -gcflags="all=-l" -o "$ABS_OUTPUT_DIR/macos-x64-libtestoptimization-static/libtestoptimization.a" *.go
echo -e "${GREEN}✓ AMD64 static library built${NC}"

echo -e "${YELLOW}Building AMD64 dynamic library...${NC}"
GOARCH=amd64 go build -tags civisibility_native -buildmode=c-shared -ldflags="-s -w" -gcflags="all=-l" -o "$ABS_OUTPUT_DIR/macos-x64-libtestoptimization-dynamic/libtestoptimization.dylib" *.go
strip -x "$ABS_OUTPUT_DIR/macos-x64-libtestoptimization-dynamic/libtestoptimization.dylib"
echo -e "${GREEN}✓ AMD64 dynamic library built${NC}"

echo -e "${YELLOW}Creating universal binaries...${NC}"

# Create universal static library
mkdir -p "$ABS_OUTPUT_DIR/macos-libtestoptimization-static"
lipo -create "$ABS_OUTPUT_DIR/macos-arm64-libtestoptimization-static/libtestoptimization.a" "$ABS_OUTPUT_DIR/macos-x64-libtestoptimization-static/libtestoptimization.a" -output "$ABS_OUTPUT_DIR/macos-libtestoptimization-static/libtestoptimization.a"
cp "$ABS_OUTPUT_DIR/macos-arm64-libtestoptimization-static/libtestoptimization.h" "$ABS_OUTPUT_DIR/macos-libtestoptimization-static/libtestoptimization.h"

# Create universal dynamic library
mkdir -p "$ABS_OUTPUT_DIR/macos-libtestoptimization-dynamic"
lipo -create "$ABS_OUTPUT_DIR/macos-arm64-libtestoptimization-dynamic/libtestoptimization.dylib" "$ABS_OUTPUT_DIR/macos-x64-libtestoptimization-dynamic/libtestoptimization.dylib" -output "$ABS_OUTPUT_DIR/macos-libtestoptimization-dynamic/libtestoptimization.dylib"
cp "$ABS_OUTPUT_DIR/macos-arm64-libtestoptimization-dynamic/libtestoptimization.h" "$ABS_OUTPUT_DIR/macos-libtestoptimization-dynamic/libtestoptimization.h"

echo -e "${GREEN}✓ Universal binaries created${NC}"

# Clean up individual architecture folders since we have universal binaries
echo -e "${YELLOW}Cleaning up individual architecture folders...${NC}"
rm -rf "$ABS_OUTPUT_DIR/macos-arm64-libtestoptimization-static"
rm -rf "$ABS_OUTPUT_DIR/macos-arm64-libtestoptimization-dynamic"
rm -rf "$ABS_OUTPUT_DIR/macos-x64-libtestoptimization-static"
rm -rf "$ABS_OUTPUT_DIR/macos-x64-libtestoptimization-dynamic"
echo -e "${GREEN}✓ Individual architecture folders cleaned up${NC}"

echo -e "${YELLOW}Building for iOS...${NC}"
GOOS=ios GOARCH=arm64 CGO_ENABLED=1 go build -tags civisibility_native -buildmode=c-archive -ldflags="-s -w" -gcflags="all=-l" -o "$ABS_OUTPUT_DIR/ios-libtestoptimization-static/libtestoptimization.a" *.go
echo -e "${GREEN}✓ iOS library built${NC}"

echo -e "${BLUE}--- Build Summary ---${NC}"
echo -e "${GREEN}✓ All libraries built successfully!${NC}"
echo -e "${YELLOW}Output directory: $ABS_OUTPUT_DIR${NC}"
echo -e "${YELLOW}Available builds:${NC}"
echo "  • macOS Universal Static:  $ABS_OUTPUT_DIR/macos-libtestoptimization-static/"
echo "  • macOS Universal Dynamic: $ABS_OUTPUT_DIR/macos-libtestoptimization-dynamic/"
echo "  • iOS Static:              $ABS_OUTPUT_DIR/ios-libtestoptimization-static/"

echo -e "${BLUE}=== Build completed successfully! ===${NC}" 