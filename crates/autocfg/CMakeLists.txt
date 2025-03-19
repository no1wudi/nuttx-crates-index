# Copyright (c) 2025 Xiaomi Corporation
# SPDX-License-Identifier: Apache-2.0

if(CONFIG_RUST_CRATE_AUTOCFG)
  nuttx_add_rust(
    CRATE_NAME autocfg
    CRATE_PATH ${CMAKE_CURRENT_SOURCE_DIR}
  )

  nuttx_add_application(
    NAME rust_crate_test_autocfg
    STACKSIZE ${CONFIG_RUST_CRATE_AUTOCFG_STACKSIZE}
    PRIORITY ${CONFIG_RUST_CRATE_AUTOCFG_PRIORITY}
  )

  add_dependencies(apps autocfg)
endif() # CONFIG_RUST_CRATE_AUTOCFG