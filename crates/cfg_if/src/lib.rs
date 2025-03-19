// Copyright (c) 2025 Xiaomi Corporation
// SPDX-License-Identifier: Apache-2.0

//! Example application demonstrating the use of the cfg-if crate
//!
//! The cfg-if crate provides a macro to define configuration-based conditional compilation,
//! which is useful for handling different platforms and feature conditions.

use cfg_if::cfg_if;

#[unsafe(no_mangle)]
pub fn rust_crate_test_cfg_if_main() {
    println!("Testing cfg-if crate functionality");

    // Demonstrate cfg-if by checking target architecture and OS
    cfg_if! {
        if #[cfg(target_arch = "arm")] {
            println!("Running on ARM architecture");
        } else if #[cfg(target_arch = "riscv32")] {
            println!("Running on RISC-V 32-bit architecture");
        } else if #[cfg(target_arch = "riscv64")] {
            println!("Running on RISC-V 64-bit architecture");
        } else {
            println!("Running on unknown architecture");
        }
    }

    // Demonstrate nested cfg-if conditions
    cfg_if! {
        if #[cfg(unix)] {
            println!("This is a Unix-like system");

            cfg_if! {
                if #[cfg(debug_assertions)] {
                    println!("Debug assertions are enabled");
                } else {
                    println!("Debug assertions are not enabled");
                }
            }
        } else {
            println!("This is not a Unix-like system");
        }
    }

    println!("cfg-if demonstration completed");
}
