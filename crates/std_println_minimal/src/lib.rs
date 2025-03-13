// Copyright (c) 2025 Xiaomi Corporation
// SPDX-License-Identifier: Apache-2.0

#[unsafe(no_mangle)]
pub fn rust_crate_test_std_println_minimal_main() {
    // This is a minimal test to measure the memory footprint
    // of using Rust's standard library println! macro
    println!("Minimal println from Rust!");
}
