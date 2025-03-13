# Copyright (c) 2025 Xiaomi Corporation
# SPDX-License-Identifier: Apache-2.0

if(CONFIG_RUST_CRATE_SMALLVEC)
  nuttx_add_rust(
    CRATE_NAME smallvec
    CRATE_PATH ${CMAKE_CURRENT_SOURCE_DIR}
  )

  nuttx_add_application(
    NAME rust_crate_test_smallvec
    STACKSIZE ${CONFIG_RUST_CRATE_SMALLVEC_STACKSIZE}
    PRIORITY ${CONFIG_RUST_CRATE_SMALLVEC_PRIORITY}
  )

  add_dependencies(apps smallvec)
endif() # CONFIG_RUST_CRATE_SMALLVEC
