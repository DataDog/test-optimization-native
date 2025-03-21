// Unless explicitly stated otherwise all files in this repository are licensed
// under the Apache License Version 2.0.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2025 Datadog, Inc.

//! Test module for managing test execution
//!
//! This module provides functionality for creating and managing test modules,
//! setting tags, error information, and closing modules.
use crate::test_optimization::lib::*;
use crate::test_optimization::utils::*;
use crate::test_optimization::*;
use std::ffi::{c_char, CString};

#[derive(Debug, Clone)]
/// Represents a test module within a session
pub struct TestModule {
    /// The ID of the session this module belongs to
    pub(in crate::test_optimization) session_id: u64,
    /// The unique identifier for this module
    pub module_id: u64,
}
impl TestModule {
    /// Sets a string tag for this module
    #[allow(dead_code)]
    pub fn set_string_tag(&self, key: impl AsRef<str>, value: impl AsRef<str>) -> bool {
        let key_cstring = CString::new(key.as_ref()).unwrap();
        let value_cstring = CString::new(value.as_ref()).unwrap();
        unsafe {
            Bool_to_bool(topt_module_set_string_tag(
                self.module_id,
                key_cstring.as_ptr() as *mut c_char,
                value_cstring.as_ptr() as *mut c_char,
            ))
        }
    }

    /// Sets a numeric tag for this module
    #[allow(dead_code)]
    pub fn set_number_tag(&self, key: impl AsRef<str>, value: f64) -> bool {
        let key_cstring = CString::new(key.as_ref()).unwrap();
        unsafe {
            Bool_to_bool(topt_module_set_number_tag(
                self.module_id,
                key_cstring.as_ptr() as *mut c_char,
                value,
            ))
        }
    }

    /// Sets error information for this module
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
            Bool_to_bool(topt_module_set_error(
                self.module_id,
                error_type_cstring.as_ptr() as *mut c_char,
                error_message_cstring.as_ptr() as *mut c_char,
                error_stacktrace_cstring.as_ptr() as *mut c_char,
            ))
        }
    }

    /// Closes this module
    #[allow(dead_code)]
    pub fn close(&self) -> bool {
        let mut now = get_now();
        unsafe {
            Bool_to_bool(topt_module_close(self.module_id, &mut now))
        }
    }

    /// Creates a new test suite within this module
    #[allow(dead_code)]
    pub fn create_test_suite(&self, name: impl AsRef<str>) -> TestSuite {
        let test_suite_name_cstring = CString::new(name.as_ref()).unwrap();
        let mut now = get_now();
        let suite_result = unsafe {
            topt_suite_create(
                self.module_id,
                test_suite_name_cstring.as_ptr() as *mut c_char,
                &mut now,
            )
        };
        TestSuite {
            suite_id: suite_result.suite_id,
            module_id: self.module_id,
            session_id: self.session_id,
        }
    }
}
