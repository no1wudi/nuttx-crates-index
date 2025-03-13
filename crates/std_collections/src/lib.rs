// Copyright (c) 2025 Xiaomi Corporation
// SPDX-License-Identifier: Apache-2.0

use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet, VecDeque};

fn demo_vec() {
    println!("\n=== Vector Demonstration ===");
    // Creating a vector
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    println!("Vector contents: {:?}", v);

    // Accessing elements
    println!("Second element: {}", v[1]);
    println!("Last element: {}", v.last().unwrap());

    // Iterating
    println!("Iterating over vector:");
    for item in &v {
        println!("  - {}", item);
    }

    // Vector operations
    v.pop();
    println!("After pop: {:?}", v);

    v.insert(0, 0);
    println!("After inserting 0 at the beginning: {:?}", v);

    let sum: i32 = v.iter().sum();
    println!("Sum of elements: {}", sum);
}

fn demo_hashmap() {
    println!("\n=== HashMap Demonstration ===");
    // Creating a HashMap
    let mut scores = HashMap::new();

    // Inserting values
    scores.insert("Alice", 95);
    scores.insert("Bob", 82);
    scores.insert("Charlie", 90);

    println!("HashMap contents: {:?}", scores);

    // Accessing values
    match scores.get("Alice") {
        Some(score) => println!("Alice's score: {}", score),
        None => println!("Alice not found"),
    }

    // Iterating
    println!("All scores:");
    for (name, score) in &scores {
        println!("  - {}: {}", name, score);
    }

    // Updating values
    *scores.entry("Bob").or_insert(0) += 5;
    println!("Bob's new score: {}", scores["Bob"]);

    // Check if key exists
    if scores.contains_key("David") {
        println!("David found");
    } else {
        println!("David not found");
    }

    // Remove an entry
    scores.remove("Charlie");
    println!("After removing Charlie: {:?}", scores);
}

fn demo_hashset() {
    println!("\n=== HashSet Demonstration ===");
    // Creating a HashSet
    let mut unique_numbers = HashSet::new();

    // Inserting values
    unique_numbers.insert(10);
    unique_numbers.insert(20);
    unique_numbers.insert(30);
    unique_numbers.insert(10); // Duplicate, will be ignored

    println!("HashSet contents: {:?}", unique_numbers);
    println!("HashSet size: {}", unique_numbers.len());

    // Check for inclusion
    if unique_numbers.contains(&20) {
        println!("Set contains 20");
    }

    // Set operations
    let mut set1 = HashSet::new();
    set1.insert(1);
    set1.insert(2);

    let mut set2 = HashSet::new();
    set2.insert(2);
    set2.insert(3);

    println!("Set1: {:?}", set1);
    println!("Set2: {:?}", set2);

    // Union
    let union: HashSet<_> = set1.union(&set2).cloned().collect();
    println!("Union: {:?}", union);

    // Intersection
    let intersection: HashSet<_> = set1.intersection(&set2).cloned().collect();
    println!("Intersection: {:?}", intersection);

    // Difference
    let difference: HashSet<_> = set1.difference(&set2).cloned().collect();
    println!("Difference (set1 - set2): {:?}", difference);
}

fn demo_vecdeque() {
    println!("\n=== VecDeque Demonstration ===");
    // Creating a double-ended queue
    let mut deque = VecDeque::new();

    // Adding elements at both ends
    deque.push_back(3);
    deque.push_back(4);
    deque.push_front(2);
    deque.push_front(1);

    println!("VecDeque contents: {:?}", deque);

    // Removing elements from both ends
    println!("Front: {:?}", deque.pop_front());
    println!("Back: {:?}", deque.pop_back());

    println!("After popping: {:?}", deque);

    // Using as a queue (FIFO)
    println!("\nUsing as a queue:");
    let mut queue = VecDeque::new();
    queue.push_back("first");
    queue.push_back("second");
    queue.push_back("third");

    while let Some(item) = queue.pop_front() {
        println!("  Dequeued: {}", item);
    }
}

fn demo_btreemap() {
    println!("\n=== BTreeMap Demonstration ===");
    // Creating a BTreeMap (ordered by key)
    let mut map = BTreeMap::new();

    // Insert some values
    map.insert(3, "three");
    map.insert(1, "one");
    map.insert(4, "four");
    map.insert(2, "two");

    // BTreeMap keeps keys in sorted order
    println!("BTreeMap contents (sorted by key):");
    for (key, value) in &map {
        println!("  {}: {}", key, value);
    }

    // Range operations
    println!("\nEntries with keys from 2 to 3:");
    for (key, value) in map.range(2..=3) {
        println!("  {}: {}", key, value);
    }

    // First and last entries
    if let Some((key, value)) = map.first_key_value() {
        println!("\nFirst entry: {}: {}", key, value);
    }

    if let Some((key, value)) = map.last_key_value() {
        println!("Last entry: {}: {}", key, value);
    }
}

fn demo_binaryheap() {
    println!("\n=== BinaryHeap Demonstration ===");
    // Creating a binary heap (max heap by default)
    let mut heap = BinaryHeap::new();

    // Adding elements
    heap.push(3);
    heap.push(5);
    heap.push(1);
    heap.push(4);
    heap.push(2);

    println!("BinaryHeap contents: {:?}", heap);
    println!("Heap size: {}", heap.len());

    // Peek at the maximum element
    if let Some(max) = heap.peek() {
        println!("Maximum element: {}", max);
    }

    // Extract elements in decreasing order
    println!("\nElements in decreasing order:");
    while let Some(item) = heap.pop() {
        println!("  {}", item);
    }
}

#[unsafe(no_mangle)]
pub fn rust_crate_test_std_collections_main() {
    println!("=== Rust Standard Library Collections Demo ===");

    // Demo all collection types
    demo_vec();
    demo_hashmap();
    demo_hashset();
    demo_vecdeque();
    demo_btreemap();
    demo_binaryheap();

    println!("\nCollections demo completed!");
}
