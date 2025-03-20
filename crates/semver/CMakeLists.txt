# Copyright (c) 2025 Xiaomi Corporation
# SPDX-License-Identifier: Apache-2.0

if(CONFIG_RUST_CRATE_SEMVER)
  nuttx_add_rust(
    CRATE_NAME semver
    CRATE_PATH ${CMAKE_CURRENT_SOURCE_DIR}
  )

  nuttx_add_application(
    NAME rust_crate_test_semver
    STACKSIZE ${CONFIG_RUST_CRATE_SEMVER_STACKSIZE}
    PRIORITY ${CONFIG_RUST_CRATE_SEMVER_PRIORITY}
  )

  add_dependencies(apps semver)
endif() # CONFIG_RUST_CRATE_SEMVER
