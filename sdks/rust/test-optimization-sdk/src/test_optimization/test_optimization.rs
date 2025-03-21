// Unless explicitly stated otherwise all files in this repository are licensed
// under the Apache License Version 2.0.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2025 Datadog, Inc.

//! Test optimization module for managing the test optimization library
//!
//! This module provides functionality for initializing and shutdown the library.
//! Also access to the backend features.

use crate::test_optimization::lib::*;
use crate::test_optimization::utils::*;
use std::collections::HashMap;
use std::ffi::{c_char, CStr, CString};
use std::ptr::null_mut;

#[derive(Debug, Clone)]
/// Represents the settings for a test session
pub struct Settings {
    /// Whether code coverage is enabled
    #[allow(dead_code)]
    pub code_coverage: bool,
    /// Settings for early flake detection
    #[allow(dead_code)]
    pub early_flake_detection: EfDSettings,
    /// Whether flaky test retries are enabled
    #[allow(dead_code)]
    pub flaky_test_retries_enabled: bool,
    /// Whether intelligent test runner is enabled
    #[allow(dead_code)]
    pub itr_enabled: bool,
    /// Whether git integration is required
    #[allow(dead_code)]
    pub require_git: bool,
    /// Whether test skipping is enabled
    #[allow(dead_code)]
    pub tests_skipping: bool,
    /// Whether known tests tracking is enabled
    #[allow(dead_code)]
    pub known_tests_enabled: bool,
    /// Settings for test management
    #[allow(dead_code)]
    pub test_management: TestManagementSettings,
}

#[derive(Debug, Clone)]
/// Settings for early flake detection
pub struct EfDSettings {
    /// Whether early flake detection is enabled
    #[allow(dead_code)]
    pub enabled: bool,
    /// Settings for slow test retries
    #[allow(dead_code)]
    pub slow_test_retries: EfdSlowTestRetriesSettings,
    /// Threshold for faulty session detection
    #[allow(dead_code)]
    pub faulty_session_threshold: i32,
}

#[derive(Debug, Clone)]
/// Settings for slow test retries in early flake detection
pub struct EfdSlowTestRetriesSettings {
    /// Number of retries for 5-minute tests
    #[allow(dead_code)]
    pub five_m: i32,
    /// Number of retries for 30-second tests
    #[allow(dead_code)]
    pub thirty_s: i32,
    /// Number of retries for 10-second tests
    #[allow(dead_code)]
    pub ten_s: i32,
    /// Number of retries for 5-second tests
    #[allow(dead_code)]
    pub five_s: i32,
}

#[derive(Debug, Clone)]
/// Settings for flaky test retries
pub struct FlakyTestRetriesSettings {
    /// Number of retries for flaky tests
    #[allow(dead_code)]
    pub retry_count: i32,
    /// Total number of retries allowed
    #[allow(dead_code)]
    pub total_retry_count: i32,
}

#[derive(Debug, Clone)]
/// Settings for test management
pub struct TestManagementSettings {
    /// Whether test management is enabled
    #[allow(dead_code)]
    pub enabled: bool,
    /// Number of retries for attempt-to-fix operations
    #[allow(dead_code)]
    pub attempt_to_fix_retries: i32,
}

#[derive(Debug, Clone)]
/// Represents a skippable test
pub struct SkippableTest {
    /// Name of the test suite
    #[allow(dead_code)]
    pub suite_name: String,
    /// Name of the test
    #[allow(dead_code)]
    pub test_name: String,
    /// Test parameters
    #[allow(dead_code)]
    pub parameters: String,
    /// Custom configurations in JSON format
    #[allow(dead_code)]
    pub custom_configurations_json: String,
}

#[derive(Debug, Clone)]
/// Represents a test managed by the test management system
pub struct TestManagementTest {
    /// Name of the module
    #[allow(dead_code)]
    pub module_name: String,
    /// Name of the test suite
    #[allow(dead_code)]
    pub suite_name: String,
    /// Name of the test
    #[allow(dead_code)]
    pub test_name: String,
    /// Whether the test is quarantined
    #[allow(dead_code)]
    pub quarantined: bool,
    /// Whether the test is disabled
    #[allow(dead_code)]
    pub disabled: bool,
    /// Whether the test is attempt-to-fix
    #[allow(dead_code)]
    pub attempt_to_fix: bool,
}

/// Language name for the test session
pub(in crate::test_optimization) static LANGUAGE_NAME: &str = "rust";
/// Runtime name for the test session
pub(in crate::test_optimization) static RUNTIME_NAME: &str = "rustc";

#[derive(Debug, Clone)]
/// Represents a test session
pub struct TestOptimization;

impl TestOptimization {
    /// Get the runtime version
    #[allow(dead_code)]
    pub fn runtime_version() -> String {
        rustc_version_runtime::version().to_string()
    }

