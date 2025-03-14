// Copyright (c) 2025 Xiaomi Corporation
// SPDX-License-Identifier: Apache-2.0

use thiserror::Error;

// Define custom error types using thiserror
#[derive(Error, Debug)]
enum DataError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Data parsing failed at line {line}: {source}")]
    Parse {
        line: usize,
        #[source]
        source: ParseError,
    },

    #[error("Missing required field: {0}")]
    MissingField(String),

    #[error("Value out of range: {0}")]
    OutOfRange(i32),
}

// Another custom error for demonstration
#[derive(Error, Debug)]
#[error("Failed to parse data: {message}")]
struct ParseError {
    message: String,
}

// Function that returns our custom error
fn process_data(value: i32) -> Result<(), DataError> {
    if value < 0 {
        return Err(DataError::OutOfRange(value));
    }

    if value > 100 {
        let parse_error = ParseError {
            message: "Value exceeds maximum".to_string(),
        };
        return Err(DataError::Parse {
            line: 42,
            source: parse_error,
        });
    }

    // Simulating a missing field error
    if value == 0 {
        return Err(DataError::MissingField("value".to_string()));
    }

    println!("Successfully processed data: {}", value);
    Ok(())
}

// Function to demonstrate displaying errors
fn demonstrate_errors() {
    let test_values = [0, -10, 150, 50];

    for &value in &test_values {
        match process_data(value) {
            Ok(_) => {
                println!("Processing successful");
            }
            Err(e) => {
                println!("Error: {}", e);

                // Demonstrate source error display
                if let Some(source) = e.source() {
                    println!("Caused by: {}", source);
                }
            }
        }
    }
}

#[unsafe(no_mangle)]
pub fn rust_crate_test_thiserror_main() {
    println!("Starting thiserror demonstration...");
    demonstrate_errors();
    println!("Thiserror demonstration completed");
}

// Extension trait to get the source of an error
trait ErrorSource {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)>;
}

impl ErrorSource for DataError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            DataError::Io(err) => Some(err),
            DataError::Parse { source, .. } => Some(source),
            _ => None,
        }
    }
}
