# Copyright (c) 2025 Xiaomi Corporation
# SPDX-License-Identifier: Apache-2.0

if(CONFIG_RUST_CRATE_STD_THREAD_LOCAL)
  nuttx_add_rust(
    CRATE_NAME std_thread_local
    CRATE_PATH ${CMAKE_CURRENT_SOURCE_DIR}
  )

  nuttx_add_application(
    NAME rust_crate_test_std_thread_local
    STACKSIZE ${CONFIG_RUST_CRATE_STD_THREAD_LOCAL_STACKSIZE}
    PRIORITY ${CONFIG_RUST_CRATE_STD_THREAD_LOCAL_PRIORITY}
  )

  add_dependencies(apps std_thread_local)
endif() # CONFIG_RUST_CRATE_STD_THREAD_LOCAL
