// Copyright (c) 2025 Xiaomi Corporation
// SPDX-License-Identifier: Apache-2.0

use bytes::{BufMut, BytesMut};

/// Entry point for the bytes crate example
///
/// This function demonstrates the usage of the BytesMut and BufMut traits
/// from the bytes crate, showing buffer manipulation operations including
/// adding data, splitting buffers, and checking capacity.
#[unsafe(no_mangle)]
pub fn rust_crate_test_bytes_main() {
    println!("Starting bytes crate example...");

    // Create a new BytesMut with capacity
    let mut buf = BytesMut::with_capacity(1024);

    // Put bytes into the buffer
    buf.put(&b"hello world"[..]);
    buf.put_u16(1234);

    // Split the buffer and verify contents
    let a = buf.split();

    // Check if buffer contents match expected value
    let expected = &b"hello world\x04\xD2"[..];
    if a == expected {
        println!("First split successful");
    } else {
        println!("First split failed: lengths don't match");
    }

    // Add more data to the buffer
    buf.put(&b"goodbye world"[..]);

    // Split again and verify
    let b = buf.split();

    // Check if second buffer contents match expected value
    let expected = &b"goodbye world"[..];
    if b == expected {
        println!("Second split successful");
    } else {
        println!("Second split failed: lengths don't match");
    }

    // Check remaining capacity
    println!("Remaining capacity: {}", buf.capacity());
    assert_eq!(buf.capacity(), 998);

    println!("Bytes crate example completed successfully!");
}
