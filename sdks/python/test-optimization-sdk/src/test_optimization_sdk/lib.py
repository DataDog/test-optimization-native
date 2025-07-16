"""
CFFI bindings to the native test optimization library.

This module provides the interface between Python and the native C library.
"""

import os
import platform
from cffi import FFI
from .constants import TEST_OPTIMIZATION_SDK_NATIVE_SEARCH_PATH

ffi = FFI()

# Define the C types and structures
ffi.cdef("""
    typedef unsigned char Bool;
    typedef uint64_t Uint64;
    typedef Uint64 topt_TslvId;
    typedef topt_TslvId topt_SessionId;
    typedef topt_TslvId topt_ModuleId;
    typedef topt_TslvId topt_SuiteId;
    typedef topt_TslvId topt_TestId;
    typedef unsigned char topt_TestStatus;

    #define topt_TestStatusPass 0
    #define topt_TestStatusFail 1
    #define topt_TestStatusSkip 2

    typedef struct {
        topt_SessionId session_id;
        Bool valid;
    } topt_SessionResult;

    typedef struct {
        topt_ModuleId module_id;
        Bool valid;
    } topt_ModuleResult;

    typedef struct {
        topt_SuiteId suite_id;
        Bool valid;
    } topt_SuiteResult;

    typedef struct {
        topt_TestId test_id;
        Bool valid;
    } topt_TestResult;

    typedef struct {
        char* key;
        char* value;
    } topt_KeyValuePair;

    typedef struct {
        topt_KeyValuePair* data;
        size_t len;
    } topt_KeyValueArray;

    typedef struct {
        char* key;
        double value;
    } topt_KeyNumberPair;

    typedef struct {
        topt_KeyNumberPair* data;
        size_t len;
    } topt_KeyNumberArray;

    typedef struct {
        char* language;
        char* runtime_name;
        char* runtime_version;
        char* working_directory;
        topt_KeyValueArray* environment_variables;
        topt_KeyValueArray* global_tags;
        Bool use_mock_tracer;
        void* unused01;
        void* unused02;
        void* unused03;
        void* unused04;
        void* unused05;
    } topt_InitOptions;

    typedef struct {
        Uint64 sec;
        Uint64 nsec;
    } topt_UnixTime;

    typedef struct {
        topt_TestStatus status;
        topt_UnixTime* finish_time;
        char* skip_reason;
        void* unused01;
        void* unused02;
        void* unused03;
        void* unused04;
        void* unused05;
    } topt_TestCloseOptions;

    typedef struct {
        int ten_s;
        int thirty_s;
        int five_m;
        int five_s;
    } topt_SettingsEarlyFlakeDetectionSlowRetries;

    typedef struct {
        Bool enabled;
        topt_SettingsEarlyFlakeDetectionSlowRetries slow_test_retries;
        int faulty_session_threshold;
    } topt_SettingsEarlyFlakeDetection;

    typedef struct {
        Bool enabled;
        int attempt_to_fix_retries;
    } topt_SettingsTestManagement;

    typedef struct {
        Bool code_coverage;
        topt_SettingsEarlyFlakeDetection early_flake_detection;
        Bool flaky_test_retries_enabled;
        Bool itr_enabled;
        Bool require_git;
        Bool tests_skipping;
        Bool known_tests_enabled;
        topt_SettingsTestManagement test_management;
        void* unused01;
        void* unused02;
        void* unused03;
        void* unused04;
        void* unused05;
    } topt_SettingsResponse;

    typedef struct {
        int retry_count;
        int total_retry_count;
    } topt_FlakyTestRetriesSettings;

    typedef struct {
        char* module_name;
        char* suite_name;
        char* test_name;
    } topt_KnownTest;

    typedef struct {
        topt_KnownTest* data;
        size_t len;
    } topt_KnownTestArray;

    typedef struct {
        char* suite_name;
        char* test_name;
        char* parameters;
        char* custom_configurations_json;
    } topt_SkippableTest;

    typedef struct {
        topt_SkippableTest* data;
        size_t len;
    } topt_SkippableTestArray;

    typedef struct {
        char* filename;
        void* bitmap;
        size_t bitmap_len;
    } topt_TestCoverageFile;

    typedef struct {
        topt_SessionId session_id;
        topt_SuiteId suite_id;
        topt_TestId test_id;
        topt_TestCoverageFile* files;
        size_t files_len;
    } topt_TestCoverage;

    typedef struct {
        char* module_name;
        char* suite_name;
        char* test_name;
        Bool quarantined;
        Bool disabled;
        Bool attempt_to_fix;
    } topt_TestManagementTestProperties;

    typedef struct {
        topt_TestManagementTestProperties* data;
        size_t len;
    } topt_TestManagementTestPropertiesArray;

    typedef struct {
        char* operation_name;
        char* service_name;
        char* resource_name;
        char* span_type;
        topt_UnixTime* start_time;
        topt_KeyValueArray* string_tags;
        topt_KeyNumberArray* number_tags;
    } topt_SpanStartOptions;

    typedef struct {
        topt_TslvId span_id;
        Bool valid;
    } topt_SpanResult;

    typedef struct {
        topt_TslvId span_id;
        topt_TslvId trace_id;
        topt_TslvId parent_span_id;
        topt_UnixTime start_time;
        topt_UnixTime finish_time;
        char* operation_name;
        topt_KeyValueArray string_tags;
        topt_KeyNumberArray number_tags;
    } topt_MockSpan;

    typedef struct {
        topt_MockSpan* data;
        size_t len;
    } topt_MockSpanArray;

    // Function declarations
    Bool topt_initialize(topt_InitOptions options);
    Bool topt_shutdown(void);
    topt_SettingsResponse topt_get_settings(void);
    topt_FlakyTestRetriesSettings topt_get_flaky_test_retries_settings(void);
    topt_KnownTestArray topt_get_known_tests(void);
    topt_SkippableTestArray topt_get_skippable_tests(void);
    topt_TestManagementTestPropertiesArray topt_get_test_management_tests(void);
    void topt_free_known_tests(topt_KnownTestArray array);
    void topt_free_skippable_tests(topt_SkippableTestArray array);
    void topt_free_test_management_tests(topt_TestManagementTestPropertiesArray array);
    void topt_send_code_coverage_payload(topt_TestCoverage* coverages, size_t coverages_length);

    topt_SessionResult topt_session_create(char* framework, char* framework_version, topt_UnixTime* start_time);
    Bool topt_session_set_string_tag(topt_SessionId session_id, char* key, char* value);
    Bool topt_session_set_number_tag(topt_SessionId session_id, char* key, double value);
    Bool topt_session_set_error(topt_SessionId session_id, char* error_type, char* error_message, char* error_stacktrace);
    void topt_session_close(topt_SessionId session_id, int exit_code, topt_UnixTime* end_time);

    topt_ModuleResult topt_module_create(topt_SessionId session_id, char* name, char* framework_name, char* framework_version, topt_UnixTime* start_time);
    Bool topt_module_set_string_tag(topt_ModuleId module_id, char* key, char* value);
    Bool topt_module_set_number_tag(topt_ModuleId module_id, char* key, double value);
    Bool topt_module_set_error(topt_ModuleId module_id, char* error_type, char* error_message, char* error_stacktrace);
    Bool topt_module_close(topt_ModuleId module_id, topt_UnixTime* end_time);

    topt_SuiteResult topt_suite_create(topt_ModuleId module_id, char* name, topt_UnixTime* start_time);
    Bool topt_suite_set_string_tag(topt_SuiteId suite_id, char* key, char* value);
    Bool topt_suite_set_number_tag(topt_SuiteId suite_id, char* key, double value);
    Bool topt_suite_set_error(topt_SuiteId suite_id, char* error_type, char* error_message, char* error_stacktrace);
    Bool topt_suite_close(topt_SuiteId suite_id, topt_UnixTime* end_time);

    topt_TestResult topt_test_create(topt_SuiteId suite_id, char* name, topt_UnixTime* start_time);
    Bool topt_test_set_string_tag(topt_TestId test_id, char* key, char* value);
    Bool topt_test_set_number_tag(topt_TestId test_id, char* key, double value);
    Bool topt_test_set_error(topt_TestId test_id, char* error_type, char* error_message, char* error_stacktrace);
    Bool topt_test_set_source(topt_TestId test_id, char* file, int* start_line, int* end_line);
    Bool topt_test_close(topt_TestId test_id, topt_TestCloseOptions options);
    Bool topt_test_set_benchmark_string_data(topt_TestId test_id, char* measure_type, topt_KeyValueArray data);
    Bool topt_test_set_benchmark_number_data(topt_TestId test_id, char* measure_type, topt_KeyNumberArray data);
    Bool topt_test_log(topt_TestId test_id, char* message, char* tags);

    topt_SpanResult topt_span_create(topt_TslvId parent_id, topt_SpanStartOptions span_options);
    Bool topt_span_set_string_tag(topt_TslvId span_id, char* key, char* value);
    Bool topt_span_set_number_tag(topt_TslvId span_id, char* key, double value);
    Bool topt_span_set_error(topt_TslvId span_id, char* error_type, char* error_message, char* error_stacktrace);
    Bool topt_span_close(topt_TslvId span_id, topt_UnixTime* end_time);

    // Debug functions
    Bool topt_debug_mock_tracer_reset(void);
    topt_MockSpanArray topt_debug_mock_tracer_get_finished_spans(void);
    topt_MockSpanArray topt_debug_mock_tracer_get_open_spans(void);
    void topt_debug_mock_tracer_free_mock_span_array(topt_MockSpanArray array);
""")

