// Copyright (c) 2025 Xiaomi Corporation
// SPDX-License-Identifier: Apache-2.0

use itertools::Itertools;
use std::println;

#[unsafe(no_mangle)]
pub fn rust_crate_test_itertools_main() {
    println!("Starting itertools example application");

    // Demonstrate some itertools functionality

    // Example 1: Using .sorted()
    let unsorted = vec![3, 1, 5, 2, 4];
    let sorted: Vec<_> = unsorted.iter().sorted().copied().collect();
    println!("Sorted: {:?}", sorted);

    // Example 2: Using .join()
    let words = ["Hello", "from", "itertools", "on", "NuttX"];
    let joined = words.iter().join(" ");
    println!("Joined: {}", joined);

    // Example 3: Using .tuple_windows()
    let numbers = vec![1, 2, 3, 4, 5];
    for (a, b) in numbers.iter().tuple_windows() {
        println!("Pair: ({}, {})", a, b);
    }

    // Example 4: Using .cartesian_product()
    let xs = 1..3;
    let ys = 'a'..='b';
    for (x, y) in xs.cartesian_product(ys) {
        println!("Product: ({}, {})", x, y);
    }

    println!("Itertools example completed");
}
