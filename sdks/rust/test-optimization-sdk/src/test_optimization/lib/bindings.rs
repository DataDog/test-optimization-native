// Unless explicitly stated otherwise all files in this repository are licensed
// under the Apache License Version 2.0.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2025 Datadog, Inc.

// libtestoptimization_bindings.rs

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]
#![allow(dead_code)]

//
// Rust bindings for the libtestoptimization.h header file.
//
// To use these bindings, make sure to link to your C library (e.g. using
// #[link(name = "testoptimization")] if your library is named "libtestoptimization.so" or similar).
// You might also need to adjust the library name accordingly.

use std::os::raw::{c_char, c_double, c_int, c_uchar, c_void};

pub type Bool = c_uchar; // C: unsigned char
pub type Uint64 = u64;
pub type topt_TslvId = Uint64;
pub type topt_SessionId = topt_TslvId;
pub type topt_ModuleId = topt_TslvId;
pub type topt_SuiteId = topt_TslvId;
pub type topt_TestId = topt_TslvId;
pub type topt_TestStatus = c_uchar;

pub const topt_TestStatusPass: topt_TestStatus = 0;
pub const topt_TestStatusFail: topt_TestStatus = 1;
pub const topt_TestStatusSkip: topt_TestStatus = 2;

#[repr(C)]
pub struct topt_SessionResult {
    pub session_id: topt_SessionId,
    pub valid: Bool,
}

#[repr(C)]
pub struct topt_ModuleResult {
    pub module_id: topt_ModuleId,
    pub valid: Bool,
}

#[repr(C)]
pub struct topt_SuiteResult {
    pub suite_id: topt_SuiteId,
    pub valid: Bool,
}

#[repr(C)]
pub struct topt_TestResult {
    pub test_id: topt_TestId,
    pub valid: Bool,
}

#[repr(C)]
pub struct topt_KeyValuePair {
    pub key: *mut c_char,
    pub value: *mut c_char,
}

#[repr(C)]
pub struct topt_KeyValueArray {
    pub data: *mut topt_KeyValuePair,
    pub len: usize,
}

#[repr(C)]
pub struct topt_KeyNumberPair {
    pub key: *mut c_char,
    pub value: c_double,
}

#[repr(C)]
pub struct topt_KeyNumberArray {
    pub data: *mut topt_KeyNumberPair,
    pub len: usize,
}

#[repr(C)]
pub struct topt_InitOptions {
    pub language: *mut c_char,
    pub runtime_name: *mut c_char,
    pub runtime_version: *mut c_char,
    pub working_directory: *mut c_char,
    pub environment_variables: *mut topt_KeyValueArray,
    pub global_tags: *mut topt_KeyValueArray,
    pub use_mock_tracer: Bool,
    pub unused01: *mut c_void,
    pub unused02: *mut c_void,
    pub unused03: *mut c_void,
    pub unused04: *mut c_void,
    pub unused05: *mut c_void,
}

#[repr(C)]
pub struct topt_UnixTime {
    pub sec: Uint64,
    pub nsec: Uint64,
}

#[repr(C)]
pub struct topt_TestCloseOptions {
    pub status: topt_TestStatus,
    pub finish_time: *mut topt_UnixTime,
    pub skip_reason: *mut c_char,
    pub unused01: *mut c_void,
    pub unused02: *mut c_void,
    pub unused03: *mut c_void,
    pub unused04: *mut c_void,
    pub unused05: *mut c_void,
}

#[repr(C)]
pub struct topt_SettingsEarlyFlakeDetectionSlowRetries {
    pub ten_s: c_int,
    pub thirty_s: c_int,
    pub five_m: c_int,
    pub five_s: c_int,
}

#[repr(C)]
pub struct topt_SettingsEarlyFlakeDetection {
    pub enabled: Bool,
    pub slow_test_retries: topt_SettingsEarlyFlakeDetectionSlowRetries,
    pub faulty_session_threshold: c_int,
}

#[repr(C)]
pub struct topt_SettingsTestManagement {
    pub enabled: Bool,
    pub attempt_to_fix_retries: c_int,
}

#[repr(C)]
pub struct topt_SettingsResponse {
    pub code_coverage: Bool,
    pub early_flake_detection: topt_SettingsEarlyFlakeDetection,
    pub flaky_test_retries_enabled: Bool,
    pub itr_enabled: Bool,
    pub require_git: Bool,
    pub tests_skipping: Bool,
    pub known_tests_enabled: Bool,
    pub test_management: topt_SettingsTestManagement,
    pub unused01: *mut c_void,
    pub unused02: *mut c_void,
    pub unused03: *mut c_void,
    pub unused04: *mut c_void,
    pub unused05: *mut c_void,
}

