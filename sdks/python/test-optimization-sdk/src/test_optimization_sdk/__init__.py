"""
Datadog Test Optimization SDK for Python.

This SDK provides integration with Datadog's Test Optimization features for Python applications.
"""

# Auto-installation logic MUST come before any other imports
import os
from pathlib import Path

# Check if we need to install the native library
def _ensure_native_library_installed():
    """Install native library if not already present"""
    # Check if we've already installed the library
    package_dir = Path(__file__).parent
    lib_dir = package_dir / "lib"
    
    # Check if any library files exist
    has_library = (
        lib_dir.exists() and 
        (any(lib_dir.glob("*.so")) or any(lib_dir.glob("*.dll")) or any(lib_dir.glob("*.dylib")))
    )
    
    if has_library:
        return  # Library already installed
    
    # If not installed, run the setup
    from .native_lib import setup_native_library
    setup_native_library()

# Run the check when the module is imported, but only if not explicitly disabled
if not os.environ.get("TEST_OPTIMIZATION_SDK_SKIP_NATIVE_INSTALL"):
    _ensure_native_library_installed()

# Only import other modules AFTER the native library is set up
from test_optimization_sdk.test_optimization import TestOptimization
from test_optimization_sdk.test_session import TestSession
from test_optimization_sdk.test_module import TestModule
from test_optimization_sdk.test_suite import TestSuite
from test_optimization_sdk.test import Test, TestStatus
from test_optimization_sdk.span import Span
from test_optimization_sdk.mock_tracer import MockTracer, MockSpan

__version__ = "0.0.1"

__all__ = [
    "TestOptimization",
    "TestSession",
    "TestModule",
    "TestSuite",
    "Test",
    "TestStatus",
    "Span",
    "MockTracer",
    "MockSpan",
] 