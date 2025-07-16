## Test Optimization Native Library

This repository contains the github action workflows and release artifact of the `libtestoptimization` native library based on the Go's tracer.

### SDKs
It also contains SDKs for different programming languages.

## Local Development

This repository provides comprehensive local development tools for building native libraries and working with SDKs across different platforms.

### Building Native Libraries

The `localdev` scripts build the native libraries locally for development and testing purposes.

#### Available Scripts

- **Unix/Linux/macOS**: `build/localdev.sh`
- **Windows PowerShell**: `build/localdev.ps1`  
- **Windows CMD**: `build/localdev.cmd`

#### Usage

```bash
# Unix/Linux/macOS
cd build
./localdev.sh                    # Build for current platform
./localdev.sh --help             # Show help

# Windows PowerShell
cd build
.\localdev.ps1                   # Build for current platform
.\localdev.ps1 -Help             # Show help

# Windows CMD
cd build
localdev.cmd                     # Build for current platform  
localdev.cmd --help              # Show help
```

#### Features

- **Cross-platform support**: macOS (Universal + iOS), Linux (ARM64/AMD64), Windows (AMD64), Android
- **Automatic repository management**: Clones/updates dd-trace-go repository
- **Platform-specific optimizations**: Compiler flags, UPX compression, library stripping
- **Output organization**: All artifacts placed in `dev-output/` directory
- **Tool caching**: Downloads and caches build tools (UPX) in `tools/` directory

#### Output Structure

```
dev-output/
├── macos-libtestoptimization-static/           # macOS universal static library
├── macos-libtestoptimization-dynamic/          # macOS universal dynamic library  
├── ios-libtestoptimization-static/             # iOS static library
├── linux-arm64-libtestoptimization-static/    # Linux ARM64 libraries
├── linux-arm64-libtestoptimization-dynamic/
├── linux-x64-libtestoptimization-static/      # Linux x64 libraries
├── linux-x64-libtestoptimization-dynamic/
├── windows-x64-libtestoptimization-static/    # Windows x64 libraries
├── windows-x64-libtestoptimization-dynamic/
└── android-arm64-libtestoptimization-dynamic/ # Android ARM64 library
```

### Rust SDK Development

The `ldevcargo` scripts provide an integrated development experience for the Rust SDK by automatically building native libraries and running cargo commands with development mode enabled.

#### Available Scripts

- **Unix/Linux/macOS**: `sdks/rust/test-optimization-sdk/ldevcargo.sh`
- **Windows PowerShell**: `sdks/rust/test-optimization-sdk/ldevcargo.ps1`
- **Windows CMD**: `sdks/rust/test-optimization-sdk/ldevcargo.cmd`

#### Usage

```bash
# Unix/Linux/macOS
cd sdks/rust/test-optimization-sdk
./ldevcargo.sh build                    # Build native libs + cargo build
./ldevcargo.sh test                     # Build native libs + cargo test
./ldevcargo.sh -sn check                # Skip native build, run cargo check
./ldevcargo.sh --skip-native clippy     # Skip native build, run cargo clippy
./ldevcargo.sh --help                   # Show help

# Windows PowerShell  
cd sdks/rust/test-optimization-sdk
pwsh -NoProfile .\ldevcargo.ps1 build           # Build native libs + cargo build
pwsh -NoProfile .\ldevcargo.ps1 -sn check       # Skip native build, run cargo check
pwsh -NoProfile .\ldevcargo.ps1 --help          # Show help

# Windows CMD
cd sdks/rust/test-optimization-sdk
ldevcargo.cmd build                     # Build native libs + cargo build
ldevcargo.cmd -sn check                 # Skip native build, run cargo check
ldevcargo.cmd --help                    # Show help
```

#### Options

- `-h, --help`: Show help message and exit
- `-sn, --skip-native`: Skip building native libraries (use existing builds)

#### Features

- **Automatic native builds**: Runs `localdev` script before cargo commands
- **Development mode**: Automatically sets `TEST_OPTIMIZATION_DEV_MODE=1`
- **Skip option**: Fast iteration with `-sn`/`--skip-native` for quick cargo operations
- **Argument filtering**: Removes script-specific flags before passing to cargo
- **Cross-platform**: Consistent interface across all platforms
- **Clean execution**: PowerShell version uses `-NoProfile` to avoid profile interference

#### Environment Variables

The following environment variables enhance the development experience:

- **`TEST_OPTIMIZATION_DEV_MODE`**: Automatically set by `ldevcargo` scripts to use local builds
- **`TEST_OPTIMIZATION_SDK_NATIVE_SEARCH_PATH`**: Custom path for native library search
- **`TEST_OPTIMIZATION_SDK_SKIP_NATIVE_INSTALL`**: Skip automatic native library installation

### Development Workflow

#### Initial Setup
```bash
# Clone the repository
git clone https://github.com/DataDog/test-optimization-native.git
cd test-optimization-native

# Build native libraries for your platform
cd build
./localdev.sh  # or localdev.ps1/localdev.cmd on Windows
```

#### Rust Development
```bash
# Navigate to Rust SDK
cd sdks/rust/test-optimization-sdk

# Full development build (builds native libs + cargo build)
./ldevcargo.sh build

# Quick iterations (skip native rebuild)
./ldevcargo.sh -sn check
./ldevcargo.sh -sn clippy  
./ldevcargo.sh -sn test

# Run with fresh native libraries
./ldevcargo.sh test
```

#### Python Development
```bash
# Navigate to Python SDK  
cd sdks/python/test-optimization-sdk

# Install in development mode with local native libraries
TEST_OPTIMIZATION_DEV_MODE=1 pip install -e .
```

### Requirements

- **Go 1.24 or later** (for native library builds)
- **Git** (for repository management)
- **Platform-specific tools**:
  - macOS: Xcode Command Line Tools, optional Homebrew with coreutils
  - Linux: GCC, build-essential
  - Windows: MinGW-w64 or Visual Studio Build Tools
- **Rust/Cargo** (for Rust SDK development)
- **Python 3.7+** (for Python SDK development)

## Copyright

Copyright (c) 2025 Datadog
[https://www.datadoghq.com](https://www.datadoghq.com/)

## License

See [license information](./LICENSE).
