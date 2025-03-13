// Copyright (c) 2025 Xiaomi Corporation
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;
use std::sync::Barrier;
use std::thread;
use std::time::Duration;

// Function to create threads with explicit stack size in NuttX
fn spawn_thread_with_stack<F, T>(name: &str, stack_size: usize, f: F) -> thread::JoinHandle<T>
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
{
    let builder = thread::Builder::new()
        .name(name.to_string())
        .stack_size(stack_size);
    builder.spawn(f).expect("Failed to spawn thread")
}

// Function to demonstrate basic barrier synchronization
fn run_basic_barrier_demo() {
    println!("Starting basic barrier demo");

    let num_threads = 4;
    let barrier = Arc::new(Barrier::new(num_threads));
    let mut handles = Vec::with_capacity(num_threads);

    for i in 0..num_threads {
        let b = Arc::clone(&barrier);

        let handle = spawn_thread_with_stack(&format!("thread-{}", i), 4 * 1024, move || {
            println!("Thread {} is waiting at the barrier", i);

            // Simulate some work
            thread::sleep(Duration::from_millis((i as u64 + 1) * 500));

            println!("Thread {} is ready and waiting on barrier", i);

            // Wait for all threads to reach this point
            let wait_result = b.wait();

            println!(
                "Thread {} has been released! Is last thread: {}",
                i,
                wait_result.is_leader()
            );

            // Do some work after the barrier
            thread::sleep(Duration::from_millis(300));

            println!("Thread {} completed", i);
        });

        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        let _ = handle.join();
    }

    println!("All threads have completed. Basic barrier demo finished.");
}

// Function to demonstrate leader election using BarrierWaitResult
fn run_leader_election_demo() {
    println!("\n--- BarrierWaitResult Demo ---");

    let num_workers = 3;
    let task_barrier = Arc::new(Barrier::new(num_workers));
    let mut worker_handles = Vec::with_capacity(num_workers);

    // Shared data that needs to be processed after synchronization
    let shared_data = Arc::new(std::sync::Mutex::new(Vec::new()));

    for i in 0..num_workers {
        let b = Arc::clone(&task_barrier);
        let data = Arc::clone(&shared_data);

        let handle = spawn_thread_with_stack(&format!("worker-{}", i), 4 * 1024, move || {
            println!("Worker {} performing initial task", i);

            // Each worker generates some data
            let worker_result = i * 10;

            // Store the worker's result in shared data
            {
                let mut shared = data.lock().unwrap();
                shared.push(worker_result);
                println!("Worker {} added result: {}", i, worker_result);
            }

            // All workers synchronize at the barrier
            println!("Worker {} waiting at synchronization point", i);
            let wait_result = b.wait();

            // The leader thread (typically the last to arrive) will process the results
            if wait_result.is_leader() {
                println!("Worker {} is the leader - processing collected data", i);

                // Leader processes the collected data
                let shared = data.lock().unwrap();
                // Fix: Change the sum type to usize since we're summing usize values
                let sum: usize = shared.iter().sum();
                let avg: f32 = sum as f32 / shared.len() as f32;

                println!("Leader calculated: sum = {}, average = {:.2}", sum, avg);
            } else {
                println!(
                    "Worker {} is not the leader - continuing with regular work",
                    i
                );
            }

            println!("Worker {} finished execution", i);
        });

        worker_handles.push(handle);
    }

    // Wait for all worker threads to complete
    for handle in worker_handles {
        let _ = handle.join();
    }

    println!("BarrierWaitResult demo completed");
}

#[unsafe(no_mangle)]
pub fn rust_crate_test_std_barrier_main() {
    println!("Starting std_barrier demo");

    // Run the first demo showing basic barrier functionality
    run_basic_barrier_demo();

    // Run the second demo showing leader election with barriers
    run_leader_election_demo();
}
