// Copyright (c) 2025 Xiaomi Corporation
// SPDX-License-Identifier: Apache-2.0

use byteorder::{BigEndian, ReadBytesExt};
use std::io::Cursor;

#[unsafe(no_mangle)]
pub fn rust_crate_test_byteorder_main() {
    println!("Starting byteorder example");

    // Create a cursor over our data
    let mut rdr = Cursor::new(vec![2, 5, 3, 0]);

    // Read using big endian byte order
    match rdr.read_u16::<BigEndian>() {
        Ok(value) => {
            println!("First u16 (BigEndian): {}", value);
            assert_eq!(517, value);
        }
        Err(e) => println!("Error reading first u16: {:?}", e),
    }

    match rdr.read_u16::<BigEndian>() {
        Ok(value) => {
            println!("Second u16 (BigEndian): {}", value);
            assert_eq!(768, value);
        }
        Err(e) => println!("Error reading second u16: {:?}", e),
    }

    println!("Byteorder example completed successfully!");
}
