# Copyright (c) 2025 Xiaomi Corporation
# SPDX-License-Identifier: Apache-2.0

if(CONFIG_RUST_CRATE_TIME)
  nuttx_add_rust(
    CRATE_NAME time
    CRATE_PATH ${CMAKE_CURRENT_SOURCE_DIR}
  )

  nuttx_add_application(
    NAME rust_crate_test_time
    STACKSIZE ${CONFIG_RUST_CRATE_TIME_STACKSIZE}
    PRIORITY ${CONFIG_RUST_CRATE_TIME_PRIORITY}
  )

  add_dependencies(apps time)
endif() # CONFIG_RUST_CRATE_TIME