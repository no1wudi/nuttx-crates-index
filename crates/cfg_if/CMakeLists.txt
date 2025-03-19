# Copyright (c) 2025 Xiaomi Corporation
# SPDX-License-Identifier: Apache-2.0

if(CONFIG_RUST_CRATE_CFG_IF)
  nuttx_add_rust(
    CRATE_NAME cfg_if
    CRATE_PATH ${CMAKE_CURRENT_SOURCE_DIR}
  )

  nuttx_add_application(
    NAME rust_crate_test_cfg_if
    STACKSIZE ${CONFIG_RUST_CRATE_CFG_IF_STACKSIZE}
    PRIORITY ${CONFIG_RUST_CRATE_CFG_IF_PRIORITY}
  )

  add_dependencies(apps cfg_if)
endif() # CONFIG_RUST_CRATE_CFG_IF