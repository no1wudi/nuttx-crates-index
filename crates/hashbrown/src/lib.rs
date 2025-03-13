// Copyright (c) 2025 Xiaomi Corporation
// SPDX-License-Identifier: Apache-2.0

use hashbrown::HashMap;

#[unsafe(no_mangle)]
pub extern "C" fn rust_crate_test_hashbrown_main() {
    // Create a new HashMap with hashbrown
    let mut map = HashMap::new();

    // Insert some key-value pairs
    map.insert("one", 1);
    map.insert("two", 2);
    map.insert("three", 3);

    // Print the map size
    println!("HashMap size: {}", map.len());

    // Check if a key exists
    if let Some(&value) = map.get("two") {
        println!("Found value for 'two': {}", value);
    }

    // Remove a key
    map.remove("three");
    println!("After removal, size: {}", map.len());

    // Iterate over key-value pairs
    println!("Contents of the HashMap:");
    for (key, value) in &map {
        println!("  {} => {}", key, value);
    }

    // Demonstrate hashbrown's entry API
    *map.entry("four").or_insert(4) += 10;
    println!("Value for 'four': {}", map.get("four").unwrap());

    println!("Hashbrown example completed successfully!");
}
