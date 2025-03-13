// Copyright (c) 2025 Xiaomi Corporation
// SPDX-License-Identifier: Apache-2.0

use serde::Deserialize;

/// This is what we're going to decode into. Each field is optional, meaning
/// that it doesn't have to be present in TOML.
#[derive(Debug, Deserialize)]
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
#[derive(Debug, Deserialize)]
struct ServerConfig {
    ip: Option<String>,
    port: Option<u64>,
}

#[derive(Debug, Deserialize)]
struct PeerConfig {
    ip: Option<String>,
    port: Option<u64>,
}

#[unsafe(no_mangle)]
pub fn rust_crate_test_toml_main() {
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
