// Copyright (c) 2025 Xiaomi Corporation
// SPDX-License-Identifier: Apache-2.0

use std::sync::mpsc;
use std::thread;

#[unsafe(no_mangle)]
pub fn rust_crate_test_std_mpsc_main() {
    test_same_thread();
    test_multi_thread();
}

fn test_same_thread() {
    println!("STD MPSC Channel Example - Same Thread");

    // Create a new channel
    let (tx, rx) = mpsc::channel();

    // Send a few messages
    println!("Sending messages through the channel...");
    tx.send("Hello").unwrap();
    tx.send("from").unwrap();
    tx.send("MPSC").unwrap();
    tx.send("channel!").unwrap();

    // Drop the sender to close the channel
    drop(tx);

    // Receive all messages
    println!("Receiving messages from the channel:");
    while let Ok(message) = rx.recv() {
        println!("Received: {}", message);
    }

    println!("All messages received. Channel closed.");
}

fn test_multi_thread() {
    println!("\nSTD MPSC Channel Example - Ping-Pong Test");

    // Create two channels - one for each direction
    let (ping_tx, ping_rx) = mpsc::channel(); // ping -> pong
    let (pong_tx, pong_rx) = mpsc::channel(); // pong -> ping

    // Number of ping-pong exchanges to perform
    const NUM_EXCHANGES: u32 = 5;

    // Spawn the ping thread
    let ping_builder = thread::Builder::new().stack_size(4 * 1024);
    let ping_handle = ping_builder
        .spawn(move || {
            println!("Ping thread: Starting ping-pong exchange");

            for i in 1..=NUM_EXCHANGES {
                // Send ping message
                let ping_msg = format!("PING {}", i);
                println!("Ping thread: Sending '{}'", ping_msg);
                ping_tx.send(ping_msg).unwrap();

                // Wait for pong response
                match pong_rx.recv_timeout(std::time::Duration::from_millis(500)) {
                    Ok(response) => {
                        println!("Ping thread: Received '{}'", response);
                        thread::sleep(std::time::Duration::from_millis(100));
                    }
                    Err(_) => {
                        println!("Ping thread: No response received, aborting");
                        break;
                    }
                }
            }

            println!("Ping thread: Exchanges completed");
            // Send termination signal
            ping_tx.send("DONE".to_string()).unwrap();
        })
        .expect("Failed to spawn ping thread");

    // Spawn the pong thread
    let pong_builder = thread::Builder::new().stack_size(4 * 1024);
    let pong_handle = pong_builder
        .spawn(move || {
            println!("Pong thread: Ready to respond");

            loop {
                // Wait for ping message
                match ping_rx.recv_timeout(std::time::Duration::from_millis(500)) {
                    Ok(message) => {
                        if message == "DONE" {
                            println!("Pong thread: Received termination signal");
                            break;
                        }

                        println!("Pong thread: Received '{}'", message);

                        // Send pong response
                        let pong_msg = format!("PONG for {}", message);
                        println!("Pong thread: Sending '{}'", pong_msg);
                        pong_tx.send(pong_msg).unwrap();
                        thread::sleep(std::time::Duration::from_millis(50));
                    }
                    Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                        println!("Pong thread: Timeout, no message received");
                        continue;
                    }
                    Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
                        println!("Pong thread: Channel closed");
                        break;
                    }
                }
            }

            println!("Pong thread: Communication ended");
        })
        .expect("Failed to spawn pong thread");

    // Wait for both threads to complete
    ping_handle.join().expect("Ping thread panicked");
    pong_handle.join().expect("Pong thread panicked");

    println!("Ping-pong test completed.");
}
