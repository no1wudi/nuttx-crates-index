# Copyright (c) 2025 Xiaomi Corporation
# SPDX-License-Identifier: Apache-2.0

if(CONFIG_RUST_CRATE_STD_PRINTLN_MINIMAL)
  nuttx_add_rust(
    CRATE_NAME std_println_minimal
    CRATE_PATH ${CMAKE_CURRENT_SOURCE_DIR}
  )

  nuttx_add_application(
    NAME rust_crate_test_std_println_minimal
    STACKSIZE ${CONFIG_RUST_CRATE_STD_PRINTLN_MINIMAL_STACKSIZE}
    PRIORITY ${CONFIG_RUST_CRATE_STD_PRINTLN_MINIMAL_PRIORITY}
  )

  add_dependencies(apps std_println_minimal)
endif() # CONFIG_RUST_CRATE_STD_PRINTLN_MINIMAL