#[repr(C)]
pub struct topt_FlakyTestRetriesSettings {
    pub retry_count: c_int,
    pub total_retry_count: c_int,
}

#[repr(C)]
pub struct topt_KnownTest {
    pub module_name: *mut c_char,
    pub suite_name: *mut c_char,
    pub test_name: *mut c_char,
}

#[repr(C)]
pub struct topt_KnownTestArray {
    pub data: *mut topt_KnownTest,
    pub len: usize,
}

#[repr(C)]
pub struct topt_SkippableTest {
    pub suite_name: *mut c_char,
    pub test_name: *mut c_char,
    pub parameters: *mut c_char,
    pub custom_configurations_json: *mut c_char,
}

#[repr(C)]
pub struct topt_SkippableTestArray {
    pub data: *mut topt_SkippableTest,
    pub len: usize,
}

#[repr(C)]
pub struct topt_TestCoverageFile {
    pub filename: *mut c_char,
    pub bitmap: *mut c_void,
    pub bitmap_len: usize,
}

#[repr(C)]
pub struct topt_TestCoverage {
    pub session_id: topt_SessionId,
    pub suite_id: topt_SuiteId,
    pub test_id: topt_TestId,
    pub files: *mut topt_TestCoverageFile,
    pub files_len: usize,
}

#[repr(C)]
pub struct topt_TestManagementTestProperties {
    pub module_name: *mut c_char,
    pub suite_name: *mut c_char,
    pub test_name: *mut c_char,
    pub quarantined: Bool,
    pub disabled: Bool,
    pub attempt_to_fix: Bool,
}

#[repr(C)]
pub struct topt_TestManagementTestPropertiesArray {
    pub data: *mut topt_TestManagementTestProperties,
    pub len: usize,
}

#[repr(C)]
pub struct topt_SpanStartOptions {
    pub operation_name: *mut c_char,
    pub service_name: *mut c_char,
    pub resource_name: *mut c_char,
    pub span_type: *mut c_char,
    pub start_time: *mut topt_UnixTime,
    pub string_tags: *mut topt_KeyValueArray,
    pub number_tags: *mut topt_KeyNumberArray,
}

#[repr(C)]
pub struct topt_SpanResult {
    pub span_id: topt_TslvId,
    pub valid: Bool,
}

#[repr(C)]
pub struct topt_MockSpan {
    pub span_id: topt_TslvId,
    pub trace_id: topt_TslvId,
    pub parent_span_id: topt_TslvId,
    pub start_time: topt_UnixTime,
    pub finish_time: topt_UnixTime,
    pub operation_name: *mut c_char,
    pub string_tags: topt_KeyValueArray,
    pub number_tags: topt_KeyNumberArray,
}

#[repr(C)]
pub struct topt_MockSpanArray {
    pub data: *mut topt_MockSpan,
    pub len: usize,
}

