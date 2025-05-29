# Copyright (c) 2025 Xiaomi Corporation
# SPDX-License-Identifier: Apache-2.0

if(CONFIG_RUST_CRATE_PROST)
  nuttx_add_rust(
    CRATE_NAME prost
    CRATE_PATH ${CMAKE_CURRENT_SOURCE_DIR}
  )

  nuttx_add_application(
    NAME rust_crate_test_prost
    STACKSIZE ${CONFIG_RUST_CRATE_PROST_STACKSIZE}
    PRIORITY ${CONFIG_RUST_CRATE_PROST_PRIORITY}
  )

  add_dependencies(apps prost)
endif() # CONFIG_RUST_CRATE_PROST
