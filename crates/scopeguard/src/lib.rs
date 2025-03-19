// Copyright (c) 2025 Xiaomi Corporation
// SPDX-License-Identifier: Apache-2.0

#[macro_use(defer)]
extern crate scopeguard;

use scopeguard::guard;

fn f() {
    defer! {
        println!("Called at return or panic");
    }
}

fn g() {
    let mut data = vec![];
    let mut file = guard(&mut data, |data| {
        // write data at return or panic
        println!("Data: {:?}", data);
    });
    // access the data through the scope guard itself
    file.extend_from_slice(b"test me\n");
}

#[unsafe(no_mangle)]
pub fn rust_crate_test_scopeguard_main() {
    f();
    g();
}
