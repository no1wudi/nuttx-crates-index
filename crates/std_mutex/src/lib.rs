// Copyright (c) 2025 Xiaomi Corporation
// SPDX-License-Identifier: Apache-2.0

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// Define a struct that will be protected by the mutex
struct Counter {
    value: u32,
}

/// Entry point for the std_mutex demo
#[unsafe(no_mangle)]
pub extern "C" fn rust_crate_test_std_mutex_main() {
    println!("Starting Rust std::sync::Mutex demo");

    // Create a mutex-protected counter
    let counter = Arc::new(Mutex::new(Counter { value: 0 }));

    // First mutex operation
    {
        // Lock the mutex to access the counter
        let mut counter_guard = counter.lock().unwrap();
        println!(
            "Main thread acquired lock, counter value: {}",
            counter_guard.value
        );

        // Increment the counter
        counter_guard.value += 1;

        // Simulate some work
        thread::sleep(Duration::from_millis(100));

        println!(
            "Main thread releasing lock, new counter value: {}",
            counter_guard.value
        );
        // Lock is automatically released when counter_guard goes out of scope
    }

    // Second mutex operation
    {
        // Lock the mutex again
        let mut counter_guard = counter.lock().unwrap();
        println!(
            "Main thread acquired lock again, counter value: {}",
            counter_guard.value
        );

        // Increment the counter
        counter_guard.value += 1;

        // Simulate some work
        thread::sleep(Duration::from_millis(100));

        println!(
            "Main thread releasing lock, new counter value: {}",
            counter_guard.value
        );
        // Lock is automatically released when counter_guard goes out of scope
    }

    // Third mutex operation
    {
        // Lock the mutex one more time
        let mut counter_guard = counter.lock().unwrap();
        println!(
            "Main thread acquired lock once more, counter value: {}",
            counter_guard.value
        );

        // Increment the counter
        counter_guard.value += 1;

        // Simulate some work
        thread::sleep(Duration::from_millis(100));

        println!(
            "Main thread releasing lock, new counter value: {}",
            counter_guard.value
        );
        // Lock is automatically released when counter_guard goes out of scope
    }

    // Show final value
    let final_value = counter.lock().unwrap().value;
    println!("Final counter value: {}", final_value);
    println!("Rust std::sync::Mutex demo completed");
}
