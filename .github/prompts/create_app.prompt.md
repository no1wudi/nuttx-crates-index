# NuttX Rust Application Creation Template

Generate a new Rust crate for NuttX using the following template structure. Please replace:
- `[NAME]` with uppercase configuration name
- `[name]` with lowercase crate name
- `[Description]` with brief app description
- `[SPECIFIC_FUNCTIONALITY]` with detailed functionality description

## Required Files

### 1. Kconfig
```kconfig
# Copyright (c) 2025 Xiaomi Corporation
# SPDX-License-Identifier: Apache-2.0

config RUST_CRATES_[NAME]
	tristate "\"[Description]\" example"
	default n
	---help---
		Enable the [Description] example application.
		This demonstrates [SPECIFIC_FUNCTIONALITY].

if RUST_CRATES_[NAME]

config RUST_CRATES_[NAME]_PRIORITY
	int "[Name] task priority"
	default 100

config RUST_CRATES_[NAME]_STACKSIZE
	int "[Name] stack size"
	default 8192
	---help---
		Stack size to be used.

endif
```

### 2. CMakeLists.txt
```cmake
# Copyright (c) 2025 Xiaomi Corporation
# SPDX-License-Identifier: Apache-2.0

if(CONFIG_RUST_CRATES_[NAME])
  nuttx_add_rust(CRATE_NAME [name] CRATE_PATH ${CMAKE_CURRENT_SOURCE_DIR})
  nuttx_add_application(
    NAME rust_crates_test_[name]
    STACKSIZE ${CONFIG_RUST_CRATES_[NAME]_STACKSIZE}
    PRIORITY ${CONFIG_RUST_CRATES_[NAME]_PRIORITY})
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
pub extern "C" fn rust_crates_test_[name]_main() {
    // Implement your functionality here
}
```

## Instructions

1. Create a new directory under `crates/[name]`
2. Copy and customize the above templates
3. Update the main function name to match your crate
4. Ensure copyright headers in all files
5. Add appropriate documentation
6. Test the integration with NuttX build system

Example usage: "Create a new Rust crate called 'led_blink' that demonstrates GPIO control for LED blinking"
