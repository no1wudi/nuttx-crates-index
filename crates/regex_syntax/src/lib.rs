// Copyright (c) 2025 Xiaomi Corporation
// SPDX-License-Identifier: Apache-2.0

use regex_syntax::Parser;

#[unsafe(no_mangle)]
pub fn rust_crate_test_regex_syntax_main() {
    println!("Starting regex-syntax demonstration");

    // Create a parser for regex patterns
    let mut parser = Parser::new();

    // Parse some example regex patterns
    match parser.parse(r"[a-z]+") {
        Ok(ast) => {
            println!("Successfully parsed pattern '[a-z]+'");
            println!("AST: {:?}", ast);
        }
        Err(err) => {
            println!("Failed to parse pattern: {}", err);
        }
    }

    // Parse a more complex pattern
    match parser.parse(r"\d{3}-\d{2}-\d{4}") {
        Ok(ast) => {
            println!("Successfully parsed pattern '\\d{{3}}-\\d{{2}}-\\d{{4}}'");
            println!("AST: {:?}", ast);
        }
        Err(err) => {
            println!("Failed to parse pattern: {}", err);
        }
    }

    // Try an invalid pattern
    match parser.parse(r"[a-z") {
        Ok(_) => {
            println!("Unexpectedly parsed invalid pattern");
        }
        Err(err) => {
            println!("Expected error for invalid pattern '[a-z': {}", err);
        }
    }

    println!("regex-syntax demonstration completed");
}
