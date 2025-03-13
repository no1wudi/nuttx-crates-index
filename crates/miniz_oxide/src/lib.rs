// Copyright (c) 2025 Xiaomi Corporation
// SPDX-License-Identifier: Apache-2.0

use miniz_oxide::deflate::compress_to_vec;
use miniz_oxide::inflate::decompress_to_vec_with_limit;

fn roundtrip(data: &[u8]) -> bool {
    println!("Original data length: {}", data.len());

    // Compress the input
    let compressed = compress_to_vec(data, 6);
    println!("Compressed data length: {}", compressed.len());

    // Decompress the compressed input with a reasonable limit
    match decompress_to_vec_with_limit(compressed.as_slice(), 60000) {
        Ok(decompressed) => {
            println!("Decompressed data length: {}", decompressed.len());

            // Check if roundtrip succeeded
            let success = data == decompressed.as_slice();
            println!("Roundtrip success: {}", success);
            success
        }
        Err(e) => {
            println!("Failed to decompress: {:?}", e);
            false
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_crate_test_miniz_oxide_main() {
    println!("Testing miniz_oxide compression/decompression...");

    // Test with a simple string
    let test_str = "Hello, world! This is a test of the miniz_oxide compression library.";
    println!("Test string: {}", test_str);

    let result = roundtrip(test_str.as_bytes());
    if result {
        println!("Simple string roundtrip test: SUCCESS");
    } else {
        println!("Simple string roundtrip test: FAILED");
    }

    // Test with a larger repeated pattern to show better compression
    let large_str = "abc".repeat(1000);
    println!("\nTesting with repeated pattern (3000 bytes)...");

    let result = roundtrip(large_str.as_bytes());
    if result {
        println!("Large string roundtrip test: SUCCESS");
    } else {
        println!("Large string roundtrip test: FAILED");
    }

    println!("miniz_oxide test complete!");
}
