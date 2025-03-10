# Copyright (c) 2025 Xiaomi Corporation
# SPDX-License-Identifier: Apache-2.0

if(CONFIG_RUST_CRATE_LIBC_PUTS)
  nuttx_add_rust(
    CRATE_NAME libc_puts
    CRATE_PATH ${CMAKE_CURRENT_SOURCE_DIR}
  )

  nuttx_add_application(
    NAME rust_crate_test_libc_puts
    STACKSIZE ${CONFIG_RUST_CRATE_LIBC_PUTS_STACKSIZE}
    PRIORITY ${CONFIG_RUST_CRATE_LIBC_PUTS_PRIORITY}
  )

  add_dependencies(apps libc_puts)
endif() # CONFIG_RUST_CRATE_LIBC_PUTS
