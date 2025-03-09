#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

lazy_static! {
    static ref HASHMAP: HashMap<u32, &'static str> = {
        let mut m = HashMap::new();
        m.insert(0, "foo");
        m.insert(1, "bar");
        m.insert(2, "baz");
        m
    };
}

fn test_lazy_static() {
    println!("Testing lazy_static...");

    // First access to `HASHMAP` initializes it
    println!("The entry for `0` is \"{}\".", HASHMAP.get(&0).unwrap());

    // Any further access to `HASHMAP` just returns the computed value
    println!("The entry for `1` is \"{}\".", HASHMAP.get(&1).unwrap());

    assert_eq!(HASHMAP.get(&0), Some(&"foo"));
    assert_eq!(HASHMAP.get(&1), Some(&"bar"));

    println!("lazy_static test completed successfully!");
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_crate_test_lazy_static_main() {
    test_lazy_static();
}
