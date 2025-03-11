// Copyright (c) 2025 Xiaomi Corporation
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;
use std::sync::RwLock;
use std::thread;
use std::time::Duration;

// Create NuttX compatible thread with sufficient stack size
fn spawn_thread<F, T>(name: &str, f: F) -> thread::JoinHandle<T>
where
    F: FnOnce() -> T,
    F: Send + 'static,
    T: Send + 'static,
{
    let builder = thread::Builder::new()
        .name(name.to_string())
        .stack_size(4096); // 4K stack size
    builder.spawn(f).expect("Thread creation failed")
}

// Function to test RwLock in a single thread context
fn test_rwlock_single_thread() {
    println!("\nSingle Thread RwLock Test");

    // Create a RwLock
    let rwlock = RwLock::new(0);

    // Acquire a read lock
    {
        let value = rwlock.read().unwrap();
        println!("Single thread read value: {}", *value);
    } // Read lock released here

    // Acquire a write lock
    {
        let mut value = rwlock.write().unwrap();
        *value = 42;
        println!("Single thread updated value to: {}", *value);
    } // Write lock released here

    // Read the updated value
    {
        let value = rwlock.read().unwrap();
        println!("Single thread read updated value: {}", *value);
    }

    println!("Single thread test completed successfully!");
}

// Function to test RwLock in a multi-thread context
fn test_rwlock_multi_thread() {
    println!("\nMulti-Thread RwLock Test");

    // Create a RwLock guarding a shared resource
    let rwlock = Arc::new(RwLock::new(0));

    // Create multiple reader threads
    let mut readers = Vec::new();

    for i in 0..3 {
        let rwlock_clone = Arc::clone(&rwlock);
        let handle = spawn_thread(&format!("reader-{}", i), move || {
            for _ in 0..5 {
                // Acquire a read lock
                let value = rwlock_clone.read().unwrap();
                println!("Reader {} read value: {}", i, *value);

                // Sleep to simulate some work
                thread::sleep(Duration::from_millis(100));

                // Read lock is automatically released here when `value` goes out of scope
            }
        });
        readers.push(handle);
    }

    // Create a writer thread
    let rwlock_clone = Arc::clone(&rwlock);
    let writer = spawn_thread("writer", move || {
        for i in 1..=5 {
            // Sleep a bit to let readers start
            thread::sleep(Duration::from_millis(200));

            // Acquire a write lock - this will block until all readers are done
            {
                let mut value = rwlock_clone.write().unwrap();
                *value = i;
                println!("Writer updated value to: {}", *value);
            } // Write lock is released here

            // Give readers a chance to acquire the lock
            thread::sleep(Duration::from_millis(200));
        }
    });

    // Wait for all threads to complete
    for (i, reader) in readers.into_iter().enumerate() {
        reader.join().unwrap();
        println!("Reader {} finished", i);
    }

    writer.join().unwrap();
    println!("Writer finished");

    // Final value check
    let final_value = *rwlock.read().unwrap();
    println!("Final value: {}", final_value);

    println!("Multi-thread test completed successfully!");
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_crate_test_std_rwlock_main() {
    println!("RwLock Example: Multiple readers, exclusive writer");

    // Run single thread test
    test_rwlock_single_thread();

    // Run multi-thread test
    test_rwlock_multi_thread();

    println!("RwLock example completed successfully!");
}
