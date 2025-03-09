// Copyright (c) 2025 Xiaomi Corporation
// SPDX-License-Identifier: Apache-2.0

use std::sync::mpsc;

#[unsafe(no_mangle)]
pub extern "C" fn rust_crate_test_std_mpsc_main() {
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
