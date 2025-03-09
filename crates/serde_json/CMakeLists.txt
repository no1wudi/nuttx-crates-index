# Copyright (c) 2025 Xiaomi Corporation
# SPDX-License-Identifier: Apache-2.0

if(CONFIG_RUST_CRATE_SERDE_JSON)
  nuttx_add_rust(CRATE_NAME serde_json CRATE_PATH ${CMAKE_CURRENT_SOURCE_DIR})
  nuttx_add_application(
    NAME rust_crate_test_serde_json
    STACKSIZE ${CONFIG_RUST_CRATE_SERDE_JSON_STACKSIZE}
    PRIORITY ${CONFIG_RUST_CRATE_SERDE_JSON_PRIORITY})
  add_dependencies(apps serde_json)
endif()
