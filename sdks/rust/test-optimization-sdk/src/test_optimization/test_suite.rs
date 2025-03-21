// Unless explicitly stated otherwise all files in this repository are licensed
// under the Apache License Version 2.0.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2025 Datadog, Inc.

//! Test suite module for managing test suites
//!
//! This module provides functionality for creating and managing test suites,
//! setting tags, error information, source code, and closing suites.
use crate::test_optimization::lib::*;
use crate::test_optimization::utils::*;
use crate::test_optimization::*;
use std::ffi::{c_char, CString};

#[derive(Debug, Clone)]
/// Represents a test suite within a module
pub struct TestSuite {
    /// The ID of the session this suite belongs to
    pub(in crate::test_optimization) session_id: u64,
    /// The ID of the module this suite belongs to
    pub(in crate::test_optimization) module_id: u64,
    /// The unique identifier for this suite
    pub suite_id: u64,
}
impl TestSuite {
    /// Gets the parent module of this suite
    #[allow(dead_code)]
    pub fn get_module(&self) -> TestModule {
        TestModule { module_id: self.module_id, session_id: self.session_id }
    }

    /// Sets a string tag for this suite
    #[allow(dead_code)]
    pub fn set_string_tag(&self, key: impl AsRef<str>, value: impl AsRef<str>) -> bool {
        let key_cstring = CString::new(key.as_ref()).unwrap();
        let value_cstring = CString::new(value.as_ref()).unwrap();
        unsafe {
            Bool_to_bool(topt_suite_set_string_tag(
                self.suite_id,
                key_cstring.as_ptr() as *mut c_char,
                value_cstring.as_ptr() as *mut c_char,
            ))
        }
    }

    /// Sets a numeric tag for this suite
    #[allow(dead_code)]
    pub fn set_number_tag(&self, key: impl AsRef<str>, value: f64) -> bool {
        let key_cstring = CString::new(key.as_ref()).unwrap();
        unsafe {
            Bool_to_bool(topt_suite_set_number_tag(
                self.suite_id,
                key_cstring.as_ptr() as *mut c_char,
                value,
            ))
        }
    }

    /// Sets error information for this suite
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
            Bool_to_bool(topt_suite_set_error(
                self.suite_id,
                error_type_cstring.as_ptr() as *mut c_char,
                error_message_cstring.as_ptr() as *mut c_char,
                error_stacktrace_cstring.as_ptr() as *mut c_char,
            ))
        }
    }

    /// Sets source code information for this suite
    #[allow(dead_code)]
    pub fn set_test_source(
        &self,
        file: impl AsRef<str>,
        start_line: *const i32,
        end_line: *const i32,
    ) -> bool {
        let file_cstring = CString::new(file.as_ref()).unwrap();
        unsafe {
            Bool_to_bool(topt_suite_set_source(
                self.suite_id,
                file_cstring.as_ptr() as *mut c_char,
                start_line as *mut i32,
                end_line as *mut i32,
            ))
        }
    }

    /// Closes this suite
    #[allow(dead_code)]
    pub fn close(&self) -> bool {
        let mut now = get_now();
        unsafe {
            Bool_to_bool(topt_suite_close(self.suite_id, &mut now))
        }
    }

    /// Creates a new test within this suite
    #[allow(dead_code)]
    pub fn create_test(&self, name: impl AsRef<str>) -> Test {
        let test_name_cstring = CString::new(name.as_ref()).unwrap();
        let mut now = get_now();
        let test_result = unsafe {
            topt_test_create(
                self.suite_id,
                test_name_cstring.as_ptr() as *mut c_char,
                &mut now,
            )
        };
        Test {
            test_id: test_result.test_id,
            suite_id: self.suite_id,
            module_id: self.module_id,
            session_id: self.session_id,
        }
    }
}
