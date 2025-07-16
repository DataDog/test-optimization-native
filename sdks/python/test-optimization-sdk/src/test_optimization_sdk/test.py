"""
Test module for managing test execution.

This module provides functionality for creating and managing tests,
setting tags, error information, source code, coverage data, benchmark
data, and closing tests with various statuses.
"""

from dataclasses import dataclass
from enum import IntEnum
from typing import Dict, List, Optional, Union, TYPE_CHECKING
import weakref

from .lib import (
    ffi,
    topt_test_set_string_tag,
    topt_test_set_number_tag,
    topt_test_set_error,
    topt_test_set_source,
    topt_test_close,
    topt_send_code_coverage_payload,
    topt_test_set_benchmark_number_data,
    topt_test_set_benchmark_string_data,
    topt_test_log
)
from .utils import get_now
from .base import BaseEntity

if TYPE_CHECKING:
    from .test_suite import TestSuite

class TestStatus(IntEnum):
    """Represents the possible statuses of a test execution."""

    __test__ = False  # Prevent pytest from collecting this as a test class

    Pass = 0  # Test passed successfully
    Fail = 1  # Test failed
    Skip = 2  # Test was skipped


@dataclass
class Test(BaseEntity):
    """Represents an individual test within a test suite."""

    __test__ = False  # Prevent pytest from collecting this as a test class

    session_id: int  # The ID of the session this test belongs to
    module_id: int  # The ID of the module this test belongs to
    suite_id: int  # The ID of the suite this test belongs to
    test_id: int  # The unique identifier for this test

    def __post_init__(self):
        """Initialize the test and set up cleanup."""
        super().__init__()

    def get_suite(self) -> "TestSuite":
        """Get the parent test suite of this test.

        Returns:
            The parent TestSuite instance
        """
        from .test_suite import TestSuite
        return TestSuite(
            suite_id=self.suite_id,
            module_id=self.module_id,
            session_id=self.session_id,
        )

    def set_string_tag(self, key: str, value: str) -> bool:
        """Set a string tag for this test.

        Args:
            key: Tag key
            value: Tag value

        Returns:
            True if successful, False otherwise

        Raises:
            RuntimeError: If the test is already closed
        """
        if self._closed:
            raise RuntimeError("Cannot set tag on closed test")

        try:
            # Create C strings for the key and value
            key_bytes = key.encode()
            value_bytes = value.encode()
            key_cstr = self._track_c_string(ffi.new("char[]", key_bytes))
            value_cstr = self._track_c_string(ffi.new("char[]", value_bytes))

            # Set the tag
            return bool(topt_test_set_string_tag(
                self.test_id,
                key_cstr,
                value_cstr,
            ))
        except Exception as e:
            raise RuntimeError(f"Failed to set string tag: {e}")

    def set_number_tag(self, key: str, value: float) -> bool:
        """Set a numeric tag for this test.

        Args:
            key: Tag key
            value: Tag value

        Returns:
            True if successful, False otherwise

        Raises:
            RuntimeError: If the test is already closed
        """
        if self._closed:
            raise RuntimeError("Cannot set tag on closed test")

        try:
            # Create C string for the key
            key_bytes = key.encode()
            key_cstr = self._track_c_string(ffi.new("char[]", key_bytes))

            # Set the tag
            return bool(topt_test_set_number_tag(
                self.test_id,
                key_cstr,
                value,
            ))
        except Exception as e:
            raise RuntimeError(f"Failed to set number tag: {e}")

    def set_error_info(
        self,
        error_type: str,
        error_message: str,
        error_stacktrace: str,
    ) -> bool:
        """Set error information for this test.

        Args:
            error_type: Type of error
            error_message: Error message
            error_stacktrace: Error stacktrace

        Returns:
            True if successful, False otherwise

        Raises:
            RuntimeError: If the test is already closed
        """
        if self._closed:
            raise RuntimeError("Cannot set error info on closed test")

        try:
            # Create C strings for the error information
            error_type_bytes = error_type.encode()
            error_message_bytes = error_message.encode()
            error_stacktrace_bytes = error_stacktrace.encode()
            
            error_type_cstr = self._track_c_string(ffi.new("char[]", error_type_bytes))
            error_message_cstr = self._track_c_string(ffi.new("char[]", error_message_bytes))
            error_stacktrace_cstr = self._track_c_string(ffi.new("char[]", error_stacktrace_bytes))

            # Set the error information
            return bool(topt_test_set_error(
                self.test_id,
                error_type_cstr,
                error_message_cstr,
                error_stacktrace_cstr,
            ))
        except Exception as e:
            raise RuntimeError(f"Failed to set error info: {e}")

    def set_test_source(
        self,
        file: str,
        start_line: Optional[int] = None,
        end_line: Optional[int] = None,
    ) -> bool:
        """Set source code information for this test.

        Args:
            file: Source file path
            start_line: Optional starting line number
            end_line: Optional ending line number

        Returns:
            True if successful, False otherwise

        Raises:
            RuntimeError: If the test is already closed
        """
        if self._closed:
            raise RuntimeError("Cannot set test source on closed test")

        try:
            # Create C string for the file path
            file_bytes = file.encode()
            file_cstr = self._track_c_string(ffi.new("char[]", file_bytes))

            # Create pointers for line numbers
            start_line_ptr = self._track_c_pointer(ffi.new("int*", start_line)) if start_line is not None else ffi.NULL
            end_line_ptr = self._track_c_pointer(ffi.new("int*", end_line)) if end_line is not None else ffi.NULL

            # Set the source information
            return bool(topt_test_set_source(
                self.test_id,
                file_cstr,
                start_line_ptr,
                end_line_ptr,
            ))
        except Exception as e:
            raise RuntimeError(f"Failed to set test source: {e}")

    def close(self, status: TestStatus) -> bool:
        """Close the test with a specified status.

        Args:
            status: The status to close the test with

        Returns:
            True if successful, False otherwise

        Raises:
            RuntimeError: If the test is already closed
        """
        if self._closed:
            raise RuntimeError("Test is already closed")

        try:
            # Get current time
            now = get_now()
            self._track_c_pointer(now)

            # Create close options using ffi.new
            close_options = self._track_c_struct(ffi.new("topt_TestCloseOptions*", {
                "status": status,
                "finish_time": now,
                "skip_reason": ffi.NULL,
                "unused01": ffi.NULL,
                "unused02": ffi.NULL,
                "unused03": ffi.NULL,
                "unused04": ffi.NULL,
                "unused05": ffi.NULL,
            }))

            # Close the test
            success = bool(topt_test_close(self.test_id, close_options[0]))
            if success:
                self._closed = True
            return success
        except Exception as e:
            raise RuntimeError(f"Failed to close test: {e}")

    def close_with_skip_reason(self, skip_reason: str) -> bool:
        """Close the test with a skip status and reason.

        Args:
            skip_reason: The reason for skipping the test

        Returns:
            True if successful, False otherwise

        Raises:
            RuntimeError: If the test is already closed
        """
        if self._closed:
            raise RuntimeError("Test is already closed")

        if skip_reason:
            try:
                # Create C string for the skip reason
                skip_reason_bytes = skip_reason.encode()
                skip_reason_cstr = self._track_c_string(ffi.new("char[]", skip_reason_bytes))

                # Get current time
                now = get_now()
                self._track_c_pointer(now)

                # Create close options using ffi.new
                close_options = self._track_c_struct(ffi.new("topt_TestCloseOptions*", {
                    "status": TestStatus.Skip,
                    "finish_time": now,
                    "skip_reason": skip_reason_cstr,
                    "unused01": ffi.NULL,
                    "unused02": ffi.NULL,
                    "unused03": ffi.NULL,
                    "unused04": ffi.NULL,
                    "unused05": ffi.NULL,
                }))

                # Close the test
                success = bool(topt_test_close(self.test_id, close_options[0]))
                if success:
                    self._closed = True
                return success
            except Exception as e:
                raise RuntimeError(f"Failed to close test with skip reason: {e}")
        else:
            return self.close(TestStatus.Skip)

    def set_coverage_data(self, files: List[str]) -> None:
        """Set code coverage data for this test.

        Args:
            files: List of file paths with coverage data

        Raises:
            RuntimeError: If the test is already closed
        """
        if self._closed:
            raise RuntimeError("Cannot set coverage data on closed test")

        try:
            # Create coverage file array
            coverage_files = []
            for file in files:
                # Create C string for the file path
                file_bytes = file.encode()
                file_cstr = self._track_c_string(ffi.new("char[]", file_bytes))

                # Create coverage file struct
                coverage_file = self._track_c_struct(ffi.new("topt_TestCoverageFile*", {
                    "filename": file_cstr,
                    "bitmap": ffi.NULL,
                    "bitmap_len": 0,
                }))
                coverage_files.append(coverage_file)

            # Create array of coverage files
            coverage_files_array = self._track_c_array(ffi.new("topt_TestCoverageFile[]", len(files)))
            for i, coverage_file in enumerate(coverage_files):
                coverage_files_array[i] = coverage_file[0]

            # Create coverage data struct
            coverage_data = self._track_c_struct(ffi.new("topt_TestCoverage*", {
                "session_id": self.session_id,
                "suite_id": self.suite_id,
                "test_id": self.test_id,
                "files": coverage_files_array,
                "files_len": len(files),
            }))

            # Send the code coverage payload
            topt_send_code_coverage_payload(coverage_data, 1)
        except Exception as e:
            raise RuntimeError(f"Failed to set coverage data: {e}")

    def set_benchmark_string_data(
        self,
        measure_type: str,
        data: Dict[str, str],
    ) -> bool:
        """Set benchmark string data for this test.

        Args:
            measure_type: Type of measurement
            data: Dictionary of key-value pairs

        Returns:
            True if successful, False otherwise

        Raises:
            RuntimeError: If the test is already closed
        """
        if self._closed:
            raise RuntimeError("Cannot set benchmark data on closed test")

        # If there is no data, we return success
        if not data:
            return True

        try:
            # Create C string for the measure type
            measure_type_bytes = measure_type.encode()
            measure_type_cstr = self._track_c_string(ffi.new("char[]", measure_type_bytes))

            # Create array of key-value pairs
            pairs_array = self._track_c_array(ffi.new("topt_KeyValuePair[]", len(data)))
            for i, (key, value) in enumerate(data.items()):
                # Create C strings for the key and value
                key_bytes = key.encode()
                value_bytes = value.encode()
                key_cstr = self._track_c_string(ffi.new("char[]", key_bytes))
                value_cstr = self._track_c_string(ffi.new("char[]", value_bytes))

                # Set the key-value pair in the array
                pairs_array[i].key = key_cstr
                pairs_array[i].value = value_cstr

            # Create key-value array using ffi.new
            kv_array = self._track_c_struct(ffi.new("topt_KeyValueArray*", {
                "data": pairs_array,
                "len": len(data),
            }))

            # Set the benchmark data
            return bool(topt_test_set_benchmark_string_data(
                self.test_id,
                measure_type_cstr,
                kv_array[0],
            ))
        except Exception as e:
            raise RuntimeError(f"Failed to set benchmark string data: {e}")

    def set_benchmark_number_data(
        self,
        measure_type: str,
        data: Dict[str, float],
    ) -> bool:
        """Set benchmark number data for this test.

        Args:
            measure_type: Type of measurement
            data: Dictionary of key-number pairs

        Returns:
            True if successful, False otherwise

        Raises:
            RuntimeError: If the test is already closed
        """
        if self._closed:
            raise RuntimeError("Cannot set benchmark data on closed test")

        # If there is no data, we return success
        if not data:
            return True

        try:
            # Create C string for the measure type
            measure_type_bytes = measure_type.encode()
            measure_type_cstr = self._track_c_string(ffi.new("char[]", measure_type_bytes))

            # Create array of key-number pairs
            pairs_array = self._track_c_array(ffi.new("topt_KeyNumberPair[]", len(data)))
            for i, (key, value) in enumerate(data.items()):
                # Create C string for the key
                key_bytes = key.encode()
                key_cstr = self._track_c_string(ffi.new("char[]", key_bytes))

                # Set the key-number pair in the array
                pairs_array[i].key = key_cstr
                pairs_array[i].value = value

            # Create key-number array using ffi.new
            kn_array = self._track_c_struct(ffi.new("topt_KeyNumberArray*", {
                "data": pairs_array,
                "len": len(data),
            }))

            # Set the benchmark data
            return bool(topt_test_set_benchmark_number_data(
                self.test_id,
                measure_type_cstr,
                kn_array[0],
            ))
        except Exception as e:
            raise RuntimeError(f"Failed to set benchmark number data: {e}") 

    def log(self, message: str, tags: Optional[str]) -> bool:
        """Log a message for this test.

        Args:
            message: Message to log
            tags: Optional tags to log

        Returns:
            True if successful, False otherwise

        Raises:
            RuntimeError: If the test is already closed
        """
        if self._closed:
            raise RuntimeError("Cannot log on closed test")

        try:
            # Create C strings for the message and tags
            message_bytes = message.encode()
            tags_bytes = tags.encode() if tags else None

            message_cstr = self._track_c_string(ffi.new("char[]", message_bytes))
            tags_cstr = self._track_c_string(ffi.new("char[]", tags_bytes)) if tags else ffi.NULL

            # Log the message
            return bool(topt_test_log(
                self.test_id,
                message_cstr,
                tags_cstr,
            ))
        except Exception as e:
            raise RuntimeError(f"Failed to log the message: {e}")
