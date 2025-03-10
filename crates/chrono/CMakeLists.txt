# Copyright (c) 2025 Xiaomi Corporation
# SPDX-License-Identifier: Apache-2.0

if(CONFIG_RUST_CRATE_CHRONO)
  nuttx_add_rust(
    CRATE_NAME chrono
    CRATE_PATH ${CMAKE_CURRENT_SOURCE_DIR}
  )

  nuttx_add_application(
    NAME rust_crate_test_chrono
    STACKSIZE ${CONFIG_RUST_CRATE_CHRONO_STACKSIZE}
    PRIORITY ${CONFIG_RUST_CRATE_CHRONO_PRIORITY}
  )

  add_dependencies(apps chrono)
endif() # CONFIG_RUST_CRATE_CHRONO