def _get_library_path() -> str:
    """Get the path to the native library."""
    # Get the appropriate library name based on the platform
    system = platform.system().lower()
    if system == "darwin":
        lib_name = "libtestoptimization.dylib"
    elif system == "windows":
        lib_name = "testoptimization.dll"
    else:  # Linux
        lib_name = "libtestoptimization.so"
    
    # First check if a custom search path is provided
    custom_search_path = os.environ.get(TEST_OPTIMIZATION_SDK_NATIVE_SEARCH_PATH)
    if custom_search_path:
        lib_path = os.path.join(custom_search_path, lib_name)
        if os.path.exists(lib_path):
            return lib_path
    
    # Then try to find the library in our package's lib directory
    package_dir = os.path.dirname(os.path.abspath(__file__))
    lib_dir = os.path.join(package_dir, "lib")
    
    # Look for the library in our package's lib directory
    lib_path = os.path.join(lib_dir, lib_name)
    if os.path.exists(lib_path):
        return lib_path
    
    # If not found in our package, fall back to the system search path
    if system == "darwin":
        return "libtestoptimization.dylib"
    elif system == "windows":
        return "testoptimization.dll"
    else:  # Linux
        return "libtestoptimization.so"

# Load the library
try:
    lib_path = _get_library_path()
    print(f"Loading test optimization library from {lib_path}")
    lib = ffi.dlopen(lib_path)
