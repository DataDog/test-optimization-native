import os
import platform
import sys
import urllib.request
import zipfile
from pathlib import Path

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
        return f"{system}-libtestoptimization-dynamic.7z"
    else:
        return f"{system}-{arch}-libtestoptimization-dynamic.7z"

def download_native_library():
    """Download the native library and extract it to the appropriate location."""
    # Get the package directory
    package_dir = Path(__file__).parent
    lib_dir = package_dir / "lib"
    lib_dir.mkdir(exist_ok=True)
    
    # Get the library filename
    lib_filename = get_library_filename()
    url = f"https://github.com/DataDog/test-optimization-native/releases/download/v0.0.1-preview/{lib_filename}"
    
    # Download the library
    print(f"Downloading native library from: {url}")
    try:
        urllib.request.urlretrieve(url, lib_dir / lib_filename)
    except Exception as e:
        print(f"Failed to download native library: {e}", file=sys.stderr)
        sys.exit(1)
    
    # Extract the library
    try:
        import py7zr
        with py7zr.SevenZipFile(lib_dir / lib_filename, 'r') as archive:
            archive.extractall(lib_dir)
        # Clean up the 7z file
        (lib_dir / lib_filename).unlink()
    except Exception as e:
        print(f"Failed to extract native library: {e}", file=sys.stderr)
        sys.exit(1)

def setup_native_library():
    """Setup the native library during package installation."""
    if not os.environ.get("TEST_OPTIMIZATION_SDK_SKIP_NATIVE_DOWNLOAD"):
        download_native_library() 