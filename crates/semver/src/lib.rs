// Copyright (c) 2025 Xiaomi Corporation
// SPDX-License-Identifier: Apache-2.0

use semver::{Version, VersionReq};

#[unsafe(no_mangle)]
pub unsafe fn rust_crate_test_semver_main() {
    // Parse a version string
    let version = Version::parse("1.2.3").unwrap();
    println!("Working with semver version: {}", version);

    // Demonstrate comparing versions
    let version_2 = Version::parse("2.0.0").unwrap();
    if version < version_2 {
        println!("Version {} is less than {}", version, version_2);
    }

    // Demonstrate version requirements
    let req = VersionReq::parse(">=1.0.0, <2.0.0").unwrap();
    println!("Requirement: {}", req);
    println!(
        "Version {} satisfies requirement: {}",
        version,
        req.matches(&version)
    );
    println!(
        "Version {} satisfies requirement: {}",
        version_2,
        req.matches(&version_2)
    );

    // Parse versions with pre-release identifiers and build metadata
    let version_with_pre = Version::parse("1.0.0-alpha.1+build.123").unwrap();
    println!("Version with pre-release and build: {}", version_with_pre);
    println!(
        "Major: {}, Minor: {}, Patch: {}",
        version_with_pre.major, version_with_pre.minor, version_with_pre.patch
    );
    println!("Pre-release: {:?}", version_with_pre.pre);
    println!("Build metadata: {:?}", version_with_pre.build);
}
