# Copyright (c) 2025 Xiaomi Corporation
# SPDX-License-Identifier: Apache-2.0

if(CONFIG_RUST_CRATE_ITOA)
  nuttx_add_rust(
    CRATE_NAME itoa
    CRATE_PATH ${CMAKE_CURRENT_SOURCE_DIR}
  )

  nuttx_add_application(
    NAME rust_crate_test_itoa
    STACKSIZE ${CONFIG_RUST_CRATE_ITOA_STACKSIZE}
    PRIORITY ${CONFIG_RUST_CRATE_ITOA_PRIORITY}
  )

  add_dependencies(apps itoa)
endif() # CONFIG_RUST_CRATE_ITOA
