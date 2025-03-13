# Copyright (c) 2025 Xiaomi Corporation
# SPDX-License-Identifier: Apache-2.0

if(CONFIG_RUST_CRATE_INDEXMAP)
  nuttx_add_rust(
    CRATE_NAME indexmap
    CRATE_PATH ${CMAKE_CURRENT_SOURCE_DIR}
  )

  nuttx_add_application(
    NAME rust_crate_test_indexmap
    STACKSIZE ${CONFIG_RUST_CRATE_INDEXMAP_STACKSIZE}
    PRIORITY ${CONFIG_RUST_CRATE_INDEXMAP_PRIORITY}
  )

  add_dependencies(apps indexmap)
endif() # CONFIG_RUST_CRATE_INDEXMAP
