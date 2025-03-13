# Copyright (c) 2025 Xiaomi Corporation
# SPDX-License-Identifier: Apache-2.0

if(CONFIG_RUST_CRATE_SHA2)
  nuttx_add_rust(
    CRATE_NAME sha2
    CRATE_PATH ${CMAKE_CURRENT_SOURCE_DIR}
  )

  nuttx_add_application(
    NAME rust_crate_test_sha2
    STACKSIZE ${CONFIG_RUST_CRATE_SHA2_STACKSIZE}
    PRIORITY ${CONFIG_RUST_CRATE_SHA2_PRIORITY}
  )

  add_dependencies(apps sha2)
endif() # CONFIG_RUST_CRATE_SHA2
