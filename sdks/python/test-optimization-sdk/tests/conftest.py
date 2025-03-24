"""
Pytest configuration file.

This file contains test fixtures and configuration for pytest.
"""

import os
import sys
import pytest


@pytest.fixture(autouse=True)
def setup_test_environment():
    """Set up the test environment before each test."""
    # Add the package root to the Python path
    package_root = os.path.abspath(os.path.join(os.path.dirname(__file__), ".."))
    if package_root not in sys.path:
        sys.path.insert(0, package_root)

    # Here you can add any other setup needed for the tests
    yield
    # Cleanup after the test if needed 