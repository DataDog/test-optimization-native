// Unless explicitly stated otherwise all files in this repository are licensed
// under the Apache License Version 2.0.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2025 Datadog, Inc.

//! Mock tracer module for testing and debugging purposes
//!
//! This module provides functionality for creating and managing mock spans,
//! setting tags, error information, and closing spans.
use crate::test_optimization::lib::*;
use crate::test_optimization::utils::*;
use std::collections::HashMap;
use std::ffi::CStr;
use std::time::{Duration, SystemTime};

#[derive(Debug, Clone)]
/// Represents a mock span for testing and debugging purposes
pub struct MockSpan {
    /// The unique identifier for this span
    #[allow(dead_code)]
    pub span_id: u64,
    /// The ID of the trace
    #[allow(dead_code)]
    pub trace_id: u64,
    /// The ID of the parent span, if any
    #[allow(dead_code)]
    pub parent_span_id: u64,
    /// The start time of the span
    #[allow(dead_code)]
    pub start_time: SystemTime,
    /// The end time of the span
    #[allow(dead_code)]
    pub finish_time: SystemTime,
    /// The name of the operation being traced
    #[allow(dead_code)]
    pub operation_name: String,
    /// The tags associated with this span
    #[allow(dead_code)]
    pub string_tags: HashMap<String, String>,
    /// The numeric tags associated with this span
    #[allow(dead_code)]
    pub number_tags: HashMap<String, f64>,
}

#[derive(Debug, Clone)]
/// Represents a mock tracer for testing and debugging purposes
pub struct MockTracer;

impl MockTracer {
    /// Resets the mock tracer to its initial state
    #[allow(dead_code)]
    pub fn reset() -> bool {
        unsafe {
            Bool_to_bool(topt_debug_mock_tracer_reset())
        }
    }

    /// Returns a vector of all finished spans in this mock tracer
    #[allow(dead_code)]
    pub fn get_finished_spans() -> Vec<MockSpan> {
        unsafe {
            // Get the array from the native side.
            let finished_array = topt_debug_mock_tracer_get_finished_spans();
            // Convert the C array into a Vec<MockSpan>.
            let spans = Self::convert_mock_span_array(&finished_array);
            // Free the native array.
            topt_debug_mock_tracer_free_mock_span_array(finished_array);
            // Return
            spans
        }
    }

    /// Returns a vector of all open spans in this mock tracer
    #[allow(dead_code)]
    pub fn get_open_spans() -> Vec<MockSpan> {
        unsafe {
            // Get the array from the native side.
            let open_array = topt_debug_mock_tracer_get_open_spans();
            // Convert the C array into a Vec<MockSpan>.
            let spans = Self::convert_mock_span_array(&open_array);
            // Free the native array.
            topt_debug_mock_tracer_free_mock_span_array(open_array);
            // Return
            spans
        }
    }

    /// Converts a topt_UnixTime to a SystemTime
    fn convert_unix_time(ut: &topt_UnixTime) -> SystemTime {
        SystemTime::UNIX_EPOCH + Duration::new(ut.sec, ut.nsec as u32)
    }

    /// Converts a C KeyValue array to a HashMap<String, String>
    fn convert_key_value_array(array: &topt_KeyValueArray) -> HashMap<String, String> {
        let mut map = HashMap::new();
        if !array.data.is_null() {
            for i in 0..array.len {
                let pair = unsafe { &*array.data.add(i) };
                let key = if pair.key.is_null() {
                    String::new()
                } else {
                    unsafe { CStr::from_ptr(pair.key).to_string_lossy().into_owned() }
                };
                let value = if pair.value.is_null() {
                    String::new()
                } else {
                    unsafe { CStr::from_ptr(pair.value).to_string_lossy().into_owned() }
                };
                map.insert(key, value);
            }
        }
        map
    }

    /// Converts a C KeyNumber array to a HashMap<String, f64>
    fn convert_key_number_array(array: &topt_KeyNumberArray) -> HashMap<String, f64> {
        let mut map = HashMap::new();
        if !array.data.is_null() {
            for i in 0..array.len {
                let pair = unsafe { &*array.data.add(i) };
                let key = if pair.key.is_null() {
                    String::new()
                } else {
                    unsafe { CStr::from_ptr(pair.key).to_string_lossy().into_owned() }
                };
                map.insert(key, pair.value);
            }
        }
        map
    }

    /// Converts a single C topt_MockSpan to our Rust MockSpan struct
    fn convert_mock_span(mock: &topt_MockSpan) -> MockSpan {
        MockSpan {
            span_id: mock.span_id,
            trace_id: mock.trace_id,
            parent_span_id: mock.parent_span_id,
            start_time: Self::convert_unix_time(&mock.start_time),
            finish_time: Self::convert_unix_time(&mock.finish_time),
            operation_name: if mock.operation_name.is_null() {
                String::new()
            } else {
                unsafe { CStr::from_ptr(mock.operation_name).to_string_lossy().into_owned() }
            },
            string_tags: Self::convert_key_value_array(&mock.string_tags),
            number_tags: Self::convert_key_number_array(&mock.number_tags),
        }
    }

    /// Converts a C array of spans into a Vec<MockSpan>
    fn convert_mock_span_array(array: &topt_MockSpanArray) -> Vec<MockSpan> {
        let mut vec = Vec::with_capacity(array.len);
        if !array.data.is_null() {
            for i in 0..array.len {
                let mock_span = unsafe { &*array.data.add(i) };
                vec.push(Self::convert_mock_span(mock_span));
            }
        }
        vec
    }
}