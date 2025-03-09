// Copyright (c) 2025 Xiaomi Corporation
// SPDX-License-Identifier: Apache-2.0

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// Simple thread creation example
fn basic_thread_example() {
    println!("Running basic thread example...");

    // Spawn a new thread
    let handle = thread::spawn(|| {
        println!("Hello from a spawned thread!");

        // Sleep for a moment to demonstrate thread execution
        thread::sleep(Duration::from_millis(500));

        println!("Spawned thread is finishing");
    });

    println!("Main thread continues execution while spawned thread runs");

    // Wait for the spawned thread to finish
    handle.join().unwrap();

    println!("Basic thread example complete");
}

// Thread example with shared data
fn shared_data_example() {
    println!("Running shared data example...");

    // Create shared data using Arc (Atomic Reference Counting) and Mutex
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    // Spawn 5 threads that will increment the counter
    for i in 0..5 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // Lock the mutex to modify the data
            let mut num = counter_clone.lock().unwrap();
            *num += 1;

            let msg = format!("Thread {} incremented counter to {}", i, *num);
            println!("{}", msg);

            // Sleep for a random duration
            thread::sleep(Duration::from_millis(200));
        });
        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // Access the final value
    let final_count = *counter.lock().unwrap();
    println!("Final counter value: {}", final_count);

    println!("Shared data example complete");
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_crates_test_std_thread_main() {
    println!("Starting std_thread demonstration");

    // Run examples
    basic_thread_example();
    println!(); // Separator
    shared_data_example();

    println!("std_thread demonstration complete!");
}
