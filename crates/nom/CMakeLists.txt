# Copyright (c) 2025 Xiaomi Corporation
# SPDX-License-Identifier: Apache-2.0

if(CONFIG_RUST_CRATE_NOM)
  nuttx_add_rust(
    CRATE_NAME nom
    CRATE_PATH ${CMAKE_CURRENT_SOURCE_DIR}
  )

  nuttx_add_application(
    NAME rust_crate_test_nom
    STACKSIZE ${CONFIG_RUST_CRATE_NOM_STACKSIZE}
    PRIORITY ${CONFIG_RUST_CRATE_NOM_PRIORITY}
  )

  add_dependencies(apps nom)
endif() # CONFIG_RUST_CRATE_NOM