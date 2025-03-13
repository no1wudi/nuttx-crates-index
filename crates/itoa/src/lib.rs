// Copyright (c) 2025 Xiaomi Corporation
// SPDX-License-Identifier: Apache-2.0

#[unsafe(no_mangle)]
pub fn rust_crate_test_itoa_main() {
    // Demonstrate itoa crate functionality
    println!("Itoa crate demonstration");

    // Using the recommended Buffer approach
    println!("\n--- Using itoa::Buffer ---");

    // Convert integers using Buffer (recommended approach)
    let mut buffer = itoa::Buffer::new();
    let printed = buffer.format(128u64);
    println!("Buffer format 128u64: {}", printed);
    assert_eq!(printed, "128");

    // More examples with different integer types
    let printed = buffer.format(42i32);
    println!("Buffer format 42i32: {}", printed);

    let printed = buffer.format(9876543210_u64);
    println!("Buffer format large integer: {}", printed);

    let printed = buffer.format(-12345_i32);
    println!("Buffer format negative integer: {}", printed);

    println!("Itoa demonstration completed");
}
