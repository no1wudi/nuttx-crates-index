# Copyright (c) 2025 Xiaomi Corporation
# SPDX-License-Identifier: Apache-2.0

if(CONFIG_RUST_CRATE_STRSIM)
  nuttx_add_rust(
    CRATE_NAME strsim
    CRATE_PATH ${CMAKE_CURRENT_SOURCE_DIR}
  )

  nuttx_add_application(
    NAME rust_crate_test_strsim
    STACKSIZE ${CONFIG_RUST_CRATE_STRSIM_STACKSIZE}
    PRIORITY ${CONFIG_RUST_CRATE_STRSIM_PRIORITY}
  )

  add_dependencies(apps strsim)
endif() # CONFIG_RUST_CRATE_STRSIM
