// Copyright (c) 2025 Xiaomi Corporation
// SPDX-License-Identifier: Apache-2.0

use zerocopy::{Immutable, IntoBytes};

#[repr(u32)]
#[derive(Debug, Default, Immutable, IntoBytes)]
enum RequestType {
    #[default]
    In = 0,
    Out = 1,
    Flush = 4,
}

#[repr(C)]
#[derive(Debug, Default, Immutable, IntoBytes)]
struct VirtioBlockRequest {
    request_type: RequestType,
    reserved: u32,
    sector: u64,
}

#[unsafe(no_mangle)]
pub fn rust_crate_test_zerocopy_main() {
    println!("Testing zerocopy crate functionality...");

    // Test RequestType::Flush
    let flush_request = VirtioBlockRequest {
        request_type: RequestType::Flush,
        sector: 42,
        ..Default::default()
    };

    // Convert the struct to bytes and verify
    assert_eq!(
        flush_request.as_bytes(),
        &[4, 0, 0, 0, 0, 0, 0, 0, 42, 0, 0, 0, 0, 0, 0, 0]
    );

    // Test RequestType::In (read request)
    let in_request = VirtioBlockRequest {
        request_type: RequestType::In,
        sector: 100,
        ..Default::default()
    };

    assert_eq!(
        in_request.as_bytes(),
        &[0, 0, 0, 0, 0, 0, 0, 0, 100, 0, 0, 0, 0, 0, 0, 0]
    );

    // Test RequestType::Out (write request)
    let out_request = VirtioBlockRequest {
        request_type: RequestType::Out,
        sector: 200,
        ..Default::default()
    };

    assert_eq!(
        out_request.as_bytes(),
        &[1, 0, 0, 0, 0, 0, 0, 0, 200, 0, 0, 0, 0, 0, 0, 0]
    );

    println!("All zerocopy tests passed!");
}
