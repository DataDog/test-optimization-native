"""
Base class for managing C memory in test optimization entities.

This module provides a base class that handles C memory management for all
test optimization entities, ensuring proper allocation and deallocation of
C memory.
"""

from typing import List, Any
from cffi import FFI


class BaseEntity:
    """Base class for test optimization entities that manage C memory."""

    def __init__(self):
        """Initialize memory tracking lists."""
        self._c_strings: List[Any] = []  # Track C strings
        self._c_structs: List[Any] = []  # Track C structs
        self._c_arrays: List[Any] = []   # Track C arrays
        self._c_pointers: List[Any] = [] # Track C pointers
        self._closed = False

    def __del__(self):
        """Clean up all C memory when the object is deleted."""
        if not self._closed:
            try:
                self.close()
            except Exception:
                pass
        
        # Clear all C memory references
        self._c_strings = []
        self._c_structs = []
        self._c_arrays = []
        self._c_pointers = []

    def __enter__(self):
        """Context manager entry."""
        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        """Context manager exit."""
        if not self._closed:
            self.close()

    def close(self):
        """Close the entity and clean up resources."""
        if not self._closed:
            self._closed = True
            # Clear all C memory references
            self._c_strings = []
            self._c_structs = []
            self._c_arrays = []
            self._c_pointers = []

    def _track_c_string(self, c_string: Any) -> Any:
        """Track a C string to prevent garbage collection.
        
        Args:
            c_string: The C string to track
            
        Returns:
            The tracked C string
        """
        self._c_strings.append(c_string)
        return c_string

    def _track_c_struct(self, c_struct: Any) -> Any:
        """Track a C struct to prevent garbage collection.
        
        Args:
            c_struct: The C struct to track
            
        Returns:
            The tracked C struct
        """
        self._c_structs.append(c_struct)
        return c_struct

    def _track_c_array(self, c_array: Any) -> Any:
        """Track a C array to prevent garbage collection.
        
        Args:
            c_array: The C array to track
            
        Returns:
            The tracked C array
        """
        self._c_arrays.append(c_array)
        return c_array

    def _track_c_pointer(self, c_pointer: Any) -> Any:
        """Track a C pointer to prevent garbage collection.
        
        Args:
            c_pointer: The C pointer to track
            
        Returns:
            The tracked C pointer
        """
        self._c_pointers.append(c_pointer)
        return c_pointer 