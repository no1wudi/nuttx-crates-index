// Copyright (c) 2025 Xiaomi Corporation
// SPDX-License-Identifier: Apache-2.0

use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

#[unsafe(no_mangle)]
pub fn rust_crate_test_std_time_main() {
    println!("=== Rust std::time Demo ===");

    // Demo 1: Duration
    demo_duration();

    // Demo 2: Instant for high-resolution time measurement
    demo_instant();

    // Demo 3: SystemTime for wall-clock time
    demo_systemtime();

    println!("=== Time Demo Complete ===");
}

fn demo_duration() {
    println!("\n=== Duration Demo ===");

    // Create durations
    let one_second = Duration::from_secs(1);
    let half_second = Duration::from_millis(500);
    let quarter_second = Duration::from_micros(250_000);

    println!("1 second: {:?}", one_second);
    println!("0.5 second: {:?}", half_second);
    println!("0.25 second: {:?}", quarter_second);

    // Arithmetic with durations
    let one_and_half = one_second + half_second;
    println!("1s + 0.5s = {:?}", one_and_half);

    // Sleep using duration
    print!("Sleeping for 1 second... ");
    thread::sleep(one_second);
    println!("Done!");
}

fn demo_instant() {
    println!("\n=== Instant Demo ===");

    // Measure time elapsed for an operation
    let start = Instant::now();

    // Simulating work with a short sleep
    thread::sleep(Duration::from_millis(100));

    let elapsed = start.elapsed();
    println!("Operation took: {:?}", elapsed);

    // Measuring multiple steps
    let start = Instant::now();

    // Step 1
    thread::sleep(Duration::from_millis(50));
    let step1 = start.elapsed();

    // Step 2
    thread::sleep(Duration::from_millis(75));
    let step2 = start.elapsed();

    println!("Step 1 took: {:?}", step1);
    println!("Step 2 took: {:?}", step2);
    println!("Total time: {:?}", step2);
}

fn demo_systemtime() {
    println!("\n=== SystemTime Demo ===");

    // Get current time
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => {
            println!("Current Unix timestamp: {} seconds", duration.as_secs());
            println!(
                "With milliseconds: {}.{:03} seconds",
                duration.as_secs(),
                duration.subsec_millis()
            );
        }
        Err(e) => {
            println!("Error getting system time: {:?}", e);
        }
    }

    // Creating SystemTime values
    let now = SystemTime::now();
    let one_hour_later = now + Duration::from_secs(3600);
    let one_hour_earlier = now - Duration::from_secs(3600);

    match one_hour_later.duration_since(now) {
        Ok(duration) => println!("Time until one hour later: {:?}", duration),
        Err(e) => println!("Error: {:?}", e),
    }

    match now.duration_since(one_hour_earlier) {
        Ok(duration) => println!("Time since one hour earlier: {:?}", duration),
        Err(e) => println!("Error: {:?}", e),
    }
}
