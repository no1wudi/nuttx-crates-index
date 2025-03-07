# Copyright (c) 2025 Xiaomi Corporation
# SPDX-License-Identifier: Apache-2.0

if(CONFIG_RUST_CRATES_ONCE_CELL)

  nuttx_add_rust(CRATE_NAME once_cell CRATE_PATH ${CMAKE_CURRENT_SOURCE_DIR})

  nuttx_add_application(
    NAME rust_crates_test_once_cell STACKSIZE
    ${CONFIG_RUST_CRATES_ONCE_CELL_STACKSIZE} PRIORITY
    ${CONFIG_RUST_CRATES_ONCE_CELL_PRIORITY})

  add_dependencies(apps once_cell)

endif() # CONFIG_RUST_CRATES_ONCE_CELL
