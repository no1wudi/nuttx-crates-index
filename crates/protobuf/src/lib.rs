// Copyright (c) 2025 Xiaomi Corporation
// SPDX-License-Identifier: Apache-2.0

// Note: This example assumes you have a .proto file and generated Rust code.
// For simplicity, we'll simulate a basic structure manually.
// In a real scenario, you'd use `protobuf-codegen` or similar build script integration.

// Simulate a simple generated message structure
mod simple_message {
    use protobuf::rt::CachedSize; // Correct path for CachedSize
    use protobuf::SpecialFields;
    use std::ptr;
    use std::sync::atomic::{AtomicPtr, Ordering};
    use std::sync::Once;

    #[derive(PartialEq, Clone, Default, Debug)]
    pub struct Simple {
        pub name: ::std::string::String,
        pub id: i32,
        pub special_fields: SpecialFields,
        pub cached_size: CachedSize, // Use the imported CachedSize
    }

    impl<'a> ::std::default::Default for &'a Simple {
        fn default() -> &'a Simple {
            <Simple as ::protobuf::Message>::default_instance()
        }
    }

    impl Simple {
        pub fn new() -> Simple {
            ::std::default::Default::default()
        }
    }

    impl ::protobuf::Message for Simple {
        const NAME: &'static str = "Simple";

        fn is_initialized(&self) -> bool {
            true // Simplified
        }

        fn merge_from(
            &mut self,
            is: &mut ::protobuf::CodedInputStream<'_>,
        ) -> ::protobuf::Result<()> {
            while let Some(tag) = is.read_raw_tag_or_eof()? {
                match tag {
                    10 => {
                        self.name = is.read_string()?;
                    }
                    16 => {
                        self.id = is.read_int32()?;
                    }
                    tag => {
                        ::protobuf::rt::read_unknown_or_skip_group(
                            tag,
                            is,
                            self.mut_special_fields().mut_unknown_fields(),
                        )?;
                    }
                };
            }
            Ok(())
        }

        fn compute_size(&self) -> u64 {
            let mut my_size = 0;
            if !self.name.is_empty() {
                my_size += ::protobuf::rt::string_size(1, &self.name);
            }
            if self.id != 0 {
                my_size += ::protobuf::rt::int32_size(2, self.id);
            }
            my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
            self.cached_size.set(my_size as u32);
            my_size
        }

        fn write_to_with_cached_sizes(
            &self,
            os: &mut ::protobuf::CodedOutputStream<'_>,
        ) -> ::protobuf::Result<()> {
            if !self.name.is_empty() {
                os.write_string(1, &self.name)?;
            }
            if self.id != 0 {
                os.write_int32(2, self.id)?;
            }
            os.write_unknown_fields(self.special_fields().unknown_fields())?;
            ::std::result::Result::Ok(())
        }

        fn special_fields(&self) -> &::protobuf::SpecialFields {
            &self.special_fields
        }

        fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
            &mut self.special_fields
        }

        fn new() -> Simple {
            Simple::new()
        }

        fn clear(&mut self) {
            self.name.clear();
            self.id = 0;
            self.special_fields.clear();
        }

        fn default_instance() -> &'static Simple {
            // Use AtomicPtr and Box::leak for safe static initialization
            static INSTANCE_PTR: AtomicPtr<Simple> = AtomicPtr::new(ptr::null_mut());
            static ONCE: Once = Once::new();

            ONCE.call_once(|| {
                let instance = Simple::new();
                // Allocate on the heap and leak to get a 'static reference
                let leaked_ptr = Box::into_raw(Box::new(instance));
                INSTANCE_PTR.store(leaked_ptr, Ordering::SeqCst);
            });

            let ptr = INSTANCE_PTR.load(Ordering::SeqCst);
            // Safety:
            // - `ptr` is initialized exactly once by `call_once`.
            // - `ptr` points to a valid `Simple` allocated via `Box::new`.
            // - The allocation was leaked using `Box::into_raw`, so the pointer
            //   remains valid for the static lifetime.
            // - The data pointed to is immutable after initialization.
            unsafe { &*ptr }
        }
    }
}

use protobuf::Message;
use simple_message::Simple;

#[unsafe(no_mangle)]
pub extern "C" fn rust_crate_test_protobuf_main() -> i32 {
    println!("Rust Protobuf Demo Started!");

    let mut msg = Simple::new();
    msg.name = "Test".to_string();
    msg.id = 123;

    println!("Original message: {:?}", msg);

    let bytes = match msg.write_to_bytes() {
        Ok(b) => {
            println!("Serialized to {} bytes", b.len());
            b
        }
        Err(e) => {
            println!("Serialization failed: {}", e);
            return 1;
        }
    };

    match Simple::parse_from_bytes(&bytes) {
        Ok(parsed_msg) => {
            println!("Deserialized message: {:?}", parsed_msg);
            if parsed_msg == msg {
                println!("Serialization/Deserialization successful!");
                0
            } else {
                println!("Deserialized message does not match original!");
                1
            }
        }
        Err(e) => {
            println!("Deserialization failed: {}", e);
            1
        }
    }
}
