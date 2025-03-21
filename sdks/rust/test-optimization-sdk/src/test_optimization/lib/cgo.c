// Unless explicitly stated otherwise all files in this repository are licensed
// under the Apache License Version 2.0.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2025 Datadog, Inc.

#ifdef _MSC_VER
__pragma(comment(lib, "legacy_stdio_definitions.lib"));

// https://github.com/golang/go/issues/42190#issuecomment-1507839987
void _rt0_amd64_windows_lib();

__pragma(section(".CRT$XCU", read));
__declspec(allocate(".CRT$XCU")) void (*init_lib)() = _rt0_amd64_windows_lib;

__pragma(comment(linker, "/include:init_lib"));
#endif
