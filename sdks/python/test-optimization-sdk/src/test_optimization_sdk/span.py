"""
Span module for managing test spans.

This module provides functionality for creating and managing test spans,
including setting tags, error information, and closing spans.
"""

from dataclasses import dataclass
from typing import Optional, Dict, List

from .lib import (
    ffi,
    topt_span_create,
    topt_span_set_string_tag,
    topt_span_set_number_tag,
    topt_span_set_error,
    topt_span_close,
)
from .utils import get_now


@dataclass
class Span:
    """Represents a test span."""

    span_id: int  # Span ID

    def __post_init__(self):
        """Initialize the span and set up cleanup."""
        self._closed = False
        self._tags: Dict[str, str] = {}
        self._number_tags: Dict[str, float] = {}

    def __del__(self):
        """Clean up resources when the span is deleted."""
        if not self._closed:
            try:
                self.close()
            except Exception:
                pass
        # Clear references to allow garbage collection
        self._tags = {}
        self._number_tags = {}

    @classmethod
    def create(
        cls,
        parent_id: int,
        operation_name: str,
        service_name: str,
        resource_name: str,
        span_type: str,
    ) -> "Span":
        """Creates a new span with the specified parameters."""
        # Create C strings for the parameters
        operation_name_c = ffi.new("char[]", operation_name.encode() + b"\0")
        service_name_c = ffi.new("char[]", service_name.encode() + b"\0")
        resource_name_c = ffi.new("char[]", resource_name.encode() + b"\0")
        span_type_c = ffi.new("char[]", span_type.encode() + b"\0")

        # Get current time
        time_ptr = get_now()

        # Create span start options
        span_start_options = ffi.new("topt_SpanStartOptions*", {
            "operation_name": operation_name_c,
            "service_name": service_name_c,
            "resource_name": resource_name_c,
            "span_type": span_type_c,
            "start_time": time_ptr,
            "string_tags": ffi.NULL,
            "number_tags": ffi.NULL,
        })

        # Create the span
        span_result = topt_span_create(parent_id, span_start_options[0])

        if not span_result.valid:
            raise RuntimeError("Failed to create span")

        return cls(span_id=span_result.span_id)

    def set_string_tag(self, key: str, value: str) -> bool:
        """Set a string tag for the span.

        Args:
            key: Tag key
            value: Tag value

        Returns:
            True if successful, False otherwise

        Raises:
            RuntimeError: If the span is already closed
        """
        if self._closed:
            raise RuntimeError("Cannot set tag on closed span")

        try:
            # Create C strings for the key and value
            key_c = ffi.new("char[]", key.encode() + b"\0")
            value_c = ffi.new("char[]", value.encode() + b"\0")

            # Set the tag
            success = bool(topt_span_set_string_tag(
                self.span_id,
                key_c,
                value_c,
            ))
            if success:
                self._tags[key] = value
            return success
        except Exception as e:
            raise RuntimeError(f"Failed to set string tag: {e}")

    def set_number_tag(self, key: str, value: float) -> bool:
        """Set a numeric tag for the span.

        Args:
            key: Tag key
            value: Tag value

        Returns:
            True if successful, False otherwise

        Raises:
            RuntimeError: If the span is already closed
        """
        if self._closed:
            raise RuntimeError("Cannot set tag on closed span")

        try:
            # Create C string for the key
            key_c = ffi.new("char[]", key.encode() + b"\0")

            # Set the tag
            success = bool(topt_span_set_number_tag(
                self.span_id,
                key_c,
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
        """Set error information for the span.

        Args:
            error_type: Type of error
            error_message: Error message
            error_stacktrace: Error stacktrace

        Returns:
            True if successful, False otherwise

        Raises:
            RuntimeError: If the span is already closed
        """
        if self._closed:
            raise RuntimeError("Cannot set error info on closed span")

        try:
            # Create C strings for the error information
            error_type_c = ffi.new("char[]", error_type.encode() + b"\0")
            error_message_c = ffi.new("char[]", error_message.encode() + b"\0")
            error_stacktrace_c = ffi.new("char[]", error_stacktrace.encode() + b"\0")

            # Set the error information
            return bool(topt_span_set_error(
                self.span_id,
                error_type_c,
                error_message_c,
                error_stacktrace_c,
            ))
        except Exception as e:
            raise RuntimeError(f"Failed to set error info: {e}")

    def close(self) -> bool:
        """Close the span.

        Returns:
            True if successful, False otherwise

        Raises:
            RuntimeError: If the span is already closed
        """
        if self._closed:
            raise RuntimeError("Span is already closed")

        try:
            # Get current time
            time_ptr = get_now()

            # Close the span
            success = bool(topt_span_close(self.span_id, time_ptr))
            if success:
                self._closed = True
            return success
        except Exception as e:
            raise RuntimeError(f"Failed to close span: {e}") 