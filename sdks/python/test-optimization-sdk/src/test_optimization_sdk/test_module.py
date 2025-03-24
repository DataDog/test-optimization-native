"""
Test module for managing test execution.

This module provides functionality for creating and managing test modules,
setting tags, error information, and closing modules.
"""

from dataclasses import dataclass
from typing import Optional

from .base import BaseEntity
from .lib import (
    ffi,
    topt_module_close,
    topt_module_set_error,
    topt_module_set_number_tag,
    topt_module_set_string_tag,
    topt_suite_create,
)
from .test_suite import TestSuite
from .utils import get_now


@dataclass
class TestModule(BaseEntity):
    """Represents a test module."""

    __test__ = False  # Prevent pytest from collecting this as a test class

    session_id: int  # The ID of the session this module belongs to
    module_id: int  # The unique identifier for this module

    def __post_init__(self):
        """Initialize the module and set up cleanup."""
        super().__init__()

    def set_string_tag(self, key: str, value: str) -> bool:
        """Sets a string tag for this module."""
        try:
            # Create C strings for the key and value
            key_bytes = key.encode()
            value_bytes = value.encode()
            key_cstr = self._track_c_string(ffi.new("char[]", key_bytes))
            value_cstr = self._track_c_string(ffi.new("char[]", value_bytes))

            return bool(
                topt_module_set_string_tag(
                    self.module_id,
                    key_cstr,
                    value_cstr,
                )
            )
        except Exception as e:
            raise RuntimeError(f"Failed to set string tag: {e}")

    def set_number_tag(self, key: str, value: float) -> bool:
        """Sets a numeric tag for this module."""
        try:
            # Create C string for the key
            key_bytes = key.encode()
            key_cstr = self._track_c_string(ffi.new("char[]", key_bytes))

            return bool(
                topt_module_set_number_tag(
                    self.module_id,
                    key_cstr,
                    value,
                )
            )
        except Exception as e:
            raise RuntimeError(f"Failed to set number tag: {e}")

    def set_error_info(
        self,
        error_type: str,
        error_message: str,
        error_stacktrace: str,
    ) -> bool:
        """Sets error information for this module."""
        try:
            # Create C strings for the error information
            error_type_bytes = error_type.encode()
            error_message_bytes = error_message.encode()
            error_stacktrace_bytes = error_stacktrace.encode()
            
            error_type_cstr = self._track_c_string(ffi.new("char[]", error_type_bytes))
            error_message_cstr = self._track_c_string(ffi.new("char[]", error_message_bytes))
            error_stacktrace_cstr = self._track_c_string(ffi.new("char[]", error_stacktrace_bytes))

            return bool(
                topt_module_set_error(
                    self.module_id,
                    error_type_cstr,
                    error_message_cstr,
                    error_stacktrace_cstr,
                )
            )
        except Exception as e:
            raise RuntimeError(f"Failed to set error info: {e}")

    def close(self) -> bool:
        """Closes this module."""
        try:
            # Get current time
            now = get_now()
            self._track_c_pointer(now)

            return bool(topt_module_close(self.module_id, now))
        except Exception as e:
            raise RuntimeError(f"Failed to close module: {e}")

    def create_test_suite(self, name: str) -> TestSuite:
        """Creates a new test suite within this module."""
        try:
            # Create C string for the suite name
            name_bytes = name.encode()
            name_cstr = self._track_c_string(ffi.new("char[]", name_bytes))

            # Get current time
            now = get_now()
            self._track_c_pointer(now)

            # Create the suite
            suite_result = topt_suite_create(
                self.module_id,
                name_cstr,
                now,
            )

            return TestSuite(
                suite_id=suite_result.suite_id,
                module_id=self.module_id,
                session_id=self.session_id,
            )
        except Exception as e:
            raise RuntimeError(f"Failed to create test suite: {e}") 