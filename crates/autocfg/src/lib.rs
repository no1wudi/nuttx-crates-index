// Copyright (c) 2025 Xiaomi Corporation
// SPDX-License-Identifier: Apache-2.0

use std::println;

// Function to demonstrate autocfg functionality
fn demonstrate_autocfg() {
    // Create a new AutoCfg instance
    let ac = autocfg::AutoCfg::new().unwrap_or_else(|e| {
        println!("Failed to create AutoCfg: {}", e);
        std::process::exit(1);
    });

    println!("Using autocfg for build-time feature detection");

    // Check for various Rust features
    println!("Rust version information:");

    if ac.probe_rustc_version(1, 60) {
        println!("- Rustc version is at least 1.60");
    } else {
        println!("- Rustc version is less than 1.60");
    }

    // Check for specific language features
    println!("\nFeature detection:");

    if ac.probe_type("i128") {
        println!("- i128 type is available");
    } else {
        println!("- i128 type is not available");
    }

    println!("\nautocfg demonstration completed successfully");
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_crate_test_autocfg_main() {
    println!("Starting autocfg demonstration...");
    demonstrate_autocfg();
}
