# Copyright (c) 2025 Xiaomi Corporation
# SPDX-License-Identifier: Apache-2.0

if(CONFIG_RUST_CRATE_RAND_CHACHA)
  nuttx_add_rust(
    CRATE_NAME rand-chacha
    CRATE_PATH ${CMAKE_CURRENT_SOURCE_DIR}
  )

  nuttx_add_application(
    NAME rust_crate_test_rand_chacha
    STACKSIZE ${CONFIG_RUST_CRATE_RAND_CHACHA_STACKSIZE}
    PRIORITY ${CONFIG_RUST_CRATE_RAND_CHACHA_PRIORITY}
  )

  add_dependencies(apps rand-chacha)
endif() # CONFIG_RUST_CRATE_RAND_CHACHA
