# NuttX Crates Index

A repository for testing and managing Rust crate compatibility list with Apache NuttX RTOS.

## Overview

This project serves as a test collection and compatibility index for Rust crates targeting the NuttX RTOS environment. It helps maintain and validate Rust crates that can be successfully built and integrated into NuttX applications.

## Prerequisites

- NuttX development environment
- CMake and build essentials
- Rust nightly toolchain with specific components:
  - rust-src

## CI/CD Pipeline

The project includes GitHub Actions workflows that:

1. Set up the build environment
2. Install required toolchains
3. Patch Rust std library for compatibility
4. Build and test crates against NuttX

## Project Integration

This repository is designed to work alongside:
- NuttX repository
- NuttX Apps repository (linked as external/)

## License

Copyright (c) 2025 Xiaomi Corporation
SPDX-License-Identifier: Apache-2.0

## Contributing

1. Fork the repository
2. Create a feature branch
3. Submit a pull request

For more information about NuttX, visit [Apache NuttX website](https://nuttx.apache.org/).
