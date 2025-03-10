# Copyright (c) 2025 Xiaomi Corporation
# SPDX-License-Identifier: Apache-2.0

if(CONFIG_RUST_CRATE_STD_BORROW)
  nuttx_add_rust(
    CRATE_NAME std_borrow
    CRATE_PATH ${CMAKE_CURRENT_SOURCE_DIR}
  )

  nuttx_add_application(
    NAME rust_crate_test_std_borrow
    STACKSIZE ${CONFIG_RUST_CRATE_STD_BORROW_STACKSIZE}
    PRIORITY ${CONFIG_RUST_CRATE_STD_BORROW_PRIORITY}
  )

  add_dependencies(apps std_borrow)
endif() # CONFIG_RUST_CRATE_STD_BORROW
