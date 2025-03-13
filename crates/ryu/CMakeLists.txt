# Copyright (c) 2025 Xiaomi Corporation
# SPDX-License-Identifier: Apache-2.0

if(CONFIG_RUST_CRATE_RYU)
  nuttx_add_rust(
    CRATE_NAME ryu
    CRATE_PATH ${CMAKE_CURRENT_SOURCE_DIR}
  )

  nuttx_add_application(
    NAME rust_crate_test_ryu
    STACKSIZE ${CONFIG_RUST_CRATE_RYU_STACKSIZE}
    PRIORITY ${CONFIG_RUST_CRATE_RYU_PRIORITY}
  )

  add_dependencies(apps ryu)
endif() # CONFIG_RUST_CRATE_RYU
