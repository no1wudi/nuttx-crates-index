// Copyright (c) 2025 Xiaomi Corporation
// SPDX-License-Identifier: Apache-2.0

use std::sync::{Arc, Condvar, Mutex};
use std::thread::{self, Builder, JoinHandle};
use std::time::Duration;

/// A simple producer-consumer example using a Condvar
struct SharedData {
    queue: Vec<i32>,
    ready: bool,
}

// Stack size for our custom threads (4KB)
const THREAD_STACK_SIZE: usize = 4 * 1024;

#[unsafe(no_mangle)]
pub extern "C" fn rust_crate_test_std_condvar_main() {
    println!("Starting Rust std::sync::Condvar example");

    // Create shared state for our threads
    let shared = Arc::new((
        Mutex::new(SharedData {
            queue: Vec::new(),
            ready: false,
        }),
        Condvar::new(),
    ));

    // Clone Arc for the producer thread
    let producer_shared = Arc::clone(&shared);
    let producer: JoinHandle<()> = Builder::new()
        .name("producer".to_string())
        .stack_size(THREAD_STACK_SIZE)
        .spawn(move || {
            // Producer waits briefly before producing data
            thread::sleep(Duration::from_millis(500));

            // Get lock on shared data
            let (shared_data, cvar) = &*producer_shared;
            let mut data = shared_data.lock().unwrap();

            // Produce some data
            for i in 0..5 {
                println!("Producer: adding item {}", i);
                data.queue.push(i);
                thread::sleep(Duration::from_millis(200));
            }

            // Signal that production is complete
            data.ready = true;
            println!("Producer: signaling consumer");

            // Notify the consumer
            cvar.notify_one();
        })
        .expect("Failed to create producer thread");

    // Clone Arc for the consumer thread
    let consumer_shared = Arc::clone(&shared);
    let consumer: JoinHandle<()> = Builder::new()
        .name("consumer".to_string())
        .stack_size(THREAD_STACK_SIZE)
        .spawn(move || {
            // Get lock on shared data
            let (shared_data, cvar) = &*consumer_shared;
            let mut data = shared_data.lock().unwrap();

            println!("Consumer: waiting for data");

            // Wait until the ready flag is set
            while !data.ready {
                data = cvar.wait(data).unwrap();
            }

            // Process the queue
            println!("Consumer: processing data");
            while let Some(item) = data.queue.pop() {
                println!("Consumer: processed item {}", item);
            }
        })
        .expect("Failed to create consumer thread");

    // Wait for both threads to complete
    producer.join().unwrap();
    consumer.join().unwrap();

    println!("Rust std::sync::Condvar example completed");
}
