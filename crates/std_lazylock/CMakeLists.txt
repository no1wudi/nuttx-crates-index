# Copyright (c) 2025 Xiaomi Corporation
# SPDX-License-Identifier: Apache-2.0

if(CONFIG_RUST_CRATE_STD_LAZYLOCK)
  nuttx_add_rust(
    CRATE_NAME std_lazylock
    CRATE_PATH ${CMAKE_CURRENT_SOURCE_DIR}
  )

  nuttx_add_application(
    NAME rust_crate_test_std_lazylock
    STACKSIZE ${CONFIG_RUST_CRATE_STD_LAZYLOCK_STACKSIZE}
    PRIORITY ${CONFIG_RUST_CRATE_STD_LAZYLOCK_PRIORITY}
  )

  add_dependencies(apps std_lazylock)
endif() # CONFIG_RUST_CRATE_STD_LAZYLOCK
