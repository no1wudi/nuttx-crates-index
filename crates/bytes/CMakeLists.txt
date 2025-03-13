# Copyright (c) 2025 Xiaomi Corporation
# SPDX-License-Identifier: Apache-2.0

if(CONFIG_RUST_CRATE_BYTES)
  nuttx_add_rust(
    CRATE_NAME bytes
    CRATE_PATH ${CMAKE_CURRENT_SOURCE_DIR}
  )

  nuttx_add_application(
    NAME rust_crate_test_bytes
    STACKSIZE ${CONFIG_RUST_CRATE_BYTES_STACKSIZE}
    PRIORITY ${CONFIG_RUST_CRATE_BYTES_PRIORITY}
  )

  add_dependencies(apps bytes)
endif() # CONFIG_RUST_CRATE_BYTES
