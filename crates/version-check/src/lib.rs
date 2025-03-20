// Copyright (c) 2025 Xiaomi Corporation
// SPDX-License-Identifier: Apache-2.0

use version_check::{Channel, Version};

#[unsafe(no_mangle)]
pub unsafe fn rust_crate_test_version_check_main() {
    println!("Starting Version Check Demo");

    // Get information about the current Rust version
    let rustc = Version::read();
    let channel = Channel::read();

    match rustc {
        Some(version) => println!("Rust version: {}", version),
        None => println!("Could not determine Rust version"),
    }

    match channel {
        Some(ch) => println!("Rust channel: {}", ch),
        None => println!("Could not determine Rust channel"),
    }

    // Demonstrate version comparison functionality
    let is_nightly = channel.map_or(false, |c| c.is_nightly());
    println!("Is nightly channel: {}", is_nightly);

    // Check if running rustc version is greater than 1.50.0
    let version_1_50 = Version::parse("1.50.0").unwrap_or(Version::parse("0.0.0").unwrap());
    let check_ver = rustc.map_or(false, |v| v > version_1_50);
    println!("Is rustc version > 1.50.0: {}", check_ver);

    println!("Version Check Demo completed");
}
