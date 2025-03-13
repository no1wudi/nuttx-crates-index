# Copyright (c) 2025 Xiaomi Corporation
# SPDX-License-Identifier: Apache-2.0

if(CONFIG_RUST_CRATE_BYTEORDER)
  nuttx_add_rust(
    CRATE_NAME byteorder
    CRATE_PATH ${CMAKE_CURRENT_SOURCE_DIR}
  )

  nuttx_add_application(
    NAME rust_crate_test_byteorder
    STACKSIZE ${CONFIG_RUST_CRATE_BYTEORDER_STACKSIZE}
    PRIORITY ${CONFIG_RUST_CRATE_BYTEORDER_PRIORITY}
  )

  add_dependencies(apps byteorder)
endif() # CONFIG_RUST_CRATE_BYTEORDER
