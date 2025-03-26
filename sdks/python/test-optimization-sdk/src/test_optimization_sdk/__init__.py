"""
Datadog Test Optimization SDK for Python.

This SDK provides integration with Datadog's Test Optimization features for Python applications.
"""

# Auto-installation logic MUST come before any other imports
import os
from pathlib import Path
from .native_lib import setup_native_library

# Run the native library setup
setup_native_library()

# Only import other modules AFTER the native library is set up
from test_optimization_sdk.test_optimization import TestOptimization
from test_optimization_sdk.test_session import TestSession
from test_optimization_sdk.test_module import TestModule
from test_optimization_sdk.test_suite import TestSuite
from test_optimization_sdk.test import Test, TestStatus
from test_optimization_sdk.span import Span
from test_optimization_sdk.mock_tracer import MockTracer, MockSpan
from test_optimization_sdk.constants import (
    TEST_OPTIMIZATION_SDK_SKIP_NATIVE_INSTALL,
    TEST_OPTIMIZATION_SDK_NATIVE_SEARCH_PATH,
    TEST_OPTIMIZATION_DOWNLOAD_URL_FORMAT,
)

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
    "TEST_OPTIMIZATION_SDK_SKIP_NATIVE_INSTALL",
    "TEST_OPTIMIZATION_SDK_NATIVE_SEARCH_PATH",
    "TEST_OPTIMIZATION_DOWNLOAD_URL_FORMAT",
] 