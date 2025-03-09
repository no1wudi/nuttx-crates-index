# Copyright (c) 2025 Xiaomi Corporation
# SPDX-License-Identifier: Apache-2.0

if(CONFIG_RUST_CRATE_CORE_PRINTLN)
  nuttx_add_rust(CRATE_NAME core_println CRATE_PATH ${CMAKE_CURRENT_SOURCE_DIR})
  nuttx_add_application(
    NAME rust_crate_test_core_println
    STACKSIZE ${CONFIG_RUST_CRATE_CORE_PRINTLN_STACKSIZE}
    PRIORITY ${CONFIG_RUST_CRATE_CORE_PRINTLN_PRIORITY})
  add_dependencies(apps core_println)
endif()
