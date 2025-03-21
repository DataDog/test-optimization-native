// Unless explicitly stated otherwise all files in this repository are licensed
// under the Apache License Version 2.0.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2025 Datadog, Inc.

//! Utility functions for the test optimization library
//!
//! This module provides utility functions for the test optimization library,
//! including converting between Rust booleans and C-style booleans, and getting
//! the current time in nanoseconds since the Unix epoch.
#![allow(non_snake_case)]

use crate::test_optimization::lib::{topt_UnixTime, Bool};
use std::time::{SystemTime, UNIX_EPOCH};

/// Gets the current time in nanoseconds since the Unix epoch
pub(in crate::test_optimization) fn get_now() -> topt_UnixTime {
    let now =SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap();
    topt_UnixTime {
        sec: now.as_secs(),
        nsec: now.subsec_nanos() as u64,
    }
}

/// Converts a C-style boolean (0 or 1) to a Rust bool
pub(in crate::test_optimization) fn Bool_to_bool(value: Bool) -> bool {
    value != 0
}
