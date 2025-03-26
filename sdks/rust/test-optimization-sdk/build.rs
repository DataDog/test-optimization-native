// Unless explicitly stated otherwise all files in this repository are licensed
// under the Apache License Version 2.0.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2025 Datadog, Inc.

use std::path::PathBuf;
use std::{env, fs, process};

const TEST_OPTIMIZATION_SDK_SKIP_NATIVE_INSTALL: &str = "TEST_OPTIMIZATION_SDK_SKIP_NATIVE_INSTALL";
const TEST_OPTIMIZATION_SDK_NATIVE_SEARCH_PATH: &str = "TEST_OPTIMIZATION_SDK_NATIVE_SEARCH_PATH";
const TEST_OPTIMIZATION_DOWNLOAD_URL_FORMAT: &str = "https://github.com/DataDog/test-optimization-native/releases/download/v0.0.1-preview/";

fn main() {
    let target = env::var("TARGET").expect("Cargo did not provide TARGET");
    let out_dir = env::var("OUT_DIR").expect("Cargo did not provide OUT_DIR");
    let platform = if target.contains("apple-darwin") { "macos" } else if target.contains("windows") { "windows" } else if target.contains("linux") { "linux" } else { panic!("Unsupported platform: {}", target) };
    let arch = if target.contains("aarch64") { "arm64" } else { "x64" };

    let lib_name = if platform == "macos" {
        format!("{}-libtestoptimization-static.7z", platform)
    } else {
        format!("{}-{}-libtestoptimization-static.7z", platform, arch)
    };

    // Check for custom native library search path
    if let Ok(search_path) = env::var(TEST_OPTIMIZATION_SDK_NATIVE_SEARCH_PATH) {
        link_from_search_path(platform, &lib_name, search_path);
    } else {
        let lib_dir = PathBuf::from(out_dir.clone());

        // Check if library files already exist
        let has_library = match platform {
            "windows" => lib_dir.join("testoptimization.lib").exists(),
            "linux" => lib_dir.join("libtestoptimization.a").exists(),
            "macos" => lib_dir.join("libtestoptimization.a").exists(),
            _ => false,
        };

        if !has_library {
            // Skip download if explicitly disabled
            if env::var(TEST_OPTIMIZATION_SDK_SKIP_NATIVE_INSTALL).is_ok() {
                println!("cargo:warning=Skipping native library installation as {} is set", TEST_OPTIMIZATION_SDK_SKIP_NATIVE_INSTALL);
                return;
            }

            download_library(out_dir, lib_name, &lib_dir);
        }

        println!("cargo:rustc-link-search=native={}", lib_dir.display());
        println!("cargo:rustc-link-lib=static=testoptimization");
    }

    other_links(target);
}

fn download_library(out_dir: String, lib_name: String, lib_dir: &PathBuf) {
    // Get the folder
    let url = format!("{}{}", TEST_OPTIMIZATION_DOWNLOAD_URL_FORMAT, lib_name);
    let lib_7z_path = PathBuf::from(out_dir.clone()).join("libtestoptimization.7z");

    // Download and extract library only if it doesn't exist
    println!("cargo:warning=Downloading native library from: {}", url);
    let response = reqwest::blocking::get(&url)
        .unwrap_or_else(|e| {
            eprintln!("Failed to download native library: {}", e);
            process::exit(1);
        })
        .bytes()
        .unwrap_or_else(|e| {
            eprintln!("Failed to read response body: {}", e);
            process::exit(1);
        });

    fs::write(&lib_7z_path, &response)
        .unwrap_or_else(|e| {
            eprintln!("Failed to write native library to disk: {}", e);
            process::exit(1);
        });

    sevenz_rust::decompress_file(lib_7z_path, lib_dir.clone()).expect("Failed to decompress native library");
}

fn link_from_search_path(platform: &str, lib_name: &String, search_path: String) {
    let search_path = PathBuf::from(search_path);

    // First check for already extracted library files
    let has_library = match platform {
        "windows" => search_path.join("testoptimization.lib").exists(),
        "linux" => search_path.join("libtestoptimization.a").exists(),
        "macos" => search_path.join("libtestoptimization.a").exists(),
        _ => false,
    };

    if has_library {
        println!("cargo:warning=Using custom native library search path: {}", search_path.display());
        println!("cargo:rustc-link-search=native={}", search_path.display());
        println!("cargo:rustc-link-lib=static=testoptimization");
        return;
    }

    let lib_7z_path = search_path.join(lib_name.clone());
    if lib_7z_path.exists() {
        println!("cargo:warning=Found .7z file in custom search path, extracting...[{}]", lib_7z_path.display());
        sevenz_rust::decompress_file(lib_7z_path, search_path.clone())
            .expect("Failed to decompress native library from custom search path");
        println!("cargo:warning=Using custom native library search path: {}", search_path.display());
        println!("cargo:rustc-link-search=native={}", search_path.display());
        println!("cargo:rustc-link-lib=static=testoptimization");
    }
}

fn other_links(target: String) {
    if !target.contains("windows") {
        // Link to the dynamic dependency
        println!("cargo:rustc-link-lib=dylib=resolv");
    } else {
        // Windows version requires cc as a build-dependency
        #[cfg(target_os = "windows")]
        configure_windows();
    }

    // If we are in osx, we need to add a couple of frameworks
    if target.contains("apple-darwin") {
        println!("cargo:rustc-link-lib=framework=CoreFoundation");
        println!("cargo:rustc-link-lib=framework=IOKit");
        println!("cargo:rustc-link-lib=framework=Security");
    }
}

#[cfg(target_os = "windows")]
fn configure_windows() {
    // Windows target
    println!("cargo::rerun-if-changed=src/test_optimization/lib/cgo.c");
    cc::Build::new()
        .file("src/test_optimization/lib/cgo.c")
        .compile("cgo");

    // Link to the lib
    println!("cargo:rustc-link-lib=static=cgo");
}
