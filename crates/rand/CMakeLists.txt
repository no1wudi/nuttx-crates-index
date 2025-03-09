# Copyright (c) 2025 Xiaomi Corporation
# SPDX-License-Identifier: Apache-2.0

if(CONFIG_RUST_CRATE_RAND)
  nuttx_add_rust(CRATE_NAME rand CRATE_PATH ${CMAKE_CURRENT_SOURCE_DIR})
  nuttx_add_application(
    NAME rust_crate_test_rand
    STACKSIZE ${CONFIG_RUST_CRATE_RAND_STACKSIZE}
    PRIORITY ${CONFIG_RUST_CRATE_RAND_PRIORITY})
  add_dependencies(apps rand)
endif()
