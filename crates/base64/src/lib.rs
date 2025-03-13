// Copyright (c) 2025 Xiaomi Corporation
// SPDX-License-Identifier: Apache-2.0

use base64::{Engine as _, prelude::BASE64_STANDARD};
use std::io::{self, Write};

#[unsafe(no_mangle)]
pub fn rust_crate_test_base64_main() {
    println!("Base64 encoding/decoding test");

    // Test encoding
    let original_data = b"Hello, NuttX Rust world!";
    let encoded = BASE64_STANDARD.encode(original_data);
    println!("Original: {}", std::str::from_utf8(original_data).unwrap());
    println!("Encoded : {}", encoded);

    // Test decoding
    match BASE64_STANDARD.decode(&encoded) {
        Ok(decoded) => {
            println!("Decoded: {}", std::str::from_utf8(&decoded).unwrap());

            // Verify the roundtrip
            if &decoded == original_data {
                println!("✓ Roundtrip successful!");
            } else {
                println!("✗ Roundtrip failed!");
            }
        }
        Err(err) => {
            println!("Decoding error: {}", err);
        }
    }

    // Demonstrate base64url variant
    let data_with_special_chars = b"Hello+/NuttX";
    let standard = BASE64_STANDARD.encode(data_with_special_chars);
    let urlsafe = base64::prelude::BASE64_URL_SAFE.encode(data_with_special_chars);

    println!("\nStandard vs URL-safe encoding:");
    println!(
        "Original    : {}",
        std::str::from_utf8(data_with_special_chars).unwrap()
    );
    println!("Standard    : {}", standard);
    println!("URL-safe    : {}", urlsafe);

    io::stdout().flush().unwrap();
}
