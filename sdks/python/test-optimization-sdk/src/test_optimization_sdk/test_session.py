"""
Test session module for managing test sessions.

This module provides functionality for creating and managing test sessions,
setting tags, error information, and closing sessions.
"""

import sys
from dataclasses import dataclass
from typing import Optional, Dict, List
import weakref

from .base import BaseEntity
from .lib import (
    ffi,
    topt_session_create,
    topt_session_set_string_tag,
    topt_session_set_number_tag,
    topt_session_set_error,
    topt_session_close,
    topt_module_create,
)
from .test_module import TestModule
from .test_optimization import TestOptimization
from .utils import get_now


@dataclass
class TestSession(BaseEntity):
    """Represents a test session."""

    __test__ = False  # Prevent pytest from collecting this as a test class

    session_id: int  # The unique identifier for this session

    def __post_init__(self):
        """Initialize the session and set up cleanup."""
        super().__init__()
        self._modules: List[TestModule] = []
        self._tags: Dict[str, str] = {}
        self._number_tags: Dict[str, float] = {}

    @classmethod
    def create(
        cls,
        framework: Optional[str] = None,
        framework_version: Optional[str] = None,
    ) -> "TestSession":
        """Create a new test session.

        Args:
            framework: Optional framework name
            framework_version: Optional framework version

        Returns:
            A new TestSession instance

        Raises:
            RuntimeError: If session creation fails
        """
        try:
            # Get current time
            time_ptr = get_now()  # Now returns a pointer to topt_UnixTime

            # Create C strings for the framework and version
            framework_c = ffi.new("char[]", framework.encode() + b"\0") if framework else ffi.NULL
            framework_version_c = ffi.new("char[]", framework_version.encode() + b"\0") if framework_version else ffi.NULL

            # Create the session
            session_result = topt_session_create(
                framework_c,
                framework_version_c,
                time_ptr,  # Pass the pointer directly
            )

            if not session_result.valid:
                raise RuntimeError("Failed to create test session")

            # Create instance and track C memory
            instance = cls(session_id=session_result.session_id)
            if framework:
                instance._track_c_string(framework_c)
            if framework_version:
                instance._track_c_string(framework_version_c)
            instance._track_c_pointer(time_ptr)

            return instance
        except Exception as e:
            raise RuntimeError(f"Failed to create test session: {e}")

    def set_string_tag(self, key: str, value: str) -> bool:
        """Set a string tag for the test session.

        Args:
            key: Tag key
            value: Tag value

        Returns:
            True if successful, False otherwise

        Raises:
            RuntimeError: If the session is already closed
        """
        if self._closed:
            raise RuntimeError("Cannot set tag on closed session")

        try:
            # Create C strings for the key and value
            key_bytes = key.encode()
            value_bytes = value.encode()
            key_cstr = self._track_c_string(ffi.new("char[]", key_bytes))
            value_cstr = self._track_c_string(ffi.new("char[]", value_bytes))

            # Set the tag
            success = bool(topt_session_set_string_tag(
                self.session_id,
                key_cstr,
                value_cstr,
            ))
            if success:
                self._tags[key] = value
            return success
        except Exception as e:
            raise RuntimeError(f"Failed to set string tag: {e}")

    def set_number_tag(self, key: str, value: float) -> bool:
        """Set a numeric tag for the test session.

        Args:
            key: Tag key
            value: Tag value

        Returns:
            True if successful, False otherwise

        Raises:
            RuntimeError: If the session is already closed
        """
        if self._closed:
            raise RuntimeError("Cannot set tag on closed session")

        try:
            # Create C string for the key
            key_bytes = key.encode()
            key_cstr = self._track_c_string(ffi.new("char[]", key_bytes))

            # Set the tag
            success = bool(topt_session_set_number_tag(
                self.session_id,
                key_cstr,
                value,
            ))
            if success:
                self._number_tags[key] = value
            return success
        except Exception as e:
            raise RuntimeError(f"Failed to set number tag: {e}")

    def set_error_info(
        self,
        error_type: str,
        error_message: str,
        error_stacktrace: str,
    ) -> bool:
        """Set error information for the test session.

        Args:
            error_type: Type of error
            error_message: Error message
            error_stacktrace: Error stacktrace

        Returns:
            True if successful, False otherwise

        Raises:
            RuntimeError: If the session is already closed
        """
        if self._closed:
            raise RuntimeError("Cannot set error info on closed session")

        try:
            # Create C strings for the error information
            error_type_bytes = error_type.encode()
            error_message_bytes = error_message.encode()
            error_stacktrace_bytes = error_stacktrace.encode()
            
            error_type_cstr = self._track_c_string(ffi.new("char[]", error_type_bytes))
            error_message_cstr = self._track_c_string(ffi.new("char[]", error_message_bytes))
            error_stacktrace_cstr = self._track_c_string(ffi.new("char[]", error_stacktrace_bytes))

            # Set the error information
            return bool(topt_session_set_error(
                self.session_id,
                error_type_cstr,
                error_message_cstr,
                error_stacktrace_cstr,
            ))
        except Exception as e:
            raise RuntimeError(f"Failed to set error info: {e}")

    def close(self, exit_code: int) -> None:
        """Close the test session.

        Args:
            exit_code: Exit code for the session

        Raises:
            RuntimeError: If the session is already closed
        """
        if self._closed:
            raise RuntimeError("Session is already closed")

        try:
            # Get current time
            now = get_now()
            self._track_c_pointer(now)

            # Close the session
            topt_session_close(self.session_id, exit_code, now)
            self._closed = True

            # If we're panicking, also shutdown the library
            if sys.exc_info()[0] is not None:
                TestOptimization.shutdown()
        except Exception as e:
            raise RuntimeError(f"Failed to close test session: {e}")

    def create_module(
        self,
        name: str,
        framework_name: str,
        framework_version: str,
    ) -> TestModule:
        """Create a new test module.

        Args:
            name: Module name
            framework_name: Framework name
            framework_version: Framework version

        Returns:
            A new TestModule instance

        Raises:
            RuntimeError: If the session is already closed
        """
        if self._closed:
            raise RuntimeError("Cannot create module on closed session")

        try:
            # Get current time
            now = get_now()
            self._track_c_pointer(now)

            # Create C strings for the module information
            name_bytes = name.encode()
            framework_name_bytes = framework_name.encode()
            framework_version_bytes = framework_version.encode()
            
            name_cstr = self._track_c_string(ffi.new("char[]", name_bytes))
            framework_name_cstr = self._track_c_string(ffi.new("char[]", framework_name_bytes))
            framework_version_cstr = self._track_c_string(ffi.new("char[]", framework_version_bytes))

            # Create the module
            module_result = topt_module_create(
                self.session_id,
                name_cstr,
                framework_name_cstr,
                framework_version_cstr,
                now,
            )

            if not module_result.valid:
                raise RuntimeError("Failed to create test module")

            # Create the module instance
            module = TestModule(
                session_id=self.session_id,
                module_id=module_result.module_id,
            )

            # Store reference to keep memory alive
            self._modules.append(module)

            return module
        except Exception as e:
            raise RuntimeError(f"Failed to create test module: {e}") 