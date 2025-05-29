// Copyright (c) 2025 Xiaomi Corporation
// SPDX-License-Identifier: Apache-2.0

use prost::Message;

#[derive(Clone, PartialEq, Message)]
pub struct Person {
    #[prost(string, tag = "1")]
    pub name: String,
    #[prost(int32, tag = "2")]
    pub id: i32,
    #[prost(string, optional, tag = "3")]
    pub email: Option<String>,
    #[prost(string, repeated, tag = "4")]
    pub phones: Vec<String>,
}

#[derive(Clone, PartialEq, Message)]
pub struct AddressBook {
    #[prost(message, repeated, tag = "1")]
    pub people: Vec<Person>,
}

#[unsafe(no_mangle)]
pub fn rust_crate_test_prost_main() {
    println!("Prost (Protocol Buffers) Demo");

    // Create a sample person
    let person = Person {
        name: "John Doe".to_string(),
        id: 1234,
        email: Some("john.doe@example.com".to_string()),
        phones: vec!["555-1234".to_string(), "555-5678".to_string()],
    };

    // Create an address book with the person
    let address_book = AddressBook {
        people: vec![person.clone()],
    };

    // Serialize to protobuf bytes
    let encoded = address_book.encode_to_vec();
    println!("Encoded {} bytes: {:?}", encoded.len(), encoded);

    // Deserialize from protobuf bytes
    match AddressBook::decode(&encoded[..]) {
        Ok(decoded) => {
            println!("Successfully decoded address book!");
            if let Some(first_person) = decoded.people.first() {
                println!("First person: {} (ID: {})", first_person.name, first_person.id);
                if let Some(email) = &first_person.email {
                    println!("Email: {}", email);
                }
                println!("Phones: {:?}", first_person.phones);
            }
        }
        Err(e) => {
            println!("Failed to decode: {}", e);
        }
    }

    // Test round-trip encoding/decoding
    let re_encoded = match AddressBook::decode(&encoded[..]) {
        Ok(decoded) => decoded.encode_to_vec(),
        Err(_) => Vec::new(),
    };

    if encoded == re_encoded {
        println!("Round-trip encoding/decoding successful!");
    } else {
        println!("Round-trip encoding/decoding failed!");
    }

    println!("Prost demo completed successfully!");
}
