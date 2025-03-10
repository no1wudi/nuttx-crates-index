// Copyright (c) 2025 Xiaomi Corporation
// SPDX-License-Identifier: Apache-2.0

#![no_std]

use core::fmt::Write;

#[cfg(target_os = "nuttx")]
mod panic {
    use core::panic::PanicInfo;
    #[panic_handler]
    fn panic(_info: &PanicInfo) -> ! {
        loop {}
    }
}

unsafe extern "C" {
    fn puts(s: *const i8) -> i32;
}

struct PutsWriter {
    buffer: [u8; 128],
    pos: usize,
}

impl PutsWriter {
    fn new() -> Self {
        Self {
            buffer: [0; 128],
            pos: 0,
        }
    }

    fn flush(&mut self) {
        if self.pos > 0 {
            // Ensure null termination
            self.buffer[self.pos] = 0;
            unsafe {
                puts(self.buffer.as_ptr() as *const i8);
            }
            self.pos = 0;
        }
    }
}

impl Write for PutsWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for byte in s.bytes() {
            if self.pos >= self.buffer.len() - 1 {
                self.flush();
            }
            self.buffer[self.pos] = byte;
            self.pos += 1;
        }
        Ok(())
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_crate_test_core_println_main() {
    let mut writer = PutsWriter::new();

    let _ = writeln!(writer, "Hello from Rust!");
    writer.flush();

    let _ = writeln!(writer, "Demonstrating different format outputs:");
    let _ = writeln!(writer, "Numbers: {}", 42);
    let _ = writeln!(writer, "Multiple values: {} and {}", "first", "second");
    writer.flush();

    let _ = writeln!(writer, "Float formatting: {:.2}", 3.1415926);
    let _ = writeln!(writer, "Debug formatting: {:?}", [1, 2, 3]);
    let _ = writeln!(
        writer,
        "Padding and alignment: |{:>10}|{:<10}|",
        "right", "left"
    );
    let _ = writeln!(writer, "Binary: {:b}, Hex: {:x}, Octal: {:o}", 42, 255, 64);
    let _ = writeln!(writer, "With width and fill: {:0>5}", 123);
    writer.flush();

    let _ = writeln!(writer, "Scientific notation: {:e}", 1000000.0);
    let _ = writeln!(writer, "Alternative hex: {:#x}", 255);
    let _ = writeln!(writer, "Sign display: {:+}", 42);
    let _ = writeln!(writer, "Mixed alignment: {:^10}", "center");
    writer.flush();

    let result: Result<i32, &str> = Err("error message");
    let _ = writeln!(writer, "Error display: {:?}", result);
    let _ = writeln!(writer, "Tuple formatting: {:?}", (10, "hello", true));
    let _ = writeln!(
        writer,
        "Precision control: {:.1} vs {:.5}",
        3.141592, 3.141592
    );
    writer.flush();

    let _ = writeln!(writer, "All core_println tests completed successfully!");
    writer.flush();
}
