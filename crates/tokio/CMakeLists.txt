# Copyright (c) 2025 Xiaomi Corporation
# SPDX-License-Identifier: Apache-2.0

if(CONFIG_RUST_CRATE_TOKIO)
  nuttx_add_rust(CRATE_NAME tokio CRATE_PATH ${CMAKE_CURRENT_SOURCE_DIR})
  nuttx_add_application(
    NAME rust_crate_test_tokio
    STACKSIZE ${CONFIG_RUST_CRATE_TOKIO_STACKSIZE}
    PRIORITY ${CONFIG_RUST_CRATE_TOKIO_PRIORITY})
  add_dependencies(apps tokio)
endif()
