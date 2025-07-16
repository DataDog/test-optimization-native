#!/bin/bash
set -e  # Exit on any error

# Colors for output
BLUE='\033[0;34m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to display help
show_help() {
    cat << EOF
Rust SDK Local Development Build Script

This script builds native libraries and runs cargo commands with dev mode enabled.

Usage:
    ./ldevcargo.sh [OPTIONS] [CARGO_ARGS...]

Options:
    -h, --help          Show this help message and exit
    -sn, --skip-native  Skip building native libraries (use existing builds)

Examples:
    ./ldevcargo.sh build                    # Build native libs + cargo build
    ./ldevcargo.sh test                     # Build native libs + cargo test
    ./ldevcargo.sh -sn check                # Skip native build, run cargo check
    ./ldevcargo.sh --skip-native clippy     # Skip native build, run cargo clippy
    ./ldevcargo.sh build --release          # Build native libs + cargo build --release

Notes:
    - Native libraries are built using ../../../build/localdev.sh
    - TEST_OPTIMIZATION_DEV_MODE=1 is automatically set
    - The --skip-native/-sn flag is filtered out before passing args to cargo

EOF
}

# Parse arguments to check for --skip-native and help
SKIP_NATIVE=false
CARGO_ARGS=()

for arg in "$@"; do
    case $arg in
        -h|--help)
            show_help
            exit 0
            ;;
        -sn|--skip-native)
            SKIP_NATIVE=true
            ;;
        *)
            CARGO_ARGS+=("$arg")
            ;;
    esac
done

echo -e "${BLUE}=== Rust SDK Local Development Build ===${NC}"

# Run localdev.sh first to build native libraries (unless skipped)
if [ "$SKIP_NATIVE" = true ]; then
    echo -e "${YELLOW}Skipping native library build (--skip-native specified)${NC}"
else
    LOCALDEV_SCRIPT="../../../build/localdev.sh"

    if [ -f "$LOCALDEV_SCRIPT" ]; then
        echo -e "${BLUE}Building native libraries first...${NC}"
        cd ../../../build
        ./localdev.sh
        cd - > /dev/null
        echo -e "${GREEN}✓ Native libraries built${NC}"
    else
        echo -e "\033[0;31mError: localdev.sh not found at $LOCALDEV_SCRIPT${NC}"
        exit 1
    fi
fi

echo -e "${BLUE}Running cargo with dev mode...${NC}"
export TEST_OPTIMIZATION_DEV_MODE=1

cargo "${CARGO_ARGS[@]}"