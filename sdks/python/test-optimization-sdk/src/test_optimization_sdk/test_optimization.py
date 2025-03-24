"""
Test optimization module for managing the test optimization library.

This module provides functionality for initializing and shutdown the library.
Also access to the backend features.
"""

import platform
import sys
from dataclasses import dataclass
from typing import Dict, List, Optional

from .lib import (
    ffi,
    topt_initialize,
    topt_shutdown,
    topt_get_settings,
    topt_get_flaky_test_retries_settings,
    topt_get_known_tests,
    topt_get_skippable_tests,
    topt_get_test_management_tests,
    topt_free_known_tests,
    topt_free_skippable_tests,
    topt_free_test_management_tests,
    topt_test_set_source,
)


@dataclass
class Settings:
    """Represents the settings for a test session."""

    code_coverage: bool  # Whether code coverage is enabled
    early_flake_detection: "EfDSettings"  # Settings for early flake detection
    flaky_test_retries_enabled: bool  # Whether flaky test retries are enabled
    itr_enabled: bool  # Whether intelligent test runner is enabled
    require_git: bool  # Whether git integration is required
    tests_skipping: bool  # Whether test skipping is enabled
    known_tests_enabled: bool  # Whether known tests tracking is enabled
    test_management: "TestManagementSettings"  # Settings for test management


@dataclass
class EfDSettings:
    """Settings for early flake detection."""

    enabled: bool  # Whether early flake detection is enabled
    slow_test_retries: "EfdSlowTestRetriesSettings"  # Settings for slow test retries
    faulty_session_threshold: int  # Threshold for faulty session detection


@dataclass
class EfdSlowTestRetriesSettings:
    """Settings for slow test retries in early flake detection."""

    five_m: int  # Number of retries for 5-minute tests
    thirty_s: int  # Number of retries for 30-second tests
    ten_s: int  # Number of retries for 10-second tests
    five_s: int  # Number of retries for 5-second tests


@dataclass
class FlakyTestRetriesSettings:
    """Settings for flaky test retries."""

    retry_count: int  # Number of retries for flaky tests
    total_retry_count: int  # Total number of retries allowed


@dataclass
class TestManagementSettings:
    """Settings for test management."""

    enabled: bool  # Whether test management is enabled
    attempt_to_fix_retries: int  # Number of retries for attempt-to-fix operations
    __test__ = False  # Tell pytest to ignore this class


@dataclass
class SkippableTest:
    """Represents a skippable test."""

    suite_name: str  # Name of the test suite
    test_name: str  # Name of the test
    parameters: str  # Test parameters
    custom_configurations_json: str  # Custom configurations in JSON format


@dataclass
class TestManagementTest:
    """Represents a test managed by the test management system."""

    module_name: str  # Name of the module
    suite_name: str  # Name of the test suite
    test_name: str  # Name of the test
    quarantined: bool  # Whether the test is quarantined
    disabled: bool  # Whether the test is disabled
    attempt_to_fix: bool  # Whether the test is attempt-to-fix
    __test__ = False  # Tell pytest to ignore this class


