# Copyright (c) 2025 Xiaomi Corporation
# SPDX-License-Identifier: Apache-2.0

if(CONFIG_RUST_CRATE_SCOPEGUARD)
  nuttx_add_rust(
    CRATE_NAME scopeguard
    CRATE_PATH ${CMAKE_CURRENT_SOURCE_DIR}
  )

  nuttx_add_application(
    NAME rust_crate_test_scopeguard
    STACKSIZE ${CONFIG_RUST_CRATE_SCOPEGUARD_STACKSIZE}
    PRIORITY ${CONFIG_RUST_CRATE_SCOPEGUARD_PRIORITY}
  )

  add_dependencies(apps scopeguard)
endif() # CONFIG_RUST_CRATE_SCOPEGUARD