import os
import platform
import sys
import urllib.request
import zipfile
from pathlib import Path
from .constants import TEST_OPTIMIZATION_SDK_SKIP_NATIVE_INSTALL, TEST_OPTIMIZATION_SDK_NATIVE_SEARCH_PATH, TEST_OPTIMIZATION_DOWNLOAD_URL_FORMAT

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
    # Check for custom search path
    custom_search_path = os.environ.get(TEST_OPTIMIZATION_SDK_NATIVE_SEARCH_PATH)
    if custom_search_path:
        custom_path = Path(custom_search_path)

        print(f"Custom search path: {custom_path}")

        # First check for already extracted library files
        if any(custom_path.glob("*.so")) or any(custom_path.glob("*.dll")) or any(custom_path.glob("*.dylib")):
            return  # Library already exists in custom path
            
        # Then check for .7z files
        lib_filename = get_library_filename()
        lib_7z_path = custom_path / lib_filename
        if lib_7z_path.exists():
            try:
                print(f"Extracting native library from {lib_7z_path}")
                import py7zr
                with py7zr.SevenZipFile(lib_7z_path, 'r') as archive:
                    archive.extractall(custom_path)
                return  # Successfully extracted from .7z
            except Exception as e:
                print(f"Failed to extract native library from {lib_7z_path}: {e}", file=sys.stderr)
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