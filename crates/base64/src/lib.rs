// Copyright (c) 2025 Xiaomi Corporation
// SPDX-License-Identifier: Apache-2.0

use base64::{Engine as _, prelude::BASE64_STANDARD};

/// Test basic encoding and decoding functionality
fn test_basic_encoding_decoding() {
    println!("Testing basic encoding and decoding...");

    let original_data = b"Hello, NuttX Rust world!";
    let encoded = BASE64_STANDARD.encode(original_data);

    println!("Original: {}", std::str::from_utf8(original_data).unwrap());
    println!("Encoded : {}", encoded);

    // Assert the encoded result is correct
    let expected_encoded = "SGVsbG8sIE51dHRYIFJ1c3Qgd29ybGQh";
    assert_eq!(
        encoded, expected_encoded,
        "Encoding result doesn't match expected output"
    );

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

            // Assert the decoded result matches original data
            assert_eq!(
                &decoded, original_data,
                "Decoded data doesn't match original data"
            );
        }
        Err(err) => {
            println!("Decoding error: {}", err);
            panic!("Decoding failed: {}", err);
        }
    }
}

/// Test URL-safe encoding variant
fn test_urlsafe_encoding() {
    println!("\nTesting standard vs URL-safe encoding...");

    let data_with_special_chars = b"Hello+/NuttX";
    let standard = BASE64_STANDARD.encode(data_with_special_chars);
    let urlsafe = base64::prelude::BASE64_URL_SAFE.encode(data_with_special_chars);

    println!("Standard vs URL-safe encoding:");
    println!(
        "Original    : {}",
        std::str::from_utf8(data_with_special_chars).unwrap()
    );
    println!("Standard    : {}", standard);
    println!("URL-safe    : {}", urlsafe);

    // Assert that standard and URL-safe encodings produce expected results
    let expected_standard = "SGVsbG8rL051dHRY";
    let expected_urlsafe = "SGVsbG8rL051dHRY";
    assert_eq!(
        standard, expected_standard,
        "Standard encoding doesn't match expected output"
    );
    assert_eq!(
        urlsafe, expected_urlsafe,
        "URL-safe encoding doesn't match expected output for this input"
    );

    // Verify decoding of both variants
    let decoded_standard = BASE64_STANDARD
        .decode(&standard)
        .expect("Failed to decode standard encoding");
    let decoded_urlsafe = base64::prelude::BASE64_URL_SAFE
        .decode(&urlsafe)
        .expect("Failed to decode URL-safe encoding");

    assert_eq!(
        &decoded_standard, data_with_special_chars,
        "Standard decoded data doesn't match original"
    );
    assert_eq!(
        &decoded_urlsafe, data_with_special_chars,
        "URL-safe decoded data doesn't match original"
    );
}

#[unsafe(no_mangle)]
pub fn rust_crate_test_base64_main() {
    println!("Base64 encoding/decoding test");

    // Run the individual test functions
    test_basic_encoding_decoding();
    test_urlsafe_encoding();

    println!("\nAll base64 tests completed successfully!");
}