    /// Initialize the test optimization library
    #[allow(dead_code)]
    pub fn init() -> bool {
        Self::init_with_values(
            LANGUAGE_NAME,
            RUNTIME_NAME,
            Self::runtime_version(),
            None::<&str>,
            false,
        )
    }

    /// Initialize the test optimization library with a working directory
    #[allow(dead_code)]
    pub fn init_with_working_dir(working_dir: &str) -> bool {
        Self::init_with_values(
            LANGUAGE_NAME,
            RUNTIME_NAME,
            Self::runtime_version(),
            Some(working_dir),
            false,
        )
    }

    /// Initialize the test optimization library with a mock tracer
    #[allow(dead_code)]
    pub fn init_mock() -> bool {
        Self::init_with_values(
            LANGUAGE_NAME,
            RUNTIME_NAME,
            Self::runtime_version(),
            None::<&str>,
            true,
        )
    }

    /// Initialize the test optimization library with a mock tracer and a working directory
    #[allow(dead_code)]
    pub fn init_mock_with_working_dir(working_dir: &str) -> bool {
        Self::init_with_values(
            LANGUAGE_NAME,
            RUNTIME_NAME,
            Self::runtime_version(),
            Some(working_dir),
            true,
        )
    }

    /// Initialize the test optimization library with specific values
    #[allow(dead_code)]
    pub fn init_with_values(
        language_name: impl AsRef<str>,
        runtime_name: impl AsRef<str>,
        runtime_version: impl AsRef<str>,
        working_directory: Option<impl AsRef<str>>,
        use_mock_tracer: bool,
    ) -> bool {
        #[cfg(target_os = "windows")]
        unsafe {
            // On Windows, call the platform-specific initialization
            // this is required on static libraries compiled by the go toolchain
            // just to start the go runtime
            _rt0_amd64_windows_lib()
        }

        // Create CStrings for the required parameters
        let language_name_cstring = CString::new(language_name.as_ref()).unwrap();
        let runtime_name_cstring = CString::new(runtime_name.as_ref()).unwrap();
        let runtime_version_cstring = CString::new(runtime_version.as_ref()).unwrap();
        // Create an optional CString for working_directory if provided
        let working_directory_cstring =
            working_directory.map(|wd| CString::new(wd.as_ref()).unwrap());

        // Build the initialization options struct, using as_ptr() so the memory is managed automatically
        let init_options = topt_InitOptions {
            language: language_name_cstring.as_ptr() as *mut c_char,
            runtime_name: runtime_name_cstring.as_ptr() as *mut c_char,
            runtime_version: runtime_version_cstring.as_ptr() as *mut c_char,
            working_directory: working_directory_cstring
                .as_ref()
                .map_or(null_mut(), |s| s.as_ptr() as *mut c_char),
            environment_variables: null_mut(),
            global_tags: null_mut(),
            use_mock_tracer: if use_mock_tracer { 1 } else { 0 },
            unused01: null_mut(),
            unused02: null_mut(),
            unused03: null_mut(),
            unused04: null_mut(),
            unused05: null_mut(),
        };

        // Initialize the library with the provided options
        unsafe { Bool_to_bool(topt_initialize(init_options)) }
    }

    /// Shutdown the test optimization library
    #[allow(dead_code)]
    pub fn shutdown() -> bool {
        unsafe { Bool_to_bool(topt_shutdown()) }
    }

    /// Get the current settings
    #[allow(dead_code)]
    pub fn get_settings() -> Settings {
        unsafe {
            let settings_response = topt_get_settings();
            Settings {
                code_coverage: Bool_to_bool(settings_response.code_coverage),
                early_flake_detection: EfDSettings {
                    enabled: Bool_to_bool(settings_response.early_flake_detection.enabled),
                    slow_test_retries: EfdSlowTestRetriesSettings {
                        ten_s: settings_response
                            .early_flake_detection
                            .slow_test_retries
                            .ten_s,
                        thirty_s: settings_response
                            .early_flake_detection
                            .slow_test_retries
                            .thirty_s,
                        five_m: settings_response
                            .early_flake_detection
                            .slow_test_retries
                            .five_m,
                        five_s: settings_response
                            .early_flake_detection
                            .slow_test_retries
                            .five_s,
                    },
                    faulty_session_threshold: settings_response
                        .early_flake_detection
                        .faulty_session_threshold,
                },
                flaky_test_retries_enabled: Bool_to_bool(
                    settings_response.flaky_test_retries_enabled,
                ),
                itr_enabled: Bool_to_bool(settings_response.itr_enabled),
                require_git: Bool_to_bool(settings_response.require_git),
                tests_skipping: Bool_to_bool(settings_response.tests_skipping),
                known_tests_enabled: Bool_to_bool(settings_response.known_tests_enabled),
                test_management: TestManagementSettings {
                    enabled: Bool_to_bool(settings_response.test_management.enabled),
                    attempt_to_fix_retries: settings_response
                        .test_management
                        .attempt_to_fix_retries,
                },
            }
        }
    }