unsafe extern "C" {
    // Library initialization and shutdown functions
    pub fn topt_initialize(options: topt_InitOptions) -> Bool;
    pub fn topt_shutdown() -> Bool;

    // Settings and configuration functions
    pub fn topt_get_settings() -> topt_SettingsResponse;
    pub fn topt_get_flaky_test_retries_settings() -> topt_FlakyTestRetriesSettings;

    // Known tests functions
    pub fn topt_get_known_tests() -> topt_KnownTestArray;
    pub fn topt_free_known_tests(knownTests: topt_KnownTestArray);

    // Skippable tests functions
    pub fn topt_get_skippable_tests() -> topt_SkippableTestArray;
    pub fn topt_free_skippable_tests(skippableTests: topt_SkippableTestArray);

    // Code coverage payload function
    pub fn topt_send_code_coverage_payload(coverages: *mut topt_TestCoverage, coverages_length: usize);

    // Test management functions
    pub fn topt_get_test_management_tests() -> topt_TestManagementTestPropertiesArray;
    pub fn topt_free_test_management_tests(testProperties: topt_TestManagementTestPropertiesArray);

    // Session functions
    pub fn topt_session_create(framework: *mut c_char, framework_version: *mut c_char, start_time: *mut topt_UnixTime) -> topt_SessionResult;
    pub fn topt_session_close(session_id: topt_SessionId, exit_code: c_int, finish_time: *mut topt_UnixTime) -> Bool;
    pub fn topt_session_set_string_tag(session_id: topt_SessionId, key: *mut c_char, value: *mut c_char) -> Bool;
    pub fn topt_session_set_number_tag(session_id: topt_SessionId, key: *mut c_char, value: c_double) -> Bool;
    pub fn topt_session_set_error(session_id: topt_SessionId, error_type: *mut c_char, error_message: *mut c_char, error_stacktrace: *mut c_char) -> Bool;

    // Module functions
    pub fn topt_module_create(session_id: topt_SessionId, name: *mut c_char, framework: *mut c_char, framework_version: *mut c_char, start_time: *mut topt_UnixTime) -> topt_ModuleResult;
    pub fn topt_module_close(module_id: topt_ModuleId, finish_time: *mut topt_UnixTime) -> Bool;
    pub fn topt_module_set_string_tag(module_id: topt_ModuleId, key: *mut c_char, value: *mut c_char) -> Bool;
    pub fn topt_module_set_number_tag(module_id: topt_ModuleId, key: *mut c_char, value: c_double) -> Bool;
    pub fn topt_module_set_error(module_id: topt_ModuleId, error_type: *mut c_char, error_message: *mut c_char, error_stacktrace: *mut c_char) -> Bool;

    // Suite functions
    pub fn topt_suite_create(module_id: topt_ModuleId, name: *mut c_char, start_time: *mut topt_UnixTime) -> topt_SuiteResult;
    pub fn topt_suite_close(suite_id: topt_SuiteId, finish_time: *mut topt_UnixTime) -> Bool;
    pub fn topt_suite_set_string_tag(suite_id: topt_SuiteId, key: *mut c_char, value: *mut c_char) -> Bool;
    pub fn topt_suite_set_number_tag(suite_id: topt_SuiteId, key: *mut c_char, value: c_double) -> Bool;
    pub fn topt_suite_set_error(suite_id: topt_SuiteId, error_type: *mut c_char, error_message: *mut c_char, error_stacktrace: *mut c_char) -> Bool;
    pub fn topt_suite_set_source(suite_id: topt_SuiteId, file: *mut c_char, start_line: *mut c_int, end_line: *mut c_int) -> Bool;

    // Test functions
    pub fn topt_test_create(suite_id: topt_SuiteId, name: *mut c_char, start_time: *mut topt_UnixTime) -> topt_TestResult;
    pub fn topt_test_close(test_id: topt_TestId, options: topt_TestCloseOptions) -> Bool;
    pub fn topt_test_set_string_tag(test_id: topt_TestId, key: *mut c_char, value: *mut c_char) -> Bool;
    pub fn topt_test_set_number_tag(test_id: topt_TestId, key: *mut c_char, value: c_double) -> Bool;
    pub fn topt_test_set_error(test_id: topt_TestId, error_type: *mut c_char, error_message: *mut c_char, error_stacktrace: *mut c_char) -> Bool;
    pub fn topt_test_set_source(test_id: topt_TestId, file: *mut c_char, start_line: *mut c_int, end_line: *mut c_int) -> Bool;
    pub fn topt_test_set_benchmark_string_data(test_id: topt_TestId, measure_type: *mut c_char, data_array: topt_KeyValueArray) -> Bool;
    pub fn topt_test_set_benchmark_number_data(test_id: topt_TestId, measure_type: *mut c_char, data_array: topt_KeyNumberArray) -> Bool;

    // Span functions
    pub fn topt_span_create(parent_id: topt_TslvId, span_options: topt_SpanStartOptions) -> topt_SpanResult;
    pub fn topt_span_close(span_id: topt_TslvId, finish_time: *mut topt_UnixTime) -> Bool;
    pub fn topt_span_set_string_tag(span_id: topt_TslvId, key: *mut c_char, value: *mut c_char) -> Bool;
    pub fn topt_span_set_number_tag(span_id: topt_TslvId, key: *mut c_char, value: c_double) -> Bool;
    pub fn topt_span_set_error(span_id: topt_TslvId, error_type: *mut c_char, error_message: *mut c_char, error_stacktrace: *mut c_char) -> Bool;

    // Debug mock tracer functions
    pub fn topt_debug_mock_tracer_reset() -> Bool;
    pub fn topt_debug_mock_tracer_get_finished_spans() -> topt_MockSpanArray;
    pub fn topt_debug_mock_tracer_get_open_spans() -> topt_MockSpanArray;
    pub fn topt_debug_mock_tracer_free_mock_span_array(spans: topt_MockSpanArray);
}
