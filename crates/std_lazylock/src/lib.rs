// Copyright (c) 2025 Xiaomi Corporation
// SPDX-License-Identifier: Apache-2.0

use std::sync::LazyLock;
use std::time::Instant;

// Define a LazyLock that will perform an "expensive" computation only once
// when it's first accessed
static EXPENSIVE_RESULT: LazyLock<String> = LazyLock::new(|| {
    println!("Computing expensive result...");
    // Simulate an expensive computation
    std::thread::sleep(std::time::Duration::from_millis(100));
    "This is a result from an expensive computation".to_string()
});

// Another example with a more complex type
static FIBONACCI_CACHE: LazyLock<Vec<u64>> = LazyLock::new(|| {
    println!("Initializing Fibonacci cache...");
    let mut cache = vec![0, 1];
    for i in 2..20 {
        let next_fib = cache[i - 1] + cache[i - 2];
        cache.push(next_fib);
    }
    cache
});

#[unsafe(no_mangle)]
pub fn rust_crate_test_std_lazylock_main() {
    println!("LazyLock Demo - Testing lazy initialization");

    // First access to EXPENSIVE_RESULT will trigger initialization
    let start = Instant::now();
    println!("First access to EXPENSIVE_RESULT");
    println!("Result: {}", *EXPENSIVE_RESULT);
    println!("Access took: {:?}", start.elapsed());

    // Second access will use the already computed value
    let start = Instant::now();
    println!("\nSecond access to EXPENSIVE_RESULT");
    println!("Result: {}", *EXPENSIVE_RESULT);
    println!("Access took: {:?}", start.elapsed());

    // Demonstrate with Fibonacci sequence
    println!("\nAccessing FIBONACCI_CACHE first time");
    let fib_10 = FIBONACCI_CACHE[10];
    println!("10th Fibonacci number: {}", fib_10);

    println!("\nAccessing FIBONACCI_CACHE second time");
    let fib_15 = FIBONACCI_CACHE[15];
    println!("15th Fibonacci number: {}", fib_15);

    // Demonstrate thread safety (just print the address in this simple example)
    println!("\nLazyLock is thread-safe");
    let ptr = EXPENSIVE_RESULT.as_ptr();
    println!("EXPENSIVE_RESULT memory address: {:p}", ptr);

    println!("LazyLock Demo completed");
}
