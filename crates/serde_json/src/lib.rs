// Copyright (c) 2025 Xiaomi Corporation
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Define test data structures
#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Person {
    name: String,
    age: u32,
    phones: Vec<String>,
    address: Address,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Address {
    street: String,
    city: String,
    zip: String,
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_crates_test_serde_json_main() {
    println!("Running serde_json tests");

    test_basic_serialization();
    test_basic_deserialization();
    test_complex_structures();
    test_json_error_handling();
    test_json_from_str();
    test_json_to_string_pretty();
    test_json_value_manipulation();
    test_json_arbitrary_types();

    println!("All serde_json tests completed");
}

fn test_basic_serialization() {
    println!("Testing basic serialization");

    // Serialize primitive types
    let number = 42;
    let serialized = serde_json::to_string(&number).unwrap();
    assert_eq!(serialized, "42");

    let boolean = true;
    let serialized = serde_json::to_string(&boolean).unwrap();
    assert_eq!(serialized, "true");

    let text = "Hello, serde_json!";
    let serialized = serde_json::to_string(&text).unwrap();
    assert_eq!(serialized, "\"Hello, serde_json!\"");

    println!("Basic serialization tests passed");
}

fn test_basic_deserialization() {
    println!("Testing basic deserialization");

    // Deserialize primitive types
    let json_number = "42";
    let number: i32 = serde_json::from_str(json_number).unwrap();
    assert_eq!(number, 42);

    let json_boolean = "true";
    let boolean: bool = serde_json::from_str(json_boolean).unwrap();
    assert_eq!(boolean, true);

    let json_string = "\"Hello, serde_json!\"";
    let text: String = serde_json::from_str(json_string).unwrap();
    assert_eq!(text, "Hello, serde_json!");

    println!("Basic deserialization tests passed");
}

fn test_complex_structures() {
    println!("Testing complex structures");

    let person = Person {
        name: "John Doe".to_string(),
        age: 30,
        phones: vec!["123-456-7890".to_string(), "987-654-3210".to_string()],
        address: Address {
            street: "123 Main St".to_string(),
            city: "Anytown".to_string(),
            zip: "12345".to_string(),
        },
    };

    // Serialize to JSON
    let serialized = serde_json::to_string(&person).unwrap();
    println!("Serialized: {}", serialized);

    // Deserialize from JSON
    let deserialized: Person = serde_json::from_str(&serialized).unwrap();
    assert_eq!(deserialized, person);

    println!("Complex structures tests passed");
}

fn test_json_error_handling() {
    println!("Testing JSON error handling");

    // Test with invalid JSON
    let invalid_json = "{\"name\": \"John\", age: 30}"; // Missing quotes around field name
    let result: Result<Person, _> = serde_json::from_str(invalid_json);
    assert!(result.is_err());
    println!("Error (expected): {:?}", result.err().unwrap());

    // Test with valid JSON but wrong structure
    let mismatched_json = "{\"name\": 42}";
    let result: Result<Person, _> = serde_json::from_str(mismatched_json);
    assert!(result.is_err());
    println!("Error (expected): {:?}", result.err().unwrap());

    println!("JSON error handling tests passed");
}

fn test_json_from_str() {
    println!("Testing JSON Value from string");

    let json_string = r#"
    {
        "name": "John Doe",
        "age": 30,
        "is_active": true,
        "scores": [88, 92, 95]
    }
    "#;

    // Parse into a generic Value
    let v: serde_json::Value = serde_json::from_str(json_string).unwrap();

    // Access values in the JSON
    assert_eq!(v["name"], "John Doe");
    assert_eq!(v["age"], 30);
    assert_eq!(v["is_active"], true);
    assert_eq!(v["scores"][1], 92);

    println!("JSON Value from string tests passed");
}

fn test_json_to_string_pretty() {
    println!("Testing pretty JSON serialization");

    let data = serde_json::json!({
        "name": "John Doe",
        "age": 30,
        "scores": [88, 92, 95]
    });

    let pretty = serde_json::to_string_pretty(&data).unwrap();
    println!("Pretty JSON:\n{}", pretty);

    // Make sure we can parse it back
    let parsed: serde_json::Value = serde_json::from_str(&pretty).unwrap();
    assert_eq!(parsed["name"], "John Doe");

    println!("Pretty JSON serialization tests passed");
}

fn test_json_value_manipulation() {
    println!("Testing JSON Value manipulation");

    // Create a JSON value using the json! macro
    let mut data = serde_json::json!({
        "name": "John",
        "age": 30,
        "tags": ["developer", "rust"]
    });

    // Modify the JSON
    data["name"] = serde_json::Value::String("John Doe".to_string());
    data["age"] = serde_json::json!(31);
    data["tags"]
        .as_array_mut()
        .unwrap()
        .push(serde_json::json!("serde"));

    // Add a new field
    data["new_field"] = serde_json::json!(true);

    // Verify changes
    assert_eq!(data["name"], "John Doe");
    assert_eq!(data["age"], 31);
    assert_eq!(data["tags"][2], "serde");
    assert_eq!(data["new_field"], true);

    println!("JSON Value manipulation tests passed");
}

fn test_json_arbitrary_types() {
    println!("Testing JSON with arbitrary types");

    // Using HashMap
    let mut map = HashMap::new();
    map.insert("key1".to_string(), 1);
    map.insert("key2".to_string(), 2);
    map.insert("key3".to_string(), 3);

    let serialized = serde_json::to_string(&map).unwrap();
    println!("Serialized HashMap: {}", serialized);

    println!("JSON with arbitrary types tests passed");
}
