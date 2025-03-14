# Copyright (c) 2025 Xiaomi Corporation
# SPDX-License-Identifier: Apache-2.0

if(CONFIG_RUST_CRATE_THISERROR)
  nuttx_add_rust(
    CRATE_NAME thiserror
    CRATE_PATH ${CMAKE_CURRENT_SOURCE_DIR}
  )

  nuttx_add_application(
    NAME rust_crate_test_thiserror
    STACKSIZE ${CONFIG_RUST_CRATE_THISERROR_STACKSIZE}
    PRIORITY ${CONFIG_RUST_CRATE_THISERROR_PRIORITY}
  )

  add_dependencies(apps thiserror)
endif() # CONFIG_RUST_CRATE_THISERROR
