# NuttX Rust Application Creation Template

Generate a new Rust crate using the following template structure. Please replace:
- `[NAME]` with uppercase configuration name
- `[name]` with lowercase crate name
- `[Description]` with brief app description
- `[SPECIFIC_FUNCTIONALITY]` with detailed functionality description

## Required Files

### 1. Kconfig

kconfig file to demonstrate the third-party Rust crate
```kconfig
# Copyright (c) 2025 Xiaomi Corporation
# SPDX-License-Identifier: Apache-2.0

config RUST_CRATE_[NAME]
	tristate "\"Crate [Name]\" example"
	default n

if RUST_CRATE_[NAME]

config RUST_CRATE_[NAME]_PRIORITY
	int "[Name] task priority"
	default 100

config RUST_CRATE_[NAME]_STACKSIZE
	int "[Name] stack size"
	default DEFAULT_TASK_STACKSIZE

endif
```

Kconfig file to demonstrate the libstd's functionality
```kconfig
# Copyright (c) 2025 Xiaomi Corporation
# SPDX-License-Identifier: Apache-2.0

config RUST_CRATE_STD_[NAME]
	tristate "\"Std [Name]\" example"
	default n

if RUST_CRATE_STD_[NAME]

config RUST_CRATE_STD_[NAME]_PRIORITY
	int "Std [Name] task priority"
	default 100

config RUST_CRATE_STD_[NAME]_STACKSIZE
	int "Std [Name] stack size"
	default DEFAULT_TASK_STACKSIZE

endif
```

kconfig file to demonstrate the libcore's functionality
```kconfig
# Copyright (c) 2025 Xiaomi Corporation
# SPDX-License-Identifier: Apache-2.0

config RUST_CRATE_CORE_[NAME]
	tristate "\"Core [Name]\" example"
	default n

if RUST_CRATE_CORE_[NAME]

config RUST_CRATE_CORE_[NAME]_PRIORITY
	int "Core [Name] task priority"
	default 100

config RUST_CRATE_CORE_[NAME]_STACKSIZE
	int "Core [Name] stack size"
	default DEFAULT_TASK_STACKSIZE

endif
```

### 2. CMakeLists.txt
```cmake
# Copyright (c) 2025 Xiaomi Corporation
# SPDX-License-Identifier: Apache-2.0

if(CONFIG_RUST_CRATE_[NAME])
  nuttx_add_rust(CRATE_NAME [name] CRATE_PATH ${CMAKE_CURRENT_SOURCE_DIR})
  nuttx_add_application(
    NAME rust_crate_test_[name]
    STACKSIZE ${CONFIG_RUST_CRATE_[NAME]_STACKSIZE}
    PRIORITY ${CONFIG_RUST_CRATE_[NAME]_PRIORITY})
  add_dependencies(apps [name])
endif()
```

### 3. Cargo.toml
```toml
# Copyright (c) 2025 Xiaomi Corporation
# SPDX-License-Identifier: Apache-2.0

[package]
name = "[name]"
version = "0.1.0"
edition = "2024"

[lib]
crate-type = ["staticlib"]

[profile.dev]
panic = "abort"

[profile.release]
panic = "abort"
lto = true
codegen-units = 1
opt-level = 'z'
```

### 4. src/lib.rs
```rust
// Copyright (c) 2025 Xiaomi Corporation
// SPDX-License-Identifier: Apache-2.0

#[unsafe(no_mangle)]
pub extern "C" fn rust_crate_test_[name]_main() {
    // Implement your functionality here
}
```

## Instructions

1. Create a new directory under `crates/[name]`
2. Copy and customize the above templates
3. Update the main function name to match your crate
4. Ensure copyright headers in all files
5. Add appropriate documentation
6. Ensure "#[unsafe(no_mangle)]" used for the entry point to meet the requirements of Rust 2024
7. Rust's libstd is available for stdlib functionality and third-party crates