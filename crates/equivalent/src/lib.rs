// Copyright (c) 2025 Xiaomi Corporation
// SPDX-License-Identifier: Apache-2.0

use equivalent::*;
use std::cmp::Ordering;
use std::println;

pub struct Pair<A, B>(pub A, pub B);

impl<'a, A: ?Sized, B: ?Sized, C, D> Equivalent<(C, D)> for Pair<&'a A, &'a B>
where
    A: Equivalent<C>,
    B: Equivalent<D>,
{
    fn equivalent(&self, key: &(C, D)) -> bool {
        self.0.equivalent(&key.0) && self.1.equivalent(&key.1)
    }
}

impl<'a, A: ?Sized, B: ?Sized, C, D> Comparable<(C, D)> for Pair<&'a A, &'a B>
where
    A: Comparable<C>,
    B: Comparable<D>,
{
    fn compare(&self, key: &(C, D)) -> Ordering {
        match self.0.compare(&key.0) {
            Ordering::Equal => self.1.compare(&key.1),
            not_equal => not_equal,
        }
    }
}

fn run_equivalent_demo() {
    println!("Running equivalent crate demonstration:");

    let key = (String::from("foo"), String::from("bar"));
    let q1 = Pair("foo", "bar");
    let q2 = Pair("boo", "bar");
    let q3 = Pair("foo", "baz");

    println!("Testing Pair equivalence:");
    println!("q1 equivalent to key: {}", q1.equivalent(&key));
    println!("q2 equivalent to key: {}", q2.equivalent(&key));
    println!("q3 equivalent to key: {}", q3.equivalent(&key));

    println!("\nTesting Pair comparison:");
    println!("q1 compare to key: {:?}", q1.compare(&key));
    println!("q2 compare to key: {:?}", q2.compare(&key));
    println!("q3 compare to key: {:?}", q3.compare(&key));

    // Also verify with assertions
    assert!(q1.equivalent(&key));
    assert!(!q2.equivalent(&key));
    assert!(!q3.equivalent(&key));

    assert_eq!(q1.compare(&key), Ordering::Equal);
    assert_eq!(q2.compare(&key), Ordering::Less);
    assert_eq!(q3.compare(&key), Ordering::Greater);

    println!("All assertions passed successfully!");
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_crate_test_equivalent_main() {
    run_equivalent_demo();
}
