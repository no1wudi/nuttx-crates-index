// Copyright (c) 2025 Xiaomi Corporation
// SPDX-License-Identifier: Apache-2.0

use time::{Date, PrimitiveDateTime, Time};
// Removed macros and format description imports

/// This function demonstrates key features of the time 0.3 crate
#[unsafe(no_mangle)]
pub extern "C" fn rust_crate_test_time_main() {
    // Current time functionality
    let now = time::OffsetDateTime::now_utc();
    println!("Current UTC time: {}", now);

    // Working with date and time components
    let date = Date::from_calendar_date(2025, time::Month::December, 25).unwrap();
    let time = Time::from_hms(14, 30, 45).unwrap();
    let datetime = PrimitiveDateTime::new(date, time);
    println!("Created datetime: {}", datetime);

    // Simple date and time display without custom formatting
    println!(
        "Year: {}, Month: {}, Day: {}",
        date.year(),
        date.month(),
        date.day()
    );
    println!(
        "Hour: {}, Minute: {}, Second: {}",
        time.hour(),
        time.minute(),
        time.second()
    );

    // Manual parsing without format descriptions
    let year = 2025;
    let month = time::Month::December;
    let day = 25;
    let hour = 14;
    let minute = 30;
    let second = 45;

    let manual_date = Date::from_calendar_date(year, month, day).unwrap();
    let manual_time = Time::from_hms(hour, minute, second).unwrap();
    let manual_datetime = PrimitiveDateTime::new(manual_date, manual_time);
    println!("Manually constructed datetime: {}", manual_datetime);

    // Date arithmetic
    let tomorrow = date.next_day().unwrap();
    println!("Tomorrow: {}", tomorrow);

    // Duration calculation
    let duration = manual_datetime - datetime;
    println!(
        "Duration between dates: {} seconds",
        duration.whole_seconds()
    );

    println!("Time crate demonstration completed!");
}
