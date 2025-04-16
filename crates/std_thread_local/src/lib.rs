// Copyright (c) 2025 Xiaomi Corporation
// SPDX-License-Identifier: Apache-2.0

//! Demonstrates usage of `std::thread_local!` for thread-local storage in Rust.

use std::cell::RefCell;
use std::thread;
use std::thread_local;

// Create a thread-local variable at the module level
thread_local! {
    static THREAD_LOCAL: RefCell<i32> = RefCell::new(6);
}

#[unsafe(no_mangle)]
pub fn rust_crate_test_std_thread_local_main() {
    // Check value in main thread before spawn
    THREAD_LOCAL.with(|value| {
        println!("Main thread initial value: {}", *value.borrow());
    });

    // Spawn a new thread
    let handle = thread::spawn(|| {
        // Set the thread-local variable to a new value
        THREAD_LOCAL.with(|value| {
            *value.borrow_mut() = 42;
            println!("New thread set value to: {}", *value.borrow());
        });
        // Check value again in the new thread
        THREAD_LOCAL.with(|value| {
            println!("New thread current value: {}", *value.borrow());
        });
    });

    // Wait for the thread to finish
    handle.join().unwrap();

    // Check value in main thread after join
    THREAD_LOCAL.with(|value| {
        println!("Main thread final value: {}", *value.borrow());
    });
}
