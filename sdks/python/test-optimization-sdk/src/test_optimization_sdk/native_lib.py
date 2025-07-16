import os
import platform
import sys
import urllib.request
import zipfile
from pathlib import Path
from .constants import TEST_OPTIMIZATION_SDK_SKIP_NATIVE_INSTALL, TEST_OPTIMIZATION_SDK_NATIVE_SEARCH_PATH, TEST_OPTIMIZATION_DEV_MODE, TEST_OPTIMIZATION_DOWNLOAD_URL_FORMAT

def get_library_filename():
    """Get the appropriate library filename based on the platform."""
    system = platform.system().lower()
    machine = platform.machine().lower()
    
    if system == "darwin":
        system = "macos"
    elif system == "windows":
        system = "windows"
    elif system == "linux":
        system = "linux"
    else:
        raise RuntimeError(f"Unsupported platform: {system}")
    
    if "arm64" in machine or "aarch64" in machine:
        arch = "arm64"
    else:
        arch = "x64"
    
    if system == "macos":
        return f"{system}-libtestoptimization-dynamic.zip"
    else:
        return f"{system}-{arch}-libtestoptimization-dynamic.zip"

def download_native_library():
    """Download the native library and extract it to the appropriate location."""
    # Get the package directory
    package_dir = Path(__file__).parent
    lib_dir = package_dir / "lib"
    lib_dir.mkdir(exist_ok=True)
    
    # Get the library filename
    lib_filename = get_library_filename()
    url = f"{TEST_OPTIMIZATION_DOWNLOAD_URL_FORMAT}{lib_filename}"
    
    # Download the library
    print(f"Downloading native library from: {url}")
    try:
        urllib.request.urlretrieve(url, lib_dir / lib_filename)
    except Exception as e:
        print(f"Failed to download native library: {e}", file=sys.stderr)
        sys.exit(1)
    
    # Extract the library
    try:
        with zipfile.ZipFile(lib_dir / lib_filename, 'r') as archive:
            archive.extractall(lib_dir)
        # Clean up the zip file
        (lib_dir / lib_filename).unlink()
    except Exception as e:
        print(f"Failed to extract native library: {e}", file=sys.stderr)
        sys.exit(1)

def setup_dev_mode_library():
    """Setup the native library from dev-output folder."""
    # Get platform and architecture info
    system = platform.system().lower()
    machine = platform.machine().lower()
    
    if system == "darwin":
        system = "macos"
    elif system == "windows":
        system = "windows"
    elif system == "linux":
        system = "linux"
    else:
        raise RuntimeError(f"Unsupported platform: {system}")
    
    if "arm64" in machine or "aarch64" in machine:
        arch = "arm64"
    else:
        arch = "x64"
    
    # Construct the dev output folder name (dynamic libraries)
    if system == "macos":
        folder_name = f"{system}-libtestoptimization-dynamic"
        lib_filename = "libtestoptimization.dylib"
    elif system == "linux":
        folder_name = f"{system}-{arch}-libtestoptimization-dynamic"
        lib_filename = "libtestoptimization.so"
    elif system == "windows":
        folder_name = f"{system}-{arch}-libtestoptimization-dynamic"
        lib_filename = "testoptimization.dll"
    else:
        raise RuntimeError(f"Unsupported platform: {system}")
    
    # The dev-output directory is relative to the repo root
    # native_lib.py is in sdks/python/test-optimization-sdk/src/test_optimization_sdk/
    # so we go up 5 levels to reach repo root
    current_file = Path(__file__).resolve()
    repo_root = current_file.parents[5]  # Go up 5 levels: test_optimization_sdk -> src -> test-optimization-sdk -> python -> sdks -> repo_root
    dev_output_path = repo_root / "dev-output" / folder_name
    
    # Check if the library file exists
    lib_file_path = dev_output_path / lib_filename
    
    if lib_file_path.exists():
        print(f"Using dev mode native library from: {dev_output_path}")
        
        # Copy the library to our lib directory
        package_dir = Path(__file__).parent
        lib_dir = package_dir / "lib"
        lib_dir.mkdir(exist_ok=True)
        
        # Copy the library file and header
        import shutil
        shutil.copy2(lib_file_path, lib_dir / lib_filename)
        
        # Also copy header file if it exists
        header_file = dev_output_path / "libtestoptimization.h"
        if header_file.exists():
            shutil.copy2(header_file, lib_dir / "libtestoptimization.h")
        
        return True
    else:
        print(f"Dev mode enabled but library not found at: {lib_file_path}", file=sys.stderr)
        print("Please run the localdev.sh script to build the native libraries first", file=sys.stderr)
        sys.exit(1)

def setup_native_library():
    """Setup the native library during package installation."""
    # Check for dev mode first (highest priority)
    if os.environ.get(TEST_OPTIMIZATION_DEV_MODE):
        setup_dev_mode_library()
        return
    
    # Check for custom search path
    custom_search_path = os.environ.get(TEST_OPTIMIZATION_SDK_NATIVE_SEARCH_PATH)
    if custom_search_path:
        custom_path = Path(custom_search_path)

        print(f"Custom search path: {custom_path}")

        # First check for already extracted library files
        if any(custom_path.glob("*.so")) or any(custom_path.glob("*.dll")) or any(custom_path.glob("*.dylib")):
            return  # Library already exists in custom path
            
        # Then check for .zip files
        lib_filename = get_library_filename()
        lib_zip_path = custom_path / lib_filename
        if lib_zip_path.exists():
            try:
                print(f"Extracting native library from {lib_zip_path}")
                with zipfile.ZipFile(lib_zip_path, 'r') as archive:
                    archive.extractall(custom_path)
                return  # Successfully extracted from .zip
            except Exception as e:
                print(f"Failed to extract native library from {lib_zip_path}: {e}", file=sys.stderr)
                sys.exit(1)
        return  # No library found in custom path
    
    # Get the package directory
    package_dir = Path(__file__).parent
    lib_dir = package_dir / "lib"
    
    # Check if we've already installed the library
    has_library = (
        lib_dir.exists() and 
        (any(lib_dir.glob("*.so")) or any(lib_dir.glob("*.dll")) or any(lib_dir.glob("*.dylib")))
    )
    
    if has_library:
        return  # Library already installed

    if not os.environ.get(TEST_OPTIMIZATION_SDK_SKIP_NATIVE_INSTALL):
        download_native_library() 