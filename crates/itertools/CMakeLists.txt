# Copyright (c) 2025 Xiaomi Corporation
# SPDX-License-Identifier: Apache-2.0

if(CONFIG_RUST_CRATE_ITERTOOLS)
  nuttx_add_rust(
    CRATE_NAME itertools
    CRATE_PATH ${CMAKE_CURRENT_SOURCE_DIR}
  )

  nuttx_add_application(
    NAME rust_crate_test_itertools
    STACKSIZE ${CONFIG_RUST_CRATE_ITERTOOLS_STACKSIZE}
    PRIORITY ${CONFIG_RUST_CRATE_ITERTOOLS_PRIORITY}
  )

  add_dependencies(apps itertools)
endif() # CONFIG_RUST_CRATE_ITERTOOLS
