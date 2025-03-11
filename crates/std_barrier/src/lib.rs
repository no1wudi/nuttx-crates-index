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

#[unsafe(no_mangle)]
pub extern "C" fn rust_crate_test_std_barrier_main() {
    println!("Starting std_barrier demo");

    // Number of threads that will synchronize on the barrier
    let num_threads = 4;

    // Create a barrier that will wait for all threads
    let barrier = Arc::new(Barrier::new(num_threads));
    let mut handles = Vec::with_capacity(num_threads);

    // Spawn multiple threads that will wait at the barrier
    for i in 0..num_threads {
        let b = Arc::clone(&barrier);

        // Use 4KB stack for each thread
        let handle = spawn_thread_with_stack(
            &format!("thread-{}", i),
            4 * 1024, // 4KB stack size
            move || {
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
            },
        );

        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        let _ = handle.join();
    }

    println!("All threads have completed. Barrier demo finished.");
}
