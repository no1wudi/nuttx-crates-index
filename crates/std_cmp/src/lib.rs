// Copyright (c) 2025 Xiaomi Corporation
// SPDX-License-Identifier: Apache-2.0

use std::cmp::{Ord, Ordering, PartialOrd, max, min};
use std::println;

// Define a custom struct to demonstrate Ord and PartialOrd traits
#[derive(Debug, Eq, PartialEq)]
struct Sensor {
    id: u32,
    priority: u8,
}

// Implement Ord and PartialOrd based on priority field
impl PartialOrd for Sensor {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Sensor {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority)
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_crate_test_std_cmp_main() {
    println!("=== Demonstrating std::cmp functionality ===");

    // Demonstrate min and max functions
    let a = 10;
    let b = 5;
    println!("min({}, {}) = {}", a, b, min(a, b));
    println!("max({}, {}) = {}", a, b, max(a, b));

    // Demonstrate Ordering enum
    let result = a.cmp(&b);
    match result {
        Ordering::Less => println!("{} is less than {}", a, b),
        Ordering::Equal => println!("{} is equal to {}", a, b),
        Ordering::Greater => println!("{} is greater than {}", a, b),
    }

    // Demonstrate custom Ord implementation with Sensor struct
    let sensor1 = Sensor { id: 1, priority: 3 };
    let sensor2 = Sensor { id: 2, priority: 1 };
    let sensor3 = Sensor { id: 3, priority: 5 };

    println!("Comparing sensors based on priority:");
    println!(
        "sensor1 ({:?}) vs sensor2 ({:?}): {:?}",
        sensor1,
        sensor2,
        sensor1.cmp(&sensor2)
    );
    println!(
        "sensor2 ({:?}) vs sensor3 ({:?}): {:?}",
        sensor2,
        sensor3,
        sensor2.cmp(&sensor3)
    );

    // Finding the minimum and maximum sensors using the Ord trait
    let sensors = [&sensor1, &sensor2, &sensor3];
    let min_sensor = sensors.iter().min().unwrap();
    let max_sensor = sensors.iter().max().unwrap();

    println!("Sensor with minimum priority: {:?}", min_sensor);
    println!("Sensor with maximum priority: {:?}", max_sensor);

    // Demonstrate Reverse adapter
    let mut numbers = vec![3, 1, 5, 2, 4];
    println!("Original numbers: {:?}", numbers);
    numbers.sort();
    println!("Sorted numbers (ascending): {:?}", numbers);
    numbers.sort_by(|a, b| b.cmp(a));
    println!("Sorted numbers (descending): {:?}", numbers);

    println!("=== std::cmp demonstration completed ===");
}
