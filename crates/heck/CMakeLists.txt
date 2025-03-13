# Copyright (c) 2025 Xiaomi Corporation
# SPDX-License-Identifier: Apache-2.0

if(CONFIG_RUST_CRATE_HECK)
  nuttx_add_rust(
    CRATE_NAME heck
    CRATE_PATH ${CMAKE_CURRENT_SOURCE_DIR}
  )

  nuttx_add_application(
    NAME rust_crate_test_heck
    STACKSIZE ${CONFIG_RUST_CRATE_HECK_STACKSIZE}
    PRIORITY ${CONFIG_RUST_CRATE_HECK_PRIORITY}
  )

  add_dependencies(apps heck)
endif() # CONFIG_RUST_CRATE_HECK
