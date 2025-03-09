// Copyright (c) 2025 Xiaomi Corporation
// SPDX-License-Identifier: Apache-2.0

use chrono::{DateTime, Local, TimeZone, Utc};

#[unsafe(no_mangle)]
pub extern "C" fn rust_crate_test_chrono_main() {
    // Get the current UTC time
    let utc_now: DateTime<Utc> = Utc::now();

    // Get the current local time
    let local_now: DateTime<Local> = Local::now();

    // Log formatted time strings using println
    println!("UTC time: {}", utc_now.format("%Y-%m-%d %H:%M:%S"));
    println!("Local time: {}", local_now.format("%Y-%m-%d %H:%M:%S"));

    // Create a specific date
    if let Some(specific_date) = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).single() {
        println!("Specific date: {}", specific_date.format("%Y-%m-%d"));
    }
}
