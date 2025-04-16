# Copyright (c) 2025 Xiaomi Corporation
# SPDX-License-Identifier: Apache-2.0

if(CONFIG_RUST_CRATE_PROTOBUF)
  nuttx_add_rust(
    CRATE_NAME protobuf
    CRATE_PATH ${CMAKE_CURRENT_SOURCE_DIR}
  )

  nuttx_add_application(
    NAME rust_crate_test_protobuf
    STACKSIZE ${CONFIG_RUST_CRATE_PROTOBUF_STACKSIZE}
    PRIORITY ${CONFIG_RUST_CRATE_PROTOBUF_PRIORITY}
  )

  add_dependencies(apps protobuf)
endif() # CONFIG_RUST_CRATE_PROTOBUF
