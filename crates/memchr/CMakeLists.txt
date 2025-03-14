# Copyright (c) 2025 Xiaomi Corporation
# SPDX-License-Identifier: Apache-2.0

if(CONFIG_RUST_CRATE_MEMCHR)
  nuttx_add_rust(
    CRATE_NAME memchr
    CRATE_PATH ${CMAKE_CURRENT_SOURCE_DIR}
  )

  nuttx_add_application(
    NAME rust_crate_test_memchr
    STACKSIZE ${CONFIG_RUST_CRATE_MEMCHR_STACKSIZE}
    PRIORITY ${CONFIG_RUST_CRATE_MEMCHR_PRIORITY}
  )

  add_dependencies(apps memchr)
endif() # CONFIG_RUST_CRATE_MEMCHR
