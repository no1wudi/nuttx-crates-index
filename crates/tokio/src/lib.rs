// Copyright (c) 2025 Xiaomi Corporation
// SPDX-License-Identifier: Apache-2.0

use tokio::time::{Duration, sleep};

/// Sample function demonstrating a basic async operation
async fn async_task(id: u32) {
    println!("Task {id} started");
    sleep(Duration::from_millis(100)).await;
    println!("Task {id} completed");
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_crate_test_tokio_main() {
    println!("Starting Tokio runtime on NuttX");

    // Create a single-threaded runtime for resource-constrained environments
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    println!("Tokio runtime created");

    // Execute the async block on the runtime
    rt.block_on(async {
        println!("Inside tokio runtime");

        // Spawn multiple tasks to demonstrate concurrency
        let handles = (0..5)
            .map(|i| {
                tokio::spawn(async move {
                    async_task(i).await;
                })
            })
            .collect::<Vec<_>>();

        // Wait for all tasks to complete
        for handle in handles {
            let _ = handle.await;
        }

        println!("All tasks completed");
    });

    println!("Tokio runtime completed");
}
