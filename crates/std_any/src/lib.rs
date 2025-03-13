// Copyright (c) 2025 Xiaomi Corporation
// SPDX-License-Identifier: Apache-2.0

use std::any::{Any, TypeId};
use std::fmt::Debug;

// A trait object demo to show using with Any
trait Animal: Any + Debug {
    fn make_sound(&self) -> &'static str;

    // Allow downcasting from trait object
    fn as_any(&self) -> &dyn Any;
}

#[derive(Debug)]
struct Cat {
    name: String,
}

impl Animal for Cat {
    fn make_sound(&self) -> &'static str {
        "Meow!"
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug)]
struct Dog {
    name: String,
}

impl Animal for Dog {
    fn make_sound(&self) -> &'static str {
        "Woof!"
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

fn print_type_info<T: Any + Debug>(value: &T) {
    println!("Type ID: {:?}", TypeId::of::<T>());
    println!("Type name: {}", std::any::type_name::<T>());
    println!("Value: {:?}", value);
}

fn downcast_animal(animal: &dyn Animal) {
    println!("Animal says: {}", animal.make_sound());

    // Try to downcast to specific type
    if let Some(cat) = animal.as_any().downcast_ref::<Cat>() {
        println!("Downcast to Cat successful: {} says Meow!", cat.name);
    } else if let Some(dog) = animal.as_any().downcast_ref::<Dog>() {
        println!("Downcast to Dog successful: {} says Woof!", dog.name);
    } else {
        println!("Unknown animal type");
    }
}

#[unsafe(no_mangle)]
pub fn rust_crate_test_std_any_main() {
    println!("===== std::any demonstration =====");

    // Basic TypeId examples
    println!("\n--- TypeId examples ---");
    println!("i32 TypeId: {:?}", TypeId::of::<i32>());
    println!("&str TypeId: {:?}", TypeId::of::<&str>());
    println!("String TypeId: {:?}", TypeId::of::<String>());

    // Type information using Any
    println!("\n--- Type information examples ---");
    let number = 42;
    let text = "Hello, Any!";
    let string = String::from("This is a String");

    print_type_info(&number);
    print_type_info(&text);
    print_type_info(&string);

    // Any for downcasting
    println!("\n--- Any for downcasting examples ---");
    let value: Box<dyn Any> = Box::new(42i32);

    if let Some(int_value) = value.downcast_ref::<i32>() {
        println!("Downcast successful: {}", int_value);
    } else {
        println!("Downcast failed");
    }

    // Any with trait objects
    println!("\n--- Any with trait objects examples ---");
    let cat = Cat {
        name: String::from("Whiskers"),
    };
    let dog = Dog {
        name: String::from("Rex"),
    };

    let animals: Vec<Box<dyn Animal>> = vec![Box::new(cat), Box::new(dog)];

    for animal in &animals {
        downcast_animal(&**animal);
    }

    println!("\n===== End of std::any demonstration =====");
}
