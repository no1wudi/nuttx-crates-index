# Copyright (c) 2025 Xiaomi Corporation
# SPDX-License-Identifier: Apache-2.0

if(CONFIG_RUST_CRATES_REGEX)

  # Build the Rust crate using nuttx_add_rust
  nuttx_add_rust(CRATE_NAME regex CRATE_PATH ${CMAKE_CURRENT_SOURCE_DIR})

  nuttx_add_application(
    NAME rust_crates_test_regex STACKSIZE
    ${CONFIG_RUST_CRATES_REGEX_STACKSIZE} PRIORITY
    ${CONFIG_RUST_CRATES_REGEX_PRIORITY})

  add_dependencies(apps regex)

endif() # CONFIG_RUST_CRATES_REGEX
