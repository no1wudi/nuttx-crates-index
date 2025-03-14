# Copyright (c) 2025 Xiaomi Corporation
# SPDX-License-Identifier: Apache-2.0

if(CONFIG_RUST_CRATE_LIBC)
  nuttx_add_rust(
    CRATE_NAME libc
    CRATE_PATH ${CMAKE_CURRENT_SOURCE_DIR}
  )

  nuttx_add_application(
    NAME rust_crate_test_libc
    STACKSIZE ${CONFIG_RUST_CRATE_LIBC_STACKSIZE}
    PRIORITY ${CONFIG_RUST_CRATE_LIBC_PRIORITY}
  )

  add_dependencies(apps libc)
endif() # CONFIG_RUST_CRATE_LIBC
