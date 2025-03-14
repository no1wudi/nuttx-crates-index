// Copyright (c) 2025 Xiaomi Corporation
// SPDX-License-Identifier: Apache-2.0

//! Example application demonstrating the strsim crate's string similarity functions

use strsim::{
    damerau_levenshtein, hamming, jaro, jaro_winkler, levenshtein, normalized_damerau_levenshtein,
    normalized_levenshtein, osa_distance, sorensen_dice,
};

/// Test Levenshtein distance between strings
fn test_levenshtein() {
    let s1 = "nuttx";
    let s2 = "nutttx";
    let s3 = "nuttxos";

    // Levenshtein distance (number of insertions, deletions or substitutions)
    println!(
        "Levenshtein distance between '{}' and '{}': {}",
        s1,
        s2,
        levenshtein(s1, s2)
    );

    println!(
        "Levenshtein distance between '{}' and '{}': {}",
        s1,
        s3,
        levenshtein(s1, s3)
    );
}

/// Test Normalized Levenshtein similarity
fn test_normalized_levenshtein() {
    let s1 = "nuttx";
    let s2 = "nutttx";

    println!(
        "Normalized Levenshtein similarity between '{}' and '{}': {:.3}",
        s1,
        s2,
        normalized_levenshtein(s1, s2)
    );
}

/// Test Jaro-Winkler similarity
fn test_jaro_winkler() {
    let s1 = "nuttx";
    let s2 = "nutttx";

    println!(
        "Jaro-Winkler similarity between '{}' and '{}': {:.3}",
        s1,
        s2,
        jaro_winkler(s1, s2)
    );
}

/// Test Sørensen-Dice coefficient
fn test_sorensen_dice() {
    let s1 = "nuttx";
    let s2 = "nutttx";

    println!(
        "Sorensen-Dice coefficient between '{}' and '{}': {:.3}",
        s1,
        s2,
        sorensen_dice(s1, s2)
    );
}

/// Test Hamming distance and Damerau-Levenshtein distance
fn test_hamming_and_damerau() {
    let a = "karolin";
    let b = "kathrin";

    // Hamming distance (only works on equal-length strings)
    match hamming(a, b) {
        Ok(distance) => println!("Hamming distance between '{}' and '{}': {}", a, b, distance),
        Err(_) => println!("Cannot compute Hamming distance - strings must be same length"),
    }

    // Damerau-Levenshtein distance (considers transpositions)
    println!(
        "Damerau-Levenshtein distance between '{}' and '{}': {}",
        a,
        b,
        damerau_levenshtein(a, b)
    );
}

/// Test additional string similarity metrics
fn test_additional_similarity_metrics() {
    println!("Testing additional string similarity metrics");

    // Test hamming with same length strings
    match hamming("hamming", "hammers") {
        Ok(distance) => println!(
            "Hamming distance between 'hamming' and 'hammers': {}",
            distance
        ),
        Err(_) => println!("Failed to compute Hamming distance"),
    }

    // Levenshtein
    println!(
        "Levenshtein distance between 'kitten' and 'sitting': {}",
        levenshtein("kitten", "sitting")
    );

    // Normalized Levenshtein
    println!(
        "Normalized Levenshtein between 'kitten' and 'sitting': {:.3}",
        normalized_levenshtein("kitten", "sitting")
    );

    // OSA distance
    println!(
        "OSA distance between 'ac' and 'cba': {}",
        osa_distance("ac", "cba")
    );

    // Damerau-Levenshtein
    println!(
        "Damerau-Levenshtein distance between 'ac' and 'cba': {}",
        damerau_levenshtein("ac", "cba")
    );

    // Normalized Damerau-Levenshtein
    println!(
        "Normalized Damerau-Levenshtein between 'levenshtein' and 'löwenbräu': {:.3}",
        normalized_damerau_levenshtein("levenshtein", "löwenbräu")
    );

    // Jaro similarity
    println!(
        "Jaro similarity between 'Friedrich Nietzsche' and 'Jean-Paul Sartre': {:.3}",
        jaro("Friedrich Nietzsche", "Jean-Paul Sartre")
    );

    // Jaro-Winkler
    println!(
        "Jaro-Winkler similarity between 'cheeseburger' and 'cheese fries': {:.3}",
        jaro_winkler("cheeseburger", "cheese fries")
    );

    // Sorensen-Dice coefficient
    println!(
        "Sorensen-Dice coefficient between 'web applications' and 'applications of the web': {:.3}",
        sorensen_dice("web applications", "applications of the web")
    );
}

#[unsafe(no_mangle)]
pub fn rust_crate_test_strsim_main() {
    println!("Starting strsim string similarity demonstration");

    test_levenshtein();
    test_normalized_levenshtein();
    test_jaro_winkler();
    test_sorensen_dice();
    test_hamming_and_damerau();
    test_additional_similarity_metrics();

    println!("Strsim demonstration completed");
}
