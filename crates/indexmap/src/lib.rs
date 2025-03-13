// Copyright (c) 2025 Xiaomi Corporation
// SPDX-License-Identifier: Apache-2.0

use indexmap::IndexMap;
use std::println;

#[unsafe(no_mangle)]
pub fn rust_crate_test_indexmap_main() {
    println!("IndexMap demo starting...");

    // Create a new empty IndexMap
    let mut map: IndexMap<String, i32> = IndexMap::new();

    // Insert key-value pairs (insertion order is preserved)
    map.insert("one".to_string(), 1);
    map.insert("two".to_string(), 2);
    map.insert("three".to_string(), 3);

    println!("IndexMap contents:");
    for (key, val) in &map {
        println!("  {} = {}", key, val);
    }

    // Access and modify values
    if let Some(val) = map.get_mut("two") {
        *val *= 10;
    }

    // IndexMap preserves insertion order
    println!("After modification:");
    for (key, val) in &map {
        println!("  {} = {}", key, val);
    }

    // Remove an entry
    // Use shift_remove instead of remove to explicitly preserve the order of remaining elements
    map.shift_remove("one");

    println!("After removal:");
    for (key, val) in &map {
        println!("  {} = {}", key, val);
    }

    println!("IndexMap demo completed!");
}
