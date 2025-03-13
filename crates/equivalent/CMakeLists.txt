# Copyright (c) 2025 Xiaomi Corporation
# SPDX-License-Identifier: Apache-2.0

if(CONFIG_RUST_CRATE_EQUIVALENT)
  nuttx_add_rust(
    CRATE_NAME equivalent
    CRATE_PATH ${CMAKE_CURRENT_SOURCE_DIR}
  )

  nuttx_add_application(
    NAME rust_crate_test_equivalent
    STACKSIZE ${CONFIG_RUST_CRATE_EQUIVALENT_STACKSIZE}
    PRIORITY ${CONFIG_RUST_CRATE_EQUIVALENT_PRIORITY}
  )

  add_dependencies(apps equivalent)
endif() # CONFIG_RUST_CRATE_EQUIVALENT
