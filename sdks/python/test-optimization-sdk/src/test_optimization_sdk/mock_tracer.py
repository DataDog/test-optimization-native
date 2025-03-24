"""
Mock tracer module for testing.

This module provides a mock tracer implementation for testing purposes.
"""

from dataclasses import dataclass
from datetime import datetime
from typing import Dict, List, Optional, Union

from .lib import ffi, lib


@dataclass
class MockSpan:
    """Represents a mock span for testing."""

    span_id: int
    trace_id: int
    parent_span_id: int
    start_time: datetime
    finish_time: datetime
    operation_name: str
    string_tags: Dict[str, str]
    number_tags: Dict[str, float]

    @staticmethod
    def _convert_unix_time(ut: "ffi.CData") -> datetime:
        """Convert a Unix time struct to a datetime object.

        Args:
            ut: A topt_UnixTime struct

        Returns:
            A datetime object
        """
        return datetime.fromtimestamp(ut.sec + ut.nsec / 1_000_000_000)

    @classmethod
    def from_c_struct(cls, span: "ffi.CData") -> "MockSpan":
        """Create a MockSpan from a C struct.

        Args:
            span: A topt_MockSpan struct

        Returns:
            A MockSpan instance
        """
        # Convert string tags
        string_tags: Dict[str, str] = {}
        for i in range(span.string_tags.len):
            pair = span.string_tags.data[i]
            key = ffi.string(pair.key).decode()
            value = ffi.string(pair.value).decode()
            string_tags[key] = value

        # Convert number tags
        number_tags: Dict[str, float] = {}
        for i in range(span.number_tags.len):
            pair = span.number_tags.data[i]
            key = ffi.string(pair.key).decode()
            number_tags[key] = pair.value

        return cls(
            span_id=span.span_id,
            trace_id=span.trace_id,
            parent_span_id=span.parent_span_id,
            start_time=cls._convert_unix_time(span.start_time),
            finish_time=cls._convert_unix_time(span.finish_time),
            operation_name=ffi.string(span.operation_name).decode(),
            string_tags=string_tags,
            number_tags=number_tags,
        )


class MockTracer:
    """Mock tracer for testing."""

    def __init__(self, session_id: int):
        """Initialize the mock tracer.

        Args:
            session_id: The session ID
        """
        self.session_id = session_id

    @staticmethod
    def reset() -> bool:
        """Reset the mock tracer.

        Returns:
            True if successful, False otherwise
        """
        return bool(lib.topt_debug_mock_tracer_reset())

    @staticmethod
    def get_finished_spans() -> List[MockSpan]:
        """Get all finished spans.

        Returns:
            A list of finished spans
        """
        # Get the spans
        spans = lib.topt_debug_mock_tracer_get_finished_spans()

        # Convert to Python objects
        result = []
        for i in range(spans.len):
            span = spans.data[i]
            result.append(MockSpan.from_c_struct(span))

        # Free the array
        lib.topt_debug_mock_tracer_free_mock_span_array(spans)

        return result

    @staticmethod
    def get_open_spans() -> List[MockSpan]:
        """Get all open spans.

        Returns:
            A list of open spans
        """
        # Get the spans
        spans = lib.topt_debug_mock_tracer_get_open_spans()

        # Convert to Python objects
        result = []
        for i in range(spans.len):
            span = spans.data[i]
            result.append(MockSpan.from_c_struct(span))

        # Free the array
        lib.topt_debug_mock_tracer_free_mock_span_array(spans)

        return result 