except OSError as e:
    raise RuntimeError(f"Failed to load test optimization library from {lib_path}: {e}")

# Export all the functions and types
globals().update({name: getattr(lib, name) for name in dir(lib) if not name.startswith("_")})

# Export all the types and functions
__all__ = [
    "Bool",
    "Uint64",
    "topt_TslvId",
    "topt_SessionId",
    "topt_ModuleId",
    "topt_SuiteId",
    "topt_TestId",
    "topt_TestStatus",
    "topt_TestStatusPass",
    "topt_TestStatusFail",
    "topt_TestStatusSkip",
    "topt_SessionResult",
    "topt_ModuleResult",
    "topt_SuiteResult",
    "topt_TestResult",
    "topt_KeyValuePair",
    "topt_KeyValueArray",
    "topt_KeyNumberPair",
    "topt_KeyNumberArray",
    "topt_InitOptions",
    "topt_UnixTime",
    "topt_TestCloseOptions",
    "topt_SettingsEarlyFlakeDetectionSlowRetries",
    "topt_SettingsEarlyFlakeDetection",
    "topt_SettingsTestManagement",
    "topt_SettingsResponse",
    "topt_FlakyTestRetriesSettings",
    "topt_KnownTest",
    "topt_KnownTestArray",
    "topt_SkippableTest",
    "topt_SkippableTestArray",
    "topt_TestCoverageFile",
    "topt_TestCoverage",
    "topt_TestManagementTestProperties",
    "topt_TestManagementTestPropertiesArray",
    "topt_SpanStartOptions",
    "topt_SpanResult",
    "topt_MockSpan",
    "topt_MockSpanArray",
    "topt_initialize",
    "topt_shutdown",
    "topt_get_settings",
    "topt_get_flaky_test_retries_settings",
    "topt_get_known_tests",
    "topt_get_skippable_tests",
    "topt_get_test_management_tests",
    "topt_free_known_tests",
    "topt_free_skippable_tests",
    "topt_free_test_management_tests",
    "topt_send_code_coverage_payload",
    "topt_session_create",
    "topt_session_set_string_tag",
    "topt_session_set_number_tag",
    "topt_session_set_error",
    "topt_session_close",
    "topt_module_create",
    "topt_module_set_string_tag",
    "topt_module_set_number_tag",
    "topt_module_set_error",
    "topt_module_close",
    "topt_suite_create",
    "topt_suite_set_string_tag",
    "topt_suite_set_number_tag",
    "topt_suite_set_error",
    "topt_suite_close",
    "topt_test_create",
    "topt_test_set_string_tag",
    "topt_test_set_number_tag",
    "topt_test_set_error",
    "topt_test_set_source",
    "topt_test_close",
    "topt_test_set_benchmark_string_data",
    "topt_test_set_benchmark_number_data",
    "topt_test_log",
    "topt_span_create",
    "topt_span_set_string_tag",
    "topt_span_set_number_tag",
    "topt_span_set_error",
    "topt_span_close",
    "topt_debug_mock_tracer_reset",
    "topt_debug_mock_tracer_get_finished_spans",
    "topt_debug_mock_tracer_get_open_spans",
    "topt_debug_mock_tracer_free_mock_span_array",
] 