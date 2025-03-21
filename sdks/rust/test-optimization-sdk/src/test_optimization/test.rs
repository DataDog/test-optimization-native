// Unless explicitly stated otherwise all files in this repository are licensed
// under the Apache License Version 2.0.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2025 Datadog, Inc.

//! Test module for managing test execution
//!
//! This module provides functionality for creating and managing tests,
//! setting tags, error information, source code, coverage data, benchmark
//! data, and closing tests with various statuses.
use crate::test_optimization::lib::*;
use crate::test_optimization::utils::*;
use crate::test_optimization::*;
use std::alloc::{alloc, dealloc, Layout};
use std::collections::HashMap;
use std::ffi::{c_char, CString};
use std::ptr::null_mut;

#[derive(Debug, Clone)]
#[allow(dead_code)]
/// Represents the possible statuses of a test execution
pub enum TestStatus {
    /// Test passed successfully
    Pass = 0,
    /// Test failed
    Fail = 1,
    /// Test was skipped
    Skip = 2,
}

#[derive(Debug, Clone)]
/// Represents an individual test within a test suite
pub struct Test {
    /// The ID of the session this test belongs to
    pub(in crate::test_optimization) session_id: u64,
    /// The ID of the module this test belongs to
    pub(in crate::test_optimization) module_id: u64,
    /// The ID of the suite this test belongs to
    pub(in crate::test_optimization) suite_id: u64,
    /// The unique identifier for this test
    pub test_id: u64,
}
impl Test {
    /// Gets the parent test suite of this test
    #[allow(dead_code)]
    pub fn get_suite(&self) -> TestSuite {
        TestSuite { suite_id: self.suite_id, module_id: self.module_id, session_id: self.session_id }
    }

    /// Sets a string tag for this test
    #[allow(dead_code)]
    pub fn set_string_tag(&self, key: impl AsRef<str>, value: impl AsRef<str>) -> bool {
        let key_cstring = CString::new(key.as_ref()).unwrap();
        let value_cstring = CString::new(value.as_ref()).unwrap();
        unsafe {
            Bool_to_bool(topt_test_set_string_tag(
                self.test_id,
                key_cstring.as_ptr() as *mut c_char,
                value_cstring.as_ptr() as *mut c_char,
            ))
        }
    }

    /// Sets a numeric tag for this test
    #[allow(dead_code)]
    pub fn set_number_tag(&self, key: impl AsRef<str>, value: f64) -> bool {
        let key_cstring = CString::new(key.as_ref()).unwrap();
        unsafe {
            Bool_to_bool(topt_test_set_number_tag(
                self.test_id,
                key_cstring.as_ptr() as *mut c_char,
                value,
            ))
        }
    }

    /// Sets error information for this test
    #[allow(dead_code)]
    pub fn set_error_info(
        &self,
        error_type: impl AsRef<str>,
        error_message: impl AsRef<str>,
        error_stacktrace: impl AsRef<str>,
    ) -> bool {
        let error_type_cstring = CString::new(error_type.as_ref()).unwrap();
        let error_message_cstring = CString::new(error_message.as_ref()).unwrap();
        let error_stacktrace_cstring = CString::new(error_stacktrace.as_ref()).unwrap();
        unsafe {
            Bool_to_bool(topt_test_set_error(
                self.test_id,
                error_type_cstring.as_ptr() as *mut c_char,
                error_message_cstring.as_ptr() as *mut c_char,
                error_stacktrace_cstring.as_ptr() as *mut c_char,
            ))
        }
    }

    /// Sets source code information for this test
    #[allow(dead_code)]
    pub fn set_test_source(
        &self,
        file: impl AsRef<str>,
        start_line: *const i32,
        end_line: *const i32,
    ) -> bool {
        let file_cstring = CString::new(file.as_ref()).unwrap();
        unsafe {
            Bool_to_bool(topt_test_set_source(
                self.test_id,
                file_cstring.as_ptr() as *mut c_char,
                start_line as *mut i32,
                end_line as *mut i32,
            ))
        }
    }

    /// Closes the test with a specified status
    #[allow(dead_code)]
    pub fn close(&self, status: TestStatus) -> bool {
        let mut now = get_now();
        let close_options = topt_TestCloseOptions {
            status: status as u8,
            finish_time: &mut now,
            skip_reason: null_mut(),
            unused01: null_mut(),
            unused02: null_mut(),
            unused03: null_mut(),
            unused04: null_mut(),
            unused05: null_mut(),
        };
        unsafe {
            Bool_to_bool(topt_test_close(self.test_id, close_options))
        }
    }

    /// Closes the test with a skip status and reason
    #[allow(dead_code)]
    pub fn close_with_skip_reason(&self, skip_reason: impl AsRef<str>) -> bool {
        let skip_reason_ref = skip_reason.as_ref();
        if !skip_reason_ref.is_empty() {
            let skip_reason_cstring = CString::new(skip_reason_ref).unwrap();
            let mut now = get_now();
            let close_options = topt_TestCloseOptions {
                status: TestStatus::Skip as u8,
                finish_time: &mut now,
                skip_reason: skip_reason_cstring.as_ptr() as *mut c_char,
                unused01: null_mut(),
                unused02: null_mut(),
                unused03: null_mut(),
                unused04: null_mut(),
                unused05: null_mut(),
            };
            unsafe { Bool_to_bool(topt_test_close(self.test_id, close_options)) }
        } else {
            self.close(TestStatus::Skip)
        }
    }