class TestOptimization:
    """Represents a test session."""

    # Language name for the test session
    LANGUAGE_NAME = "python"
    # Runtime name for the test session
    RUNTIME_NAME = "python"

    @staticmethod
    def runtime_version() -> str:
        """Get the runtime version."""
        return f"{sys.version_info.major}.{sys.version_info.minor}.{sys.version_info.micro}"

    @classmethod
    def init(cls) -> bool:
        """Initialize the test optimization library."""
        return cls.init_with_values(
            cls.LANGUAGE_NAME,
            cls.RUNTIME_NAME,
            cls.runtime_version(),
            None,
            False,
        )

    @classmethod
    def init_with_working_dir(cls, working_dir: str) -> bool:
        """Initialize the test optimization library with a working directory."""
        return cls.init_with_values(
            cls.LANGUAGE_NAME,
            cls.RUNTIME_NAME,
            cls.runtime_version(),
            working_dir,
            False,
        )

    @classmethod
    def init_mock(cls) -> bool:
        """Initialize the test optimization library with a mock tracer."""
        return cls.init_with_values(
            cls.LANGUAGE_NAME,
            cls.RUNTIME_NAME,
            cls.runtime_version(),
            None,
            True,
        )

    @classmethod
    def init_mock_with_working_dir(cls, working_dir: str) -> bool:
        """Initialize the test optimization library with a mock tracer and a working directory."""
        return cls.init_with_values(
            cls.LANGUAGE_NAME,
            cls.RUNTIME_NAME,
            cls.runtime_version(),
            working_dir,
            True,
        )

    @classmethod
    def init_with_values(
        cls,
        language_name: str,
        runtime_name: str,
        runtime_version: str,
        working_directory: Optional[str],
        use_mock_tracer: bool,
    ) -> bool:
        """Initialize the test optimization library with specific values."""
        # Create C strings for the required parameters
        language_name_c = ffi.new("char[]", language_name.encode())
        runtime_name_c = ffi.new("char[]", runtime_name.encode())
        runtime_version_c = ffi.new("char[]", runtime_version.encode())
        working_directory_c = ffi.new("char[]", working_directory.encode()) if working_directory else ffi.NULL

        # Build the initialization options struct
        init_options = ffi.new("topt_InitOptions*", {
            "language": language_name_c,
            "runtime_name": runtime_name_c,
            "runtime_version": runtime_version_c,
            "working_directory": working_directory_c,
            "environment_variables": ffi.NULL,
            "global_tags": ffi.NULL,
            "use_mock_tracer": 1 if use_mock_tracer else 0,
            "unused01": ffi.NULL,
            "unused02": ffi.NULL,
            "unused03": ffi.NULL,
            "unused04": ffi.NULL,
            "unused05": ffi.NULL,
        })

        # Initialize the library with the provided options
        return bool(topt_initialize(init_options))

    @staticmethod
    def shutdown() -> bool:
        """Shutdown the test optimization library."""
        return bool(topt_shutdown())

    @staticmethod
    def get_settings() -> Settings:
        """Get the current settings."""
        settings = topt_get_settings()
        return Settings(
            code_coverage=bool(settings.code_coverage),
            early_flake_detection=EfDSettings(
                enabled=bool(settings.early_flake_detection.enabled),
                slow_test_retries=EfdSlowTestRetriesSettings(
                    five_m=settings.early_flake_detection.slow_test_retries.five_m,
                    thirty_s=settings.early_flake_detection.slow_test_retries.thirty_s,
                    ten_s=settings.early_flake_detection.slow_test_retries.ten_s,
                    five_s=settings.early_flake_detection.slow_test_retries.five_s,
                ),
                faulty_session_threshold=settings.early_flake_detection.faulty_session_threshold,
            ),
            flaky_test_retries_enabled=bool(settings.flaky_test_retries_enabled),
            itr_enabled=bool(settings.itr_enabled),
            require_git=bool(settings.require_git),
            tests_skipping=bool(settings.tests_skipping),
            known_tests_enabled=bool(settings.known_tests_enabled),
            test_management=TestManagementSettings(
                enabled=bool(settings.test_management.enabled),
                attempt_to_fix_retries=settings.test_management.attempt_to_fix_retries,
            ),
        )

    @staticmethod
    def get_flaky_test_retries_settings() -> FlakyTestRetriesSettings:
        """Get the current flaky test retries settings."""
        settings = topt_get_flaky_test_retries_settings()
        return FlakyTestRetriesSettings(
            retry_count=settings.retry_count,
            total_retry_count=settings.total_retry_count,
        )

    @staticmethod
    def get_known_tests() -> Dict[str, Dict[str, List[str]]]:
        """Get the list of known tests."""
        # Get the array from the native side
        known_array = topt_get_known_tests()
        try:
            # Convert the C array into a Dict[str, Dict[str, List[str]]]
            result = {}
            if known_array.data:
                for i in range(known_array.len):
                    test = known_array.data[i]
                    module_name = test.module_name.decode() if test.module_name else ""
                    suite_name = test.suite_name.decode() if test.suite_name else ""
                    test_name = test.test_name.decode() if test.test_name else ""

                    if module_name not in result:
                        result[module_name] = {}
                    if suite_name not in result[module_name]:
                        result[module_name][suite_name] = []
                    result[module_name][suite_name].append(test_name)

            return result
        finally:
            # Free the native array
            topt_free_known_tests(known_array)

    @staticmethod
    def get_skippable_tests() -> Dict[str, Dict[str, List[SkippableTest]]]:
        """Get the list of skippable tests."""
        # Get the array from the native side
        skippable_array = topt_get_skippable_tests()
        try:
            # Convert the C array into a Dict[str, Dict[str, List[SkippableTest]]]
            result = {}
            if skippable_array.data:
                for i in range(skippable_array.len):
                    test = skippable_array.data[i]
                    suite_name = test.suite_name.decode() if test.suite_name else ""
                    test_name = test.test_name.decode() if test.test_name else ""

                    if suite_name not in result:
                        result[suite_name] = {}
                    if test_name not in result[suite_name]:
                        result[suite_name][test_name] = []

                    result[suite_name][test_name].append(
                        SkippableTest(
                            suite_name=suite_name,
                            test_name=test_name,
                            parameters=test.parameters.decode() if test.parameters else "",
                            custom_configurations_json=test.custom_configurations_json.decode()
                            if test.custom_configurations_json
                            else "",
                        )
                    )

            return result
        finally:
            # Free the native array
            topt_free_skippable_tests(skippable_array)

    @staticmethod
    def get_test_management_tests() -> Dict[str, Dict[str, Dict[str, TestManagementTest]]]:
        """Get the list of test management tests."""
        # Get the array from the native side
        test_array = topt_get_test_management_tests()
        try:
            # Convert the C array into a Dict[str, Dict[str, Dict[str, TestManagementTest]]]
            result = {}
            if test_array.data:
                for i in range(test_array.len):
                    test = test_array.data[i]
                    module_name = test.module_name.decode() if test.module_name else ""
                    suite_name = test.suite_name.decode() if test.suite_name else ""
                    test_name = test.test_name.decode() if test.test_name else ""

                    if module_name not in result:
                        result[module_name] = {}
                    if suite_name not in result[module_name]:
                        result[module_name][suite_name] = {}
                    if test_name not in result[module_name][suite_name]:
                        result[module_name][suite_name][test_name] = TestManagementTest(
                            module_name=module_name,
                            suite_name=suite_name,
                            test_name=test_name,
                            quarantined=bool(test.quarantined),
                            disabled=bool(test.disabled),
                            attempt_to_fix=bool(test.attempt_to_fix),
                        )

            return result
        finally:
            # Free the native array
            topt_free_test_management_tests(test_array)
