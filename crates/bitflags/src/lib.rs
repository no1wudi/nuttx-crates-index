// Copyright (c) 2025 Xiaomi Corporation
// SPDX-License-Identifier: Apache-2.0

use bitflags::bitflags;

// Define a set of file permissions using bitflags
bitflags! {
    #[derive(Debug, Copy, Clone)]
    struct Permissions: u32 {
        const READ = 0b00000100;
        const WRITE = 0b00000010;
        const EXECUTE = 0b00000001;
        const ALL = Self::READ.bits() | Self::WRITE.bits() | Self::EXECUTE.bits();
    }
}

// Define a set of network flags using bitflags
bitflags! {
    #[derive(Debug, Copy, Clone)]
    struct NetworkFlags: u8 {
        const TCP = 0b0001;
        const UDP = 0b0010;
        const IPV4 = 0b0100;
        const IPV6 = 0b1000;
    }
}

#[unsafe(no_mangle)]
pub unsafe fn rust_crate_test_bitflags_main() {
    println!("Starting bitflags crate demonstration...");

    // Demonstrate file permissions
    println!("\nFile Permissions Demonstration:");
    let mut perms = Permissions::READ | Permissions::WRITE;
    println!("Initial permissions: {:?}", perms);
    println!("Can read: {}", perms.contains(Permissions::READ));
    println!("Can write: {}", perms.contains(Permissions::WRITE));
    println!("Can execute: {}", perms.contains(Permissions::EXECUTE));

    // Add execute permission
    perms.insert(Permissions::EXECUTE);
    println!("After adding execute: {:?}", perms);
    println!("Has all permissions: {}", perms.contains(Permissions::ALL));

    // Remove write permission
    perms.remove(Permissions::WRITE);
    println!("After removing write: {:?}", perms);

    // Toggle permissions
    perms.toggle(Permissions::READ);
    println!("After toggling read: {:?}", perms);

    // Empty and is_empty
    let empty = Permissions::empty();
    println!(
        "Empty permissions: {:?}, is empty: {}",
        empty,
        empty.is_empty()
    );

    // Network flags demonstration
    println!("\nNetwork Flags Demonstration:");
    let tcp_ipv4 = NetworkFlags::TCP | NetworkFlags::IPV4;
    println!("TCP over IPv4: {:?}", tcp_ipv4);

    let udp_ipv6 = NetworkFlags::UDP | NetworkFlags::IPV6;
    println!("UDP over IPv6: {:?}", udp_ipv6);

    // Intersection
    let common = tcp_ipv4 & udp_ipv6;
    println!("Common flags between TCP/IPv4 and UDP/IPv6: {:?}", common);

    // Union
    let all_protocols = tcp_ipv4 | udp_ipv6;
    println!("All protocols combined: {:?}", all_protocols);

    println!("\nBitflags demonstration completed!");
}
