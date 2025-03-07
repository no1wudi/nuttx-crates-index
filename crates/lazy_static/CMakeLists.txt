# Copyright (c) 2025 Xiaomi Corporation
# SPDX-License-Identifier: Apache-2.0

if(CONFIG_RUST_CRATES_LAZY_STATIC)

  nuttx_add_rust(CRATE_NAME lazy_static CRATE_PATH ${CMAKE_CURRENT_SOURCE_DIR})

  nuttx_add_application(
    NAME rust_crates_test_lazy_static STACKSIZE
    ${CONFIG_RUST_CRATES_LAZY_STATIC_STACKSIZE} PRIORITY
    ${CONFIG_RUST_CRATES_LAZY_STATIC_PRIORITY})

endif() # CONFIG_RUST_CRATES_LAZY_STATIC
