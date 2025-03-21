// Unless explicitly stated otherwise all files in this repository are licensed
// under the Apache License Version 2.0.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2025 Datadog, Inc.

//! Span module for managing tracing spans
//!
//! This module provides functionality for creating and managing spans,
//! setting tags, error information, and closing spans.
use crate::test_optimization::lib::*;
use crate::test_optimization::utils::*;
use std::ffi::{c_char, CString};
use std::ptr::null_mut;

#[derive(Debug, Clone)]
#[allow(dead_code)]
/// Represents a tracing span for performance monitoring
pub struct Span {
    /// The unique identifier for this span
    pub span_id: u64,
    /// The ID of the parent span, if any
    pub parent_id: u64,
}
impl Span {
    /// Creates a new span with the specified parameters
    #[allow(dead_code)]
    pub fn create(
        parent_id: u64,
        operation_name: impl AsRef<str>,
        service_name: impl AsRef<str>,
        resource_name: impl AsRef<str>,
        span_type: impl AsRef<str>,
    ) -> Self {
        let operation_name_cstring = CString::new(operation_name.as_ref()).unwrap();
        let service_name_cstring = CString::new(service_name.as_ref()).unwrap();
        let resource_name_cstring = CString::new(resource_name.as_ref()).unwrap();
        let span_type_cstring = CString::new(span_type.as_ref()).unwrap();
        let mut now = get_now();

        let span_start_options = topt_SpanStartOptions {
            operation_name: operation_name_cstring.as_ptr() as *mut c_char,
            service_name: service_name_cstring.as_ptr() as *mut c_char,
            resource_name: resource_name_cstring.as_ptr() as *mut c_char,
            span_type: span_type_cstring.as_ptr() as *mut c_char,
            start_time: &mut now,
            string_tags: null_mut(),
            number_tags: null_mut(),
        };

        let span_result = unsafe {
            topt_span_create(parent_id, span_start_options)
        };

        Self { span_id: span_result.span_id, parent_id }
    }

    /// Sets a string tag for this span
    #[allow(dead_code)]
    pub fn set_string_tag(&self, key: impl AsRef<str>, value: impl AsRef<str>) -> bool {
        let key_cstring = CString::new(key.as_ref()).unwrap();
        let value_cstring = CString::new(value.as_ref()).unwrap();
        unsafe {
            Bool_to_bool(topt_span_set_string_tag(
                self.span_id,
                key_cstring.as_ptr() as *mut c_char,
                value_cstring.as_ptr() as *mut c_char,
            ))
        }
    }

    /// Sets a numeric tag for this span
    #[allow(dead_code)]
    pub fn set_number_tag(&self, key: impl AsRef<str>, value: f64) -> bool {
        let key_cstring = CString::new(key.as_ref()).unwrap();
        unsafe {
            Bool_to_bool(topt_span_set_number_tag(self.span_id, key_cstring.as_ptr() as *mut c_char, value))
        }
    }

    /// Sets error information for this span
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
            Bool_to_bool(topt_span_set_error(
                self.span_id,
                error_type_cstring.as_ptr() as *mut c_char,
                error_message_cstring.as_ptr() as *mut c_char,
                error_stacktrace_cstring.as_ptr() as *mut c_char,
            ))
        }
    }

    /// Closes this span
    #[allow(dead_code)]
    pub fn close(&self) -> bool {
        let mut now = get_now();
        unsafe {
            Bool_to_bool(topt_span_close(self.span_id, &mut now))
        }
    }
}
