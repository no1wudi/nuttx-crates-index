// Copyright (c) 2025 Xiaomi Corporation
// SPDX-License-Identifier: Apache-2.0

use std::thread;
use std::time::Duration;

// Thread example with default stack size
fn default_stack_size_example() {
    println!("Running default stack size example...");

    // Spawn a thread using the default settings
    let handle = thread::spawn(|| {
        println!("Hello from a thread with default stack size!");

        // Sleep for a moment to demonstrate thread execution
        thread::sleep(Duration::from_millis(200));

        println!("Default stack size thread is finishing");
    });

    println!("Main thread continues while default stack thread runs");

    // Wait for the thread to complete
    handle.join().unwrap();

    println!("Default stack size example complete");
}

// Thread example with custom stack size
fn custom_stack_size_example() {
    println!("Running custom stack size example...");

    // Create a builder with 4KB stack size
    let builder = thread::Builder::new().stack_size(4 * 1024); // 4KB stack size

    // Spawn a thread using the builder
    let handle = builder
        .spawn(|| {
            println!("Hello from a thread with 4KB stack!");

            // Sleep for a moment to demonstrate thread execution
            thread::sleep(Duration::from_millis(300));

            println!("Custom stack size thread is finishing");
        })
        .unwrap();

    println!("Main thread continues while custom stack thread runs");

    // Wait for the thread to complete
    handle.join().unwrap();

    println!("Custom stack size example complete");
}

#[unsafe(no_mangle)]
pub fn rust_crate_test_std_thread_main() {
    println!("Starting std_thread demonstration");

    default_stack_size_example();
    custom_stack_size_example();

    println!("std_thread demonstration complete!");
}
