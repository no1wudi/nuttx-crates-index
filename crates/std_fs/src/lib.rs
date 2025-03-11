// Copyright (c) 2025 Xiaomi Corporation
// SPDX-License-Identifier: Apache-2.0

use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

#[unsafe(no_mangle)]
pub extern "C" fn rust_crate_test_std_fs_main() {
    println!("Rust std::fs Demo");

    // Ensure /tmp exists first
    let tmp_base = Path::new("/tmp");
    if !tmp_base.exists() {
        println!("/tmp directory does not exist! The filesystem may not be properly mounted.");
        panic!();
    }

    // Define the working directory in /tmp
    let tmp_dir = "/tmp/rust_fs_test";

    // Create a directory
    println!("Creating directory: {}", tmp_dir);
    fs::create_dir_all(tmp_dir).unwrap();
    println!("Directory created successfully");

    // Create and write to a file
    let file_path = format!("{}/test.txt", tmp_dir);
    println!("Creating file: {}", file_path);

    let mut file = File::create(&file_path).unwrap();
    file.write_all(b"Hello from Rust on NuttX!\n").unwrap();
    println!("Data written successfully");

    // Check file size after writing
    let metadata = fs::metadata(&file_path).unwrap();
    println!("File size after initial write: {} bytes", metadata.len());

    // Reading from the file
    println!("Reading file content:");
    let mut file = File::open(&file_path).unwrap();

    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    println!("File content: {}", content.trim());

    // Verify data matches what was written
    assert_eq!(content.trim(), "Hello from Rust on NuttX!");
    println!("Verified: Read data matches written data");

    // Append to the file
    println!("Appending to file");
    let mut file = OpenOptions::new().append(true).open(&file_path).unwrap();

    file.write_all(b"This is appended text.\n").unwrap();
    println!("Data appended successfully");

    // Check file size after append
    let metadata = fs::metadata(&file_path).unwrap();
    println!("File size after append: {} bytes", metadata.len());

    // Reading complete file content
    println!("Reading updated file content:");
    let content = fs::read_to_string(&file_path).unwrap();
    println!("Updated content:\n{}", content);

    // Verify complete content after append
    assert!(content.contains("Hello from Rust on NuttX!"));
    assert!(content.contains("This is appended text."));
    println!("Verified: Complete content contains both original and appended data");

    // List directory contents
    println!("Listing files in {}:", tmp_dir);
    for entry in fs::read_dir(tmp_dir).unwrap() {
        let entry = entry.unwrap();
        println!("- {}", entry.file_name().to_string_lossy());
    }

    // Get file metadata
    println!("File metadata:");
    let metadata = fs::metadata(&file_path).unwrap();
    println!("Size: {} bytes", metadata.len());
    println!("Read-only: {}", metadata.permissions().readonly());
    println!("Is file: {}", metadata.is_file());

    // Check if path exists
    let check_path = Path::new(&file_path);
    println!("{} exists: {}", file_path, check_path.exists());

    // Clean up - remove file and directory
    println!("Cleaning up - removing file");
    fs::remove_file(&file_path).unwrap();
    println!("File removed successfully");

    println!("Removing directory");
    fs::remove_dir(tmp_dir).unwrap();
    println!("Directory removed successfully");

    println!("std::fs demo completed");
}
