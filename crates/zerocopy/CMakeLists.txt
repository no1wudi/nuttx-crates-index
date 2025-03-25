# Copyright (c) 2025 Xiaomi Corporation
# SPDX-License-Identifier: Apache-2.0

if(CONFIG_RUST_CRATE_ZEROCOPY)
  nuttx_add_rust(
    CRATE_NAME zerocopy
    CRATE_PATH ${CMAKE_CURRENT_SOURCE_DIR}
  )

  nuttx_add_application(
    NAME rust_crate_test_zerocopy
    STACKSIZE ${CONFIG_RUST_CRATE_ZEROCOPY_STACKSIZE}
    PRIORITY ${CONFIG_RUST_CRATE_ZEROCOPY_PRIORITY}
  )

  add_dependencies(apps zerocopy)
endif() # CONFIG_RUST_CRATE_ZEROCOPY
