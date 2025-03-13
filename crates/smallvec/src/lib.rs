// Copyright (c) 2025 Xiaomi Corporation
// SPDX-License-Identifier: Apache-2.0

use smallvec::SmallVec;

#[unsafe(no_mangle)]
pub fn rust_crate_test_smallvec_main() {
    // Demonstrate smallvec functionality
    println!("Starting smallvec example...");

    // Create a SmallVec with inline storage for 4 elements
    let mut vec = SmallVec::<[i32; 4]>::new();

    // Add elements to the smallvec
    println!("Adding elements to SmallVec...");
    vec.push(1);
    vec.push(2);
    vec.push(3);
    vec.push(4);

    // Print the vector elements
    println!("SmallVec elements (using inline storage):");
    for (i, &item) in vec.iter().enumerate() {
        println!("  [{}]: {}", i, item);
    }
    println!("Is using heap: {}", vec.spilled());

    // Add more elements beyond the inline capacity
    println!("Adding more elements beyond capacity...");
    vec.push(5);
    vec.push(6);
    vec.push(7);

    // Print the vector elements after spilling to heap
    println!("SmallVec elements (spilled to heap):");
    for (i, &item) in vec.iter().enumerate() {
        println!("  [{}]: {}", i, item);
    }

    // Test smallvec features
    println!("SmallVec length: {}", vec.len());
    println!("SmallVec capacity: {}", vec.capacity());
    println!("Is using heap: {}", vec.spilled());

    // Clear the vector and show it goes back to inline storage
    vec.clear();
    println!("After clear:");
    println!("SmallVec length: {}", vec.len());
    println!("Is using heap: {}", vec.spilled());

    println!("Smallvec example completed successfully!");
}
