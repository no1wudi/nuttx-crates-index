# Copyright (c) 2025 Xiaomi Corporation
# SPDX-License-Identifier: Apache-2.0

if(CONFIG_RUST_CRATE_STD_PRINTLN)
  nuttx_add_rust(
    CRATE_NAME std_println
    CRATE_PATH ${CMAKE_CURRENT_SOURCE_DIR}
  )

  nuttx_add_application(
    NAME rust_crate_test_std_println
    STACKSIZE ${CONFIG_RUST_CRATE_STD_PRINTLN_STACKSIZE}
    PRIORITY ${CONFIG_RUST_CRATE_STD_PRINTLN_PRIORITY}
  )

  add_dependencies(apps std_println)
endif() # CONFIG_RUST_CRATE_STD_PRINTLN
