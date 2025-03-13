// Copyright (c) 2025 Xiaomi Corporation
// SPDX-License-Identifier: Apache-2.0

use std::boxed::Box;

// A simple struct to demonstrate box allocation
struct RecursiveStructure {
    value: i32,
    // Self-referential structure using Box
    child: Option<Box<RecursiveStructure>>,
}

impl RecursiveStructure {
    fn new(value: i32) -> Self {
        RecursiveStructure { value, child: None }
    }

    fn add_child(&mut self, value: i32) {
        let mut new_child = Box::new(Self::new(value));

        // If we already have a child, make it the new child's child
        if let Some(existing_child) = self.child.take() {
            new_child.child = Some(existing_child);
        }

        self.child = Some(new_child);
    }

    fn print(&self, depth: usize) {
        let indent = "  ".repeat(depth);
        println!("{}Node value: {}", indent, self.value);

        if let Some(child) = &self.child {
            child.print(depth + 1);
        }
    }
}

// Function to demonstrate Box usage with large stack allocation
fn demonstrate_box_for_large_data() {
    // Allocate large array on heap instead of stack
    let large_array: Box<[u8; 1024]> = Box::new([0; 1024]);

    println!(
        "Large array allocated on heap, first element: {}",
        large_array[0]
    );
    println!(
        "Size of Box<[u8; 1024]>: {} bytes",
        std::mem::size_of_val(&large_array)
    );
}

#[unsafe(no_mangle)]
pub fn rust_crate_test_std_boxed_main() {
    println!("=== Box Demo: Heap allocation in Rust ===");

    // Create a base structure
    let mut root = RecursiveStructure::new(1);

    // Add some children to demonstrate the recursive structure
    root.add_child(2);
    root.add_child(3);
    root.add_child(4);

    println!("Recursive structure contents:");
    root.print(0);

    // Show how Box can be used for large data
    demonstrate_box_for_large_data();

    // Dynamic dispatch example with Box<dyn Trait>
    trait Animal {
        fn make_sound(&self);
    }

    struct Dog;
    impl Animal for Dog {
        fn make_sound(&self) {
            println!("Woof!");
        }
    }

    struct Cat;
    impl Animal for Cat {
        fn make_sound(&self) {
            println!("Meow!");
        }
    }

    // Store different types in the same vector using Box<dyn Animal>
    let animals: Vec<Box<dyn Animal>> = vec![Box::new(Dog), Box::new(Cat)];

    println!("Dynamic dispatch example:");
    for animal in animals {
        animal.make_sound();
    }

    println!("Box demo completed successfully");
}
