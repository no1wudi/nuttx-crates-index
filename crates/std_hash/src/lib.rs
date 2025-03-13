// Copyright (c) 2025 Xiaomi Corporation
// SPDX-License-Identifier: Apache-2.0

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

// Calculate the hash value of the given value
fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

// Demonstrate different hash calculations
fn demo_hash_functions() {
    println!("Hash demo using std::hash");

    // Hash simple types
    let int_val = 42;
    println!("Hash of integer {}: {}", int_val, calculate_hash(&int_val));

    let float_val = 3.14;
    // Float doesn't implement Hash, so we'll hash its string representation
    let float_str = float_val.to_string();
    println!(
        "Hash of float {}: {}",
        float_val,
        calculate_hash(&float_str)
    );

    let string_val = "Hello, NuttX!";
    println!(
        "Hash of string '{}': {}",
        string_val,
        calculate_hash(&string_val)
    );

    // Hash compound type (tuple) - using string for float
    let tuple_val = (1, "two", "3.0");
    println!("Hash of tuple: {}", calculate_hash(&tuple_val));

    // Hash collections
    let vec_val = vec![1, 2, 3, 4, 5];
    println!("Hash of vector: {}", calculate_hash(&vec_val));

    // Custom struct with Hash derive
    #[derive(Hash)]
    struct Person {
        name: String,
        age: u8,
    }

    let person = Person {
        name: String::from("John Doe"),
        age: 30,
    };
    println!("Hash of Person struct: {}", calculate_hash(&person));
}

#[unsafe(no_mangle)]
pub fn rust_crate_test_std_hash_main() {
    demo_hash_functions();
}
