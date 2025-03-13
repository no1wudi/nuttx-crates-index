// Copyright (c) 2025 Xiaomi Corporation
// SPDX-License-Identifier: Apache-2.0

use std::ffi::OsStr;
use std::path::{Path, PathBuf};

/// This demo demonstrates various functionality from the std::path module
#[unsafe(no_mangle)]
pub fn rust_crate_test_std_path_main() {
    println!("\n=== std::path demonstration ===\n");

    // Create a Path from a string slice
    let path = Path::new("/usr/local/bin/file.txt");

    // Path components and properties
    println!("Path: {}", path.display());
    println!("File name: {:?}", path.file_name());
    println!("File stem: {:?}", path.file_stem());
    println!("Extension: {:?}", path.extension());
    println!("Parent directory: {:?}", path.parent());
    println!("Is absolute: {}", path.is_absolute());

    // Path manipulation with PathBuf
    let mut path_buf = PathBuf::from("/home/user");
    println!("\nInitial PathBuf: {}", path_buf.display());

    // Append to path
    path_buf.push("documents");
    path_buf.push("work");
    println!("After push operations: {}", path_buf.display());

    // Pop last component
    path_buf.pop();
    println!("After pop: {}", path_buf.display());

    // Set file name
    path_buf.set_file_name("notes.txt");
    println!("After setting filename: {}", path_buf.display());

    // Set extension
    path_buf.set_extension("md");
    println!("After changing extension: {}", path_buf.display());

    // Path normalization
    let complex_path = PathBuf::from("/home/./user/../tmp/test/../file.txt");

    // Note: Path doesn't have built-in normalization,
    // but we can demonstrate component iteration
    println!("\nComponents of complex path:");
    for component in complex_path.components() {
        println!("  {:?}", component);
    }

    // Path joining and concatenation
    let base = Path::new("/usr");
    let joined = base.join("local").join("bin");
    println!("\nJoined path: {}", joined.display());

    // Converting between Path and string types
    if let Some(path_str) = path.to_str() {
        println!("\nPath as string: {}", path_str);
    }

    // Working with OsStr
    let filename = OsStr::new("config.json");
    let config_path = Path::new("/etc").join(filename);
    println!("Config path: {}", config_path.display());

    println!("\n=== std::path demo completed ===");
}
