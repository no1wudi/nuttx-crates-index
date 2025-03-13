// Copyright (c) 2025 Xiaomi Corporation
// SPDX-License-Identifier: Apache-2.0

use heck::{ToKebabCase, ToLowerCamelCase, ToSnakeCase, ToTitleCase};

#[unsafe(no_mangle)]
pub fn rust_crate_test_heck_main() {
    // Example strings for case conversion
    let test_string = "hello_world_example";
    let pascal_string = "HelloWorldExample";

    println!("Heck crate demonstration - case conversion utilities");
    println!("---------------------------------------------------");

    // Convert to various cases
    println!("Original string: {}", test_string);
    println!("To camelCase: {}", test_string.to_lower_camel_case());
    println!("To TitleCase: {}", test_string.to_title_case());
    println!("To kebab-case: {}", test_string.to_kebab_case());

    println!("\nOriginal PascalCase: {}", pascal_string);
    println!("To snake_case: {}", pascal_string.to_snake_case());

    println!("\nHeck crate example completed successfully");
}
