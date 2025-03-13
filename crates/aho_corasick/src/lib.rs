// Copyright (c) 2025 Xiaomi Corporation
// SPDX-License-Identifier: Apache-2.0

#[unsafe(no_mangle)]
pub fn rust_crate_test_aho_corasick_main() {
    println!("Aho-Corasick Pattern Matching Examples");

    // Example 1: Basic pattern matching
    example_basic_matching();

    // Example 2: Case insensitive matching
    example_case_insensitive();

    // Example 3: Stream replacement
    example_stream_replace();

    // Example 4: Default leftmost-longest matching
    example_leftmost_longest();

    // Example 5: Leftmost-first matching
    example_leftmost_first();
}

fn example_basic_matching() {
    println!("\n=== Basic Pattern Matching ===");

    let patterns = &["apple", "maple", "Snapple"];
    let haystack = "Nobody likes maple in their apple flavored Snapple.";

    let ac = aho_corasick::AhoCorasick::new(patterns).unwrap();
    let mut matches = vec![];
    for mat in ac.find_iter(haystack) {
        matches.push((mat.pattern(), mat.start(), mat.end()));
        println!(
            "Found pattern '{}' at positions {}-{}",
            patterns[mat.pattern().as_usize()],
            mat.start(),
            mat.end()
        );
    }

    assert_eq!(
        matches,
        vec![
            (aho_corasick::PatternID::must(1), 13, 18),
            (aho_corasick::PatternID::must(0), 28, 33),
            (aho_corasick::PatternID::must(2), 43, 50),
        ]
    );
}

fn example_case_insensitive() {
    println!("\n=== Case Insensitive Matching ===");

    let patterns = &["apple", "maple", "snapple"];
    let haystack = "Nobody likes maple in their apple flavored Snapple.";

    let ac = aho_corasick::AhoCorasick::builder()
        .ascii_case_insensitive(true)
        .build(patterns)
        .unwrap();

    let mut matches = vec![];
    for mat in ac.find_iter(haystack) {
        matches.push((mat.pattern(), mat.start(), mat.end()));
        println!(
            "Found case-insensitive pattern '{}' at positions {}-{}",
            patterns[mat.pattern().as_usize()],
            mat.start(),
            mat.end()
        );
    }

    assert_eq!(
        matches,
        vec![
            (aho_corasick::PatternID::must(1), 13, 18),
            (aho_corasick::PatternID::must(0), 28, 33),
            (aho_corasick::PatternID::must(2), 43, 50),
        ]
    );
}

fn example_stream_replace() {
    println!("\n=== Stream Replacement ===");

    let patterns = &["fox", "brown", "quick"];
    let replace_with = &["sloth", "grey", "slow"];

    let rdr = "The quick brown fox.";
    let mut wtr = vec![];

    let ac = aho_corasick::AhoCorasick::new(patterns).unwrap();
    ac.try_stream_replace_all(rdr.as_bytes(), &mut wtr, replace_with)
        .expect("stream_replace_all failed");

    let result = String::from_utf8(wtr).unwrap();
    println!("Original: \"{}\"", rdr);
    println!("Replaced: \"{}\"", result);

    assert_eq!("The slow grey sloth.", result);
}

fn example_leftmost_longest() {
    println!("\n=== Leftmost-Longest Matching (Default) ===");

    let patterns = &["Samwise", "Sam"];
    let haystack = "Samwise";

    let ac = aho_corasick::AhoCorasick::new(patterns).unwrap();
    let mat = ac.find(haystack).expect("should have a match");

    println!("Pattern found: \"{}\"", &haystack[mat.start()..mat.end()]);
    assert_eq!("Samwise", &haystack[mat.start()..mat.end()]);
}

fn example_leftmost_first() {
    println!("\n=== Leftmost-First Matching ===");

    let patterns = &["Samwise", "Sam"];
    let haystack = "Samwise";

    let ac = aho_corasick::AhoCorasick::builder()
        .match_kind(aho_corasick::MatchKind::LeftmostFirst)
        .build(patterns)
        .unwrap();

    let mat = ac.find(haystack).expect("should have a match");

    println!("Pattern found: \"{}\"", &haystack[mat.start()..mat.end()]);
    assert_eq!("Samwise", &haystack[mat.start()..mat.end()]);
}
