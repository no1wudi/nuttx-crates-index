// Copyright (c) 2025 Xiaomi Corporation
// SPDX-License-Identifier: Apache-2.0

use serde::Deserialize;
use serde::Serialize;

/// This is what we're going to decode into. Each field is optional, meaning
/// that it doesn't have to be present in TOML.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
struct Config {
    global_string: Option<String>,
    global_integer: Option<u64>,
    server: Option<ServerConfig>,
    peers: Option<Vec<PeerConfig>>,
}

/// Sub-structs are decoded from tables, so this will decode from the `[server]`
/// table.
///
/// Again, each field is optional, meaning they don't have to be present.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
struct ServerConfig {
    ip: Option<String>,
    port: Option<u64>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
struct PeerConfig {
    ip: Option<String>,
    port: Option<u64>,
}

fn test_toml_decode() {
    let toml_str = r#"
        global_string = "test"
        global_integer = 5

        [server]
        ip = "127.0.0.1"
        port = 80

        [[peers]]
        ip = "127.0.0.1"
        port = 8080

        [[peers]]
        ip = "127.0.0.1"
    "#;

    match toml::from_str::<Config>(toml_str) {
        Ok(decoded) => {
            println!("TOML config parsed successfully:");

            if let Some(global_str) = &decoded.global_string {
                println!("Global string: {}", global_str);
            }

            if let Some(global_int) = decoded.global_integer {
                println!("Global integer: {}", global_int);
            }

            if let Some(server) = &decoded.server {
                println!("Server: ip={:?}, port={:?}", server.ip, server.port);
            }

            if let Some(peers) = &decoded.peers {
                println!("Found {} peers:", peers.len());
                for (i, peer) in peers.iter().enumerate() {
                    println!("  Peer {}: ip={:?}, port={:?}", i + 1, peer.ip, peer.port);
                }
            }
        }
        Err(e) => {
            println!("Failed to parse TOML: {}", e);
        }
    }

    println!("TOML parsing example completed!");
}

fn test_toml_encode_decode() {
    let config = Config {
        global_string: Some("encoded".to_string()),
        global_integer: Some(42),
        server: Some(ServerConfig {
            ip: Some("10.0.0.1".to_string()),
            port: Some(8081),
        }),
        peers: Some(vec![
            PeerConfig {
                ip: Some("10.0.0.2".to_string()),
                port: Some(9000),
            },
            PeerConfig {
                ip: Some("10.0.0.3".to_string()),
                port: None,
            },
        ]),
    };

    // Encode struct to TOML string
    let toml_str = match toml::to_string(&config) {
        Ok(s) => s,
        Err(e) => {
            println!("Failed to encode TOML: {}", e);
            return;
        }
    };
    println!("Encoded TOML string:\n{}", toml_str);

    // Decode TOML string back to struct
    match toml::from_str::<Config>(&toml_str) {
        Ok(decoded) => {
            println!("Decoded struct from TOML:");
            println!("{:?}", decoded);
            // Verify that the decoded struct matches the original
            if decoded == config {
                println!("Verification passed: Decoded struct matches the original.");
            } else {
                println!("Verification failed: Decoded struct does not match the original.");
            }
        }
        Err(e) => {
            println!("Failed to decode TOML: {}", e);
        }
    }
}

#[unsafe(no_mangle)]
pub fn rust_crate_test_toml_main() {
    test_toml_decode();
    test_toml_encode_decode();
}
