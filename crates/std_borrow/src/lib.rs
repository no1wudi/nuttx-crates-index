// Copyright (c) 2025 Xiaomi Corporation
// SPDX-License-Identifier: Apache-2.0

use std::borrow::Borrow;
use std::borrow::BorrowMut;
use std::borrow::Cow;
use std::borrow::ToOwned;
use std::fmt::Write;

fn demo_cow() {
    println!("\n=== Cow (Clone on Write) Examples ===");

    // Borrow case - no allocation needed
    let borrowed_data: Cow<str> = Cow::Borrowed("hello world");
    println!("Borrowed data: {}", borrowed_data);

    // Mutation case - will allocate and convert to owned data
    let mut data: Cow<str> = Cow::Borrowed("hello");
    println!("Before modification: {}", data);

    // This will cause a clone to happen since we need to modify
    data.to_mut().push_str(" world");
    println!("After modification: {}", data);

    // Directly creating an owned version
    let owned_data: Cow<str> = Cow::Owned(String::from("owned string"));
    println!("Owned data: {}", owned_data);

    // Cow with other types
    let borrowed_numbers: Cow<[i32]> = Cow::Borrowed(&[1, 2, 3]);
    println!("Borrowed numbers: {:?}", borrowed_numbers);

    let mut numbers: Cow<[i32]> = Cow::Borrowed(&[1, 2, 3]);
    numbers.to_mut().push(4); // Will clone and modify
    println!("Modified numbers: {:?}", numbers);
}

fn demo_borrow_trait() {
    println!("\n=== Borrow and BorrowMut Traits Examples ===");

    // String implements Borrow<str>
    let s = String::from("hello");
    print_borrowed(&s); // Implicitly borrows as &str

    // Vec<T> implements Borrow<[T]>
    let v = vec![1, 2, 3];
    print_slice(&v); // Implicitly borrows as &[i32]

    // Using BorrowMut
    let mut s = String::from("hello");
    modify_string(&mut s);
    println!("Modified string: {}", s);
}

// Functions that demonstrate Borrow trait usage
fn print_borrowed<T: AsRef<str>>(value: &T) {
    let borrowed: &str = value.as_ref();
    println!("Borrowed string: {}", borrowed);
}

fn print_slice<T: Borrow<[i32]>>(value: &T) {
    let borrowed: &[i32] = value.borrow();
    println!("Borrowed slice: {:?}", borrowed);
}

// Function that demonstrates BorrowMut trait usage
fn modify_string<T: BorrowMut<String>>(value: &mut T) {
    let borrowed: &mut String = value.borrow_mut();
    borrowed.push_str(" world");
}

fn demo_to_owned() {
    println!("\n=== ToOwned Trait Examples ===");

    // str to String conversion using to_owned
    let s: &str = "hello";
    let owned_string = s.to_owned();
    println!("&str to String: {:?}", owned_string);

    // [T] to Vec<T> conversion using to_owned
    let slice: &[i32] = &[1, 2, 3];
    let owned_vec = slice.to_owned();
    println!("&[i32] to Vec<i32>: {:?}", owned_vec);

    // Custom implementation example with a string buffer
    let mut buffer = String::new();
    writeln!(buffer, "Line 1").unwrap();
    writeln!(buffer, "Line 2").unwrap();

    // Convert a &str to another format using to_owned
    let owned_data = buffer.to_owned();
    println!("Custom string buffer:\n{}", owned_data);
}

#[unsafe(no_mangle)]
pub fn rust_crate_test_std_borrow_main() {
    println!("Starting std::borrow showcase");

    // Example 1: Using Cow (Clone-on-write) for efficient string operations
    demo_cow();

    // Example 2: Using Borrow and BorrowMut traits
    demo_borrow_trait();

    // Example 3: Using ToOwned trait
    demo_to_owned();

    println!("std::borrow showcase completed");
}
