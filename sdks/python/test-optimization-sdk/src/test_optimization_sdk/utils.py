"""
Utility functions for the test optimization SDK.

This module provides utility functions for the test optimization SDK,
including converting between Python booleans and C-style booleans, and getting
the current time in nanoseconds since the Unix epoch.
"""

import time
from typing import Union, Optional
import sys

from .lib import ffi


def get_now() -> "ffi.CData":
    """Get the current time in nanoseconds since the Unix epoch.

    Returns:
        A pointer to a topt_UnixTime struct containing the current time

    Raises:
        RuntimeError: If time retrieval fails
    """
    try:
        # Get current time in nanoseconds
        now = time.time_ns()
        
        # Convert to seconds and remaining nanoseconds
        sec = now // 1_000_000_000
        nsec = now % 1_000_000_000
        
        # Check for overflow
        if sec > sys.maxsize:
            raise RuntimeError("Time value too large for system")
        
        # Create and return the pointer to the time struct
        return ffi.new("topt_UnixTime*", {
            "sec": sec,
            "nsec": nsec,
        })
    except Exception as e:
        raise RuntimeError(f"Failed to get current time: {e}")


def bool_to_bool(value: Union[bool, int, float, str, None]) -> int:
    """Convert a Python value to a C-style boolean (0 or 1).

    Args:
        value: A Python value to convert

    Returns:
        A C-style boolean (0 or 1)

    Raises:
        TypeError: If value cannot be converted to a boolean
    """
    if value is None:
        return 0
    
    if isinstance(value, bool):
        return 1 if value else 0
    
    if isinstance(value, int):
        return 1 if value != 0 else 0
    
    if isinstance(value, float):
        return 1 if value != 0.0 else 0
    
    if isinstance(value, str):
        value = value.lower()
        if value in ("true", "1", "yes", "on"):
            return 1
        if value in ("false", "0", "no", "off"):
            return 0
        raise TypeError(f"Cannot convert string '{value}' to boolean")
    
    raise TypeError(f"Cannot convert {type(value)} to boolean") 