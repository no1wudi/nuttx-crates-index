// Copyright (c) 2025 Xiaomi Corporation
// SPDX-License-Identifier: Apache-2.0

use smallvec::{Array, SmallVec};

// Helper function to create and fill the SmallVec up to its inline capacity
fn create_and_fill_inline<A: Array<Item = i32>>() -> SmallVec<A> {
    let mut vec = SmallVec::<A>::new();
    println!("Adding elements to SmallVec (inline)...");
    vec.push(1);
    vec.push(2);
    vec.push(3);
    vec.push(4);
    vec
}

// Helper function to print the status of the SmallVec
fn print_vec_status<A: Array<Item = i32>>(vec: &SmallVec<A>, description: &str) {
    println!("SmallVec elements ({}):", description);
    for (i, &item) in vec.iter().enumerate() {
        println!("  [{}]: {}", i, item);
    }
    println!("Is using heap: {}", vec.spilled());
}

// Helper function to add elements beyond the inline capacity
fn fill_beyond_capacity<A: Array<Item = i32>>(vec: &mut SmallVec<A>) {
    println!("Adding more elements beyond capacity...");
    vec.push(5);
    vec.push(6);
    vec.push(7);
}

// Helper function to print properties of the SmallVec
fn print_vec_properties<A: Array<Item = i32>>(vec: &SmallVec<A>) {
    println!("SmallVec length: {}", vec.len());
    println!("SmallVec capacity: {}", vec.capacity());
    println!("Is using heap: {}", vec.spilled());
}

#[unsafe(no_mangle)]
pub fn rust_crate_test_smallvec_main() {
    println!("Starting smallvec example...");

    // Create and fill the SmallVec using inline storage
    let mut vec = create_and_fill_inline::<[i32; 4]>();

    // Print status while using inline storage
    print_vec_status(&vec, "using inline storage");

    // Add more elements to trigger spilling
    fill_beyond_capacity(&mut vec);

    // Print status after spilling to heap
    print_vec_status(&vec, "spilled to heap");

    // Print vector properties
    print_vec_properties(&vec);

    // Clear the vector and show it goes back to inline storage
    vec.clear();
    println!("After clear:");
    println!("SmallVec length: {}", vec.len());
    println!("Is using heap: {}", vec.spilled());

    println!("Smallvec example completed successfully!");
}
