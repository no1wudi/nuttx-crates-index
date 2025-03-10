// Copyright (c) 2025 Xiaomi Corporation
// SPDX-License-Identifier: Apache-2.0

//! Regular expression lite example for NuttX
//!
//! This module demonstrates the usage of lightweight regular expressions in Rust
//! for common pattern matching tasks. It provides examples of email validation,
//! phone number formatting, capture groups, and text replacement.

extern crate regex_lite;

use regex_lite::Regex;

/// Demonstrates basic regular expression functionality.
///
/// # Example Output
/// ```text
/// Is 'user@example.com' a valid email? true
/// Is 'invalid.email@' a valid email? false
/// Is 'not-an-email' a valid email? false
/// ```
///
/// # Implementation Details
/// - Validates email formats using pattern matching
/// - Checks phone number formatting
/// - Demonstrates basic capture group functionality
fn test_basic_patterns() {
    println!("Testing basic regex-lite patterns...");

    // Basic email pattern matching
    let email_re = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    let test_emails = ["user@example.com", "invalid.email@", "not-an-email"];

    println!("\nValidating email addresses:");
    for email in test_emails {
        println!("Is '{}' a valid email? {}", email, email_re.is_match(email));
    }

    // Simple phone number pattern
    let phone_re = Regex::new(r"^\d{3}-\d{3}-\d{4}$").unwrap();
    let test_phones = ["123-456-7890", "1234-567-890", "123-456-789"];

    println!("\nValidating phone numbers:");
    for phone in test_phones {
        println!(
            "Is '{}' a valid phone number? {}",
            phone,
            phone_re.is_match(phone)
        );
    }

    // Capturing example
    let capture_re = Regex::new(r"(\w+):(\d+)").unwrap();
    let text = "count:42 items:123";

    println!("\nExtracting key-value pairs:");
    for cap in capture_re.captures_iter(text) {
        println!("Found key: {}, value: {}", &cap[1], &cap[2]);
    }
}

/// Demonstrates text replacement functionality using regex patterns.
///
/// # Example Output
/// ```text
/// Original: The quick brown fox jumps over the lazy dog
/// Censored: The q**** brown fox j**** over the lazy dog
/// ```
///
/// # Implementation Details
/// - Uses regex to find specific words
/// - Replaces matched words with censored versions
fn test_text_replacement() {
    println!("\nTesting text replacement:");

    let text = "The quick brown fox jumps over the lazy dog";
    let re = Regex::new(r"(quick|jumps)").unwrap();

    let censored = re.replace_all(text, |caps: &regex_lite::Captures| {
        let word = &caps[1];
        let first_char = word.chars().next().unwrap();
        let stars = "*".repeat(word.len() - 1);
        format!("{}{}", first_char, stars)
    });

    println!("Original: {}", text);
    println!("Censored: {}", censored);
}

/// Validates URLs using regex patterns.
///
/// # Example Output
/// ```text
/// Is 'https://www.example.com' a valid URL? true
/// Is 'http://example' a valid URL? false
/// Is 'not-a-url' a valid URL? false
/// ```
///
/// # Implementation Details
/// - Checks for proper URL format with protocol and domain
/// - Validates against common URL patterns
fn test_url_validation() {
    println!("\nTesting URL validation:");

    let url_re = Regex::new(r"^(https?|ftp)://[^\s/$.?#].[^\s]*$").unwrap();
    let test_urls = ["https://www.example.com", "http://example", "not-a-url"];

    for url in test_urls {
        println!("Is '{}' a valid URL? {}", url, url_re.is_match(url));
    }
}

/// Main entry point for the regex-lite example
///
/// This function is marked as no_mangle to ensure it's callable from C code,
/// allowing integration with the NuttX system.
#[unsafe(no_mangle)]
pub extern "C" fn rust_crate_test_regex_lite_main() {
    println!("Starting regex-lite examples...");

    test_basic_patterns();
    test_text_replacement();
    test_url_validation();

    println!("\nAll regex-lite tests completed successfully!");
}
