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
const TEST_OPTIMIZATION_DOWNLOAD_URL_FORMAT: &str = "https://github.com/DataDog/test-optimization-native/releases/download/v0.0.2-preview/";

fn main() {
    let target = env::var("TARGET").expect("Cargo did not provide TARGET");
    let out_dir = env::var("OUT_DIR").expect("Cargo did not provide OUT_DIR");

    let target_os = env::var("CARGO_CFG_TARGET_OS").expect("Cargo did not provide CARGO_CFG_TARGET_OS");
    let target_env = env::var("CARGO_CFG_TARGET_ENV").expect("Cargo did not provide CARGO_CFG_TARGET_ENV");

    let platform = match target_os.as_str() {
        "macos" => "macos",
        "windows" => "windows",
        "linux" => "linux",
        _ => panic!("Unsupported platform OS: {}", target_os),
    };

    let arch = if target.contains("aarch64") { "arm64" } else if target.contains("x86_64") { "x64" } else { panic!("Unsupported architecture in TARGET: {}", target) };

    let lib_name = match platform {
        "macos" => format!("{}-libtestoptimization-static.zip", platform),
        "linux" => {
            if target_env == "musl" {
                println!("cargo:warning=Detected Linux/musl target (Alpine). Using specific musl library.");
                format!("{}-{}-libtestoptimization-static-musl.zip", platform, arch)
            } else {
                format!("{}-{}-libtestoptimization-static.zip", platform, arch)
            }
        }
        "windows" | _ => format!("{}-{}-libtestoptimization-static.zip", platform, arch),
    };

    println!("cargo:warning=Target: {}, Platform: {}, Arch: {}, Env: {}, Lib Filename: {}", target, platform, arch, target_env, lib_name);

    // Check for custom native library search path
    if let Ok(search_path) = env::var(TEST_OPTIMIZATION_SDK_NATIVE_SEARCH_PATH) {
        link_from_search_path(platform, &lib_name, &search_path);
    } else {
        let lib_dir = Path::new(&out_dir);

        // Check if library files already exist
        let has_library = match platform {
            "windows" => lib_dir.join("testoptimization.lib").exists(),
            "linux" | "macos" => lib_dir.join("libtestoptimization.a").exists(),
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

    other_links(&target_os);
}

fn download_library(out_dir: &str, lib_name: &str, lib_dir: &Path) {
    // Get the folder
    let url = format!("{}{}", TEST_OPTIMIZATION_DOWNLOAD_URL_FORMAT, lib_name);
    let lib_zip_path = Path::new(out_dir).join("libtestoptimization.zip");

    // Download and extract library only if it doesn't exist
    println!("cargo:warning=Downloading native library from: {}", url);

    {
        let mut response = ureq::get(&url)
            .call()
            .unwrap_or_else(|e| {
                eprintln!("Failed to download native library: {}", e);
                process::exit(1);
            });

        if !response.status().is_success() {
            eprintln!("Error: Failed to download native library from {}. Server responded with status: {}", url, response.status());
            process::exit(1);
        }

        let mut reader = response.body_mut().as_body().into_reader();
        let mut file = BufWriter::new(File::create(&lib_zip_path).unwrap());
        io::copy(&mut reader, &mut file).unwrap_or_else(|e| {
            eprintln!("Failed to write native library to disk: {}", e);
            process::exit(1);
        });
        file.flush().unwrap_or_else(|e| {
            eprintln!("Error: Failed to flush file buffer for {:?}: {}", lib_zip_path, e);
            process::exit(1);
        });
    }

    extract_zip(&lib_zip_path, lib_dir).expect("Failed to decompress native library");
}

fn extract_zip(zip_path: &Path, target_dir: &Path) -> io::Result<()> {
    let file = File::open(zip_path)?;
    let mut archive = zip::ZipArchive::new(io::BufReader::new(file))?;
    
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = target_dir.join(file.mangled_name());

        if !outpath.starts_with(target_dir) {
            eprintln!("Security error: Zip entry '{}' tried to write outside the target folder (calculated path: {}). Aborting...", file.name(), outpath.display());
            return Err(io::Error::new(io::ErrorKind::PermissionDenied, "Zip Slip detected: Attempt to write outside target directory"));
        }
        
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
        "linux" | "macos" => search_path.join("libtestoptimization.a").exists(),
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

fn other_links(target_os: &str) {
    match target_os {
        "linux" | "macos" => println!("cargo:rustc-link-lib=dylib=resolv"),
        "windows" => {
            // Windows version requires cc as a build-dependency
            #[cfg(target_os = "windows")]
            configure_windows();
        }
        _ => {}
    }

    // If we are in osx, we need to add a couple of frameworks
    if target_os == "macos" {
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