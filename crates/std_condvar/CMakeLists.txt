# Copyright (c) 2025 Xiaomi Corporation
# SPDX-License-Identifier: Apache-2.0

if(CONFIG_RUST_CRATE_STD_CONDVAR)
  nuttx_add_rust(
    CRATE_NAME std_condvar
    CRATE_PATH ${CMAKE_CURRENT_SOURCE_DIR}
  )

  nuttx_add_application(
    NAME rust_crate_test_std_condvar
    STACKSIZE ${CONFIG_RUST_CRATE_STD_CONDVAR_STACKSIZE}
    PRIORITY ${CONFIG_RUST_CRATE_STD_CONDVAR_PRIORITY}
  )

  add_dependencies(apps std_condvar)
endif() # CONFIG_RUST_CRATE_STD_CONDVAR
