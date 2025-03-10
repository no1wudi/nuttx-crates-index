// Copyright (c) 2025 Xiaomi Corporation
// SPDX-License-Identifier: Apache-2.0

use std::alloc::{Layout, alloc, dealloc};
use std::ptr::NonNull;

// A simple implementation of a custom vector using std::alloc
struct CustomVec<T> {
    ptr: NonNull<T>,
    cap: usize,
    len: usize,
}

impl<T> CustomVec<T> {
    fn new() -> Self {
        // Initial capacity of 4 elements
        let cap = 4;
        // Calculate the memory layout for T elements
        let layout = Layout::array::<T>(cap).expect("Failed to create layout");

        // Allocate memory
        let ptr =
            unsafe { NonNull::new(alloc(layout) as *mut T).expect("Failed to allocate memory") };

        CustomVec { ptr, cap, len: 0 }
    }

    fn push(&mut self, value: T) {
        // Check if we need to resize
        if self.len == self.cap {
            // Double the capacity
            let new_cap = self.cap * 2;
            // Calculate the new layout
            let new_layout = Layout::array::<T>(new_cap).expect("Failed to create layout");
            let old_layout = Layout::array::<T>(self.cap).expect("Failed to create layout");

            // Allocate new memory
            let new_ptr = unsafe {
                let ptr = alloc(new_layout) as *mut T;
                let new_ptr = NonNull::new(ptr).expect("Failed to allocate memory");

                // Copy old elements to new buffer
                std::ptr::copy_nonoverlapping(self.ptr.as_ptr(), new_ptr.as_ptr(), self.len);

                // Deallocate old memory
                dealloc(self.ptr.as_ptr() as *mut u8, old_layout);

                new_ptr
            };

            self.ptr = new_ptr;
            self.cap = new_cap;
        }

        // Add the new element
        unsafe {
            std::ptr::write(self.ptr.as_ptr().add(self.len), value);
        }

        self.len += 1;
    }

    fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            unsafe { Some(&*self.ptr.as_ptr().add(index)) }
        } else {
            None
        }
    }
}

impl<T> Drop for CustomVec<T> {
    fn drop(&mut self) {
        // Cleanup allocated elements
        unsafe {
            for i in 0..self.len {
                std::ptr::drop_in_place(self.ptr.as_ptr().add(i));
            }
            let layout = Layout::array::<T>(self.cap).expect("Failed to create layout");
            dealloc(self.ptr.as_ptr() as *mut u8, layout);
        }
    }
}

// Simple struct to test allocation
struct Point {
    x: i32,
    y: i32,
}

// Simple demonstration of Rust global allocator
fn demonstrate_global_allocator() {
    println!("Demonstrating Box allocation (uses global allocator)");
    let boxed_value = Box::new(42);
    println!("Allocated Box value: {}", *boxed_value);

    // Box is automatically deallocated when it goes out of scope
    println!("Box will be deallocated now");
}

// Demonstrate manual memory management with std::alloc
fn demonstrate_manual_allocation() {
    println!("Demonstrating manual memory allocation with std::alloc");

    // Create a layout for an i32 value
    let layout = Layout::new::<i32>();

    unsafe {
        // Allocate memory
        let ptr = alloc(layout) as *mut i32;
        if ptr.is_null() {
            println!("Memory allocation failed!");
            return;
        }

        // Write to the allocated memory
        *ptr = 123;
        println!("Manually allocated value: {}", *ptr);

        // Deallocate the memory
        dealloc(ptr as *mut u8, layout);
    }

    println!("Manual memory management completed");
}

// Demonstrate our custom vector implementation
fn demonstrate_custom_vec() {
    println!("Demonstrating custom vector implementation");

    let mut vec = CustomVec::<Point>::new();

    // Add some points
    vec.push(Point { x: 1, y: 2 });
    vec.push(Point { x: 3, y: 4 });
    vec.push(Point { x: 5, y: 6 });
    vec.push(Point { x: 7, y: 8 });

    // This push should cause a reallocation
    vec.push(Point { x: 9, y: 10 });

    // Print the points
    for i in 0..5 {
        if let Some(point) = vec.get(i) {
            println!("Point {}: ({}, {})", i, point.x, point.y);
        }
    }

    println!("Custom vector demonstration completed");
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_crate_test_std_alloc_main() {
    println!("Starting std::alloc demonstration");

    demonstrate_global_allocator();
    demonstrate_manual_allocation();
    demonstrate_custom_vec();

    println!("std::alloc demonstration completed");
}
