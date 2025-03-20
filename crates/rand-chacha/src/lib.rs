// Copyright (c) 2025 Xiaomi Corporation
// SPDX-License-Identifier: Apache-2.0

use rand_chacha::ChaCha12Rng;
use rand_chacha::rand_core::RngCore;
use rand_chacha::rand_core::SeedableRng;

#[unsafe(no_mangle)]
pub fn rust_crate_test_rand_chacha_main() {
    let mut rng = ChaCha12Rng::from_seed(Default::default());
    let x = rng.next_u64();
    assert_eq!(x, 0x53f955076a9af49b);
}
