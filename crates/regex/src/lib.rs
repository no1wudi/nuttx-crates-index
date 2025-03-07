// Copyright (c) 2025 Xiaomi Corporation
// SPDX-License-Identifier: Apache-2.0

//! Regular expression example for NuttX
//!
//! This module demonstrates the usage of regular expressions in Rust
//! to parse structured text data. It provides several examples of common regex
//! patterns and use cases including parsing structured data, name matching,
//! date validation, and regex sets.

use regex::Regex;
use regex::RegexSet;

/// Demonstrates basic regular expression functionality by parsing structured data.
///
/// # Example Output
/// ```text
/// File: path/to/foo, Line: 54, Content: Blue Harvest
/// File: path/to/bar, Line: 90, Content: Something, Something, Something, Dark Side
/// File: path/to/baz, Line: 3, Content: It's a Trap!
/// ```
///
/// # Implementation Details
/// - Uses a regex pattern to parse colon-separated data
/// - Extracts file paths, line numbers, and content
/// - Converts line numbers to u64
fn test_basic_example() {
    println!("Testing regex examples...");

    // Original example
    let re = Regex::new(r"(?m)^([^:]+):([0-9]+):(.+)$").unwrap();
    let hay = "\
path/to/foo:54:Blue Harvest
path/to/bar:90:Something, Something, Something, Dark Side
path/to/baz:3:It's a Trap!
";

    println!("\nParsing structured data:");
    for (_, [path, lineno, line]) in re.captures_iter(hay).map(|c| c.extract()) {
        if let Ok(num) = lineno.parse::<u64>() {
            println!("File: {}, Line: {}, Content: {}", path, num, line);
        }
    }
}

/// Demonstrates named capture groups by matching a name pattern with a middle initial.
///
/// # Example Output
/// ```text
/// Found middle initial: J
/// ```
///
/// # Implementation Details
/// - Uses named capture groups with `?<name>` syntax
/// - Extracts the middle initial from a full name
fn test_name_matching() {
    println!("\nTesting name matching with middle initial:");
    let re = Regex::new(r"Homer (?<middle>.)\. Simpson").unwrap();
    let hay = "Homer J. Simpson";
    if let Some(caps) = re.captures(hay) {
        println!("Found middle initial: {}", &caps["middle"]);
    }
}

/// Validates date strings using a regular expression pattern.
///
/// # Example Output
/// ```text
/// Is '2010-03-14' a valid date? true
/// Is '2023-99-99' a valid date? true
/// Is 'not-a-date' a valid date? false
/// ```
///
/// # Implementation Details
/// - Checks if strings match the YYYY-MM-DD format
/// - Note: Only validates format, not actual date validity
fn test_date_validation() {
    println!("\nTesting date validation:");
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    let dates = ["2010-03-14", "2023-99-99", "not-a-date"];
    for date in dates {
        println!("Is '{}' a valid date? {}", date, re.is_match(date));
    }
}

/// Extracts dates from text using named capture groups.
///
/// # Example Output
/// ```text
/// Found date: 04/14/1865
/// Found date: 07/02/1881
/// Found date: 09/06/1901
/// ```
///
/// # Implementation Details
/// - Uses named capture groups for year, month, and day
/// - Demonstrates iterating over multiple matches in text
fn test_date_extraction() {
    println!("\nTesting date extraction:");
    let re = Regex::new(r"(?<y>[0-9]{4})-(?<m>[0-9]{2})-(?<d>[0-9]{2})").unwrap();
    let hay = "Important dates: 1865-04-14, 1881-07-02, 1901-09-06";

    for caps in re.captures_iter(hay) {
        println!("Found date: {}/{}/{}", &caps["m"], &caps["d"], &caps["y"]);
    }
}

/// Demonstrates the usage of RegexSet for matching multiple patterns simultaneously.
///
/// # Example Output
/// ```text
/// Patterns matched in 'foobar123': [0, 1, 2, 3, 4]
/// ```
///
/// # Implementation Details
/// - Creates a set of regular expressions
/// - Shows which patterns match in a single string
/// - Useful for checking multiple patterns efficiently
fn test_regex_set() {
    println!("\nTesting regex set matching:");
    let set = RegexSet::new(&[r"\w+", r"\d+", r"foo", r"bar", r"foobar"]).unwrap();

    let test_str = "foobar123";
    let matches: Vec<_> = set.matches(test_str).into_iter().collect();
    println!("Patterns matched in '{}': {:?}", test_str, matches);
}

/// Main entry point for the regex example
///
/// This function is marked as no_mangle to ensure it's callable from C code,
/// allowing integration with the NuttX system.
#[unsafe(no_mangle)]
pub extern "C" fn rust_crates_test_regex_main() {
    println!("Starting regex examples...");

    test_basic_example();
    test_name_matching();
    test_date_validation();
    test_date_extraction();
    test_regex_set();

    println!("\nAll regex tests completed successfully!");
}
