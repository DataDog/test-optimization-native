// Unless explicitly stated otherwise all files in this repository are licensed
// under the Apache License Version 2.0.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2025 Datadog, Inc.

#[cfg_attr(all(windows, target_env = "msvc"), link(
    name = "legacy_stdio_definitions",
    kind = "dylib"
))]
unsafe extern "C" {
    #[cfg(target_os = "windows")]
    pub fn _rt0_amd64_windows_lib();
}