    /// Get the flaky test retries settings
    #[allow(dead_code)]
    pub fn get_flaky_test_retries_settings() -> FlakyTestRetriesSettings {
        unsafe {
            let response = topt_get_flaky_test_retries_settings();
            FlakyTestRetriesSettings {
                retry_count: response.retry_count,
                total_retry_count: response.total_retry_count,
            }
        }
    }

    /// Get the known tests
    #[allow(dead_code)]
    pub fn get_known_tests() -> HashMap<String, HashMap<String, Vec<String>>> {
        unsafe {
            let mut modules_map: HashMap<String, HashMap<String, Vec<String>>> = HashMap::new();
            let known_tests = topt_get_known_tests();
            for i in 0..known_tests.len {
                let element = &*known_tests.data.add(i);

                let module_name_c = CStr::from_ptr(element.module_name);
                let suite_name_c = CStr::from_ptr(element.suite_name);
                let test_name_c = CStr::from_ptr(element.test_name);

                let module_name_string = module_name_c.to_string_lossy().into_owned();
                let suite_name_string = suite_name_c.to_string_lossy().into_owned();
                let test_name = test_name_c.to_string_lossy().into_owned();

                let suites_map = modules_map
                    .entry(module_name_string)
                    .or_insert_with(HashMap::new);
                let tests_vec = suites_map.entry(suite_name_string).or_insert_with(Vec::new);
                tests_vec.push(test_name);
            }
            topt_free_known_tests(known_tests);
            modules_map
        }
    }

    /// Get the skippable tests
    #[allow(dead_code)]
    pub fn get_skippable_tests() -> HashMap<String, HashMap<String, Vec<SkippableTest>>> {
        unsafe {
            let mut suites_map: HashMap<String, HashMap<String, Vec<SkippableTest>>> =
                HashMap::new();
            let skippable_tests = topt_get_skippable_tests();
            for i in 0..skippable_tests.len {
                let element = &*skippable_tests.data.add(i);

                let suite_name_c = CStr::from_ptr(element.suite_name);
                let test_name_c = CStr::from_ptr(element.test_name);
                let parameters_c = CStr::from_ptr(element.parameters);
                let custom_configurations_json_c =
                    CStr::from_ptr(element.custom_configurations_json);

                let suite_name_string = suite_name_c.to_string_lossy().into_owned();
                let test_name_string = test_name_c.to_string_lossy().into_owned();
                let parameters_string = parameters_c.to_string_lossy().into_owned();
                let custom_configurations_json_string =
                    custom_configurations_json_c.to_string_lossy().into_owned();

                let suites_map_entry = suites_map
                    .entry(suite_name_string.clone())
                    .or_insert_with(HashMap::new);
                let tests_vec = suites_map_entry
                    .entry(test_name_string.clone())
                    .or_insert_with(Vec::new);

                tests_vec.push(SkippableTest {
                    suite_name: suite_name_string,
                    test_name: test_name_string,
                    parameters: parameters_string,
                    custom_configurations_json: custom_configurations_json_string,
                });
            }
            topt_free_skippable_tests(skippable_tests);
            suites_map
        }
    }

    /// Get the test management tests
    #[allow(dead_code)]
    pub fn get_test_management_tests(
    ) -> HashMap<String, HashMap<String, HashMap<String, TestManagementTest>>> {
        unsafe {
            let mut modules_map: HashMap<
                String,
                HashMap<String, HashMap<String, TestManagementTest>>,
            > = HashMap::new();
            let test_management_tests = topt_get_test_management_tests();
            for i in 0..test_management_tests.len {
                let element = &*test_management_tests.data.add(i);

                let module_name_c = CStr::from_ptr(element.module_name);
                let suite_name_c = CStr::from_ptr(element.suite_name);
                let test_name_c = CStr::from_ptr(element.test_name);

                let module_name_string = module_name_c.to_string_lossy().into_owned();
                let suite_name_string = suite_name_c.to_string_lossy().into_owned();
                let test_name_string = test_name_c.to_string_lossy().into_owned();

                let modules_map_entry = modules_map
                    .entry(module_name_string.clone())
                    .or_insert_with(HashMap::new);
                let suites_map_entry = modules_map_entry
                    .entry(suite_name_string.clone())
                    .or_insert_with(HashMap::new);
                _ = suites_map_entry.entry(test_name_string.clone()).or_insert(
                    TestManagementTest {
                        module_name: module_name_string,
                        suite_name: suite_name_string,
                        test_name: test_name_string,
                        quarantined: Bool_to_bool(element.quarantined),
                        disabled: Bool_to_bool(element.disabled),
                        attempt_to_fix: Bool_to_bool(element.attempt_to_fix),
                    },
                );
            }
            topt_free_test_management_tests(test_management_tests);
            modules_map
        }
    }
}
