// Copyright (c) 2025 Xiaomi Corporation
// SPDX-License-Identifier: Apache-2.0

use std::fmt::Write;
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::ptr::NonNull;

/// A self-referential struct that contains a pointer to one of its own fields.
/// This kind of structure requires Pin to ensure memory safety.
struct SelfReferential {
    // The actual data
    data: String,
    // A pointer to the data field
    data_ptr: Option<NonNull<String>>,
    // This marker prevents the type from being moved after it's initialized
    _marker: PhantomPinned,
}

impl SelfReferential {
    fn new(data: String) -> Self {
        SelfReferential {
            data,
            data_ptr: None,
            _marker: PhantomPinned,
        }
    }

    // This method must take a pinned mutable reference to ensure safety
    fn init(self: Pin<&mut Self>) {
        // Safety: We know the struct is pinned, so the `.data` field won't move
        let this = unsafe { self.get_unchecked_mut() };
        let data_ptr = NonNull::from(&this.data);
        this.data_ptr = Some(data_ptr);
    }

    // Method to get the data via self-reference
    fn get_data_via_ref<'a>(self: Pin<&'a Self>) -> &'a str {
        // Safety: We're sure the data field hasn't moved since we're still pinned
        unsafe { self.data_ptr.unwrap().as_ref() }
    }

    // Method to update data (with caution, keeping the address the same)
    fn update_data(self: Pin<&mut Self>, new_data: &str) {
        // Safety: We're modifying the string in-place, not moving it
        let this = unsafe { self.get_unchecked_mut() };
        this.data.clear();
        this.data.push_str(new_data);
    }
}

// Safety: We only need this if our type isn't already Send and Sync
// unsafe impl Send for SelfReferential {}
// unsafe impl Sync for SelfReferential {}

// A second example showing how to pin data on the heap
fn heap_pinning_example() -> String {
    let mut output = String::new();

    // Create some data on the heap with Box
    let boxed = Box::new(42);
    writeln!(&mut output, "Original boxed value: {}", *boxed).unwrap();

    // Convert the Box into a Pin<Box<T>>
    let mut pinned_boxed = Box::pin(42);
    writeln!(&mut output, "Original pinned value: {}", *pinned_boxed).unwrap();

    // We can still modify the data through Pin if the data type allows mutation
    *pinned_boxed = 100;
    writeln!(&mut output, "Modified pinned value: {}", *pinned_boxed).unwrap();

    // We can't move out of the pinned data
    // This would fail to compile: let value = *pinned_boxed;

    // We can create a reference to the pinned data
    let reference = &*pinned_boxed;
    writeln!(&mut output, "Reference to pinned data: {}", reference).unwrap();

    output
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_crate_test_std_pin_main() {
    println!("=== Std::Pin Example ===");

    // Example 1: Self-referential struct with Pin
    // Create the struct and wrap it in Box::pin
    let mut boxed = Box::pin(SelfReferential::new("Original data".to_string()));

    // Initialize the self-reference
    boxed.as_mut().init();

    // Access data through self-reference
    println!("Data via self-ref: {}", boxed.as_ref().get_data_via_ref());

    // Update the data safely
    boxed.as_mut().update_data("Updated data");
    println!(
        "Updated data via self-ref: {}",
        boxed.as_ref().get_data_via_ref()
    );

    // Example 2: Heap pinning
    let heap_example_output = heap_pinning_example();
    print!("{}", heap_example_output);

    println!("=== End of Pin Example ===");
}
