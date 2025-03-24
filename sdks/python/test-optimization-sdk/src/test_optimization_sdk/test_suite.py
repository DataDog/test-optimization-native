"""
Test suite module for managing test suites.

This module provides functionality for creating and managing test suites,
setting tags, error information, source code, and closing suites.
"""

from dataclasses import dataclass
from typing import Optional, TYPE_CHECKING

from .lib import (
    topt_suite_set_string_tag,
    topt_suite_set_number_tag,
    topt_suite_set_error,
    topt_suite_close,
    topt_test_create,
)
from .utils import get_now

if TYPE_CHECKING:
    from .test_module import TestModule
    from .test import Test

@dataclass
class TestSuite:
    """Represents a test suite within a module."""

    __test__ = False  # Prevent pytest from collecting this as a test class

    session_id: int  # The ID of the session this suite belongs to
    module_id: int  # The ID of the module this suite belongs to
    suite_id: int  # The unique identifier for this suite

    def get_module(self) -> "TestModule":
        """Get the parent module of this suite.

        Returns:
            The parent TestModule instance
        """
        from .test_module import TestModule
        return TestModule(
            module_id=self.module_id,
            session_id=self.session_id,
        )

    def set_string_tag(self, key: str, value: str) -> bool:
        """Set a string tag for this suite.

        Args:
            key: Tag key
            value: Tag value

        Returns:
            True if successful, False otherwise
        """
        # Create C strings for the key and value
        key_bytes = key.encode()
        value_bytes = value.encode()

        # Set the tag
        return bool(topt_suite_set_string_tag(
            self.suite_id,
            key_bytes,
            value_bytes,
        ))

    def set_number_tag(self, key: str, value: float) -> bool:
        """Set a numeric tag for this suite.

        Args:
            key: Tag key
            value: Tag value

        Returns:
            True if successful, False otherwise
        """
        # Create C string for the key
        key_bytes = key.encode()

        # Set the tag
        return bool(topt_suite_set_number_tag(
            self.suite_id,
            key_bytes,
            value,
        ))

    def set_error_info(
        self,
        error_type: str,
        error_message: str,
        error_stacktrace: str,
    ) -> bool:
        """Set error information for this suite.

        Args:
            error_type: Type of error
            error_message: Error message
            error_stacktrace: Error stacktrace

        Returns:
            True if successful, False otherwise
        """
        # Create C strings for the error information
        error_type_bytes = error_type.encode()
        error_message_bytes = error_message.encode()
        error_stacktrace_bytes = error_stacktrace.encode()

        # Set the error information
        return bool(topt_suite_set_error(
            self.suite_id,
            error_type_bytes,
            error_message_bytes,
            error_stacktrace_bytes,
        ))

    def set_test_source(
        self,
        file: str,
        start_line: Optional[int] = None,
        end_line: Optional[int] = None,
    ) -> bool:
        """Set source code information for this suite.

        Args:
            file: Source file path
            start_line: Optional starting line number
            end_line: Optional ending line number

        Returns:
            True if successful, False otherwise
        """
        # Create C string for the file path
        file_bytes = file.encode()

        # Create pointers for line numbers
        start_line_ptr = start_line if start_line is not None else 0
        end_line_ptr = end_line if end_line is not None else 0

        # Set the source information
        return bool(topt_suite_set_source(
            self.suite_id,
            file_bytes,
            start_line_ptr,
            end_line_ptr,
        ))

    def close(self) -> bool:
        """Close this suite.

        Returns:
            True if successful, False otherwise
        """
        # Get current time
        now = get_now()

        # Close the suite
        return bool(topt_suite_close(self.suite_id, now))

    def create_test(self, name: str) -> "Test":
        """Create a new test within this suite.

        Args:
            name: Test name

        Returns:
            A new Test instance
        """
        # Get current time
        now = get_now()

        # Create C string for the test name
        name_bytes = name.encode()

        # Create the test
        test_result = topt_test_create(
            self.suite_id,
            name_bytes,
            now,
        )

        from .test import Test
        return Test(
            test_id=test_result.test_id,
            suite_id=self.suite_id,
            module_id=self.module_id,
            session_id=self.session_id,
        ) 