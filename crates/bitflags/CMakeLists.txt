# Copyright (c) 2025 Xiaomi Corporation
# SPDX-License-Identifier: Apache-2.0

if(CONFIG_RUST_CRATE_BITFLAGS)
  nuttx_add_rust(
    CRATE_NAME bitflags
    CRATE_PATH ${CMAKE_CURRENT_SOURCE_DIR}
  )

  nuttx_add_application(
    NAME rust_crate_test_bitflags
    STACKSIZE ${CONFIG_RUST_CRATE_BITFLAGS_STACKSIZE}
    PRIORITY ${CONFIG_RUST_CRATE_BITFLAGS_PRIORITY}
  )

  add_dependencies(apps bitflags)
endif() # CONFIG_RUST_CRATE_BITFLAGS
