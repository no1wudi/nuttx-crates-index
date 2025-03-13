// Copyright (c) 2025 Xiaomi Corporation
// SPDX-License-Identifier: Apache-2.0

use sha2::{Digest, Sha256};

#[unsafe(no_mangle)]
pub unsafe fn rust_crate_test_sha2_main() {
    // Create a new SHA-256 hasher
    let mut hasher = Sha256::new();

    // Update with binary data
    let data = b"Hello world!";
    hasher.update(data);

    // Update with string data
    // `update` can be called repeatedly and is generic over `AsRef<[u8]>`
    hasher.update("String data");

    // Note that calling `finalize()` consumes hasher
    let hash = hasher.finalize();

    // Print the binary hash
    println!("SHA-256 hash: {:x}", hash);

    // Print each byte of the hash individually
    print!("Hash bytes: ");
    for byte in hash {
        print!("{:02x}", byte);
    }
    println!();
}
