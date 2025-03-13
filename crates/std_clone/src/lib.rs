// Copyright (c) 2025 Xiaomi Corporation
// SPDX-License-Identifier: Apache-2.0

use std::clone::Clone;
use std::println;

// Define a custom struct to demonstrate Clone trait implementation
#[derive(Debug)]
struct Device {
    id: u32,
    name: String,
}

// Implement Clone for our custom struct
impl Clone for Device {
    fn clone(&self) -> Self {
        println!("Custom cloning logic for Device: {}", self.id);
        Device {
            id: self.id,
            name: self.name.clone(),
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_crate_test_std_clone_main() {
    println!("=== Demonstrating std::clone functionality ===");

    // Demonstrate cloning primitive types
    let number = 42;
    let number_clone = number.clone();
    println!(
        "Original number: {}, Cloned number: {}",
        number, number_clone
    );

    // Demonstrate cloning standard collections
    let vector = vec![1, 2, 3, 4, 5];
    let vector_clone = vector.clone();
    println!(
        "Original vector: {:?}, Cloned vector: {:?}",
        vector, vector_clone
    );

    // Demonstrate clone_from method
    let mut string1 = String::from("Hello");
    let string2 = String::from("World");
    println!(
        "Before clone_from - string1: {}, string2: {}",
        string1, string2
    );
    string1.clone_from(&string2);
    println!(
        "After clone_from - string1: {}, string2: {}",
        string1, string2
    );

    // Demonstrate custom clone implementation
    let device = Device {
        id: 1,
        name: String::from("Sensor"),
    };
    let device_clone = device.clone();

    println!("Original device: {:?}", device);
    println!("Cloned device: {:?}", device_clone);

    println!("=== std::clone demonstration completed ===");
}
