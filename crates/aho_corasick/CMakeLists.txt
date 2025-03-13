# Copyright (c) 2025 Xiaomi Corporation
# SPDX-License-Identifier: Apache-2.0

if(CONFIG_RUST_CRATE_AHO_CORASICK)
  nuttx_add_rust(
    CRATE_NAME aho_corasick
    CRATE_PATH ${CMAKE_CURRENT_SOURCE_DIR}
  )

  nuttx_add_application(
    NAME rust_crate_test_aho_corasick
    STACKSIZE ${CONFIG_RUST_CRATE_AHO_CORASICK_STACKSIZE}
    PRIORITY ${CONFIG_RUST_CRATE_AHO_CORASICK_PRIORITY}
  )

  add_dependencies(apps aho_corasick)
endif() # CONFIG_RUST_CRATE_AHO_CORASICK