    /// Sets code coverage data for this test
    #[allow(dead_code)]
    pub fn set_coverage_data(&self, files: &[impl AsRef<str>]) {
        unsafe {
            // Allocate memory for an array of topt_TestCoverageFile
            let layout = Layout::array::<topt_TestCoverageFile>(files.len()).unwrap();
            let coverage_file_ptr = alloc(layout) as *mut topt_TestCoverageFile;
            // Create a vector to hold the CString values so they remain valid
            let mut cstrings = Vec::with_capacity(files.len());
            for (idx, file) in files.iter().enumerate() {
                // Create a CString from the file string
                let cstr = CString::new(file.as_ref()).unwrap();
                // Store the CString to keep it alive
                cstrings.push(cstr);
                // Get the pointer to the stored CString
                let filename_ptr = cstrings.last().unwrap().as_ptr() as *mut c_char;
                *coverage_file_ptr.add(idx) = topt_TestCoverageFile {
                    filename: filename_ptr,
                    bitmap: null_mut(),
                    bitmap_len: 0,
                };
            }

            let mut coverage_data = topt_TestCoverage {
                session_id: self.session_id,
                suite_id: self.suite_id,
                test_id: self.test_id,
                files: coverage_file_ptr,
                files_len: files.len(),
            };

            // Send the code coverage payload
            topt_send_code_coverage_payload(&mut coverage_data, 1);

            // Deallocate the memory for the array of topt_TestCoverageFile
            dealloc(coverage_file_ptr as *mut u8, layout);
            // The CString objects in `cstrings` are automatically freed when they go out of scope.
        }
    }

    /// Sets benchmark string data for this test
    #[allow(dead_code)]
    pub fn set_benchmark_string_data<K: AsRef<str>, V: AsRef<str>>(
        &self,
        measure_type: impl AsRef<str>,
        data: &HashMap<K, V>,
    ) -> bool {
        // If there is no data, we return success.
        let num_pairs = data.len();
        if num_pairs == 0 {
            return true;
        }
        // Allocate memory for an array of topt_KeyValuePair.
        let layout = Layout::array::<topt_KeyValuePair>(num_pairs).unwrap();
        let kv_array_ptr = unsafe { alloc(layout) as *mut topt_KeyValuePair };

        // Store CStrings to keep them alive during the call.
        let mut cstrings: Vec<CString> = Vec::with_capacity(num_pairs * 2);
        for (i, (key, value)) in data.iter().enumerate() {
            // Convert the key and value to CStrings using their AsRef<str> implementation.
            let key_c = CString::new(key.as_ref()).unwrap();
            let value_c = CString::new(value.as_ref()).unwrap();

            // Prepare the key-value pair.
            let kv = topt_KeyValuePair {
                key: key_c.as_ptr() as *mut c_char,
                value: value_c.as_ptr() as *mut c_char,
            };
            unsafe {
                *kv_array_ptr.add(i) = kv;
            }

            // Push them to keep them alive during the FFI call.
            cstrings.push(key_c);
            cstrings.push(value_c);
        }

        // Build the topt_KeyValueArray.
        let kv_array = topt_KeyValueArray {
            data: kv_array_ptr,
            len: num_pairs,
        };
        let measure_type_c = CString::new(measure_type.as_ref()).unwrap();
        // Call the FFI function.
        let result = unsafe {
            Bool_to_bool(topt_test_set_benchmark_string_data(
                self.test_id,
                measure_type_c.as_ptr() as *mut c_char,
                kv_array,
            ))
        };
        // Free the allocated array memory.
        unsafe { dealloc(kv_array_ptr as *mut u8, layout); }
        result
    }

    /// Sets benchmark numeric data for this test
    #[allow(dead_code)]
    pub fn set_benchmark_number_data<K: AsRef<str>>(
        &self,
        measure_type: impl AsRef<str>,
        data: &HashMap<K, f64>,
    ) -> bool {
        let num_pairs = data.len();
        if num_pairs == 0 {
            return true;
        }
        // Allocate memory for an array of topt_KeyNumberPair.
        let layout = Layout::array::<topt_KeyNumberPair>(num_pairs).unwrap();
        let kn_array_ptr = unsafe { alloc(layout) as *mut topt_KeyNumberPair };

        // Keep keys alive in a vector of CStrings.
        let mut cstrings: Vec<CString> = Vec::with_capacity(num_pairs);
        let mut i = 0;
        for (key, &value) in data.iter() {
            let key_c = CString::new(key.as_ref()).unwrap();
            let kn = topt_KeyNumberPair {
                key: key_c.as_ptr() as *mut c_char,
                value,
            };
            unsafe {
                *kn_array_ptr.add(i) = kn;
            }
            cstrings.push(key_c);
            i += 1;
        }
        let kn_array = topt_KeyNumberArray {
            data: kn_array_ptr,
            len: num_pairs,
        };
        let measure_type_c = CString::new(measure_type.as_ref()).unwrap();
        let result = unsafe {
            Bool_to_bool(topt_test_set_benchmark_number_data(
                self.test_id,
                measure_type_c.as_ptr() as *mut c_char,
                kn_array,
            ))
        };
        unsafe { dealloc(kn_array_ptr as *mut u8, layout); }
        result
    }
}
