# Copyright (c) 2025 Xiaomi Corporation
# SPDX-License-Identifier: Apache-2.0

if(CONFIG_RUST_CRATE_STD_BOXED)
  nuttx_add_rust(
    CRATE_NAME std_boxed
    CRATE_PATH ${CMAKE_CURRENT_SOURCE_DIR}
  )

  nuttx_add_application(
    NAME rust_crate_test_std_boxed
    STACKSIZE ${CONFIG_RUST_CRATE_STD_BOXED_STACKSIZE}
    PRIORITY ${CONFIG_RUST_CRATE_STD_BOXED_PRIORITY}
  )

  add_dependencies(apps std_boxed)
endif() # CONFIG_RUST_CRATE_STD_BOXED
