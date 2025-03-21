// Unless explicitly stated otherwise all files in this repository are licensed
// under the Apache License Version 2.0.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2025 Datadog, Inc.

//! Test session module for managing test sessions
//!
//! This module provides functionality for creating and managing test sessions,
//! setting tags, error information, and closing sessions.
use crate::test_optimization::lib::*;
use crate::test_optimization::test_optimization::*;
use crate::test_optimization::utils::*;
use crate::test_optimization::*;
use std::ffi::{c_char, CString};
use std::ptr::null_mut;
use std::thread::panicking;

#[derive(Debug, Clone)]
/// Represents a test session
pub struct TestSession {
    /// Session ID
    #[allow(dead_code)]
    pub session_id: u64,
}
impl TestSession {
    /// Creates a new test session
    #[allow(dead_code)]
    pub fn create(
        framework: Option<impl AsRef<str>>,
        framework_version: Option<impl AsRef<str>>,
    ) -> Self {
        let mut now = get_now();
        let framework_cstring = framework.map(|wd| CString::new(wd.as_ref()).unwrap());
        let framework_version_cstring =
            framework_version.map(|wd| CString::new(wd.as_ref()).unwrap());
        let session_result = unsafe {
            topt_session_create(
                framework_cstring
                    .as_ref()
                    .map_or(null_mut(), |s| s.as_ptr() as *mut c_char),
                framework_version_cstring
                    .as_ref()
                    .map_or(null_mut(), |s| s.as_ptr() as *mut c_char),
                &mut now,
            )
        };
        Self {
            session_id: session_result.session_id,
        }
    }

    /// Set a string tag for the test session
    #[allow(dead_code)]
    pub fn set_string_tag(&self, key: impl AsRef<str>, value: impl AsRef<str>) -> bool {
        let key_cstring = CString::new(key.as_ref()).unwrap();
        let value_cstring = CString::new(value.as_ref()).unwrap();
        unsafe {
            Bool_to_bool(topt_session_set_string_tag(
                self.session_id,
                key_cstring.as_ptr() as *mut c_char,
                value_cstring.as_ptr() as *mut c_char,
            ))
        }
    }

    /// Set a numeric tag for the test session
    #[allow(dead_code)]
    pub fn set_number_tag(&self, key: impl AsRef<str>, value: f64) -> bool {
        let key_cstring = CString::new(key.as_ref()).unwrap();
        unsafe {
            Bool_to_bool(topt_session_set_number_tag(
                self.session_id,
                key_cstring.as_ptr() as *mut c_char,
                value,
            ))
        }
    }

    /// Set error information for the test session
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
            Bool_to_bool(topt_session_set_error(
                self.session_id,
                error_type_cstring.as_ptr() as *mut c_char,
                error_message_cstring.as_ptr() as *mut c_char,
                error_stacktrace_cstring.as_ptr() as *mut c_char,
            ))
        }
    }

    /// Close the test session
    #[allow(dead_code)]
    pub fn close(&self, exit_code: i32) {
        let mut now = get_now();
        unsafe {
            if panicking() {
                topt_session_close(self.session_id, 1, &mut now);
                TestOptimization::shutdown();
            } else {
                topt_session_close(self.session_id, exit_code, &mut now);
            }
        }
    }

    /// Create a new test module
    #[allow(dead_code)]
    pub fn create_module(
        &self,
        name: impl AsRef<str>,
        framework_name: impl AsRef<str>,
        framework_version: impl AsRef<str>,
    ) -> TestModule {
        let module_name_cstring = CString::new(name.as_ref()).unwrap();
        let framework_name_cstring = CString::new(framework_name.as_ref()).unwrap();
        let framework_version_cstring = CString::new(framework_version.as_ref()).unwrap();

        let mut now = get_now();
        let module_result = unsafe {
            topt_module_create(
                self.session_id,
                module_name_cstring.as_ptr() as *mut c_char,
                framework_name_cstring.as_ptr() as *mut c_char,
                framework_version_cstring.as_ptr() as *mut c_char,
                &mut now,
            )
        };

        TestModule {
            session_id: self.session_id,
            module_id: module_result.module_id,
        }
    }
}
