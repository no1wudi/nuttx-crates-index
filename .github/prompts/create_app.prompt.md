# NuttX Rust Application Creation Template

Generate a new Rust crate using the following template structure. Please replace:
- `[NAME]` with uppercase configuration name (use underscores for hyphens, e.g., "XXX YYY" for "xxx-yyy")
- `[name]` with lowercase crate name (keep hyphens if present, e.g., "xxx-yyy")
- `[Description]` with brief app description
- `[SPECIFIC_FUNCTIONALITY]` with detailed functionality description

## Required Files

### 1. Kconfig

kconfig file to demonstrate the third-party Rust crate
```kconfig
# Copyright (c) 2025 Xiaomi Corporation
# SPDX-License-Identifier: Apache-2.0

config RUST_CRATE_[NAME]
	tristate "Crate [Name]"
	default n

if RUST_CRATE_[NAME]

config RUST_CRATE_[NAME]_PRIORITY
	int "Crate [Name] task priority"
	default 100

config RUST_CRATE_[NAME]_STACKSIZE
	int "Crate [Name] stack size"
	default DEFAULT_TASK_STACKSIZE

endif
```

Kconfig file to demonstrate the libstd's functionality
```kconfig
# Copyright (c) 2025 Xiaomi Corporation
# SPDX-License-Identifier: Apache-2.0

config RUST_CRATE_STD_[NAME]
	tristate "Std [Name]"
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
	tristate "Core [Name]"
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
  nuttx_add_rust(
    CRATE_NAME [name]
    CRATE_PATH ${CMAKE_CURRENT_SOURCE_DIR}
  )

  nuttx_add_application(
    NAME rust_crate_test_[name_underscored]
    STACKSIZE ${CONFIG_RUST_CRATE_[NAME]_STACKSIZE}
    PRIORITY ${CONFIG_RUST_CRATE_[NAME]_PRIORITY}
  )

  add_dependencies(apps [name])
endif() # CONFIG_RUST_CRATE_[NAME]
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

```

### 4. src/lib.rs
```rust
// Copyright (c) 2025 Xiaomi Corporation
// SPDX-License-Identifier: Apache-2.0

#[unsafe(no_mangle)]
pub fn rust_crate_test_[name_underscored]_main() {
    // Implement your functionality here
}
```

## Example for hyphenated crate name "xxx-yyy"

### Kconfig
```kconfig
# Copyright (c) 2025 Xiaomi Corporation
# SPDX-License-Identifier: Apache-2.0

config RUST_CRATE_XXX_YYY
	tristate "Crate Xxx Yyy"
	default n

if RUST_CRATE_XXX_YYY

config RUST_CRATE_XXX_YYY_PRIORITY
	int "Crate Xxx Yyy task priority"
	default 100

config RUST_CRATE_XXX_YYY_STACKSIZE
	int "Crate Xxx Yyy stack size"
	default DEFAULT_TASK_STACKSIZE

endif
```

### CMakeLists.txt
```cmake
# Copyright (c) 2025 Xiaomi Corporation
# SPDX-License-Identifier: Apache-2.0

if(CONFIG_RUST_CRATE_XXX_YYY)
  nuttx_add_rust(
    CRATE_NAME xxx_yyy
    CRATE_PATH ${CMAKE_CURRENT_SOURCE_DIR}
  )

  nuttx_add_application(
    NAME rust_crate_test_xxx_yyy
    STACKSIZE ${CONFIG_RUST_CRATE_XXX_YYY_STACKSIZE}
    PRIORITY ${CONFIG_RUST_CRATE_XXX_YYY_PRIORITY}
  )

  add_dependencies(apps xxx_yyy)
endif() # CONFIG_RUST_CRATE_XXX_YYY
```

### src/lib.rs
```rust
// Copyright (c) 2025 Xiaomi Corporation
// SPDX-License-Identifier: Apache-2.0

#[unsafe(no_mangle)]
pub fn rust_crate_test_xxx_yyy_main() {
    // Implementation goes here
}
```

## Instructions

1. Create a new directory under `crates/[name]`
   - For 3rd party crates, use `crates/[name]` as the directory and crate name
   - For libstd functionality, use `crates/std_[name]` as the directory and crate name
   - For libcore functionality, use `crates/core_[name]` as the directory crate name
   - Use the above templates to create `Kconfig`, `CMakeLists.txt`, and `Cargo.toml` files
   - Create a `src` directory and add `lib.rs` file
2. Copy and customize the above templates
3. Update the main function name to match your crate:
   - For hyphenated names (e.g., "xxx-yyy"), use underscores in C function names: `rust_crate_test_xxx_yyy_main`
   - In Kconfig, use uppercase with underscores: `RUST_CRATE_XXX_YYY`
4. Ensure copyright headers in all files
5. Add appropriate documentation
6. Ensure "#[unsafe(no_mangle)]" used for the entry point to meet the requirements of Rust 2024
7. Rust's libstd is available for stdlib functionality and third-party crates
8. Create thread with 4K stack size instead of use std::thread::spawn directly if you want to use threads
