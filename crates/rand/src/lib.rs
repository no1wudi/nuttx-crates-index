// Copyright (c) 2025 Xiaomi Corporation
// SPDX-License-Identifier: Apache-2.0

use rand::{rngs::StdRng, Rng, SeedableRng};
use std::time::{SystemTime, UNIX_EPOCH};

#[unsafe(no_mangle)]
pub extern "C" fn rust_crate_test_rand_main() {
    // Get current time and use it as seed
    let now = SystemTime::now();
    let duration = now.duration_since(UNIX_EPOCH).expect("Time went backwards");

    // Create a seed from the time components
    let mut seed = [0u8; 32];
    let seconds = duration.as_secs();
    let nanos = duration.subsec_nanos();

    // Fill first 8 bytes with seconds
    for i in 0..8 {
        if i < 8 {
            seed[i] = ((seconds >> (i * 8)) & 0xff) as u8;
        }
    }

    // Fill next 4 bytes with nanoseconds
    for i in 0..4 {
        if i + 8 < 32 {
            seed[i + 8] = ((nanos >> (i * 8)) & 0xff) as u8;
        }
    }

    let mut rng = StdRng::from_seed(seed);
    let random_value: u32 = rng.random();
    println!("Random value: {}", random_value);
}
