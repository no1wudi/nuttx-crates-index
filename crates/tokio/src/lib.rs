// Copyright (c) 2025 Xiaomi Corporation
// SPDX-License-Identifier: Apache-2.0

use std::io;
use tokio::fs;
use tokio::time::{Duration, sleep};

/// Sample function demonstrating a basic async operation
async fn async_task(id: u32) -> String {
    println!("Task {id} started");
    sleep(Duration::from_millis(100)).await;
    println!("Task {id} completed");
    format!("Task {id} result")
}

/// Test function for concurrency using multiple spawned tasks
async fn test_task_concurrency() {
    println!("Testing task concurrency");

    // Spawn multiple tasks to demonstrate concurrency
    let handles = (0..5)
        .map(|i| tokio::spawn(async move { async_task(i).await }))
        .collect::<Vec<_>>();

    // Wait for all tasks to complete and collect results
    let mut results = Vec::new();
    for handle in handles {
        if let Ok(result) = handle.await {
            results.push(result);
        }
    }

    println!("Completed {} concurrent tasks", results.len());
    assert_eq!(results.len(), 5, "All tasks should complete successfully");
}

/// Test file write and read operations
async fn test_fs_read_write() -> io::Result<()> {
    let test_path = "/tmp/tokio_fs_test.txt";
    let test_content = "Hello from Tokio fs on NuttX!";

    // Write to file
    fs::write(test_path, test_content).await?;
    println!("Successfully wrote to {}", test_path);

    // Read from file
    let read_content = fs::read_to_string(test_path).await?;
    println!("Read from file: {}", read_content);

    // Verify content matches
    assert_eq!(
        test_content, read_content,
        "File content doesn't match what was written"
    );

    Ok(())
}

/// Test file metadata and removal
async fn test_fs_metadata_and_removal() -> io::Result<()> {
    let test_path = "/tmp/tokio_fs_test.txt";

    // Test file existence and get metadata
    let metadata = fs::metadata(test_path).await?;
    println!("File size: {} bytes", metadata.len());

    // Test file removal
    fs::remove_file(test_path).await?;
    println!("Successfully removed test file");

    Ok(())
}

/// Run all filesystem tests in sequence
async fn test_fs_operations() -> io::Result<()> {
    println!("Starting fs operations test");

    test_fs_read_write().await?;
    test_fs_metadata_and_removal().await?;

    println!("All filesystem tests completed successfully");
    Ok(())
}

/// Function to run the single thread tokio test
async fn run_single_thread_test() {
    println!("Inside tokio runtime");

    // Test concurrent tasks
    test_task_concurrency().await;
    println!("Concurrency tests completed");

    // Test fs operations
    match test_fs_operations().await {
        Ok(_) => println!("FS operations test completed successfully"),
        Err(e) => {
            eprintln!("FS operations test failed: {}", e);
            panic!("File system tests failed");
        }
    }
}

#[unsafe(no_mangle)]
pub fn rust_crate_test_tokio_main() {
    println!("Starting Tokio runtime on NuttX");

    // Create a single-threaded runtime for resource-constrained environments
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    println!("Tokio runtime created");

    // Execute the test function on the runtime
    rt.block_on(run_single_thread_test());

    println!("Tokio runtime completed");
}
