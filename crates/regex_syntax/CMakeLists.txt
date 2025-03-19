# Copyright (c) 2025 Xiaomi Corporation
# SPDX-License-Identifier: Apache-2.0

if(CONFIG_RUST_CRATE_REGEX_SYNTAX)
  nuttx_add_rust(
    CRATE_NAME regex_syntax
    CRATE_PATH ${CMAKE_CURRENT_SOURCE_DIR}
  )

  nuttx_add_application(
    NAME rust_crate_test_regex_syntax
    STACKSIZE ${CONFIG_RUST_CRATE_REGEX_SYNTAX_STACKSIZE}
    PRIORITY ${CONFIG_RUST_CRATE_REGEX_SYNTAX_PRIORITY}
  )

  add_dependencies(apps regex_syntax)
endif() # CONFIG_RUST_CRATE_REGEX_SYNTAX