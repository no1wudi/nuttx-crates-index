// Copyright (c) 2025 Xiaomi Corporation
// SPDX-License-Identifier: Apache-2.0

use memchr::memchr;
use memchr::memchr2;
use memchr::memchr3;
use memchr::memchr3_iter;
use memchr::memmem;
use memchr::memrchr;

#[unsafe(no_mangle)]
pub fn rust_crate_test_memchr_main() {
    println!("Testing memchr crate functionality");

    // Sample haystack for our searches
    let haystack = b"Hello, world! This is a test string";

    test_memchr(haystack);
    test_memrchr(haystack);
    test_memchr2(haystack);
    test_memchr3(haystack);
    test_memchr3_iter();
    test_memmem_find_iter();
    test_memmem_finder();
    test_throughput_demo();

    println!("memchr testing completed");
}

fn test_memchr(haystack: &[u8]) {
    // Using memchr to find the first occurrence of a byte
    if let Some(pos) = memchr(b'w', haystack) {
        println!("Found 'w' at position {}", pos);
    } else {
        println!("Character 'w' not found");
    }
}

fn test_memrchr(haystack: &[u8]) {
    // Using memrchr to find the last occurrence of a byte
    if let Some(pos) = memrchr(b't', haystack) {
        println!("Last 't' found at position {}", pos);
    } else {
        println!("Character 't' not found");
    }
}

fn test_memchr2(haystack: &[u8]) {
    // Using memchr2 to find the first occurrence of either of two bytes
    if let Some(pos) = memchr2(b'e', b'o', haystack) {
        println!("First 'e' or 'o' found at position {}", pos);
    } else {
        println!("Neither 'e' nor 'o' found");
    }
}

fn test_memchr3(haystack: &[u8]) {
    // Using memchr3 to find the first occurrence of any of three bytes
    if let Some(pos) = memchr3(b'H', b'T', b'!', haystack) {
        println!("First 'H', 'T', or '!' found at position {}", pos);
    } else {
        println!("None of 'H', 'T', or '!' found");
    }
}

fn test_memchr3_iter() {
    println!("Testing memchr3_iter with reverse iteration");

    let haystack = b"xyzaxyzbxyzc";

    let mut it = memchr3_iter(b'a', b'b', b'c', haystack).rev();
    let positions = [it.next(), it.next(), it.next(), it.next()];

    println!("Reverse positions of 'a', 'b', 'c': {:?}", positions);

    if positions == [Some(11), Some(7), Some(3), None] {
        println!("memchr3_iter test passed");
    } else {
        println!("memchr3_iter test failed");
    }
}

fn test_memmem_find_iter() {
    println!("Testing memmem::find_iter");

    let haystack = b"foo bar foo baz foo";

    let mut it = memmem::find_iter(haystack, "foo");
    let positions = [it.next(), it.next(), it.next(), it.next()];

    println!("Positions of 'foo': {:?}", positions);

    if positions == [Some(0), Some(8), Some(16), None] {
        println!("memmem::find_iter test passed");
    } else {
        println!("memmem::find_iter test failed");
    }
}

fn test_memmem_finder() {
    println!("Testing memmem::Finder");

    let finder = memmem::Finder::new("foo");

    let pos1 = finder.find(b"baz foo quux");
    let pos2 = finder.find(b"quux baz bar");

    println!("Position of 'foo' in first haystack: {:?}", pos1);
    println!("Position of 'foo' in second haystack: {:?}", pos2);

    if pos1 == Some(4) && pos2 == None {
        println!("memmem::Finder test passed");
    } else {
        println!("memmem::Finder test failed");
    }
}

fn test_throughput_demo() {
    println!("Demonstrating throughput advantage");

    // Create a large haystack where the target byte doesn't exist
    let large_haystack = vec![0u8; 1_000_000];

    // Using memchr (optimized)
    let start = std::time::Instant::now();
    let result1 = memchr(b'z', &large_haystack);
    let duration1 = start.elapsed();

    // Using naive implementation
    let start = std::time::Instant::now();
    let result2 = large_haystack.iter().position(|&b| b == b'z');
    let duration2 = start.elapsed();

    println!("memchr result: {:?}, time: {:?}", result1, duration1);
    println!("naive search result: {:?}, time: {:?}", result2, duration2);
    println!(
        "memchr speedup: {:.2}x",
        duration2.as_nanos() as f64 / duration1.as_nanos() as f64
    );
}
