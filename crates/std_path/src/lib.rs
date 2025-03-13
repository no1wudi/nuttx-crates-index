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
    assert_eq!(path.file_name(), Some(OsStr::new("file.txt")));

    println!("File stem: {:?}", path.file_stem());
    assert_eq!(path.file_stem(), Some(OsStr::new("file")));

    println!("Extension: {:?}", path.extension());
    assert_eq!(path.extension(), Some(OsStr::new("txt")));

    println!("Parent directory: {:?}", path.parent());
    assert_eq!(path.parent(), Some(Path::new("/usr/local/bin")));

    println!("Is absolute: {}", path.is_absolute());
    assert!(path.is_absolute());

    // Path manipulation with PathBuf
    let mut path_buf = PathBuf::from("/home/user");
    println!("\nInitial PathBuf: {}", path_buf.display());
    assert_eq!(path_buf, PathBuf::from("/home/user"));

    // Append to path
    path_buf.push("documents");
    path_buf.push("work");
    println!("After push operations: {}", path_buf.display());
    assert_eq!(path_buf, PathBuf::from("/home/user/documents/work"));

    // Pop last component
    path_buf.pop();
    println!("After pop: {}", path_buf.display());
    assert_eq!(path_buf, PathBuf::from("/home/user/documents"));

    // Set file name
    path_buf.set_file_name("notes.txt");
    println!("After setting filename: {}", path_buf.display());
    assert_eq!(path_buf, PathBuf::from("/home/user/notes.txt"));

    // Set extension
    path_buf.set_extension("md");
    println!("After changing extension: {}", path_buf.display());
    assert_eq!(path_buf, PathBuf::from("/home/user/notes.md"));

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
    assert_eq!(joined, PathBuf::from("/usr/local/bin"));

    // Converting between Path and string types
    if let Some(path_str) = path.to_str() {
        println!("\nPath as string: {}", path_str);
        assert_eq!(path_str, "/usr/local/bin/file.txt");
    }

    // Working with OsStr
    let filename = OsStr::new("config.json");
    let config_path = Path::new("/etc").join(filename);
    println!("Config path: {}", config_path.display());
    assert_eq!(config_path, PathBuf::from("/etc/config.json"));

    println!("\n=== std::path demo completed ===");
}
