// Copyright (c) 2025 Xiaomi Corporation
// SPDX-License-Identifier: Apache-2.0

use std::cell::{Cell, RefCell};
use std::println;

fn demonstrate_cell() {
    println!("Demonstrating std::cell::Cell...");

    // Cell<T> allows getting and setting of the contained value
    // even when shared references to the Cell exist
    let cell = Cell::new(10);
    println!("Initial cell value: {}", cell.get());

    // We can mutate the value inside the Cell even with immutable references
    cell.set(20);
    println!("After set: {}", cell.get());

    // Cell is useful for simple Copy types
    let cell_ref = &cell;
    cell_ref.set(30);
    println!("After set through reference: {}", cell.get());

    // Replace returns the old value
    let prev = cell.replace(40);
    println!("Previous value: {}, New value: {}", prev, cell.get());
}

fn demonstrate_refcell() {
    println!("\nDemonstrating std::cell::RefCell...");

    // RefCell<T> enforces borrowing rules at runtime instead of compile time
    let ref_cell = RefCell::new(vec![1, 2, 3]);

    // Get an immutable borrow
    {
        let borrowed = ref_cell.borrow();
        println!("RefCell contents: {:?}", *borrowed);
    } // borrow is dropped here

    // Get a mutable borrow
    {
        let mut mut_borrowed = ref_cell.borrow_mut();
        mut_borrowed.push(4);
        mut_borrowed.push(5);
        println!("Modified RefCell contents: {:?}", *mut_borrowed);
    } // mutable borrow is dropped here

    // Try to handle a panic situation with try_borrow/try_borrow_mut
    println!("Current RefCell contents: {:?}", *ref_cell.borrow());

    // RefCell also provides runtime checking of Rust's borrowing rules
    println!(
        "RefCell's borrow state: {:?}",
        ref_cell.try_borrow_mut().is_ok()
    );
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_crate_test_std_cell_main() {
    println!("Starting std_cell example");

    demonstrate_cell();
    demonstrate_refcell();

    println!("std_cell example completed");
}
