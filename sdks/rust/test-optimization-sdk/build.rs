// Unless explicitly stated otherwise all files in this repository are licensed
// under the Apache License Version 2.0.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2025 Datadog, Inc.

use std::path::{Path};
use std::{env, fs, io, process};
use std::fs::File;
use std::io::{BufWriter, Write};
use ureq::AsSendBody;

const TEST_OPTIMIZATION_SDK_SKIP_NATIVE_INSTALL: &str = "TEST_OPTIMIZATION_SDK_SKIP_NATIVE_INSTALL";
const TEST_OPTIMIZATION_SDK_NATIVE_SEARCH_PATH: &str = "TEST_OPTIMIZATION_SDK_NATIVE_SEARCH_PATH";
const TEST_OPTIMIZATION_DEV_MODE: &str = "TEST_OPTIMIZATION_DEV_MODE";
const TEST_OPTIMIZATION_DOWNLOAD_URL_FORMAT: &str = "https://github.com/DataDog/test-optimization-native/releases/download/v0.0.4-preview/";

fn main() {
    let target = env::var("TARGET").expect("Cargo did not provide TARGET");
    let out_dir = env::var("OUT_DIR").expect("Cargo did not provide OUT_DIR");
    let platform = if target.contains("apple-darwin") { "macos" } else if target.contains("windows") { "windows" } else if target.contains("linux") { "linux" } else { panic!("Unsupported platform: {}", target) };
    let arch = if target.contains("aarch64") { "arm64" } else { "x64" };

    let lib_name = if platform == "macos" {
        format!("{}-libtestoptimization-static.zip", platform)
    } else {
        format!("{}-{}-libtestoptimization-static.zip", platform, arch)
    };

    // Check for dev mode first (highest priority)
    if env::var(TEST_OPTIMIZATION_DEV_MODE).is_ok() {
        link_from_dev_output(platform, arch);
        other_links(&target);
        return;
    }

    // Check for custom native library search path
    if let Ok(search_path) = env::var(TEST_OPTIMIZATION_SDK_NATIVE_SEARCH_PATH) {
        link_from_search_path(platform, &lib_name, &search_path);
    } else {
        let lib_dir = Path::new(&out_dir);

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

            download_library(&out_dir, &lib_name, &lib_dir);
        }

        println!("cargo:rustc-link-search=native={}", lib_dir.display());
        println!("cargo:rustc-link-lib=static=testoptimization");
    }

    other_links(&target);
}

fn download_library(out_dir: &str, lib_name: &str, lib_dir: &Path) {
    // Get the folder
    let url = format!("{}{}", TEST_OPTIMIZATION_DOWNLOAD_URL_FORMAT, lib_name);
    let lib_zip_path = Path::new(out_dir).join("libtestoptimization.zip");

    // Download and extract library only if it doesn't exist
    println!("cargo:warning=Downloading native library from: {}", url);

    let mut response = ureq::get(&url)
        .call()
        .unwrap_or_else(|e| {
            eprintln!("Failed to download native library: {}", e);
            process::exit(1);
        });

    let mut reader = response.body_mut().as_body().into_reader();
    let mut file = BufWriter::new(File::create(&lib_zip_path).unwrap());
    io::copy(&mut reader, &mut file).unwrap_or_else(|e| {
        eprintln!("Failed to write native library to disk: {}", e);
        process::exit(1);
    });
    file.flush().unwrap();

    extract_zip(&lib_zip_path, lib_dir).expect("Failed to decompress native library");
}

fn extract_zip(zip_path: &Path, target_dir: &Path) -> io::Result<()> {
    let file = File::open(zip_path)?;
    let mut archive = zip::ZipArchive::new(io::BufReader::new(file))?;
    
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = target_dir.join(file.name());
        
        if file.name().ends_with('/') {
            fs::create_dir_all(&outpath)?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p)?;
                }
            }
            let mut outfile = File::create(&outpath)?;
            std::io::copy(&mut file, &mut outfile)?;
        }
    }
    Ok(())
}

fn link_from_search_path(platform: &str, lib_name: &str, search_path: &str) {
    let search_path = Path::new(search_path);

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

    let lib_zip_path = search_path.join(&lib_name);
    if lib_zip_path.exists() {
        println!("cargo:warning=Found .zip file in custom search path, extracting...[{}]", lib_zip_path.display());
        extract_zip(&lib_zip_path, &search_path)
            .expect("Failed to decompress native library from custom search path");
        println!("cargo:warning=Using custom native library search path: {}", search_path.display());
        println!("cargo:rustc-link-search=native={}", search_path.display());
        println!("cargo:rustc-link-lib=static=testoptimization");
    }
}

fn link_from_dev_output(platform: &str, arch: &str) {
    // Construct the dev output folder name based on platform and arch
    let folder_name = if platform == "macos" {
        format!("{}-libtestoptimization-static", platform)
    } else {
        format!("{}-{}-libtestoptimization-static", platform, arch)
    };

    // The dev-output directory is relative to the repo root
    // build.rs is in sdks/rust/test-optimization-sdk/, so we go up 3 levels
    let dev_output_path = Path::new("../../../dev-output").join(&folder_name);

    // Check if the library files exist
    let has_library = match platform {
        "windows" => dev_output_path.join("testoptimization.lib").exists(),
        "linux" => dev_output_path.join("libtestoptimization.a").exists(), 
        "macos" => dev_output_path.join("libtestoptimization.a").exists(),
        _ => false,
    };

    if has_library {
        println!("cargo:warning=Using dev mode native library from: {}", dev_output_path.display());
        println!("cargo:rustc-link-search=native={}", dev_output_path.display());
        println!("cargo:rustc-link-lib=static=testoptimization");
    } else {
        println!("cargo:warning=Dev mode enabled but library not found at: {}", dev_output_path.display());
        println!("cargo:warning=Please run the localdev.sh script to build the native libraries first");
        process::exit(1);
    }
}

fn other_links(target: &str) {
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
