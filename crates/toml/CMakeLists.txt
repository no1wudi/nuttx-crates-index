# Copyright (c) 2025 Xiaomi Corporation
# SPDX-License-Identifier: Apache-2.0

if(CONFIG_RUST_CRATE_TOML)
  nuttx_add_rust(
    CRATE_NAME toml
    CRATE_PATH ${CMAKE_CURRENT_SOURCE_DIR}
  )

  nuttx_add_application(
    NAME rust_crate_test_toml
    STACKSIZE ${CONFIG_RUST_CRATE_TOML_STACKSIZE}
    PRIORITY ${CONFIG_RUST_CRATE_TOML_PRIORITY}
  )

  add_dependencies(apps toml)
endif() # CONFIG_RUST_CRATE_TOML
