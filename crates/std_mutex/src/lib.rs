// Copyright (c) 2025 Xiaomi Corporation
// SPDX-License-Identifier: Apache-2.0

use std::sync::{Arc, Mutex};
use std::thread::{self, Builder};
use std::time::Duration;

// Define a struct that will be protected by the mutex
struct Counter {
    value: u32,
}

/// Entry point for the std_mutex demo
#[unsafe(no_mangle)]
pub extern "C" fn rust_crate_test_std_mutex_main() {
    println!("Starting Rust std::sync::Mutex demo");

    // Run the single-threaded demo
    single_thread_demo();

    // Run the multi-threaded demo
    multi_thread_demo();

    println!("Rust std::sync::Mutex demo completed");
}

/// Demonstrates mutex usage in a single thread
fn single_thread_demo() {
    println!("\n=== Single-threaded mutex demo ===");

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
    println!("Single-thread demo final counter value: {}", final_value);

    // Verify the counter has been incremented exactly 3 times
    assert_eq!(
        final_value, 3,
        "Single-thread demo failed: expected counter value to be 3"
    );
    println!("Single-thread test passed: counter value is 3 as expected");
}

/// Demonstrates mutex usage across multiple threads
fn multi_thread_demo() {
    println!("\n=== Multi-threaded mutex demo ===");

    // Create a mutex-protected counter shared among threads
    let counter = Arc::new(Mutex::new(Counter { value: 0 }));
    let mut handles = vec![];

    // Create multiple threads to increment the counter
    for id in 0..3 {
        let counter_clone = Arc::clone(&counter);

        // Create thread with 4KB stack size
        let handle = Builder::new()
            .name(format!("thread-{}", id))
            .stack_size(4 * 1024) // 4KB stack size
            .spawn(move || {
                for _ in 0..3 {
                    // Try to acquire the lock
                    let mut counter_guard = counter_clone.lock().unwrap();

                    println!(
                        "Thread-{} acquired lock, counter value: {}",
                        id, counter_guard.value
                    );

                    // Increment the counter
                    counter_guard.value += 1;

                    // Simulate some work
                    thread::sleep(Duration::from_millis(50));

                    println!(
                        "Thread-{} releasing lock, new counter value: {}",
                        id, counter_guard.value
                    );

                    // Lock is automatically released when counter_guard goes out of scope

                    // Add a small delay between attempts
                    thread::sleep(Duration::from_millis(10));
                }
            })
            .expect("Failed to spawn thread");

        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // Show final value
    let final_value = counter.lock().unwrap().value;
    println!("Multi-thread demo final counter value: {}", final_value);

    // Verify the counter has been incremented exactly 9 times (3 threads x 3 increments each)
    assert_eq!(
        final_value, 9,
        "Multi-thread demo failed: expected counter value to be 9"
    );
    println!("Multi-thread test passed: counter value is 9 as expected");
}
