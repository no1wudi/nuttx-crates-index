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

    demonstrate_file_permissions();
    demonstrate_network_flags();

    println!("\nBitflags demonstration completed!");
}

fn demonstrate_file_permissions() {
    println!("\nFile Permissions Demonstration:");
    let mut perms = Permissions::READ | Permissions::WRITE;
    println!("Initial permissions: {:?}", perms);
    println!("Can read: {}", perms.contains(Permissions::READ));
    println!("Can write: {}", perms.contains(Permissions::WRITE));
    println!("Can execute: {}", perms.contains(Permissions::EXECUTE));

    // Assert initial permissions
    assert!(
        perms.contains(Permissions::READ),
        "Should have READ permission"
    );
    assert!(
        perms.contains(Permissions::WRITE),
        "Should have WRITE permission"
    );
    assert!(
        !perms.contains(Permissions::EXECUTE),
        "Should not have EXECUTE permission"
    );

    // Add execute permission
    perms.insert(Permissions::EXECUTE);
    println!("After adding execute: {:?}", perms);
    println!("Has all permissions: {}", perms.contains(Permissions::ALL));

    // Assert after adding execute
    assert!(
        perms.contains(Permissions::ALL),
        "Should have ALL permissions after inserting EXECUTE"
    );

    // Remove write permission
    perms.remove(Permissions::WRITE);
    println!("After removing write: {:?}", perms);

    // Assert after removing write
    assert!(
        !perms.contains(Permissions::WRITE),
        "Should not have WRITE permission after removing it"
    );
    assert!(
        perms.contains(Permissions::READ | Permissions::EXECUTE),
        "Should still have READ and EXECUTE permissions"
    );

    // Toggle permissions
    perms.toggle(Permissions::READ);
    println!("After toggling read: {:?}", perms);

    // Assert after toggling read
    assert!(
        !perms.contains(Permissions::READ),
        "Should not have READ permission after toggling it"
    );
    assert!(
        perms.contains(Permissions::EXECUTE),
        "Should still have EXECUTE permission"
    );

    // Empty and is_empty
    let empty = Permissions::empty();
    println!(
        "Empty permissions: {:?}, is empty: {}",
        empty,
        empty.is_empty()
    );

    // Assert empty permissions
    assert!(empty.is_empty(), "Empty permissions should be empty");
    assert_eq!(
        empty.bits(),
        0,
        "Empty permissions should have zero bits set"
    );
}

fn demonstrate_network_flags() {
    println!("\nNetwork Flags Demonstration:");
    let tcp_ipv4 = NetworkFlags::TCP | NetworkFlags::IPV4;
    println!("TCP over IPv4: {:?}", tcp_ipv4);

    // Assert TCP over IPv4
    assert!(
        tcp_ipv4.contains(NetworkFlags::TCP),
        "Should contain TCP flag"
    );
    assert!(
        tcp_ipv4.contains(NetworkFlags::IPV4),
        "Should contain IPV4 flag"
    );
    assert!(
        !tcp_ipv4.contains(NetworkFlags::UDP),
        "Should not contain UDP flag"
    );
    assert!(
        !tcp_ipv4.contains(NetworkFlags::IPV6),
        "Should not contain IPV6 flag"
    );

    let udp_ipv6 = NetworkFlags::UDP | NetworkFlags::IPV6;
    println!("UDP over IPv6: {:?}", udp_ipv6);

    // Assert UDP over IPv6
    assert!(
        udp_ipv6.contains(NetworkFlags::UDP),
        "Should contain UDP flag"
    );
    assert!(
        udp_ipv6.contains(NetworkFlags::IPV6),
        "Should contain IPV6 flag"
    );

    // Intersection
    let common = tcp_ipv4 & udp_ipv6;
    println!("Common flags between TCP/IPv4 and UDP/IPv6: {:?}", common);

    // Assert intersection
    assert!(
        common.is_empty(),
        "There should be no common flags between TCP/IPv4 and UDP/IPv6"
    );

    // Union
    let all_protocols = tcp_ipv4 | udp_ipv6;
    println!("All protocols combined: {:?}", all_protocols);

    // Assert union
    assert!(
        all_protocols.contains(
            NetworkFlags::TCP | NetworkFlags::UDP | NetworkFlags::IPV4 | NetworkFlags::IPV6
        ),
        "All protocols should contain all flags"
    );
    assert_eq!(
        all_protocols.bits(),
        0b1111,
        "All protocols should have all bits set"
    );
}
