# Copyright (c) 2025 Xiaomi Corporation
# SPDX-License-Identifier: Apache-2.0

if(CONFIG_RUST_CRATE_MINIZ_OXIDE)
  nuttx_add_rust(
    CRATE_NAME miniz_oxide
    CRATE_PATH ${CMAKE_CURRENT_SOURCE_DIR}
  )

  nuttx_add_application(
    NAME rust_crate_test_miniz_oxide
    STACKSIZE ${CONFIG_RUST_CRATE_MINIZ_OXIDE_STACKSIZE}
    PRIORITY ${CONFIG_RUST_CRATE_MINIZ_OXIDE_PRIORITY}
  )

  add_dependencies(apps miniz_oxide)
endif() # CONFIG_RUST_CRATE_MINIZ_OXIDE
