# Copyright (c) 2025 Xiaomi Corporation
# SPDX-License-Identifier: Apache-2.0

if(CONFIG_RUST_CRATE_REGEX_LITE)

  nuttx_add_rust(CRATE_NAME regex_lite CRATE_PATH ${CMAKE_CURRENT_SOURCE_DIR})

  nuttx_add_application(
    NAME rust_crate_test_regex_lite STACKSIZE
    ${CONFIG_RUST_CRATE_REGEX_LITE_STACKSIZE} PRIORITY
    ${CONFIG_RUST_CRATE_REGEX_LITE_PRIORITY})

  add_dependencies(apps regex_lite)

endif() # CONFIG_RUST_CRATE_REGEX_LITE
