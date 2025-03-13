// Copyright (c) 2025 Xiaomi Corporation
// SPDX-License-Identifier: Apache-2.0

use std::sync::Once;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{OnceLock, OnceState};
use std::thread;

// This function will be executed only once across all threads
static INIT: Once = Once::new();
static VALUE: AtomicU32 = AtomicU32::new(0);

fn initialize_value() {
    // This code will only run once, even if multiple threads try to call it
    VALUE.store(42, Ordering::SeqCst);
    println!("Value initialized to: {}", VALUE.load(Ordering::SeqCst));
}

fn create_thread_with_stack_size() -> thread::JoinHandle<()> {
    // Create a thread with 4K stack size
    thread::Builder::new()
        .stack_size(4096)
        .spawn(|| {
            println!("Thread started, calling initialize once...");
            INIT.call_once(|| initialize_value());
            println!("Thread read value: {}", VALUE.load(Ordering::SeqCst));
        })
        .expect("Failed to spawn thread")
}

// Demonstrate OnceLock which is a thread-safe cell that can be written to only once
fn demonstrate_once_lock() {
    println!("\nDemonstrating std::sync::OnceLock:");

    static LOCK: OnceLock<String> = OnceLock::new();

    let handle1 = thread::Builder::new()
        .stack_size(4096)
        .spawn(|| {
            // First thread tries to initialize the value
            let value = LOCK.get_or_init(|| {
                println!("Thread 1: Initializing OnceLock value");
                "Hello from OnceLock".to_string()
            });
            println!("Thread 1: Retrieved value: {}", value);
        })
        .expect("Failed to spawn thread");

    let handle2 = thread::Builder::new()
        .stack_size(4096)
        .spawn(|| {
            // Second thread also tries to initialize, but will get the already set value
            let value = LOCK.get_or_init(|| {
                println!("Thread 2: Initializing OnceLock value (should not happen)");
                "Different value".to_string()
            });
            println!("Thread 2: Retrieved value: {}", value);
        })
        .expect("Failed to spawn thread");

    // Wait for threads to complete
    let _ = handle1.join();
    let _ = handle2.join();

    // Check the final value
    if let Some(value) = LOCK.get() {
        println!("OnceLock final value: {}", value);
    }
}

// Demonstrate OnceState which provides more control over initialization
fn demonstrate_once_state() {
    println!("\nDemonstrating std::sync::OnceState:");

    static INIT_STATE: Once = Once::new();
    static STATE_VALUE: AtomicU32 = AtomicU32::new(0);

    fn initialize_with_state(state: &OnceState) -> u32 {
        if state.is_poisoned() {
            println!("OnceState: Previous initialization panicked, recovering...");
            100 // Recovery value
        } else {
            println!("OnceState: Normal initialization");
            42 // Normal value
        }
    }

    // First successful initialization
    INIT_STATE.call_once_force(|state| {
        let value = initialize_with_state(state);
        STATE_VALUE.store(value, Ordering::SeqCst);
    });

    println!(
        "OnceState value after initialization: {}",
        STATE_VALUE.load(Ordering::SeqCst)
    );

    // Try to initialize again, won't happen
    INIT_STATE.call_once_force(|state| {
        println!("This should not print, as initialization already happened");
        let value = initialize_with_state(state);
        STATE_VALUE.store(value, Ordering::SeqCst);
    });

    println!(
        "OnceState final value: {}",
        STATE_VALUE.load(Ordering::SeqCst)
    );
}

// Use export_name which is safer than no_mangle
#[unsafe(no_mangle)]
pub fn rust_crate_test_std_once_main() {
    println!("Testing std::sync::Once functionality");

    // Create several threads to demonstrate that initialization happens only once
    let handle1 = create_thread_with_stack_size();
    let handle2 = create_thread_with_stack_size();
    let handle3 = create_thread_with_stack_size();

    // Main thread also tries to initialize
    INIT.call_once(|| initialize_value());
    println!("Main thread read value: {}", VALUE.load(Ordering::SeqCst));

    // Wait for all threads to complete
    let _ = handle1.join();
    let _ = handle2.join();
    let _ = handle3.join();

    println!("All threads completed, Once demonstration finished");

    // Run additional demonstrations
    demonstrate_once_lock();
    demonstrate_once_state();

    println!("All synchronization primitive demonstrations completed");
}
