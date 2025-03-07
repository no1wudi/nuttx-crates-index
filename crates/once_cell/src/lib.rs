extern crate once_cell;

use once_cell::sync::{Lazy, OnceCell};
use std::collections::HashMap;

static HASHMAP: Lazy<HashMap<u32, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert(0, "foo");
    m.insert(1, "bar");
    m.insert(2, "baz");
    m
});

// Same, but completely without macros
fn hashmap() -> &'static HashMap<u32, &'static str> {
    static INSTANCE: OnceCell<HashMap<u32, &'static str>> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        let mut m = HashMap::new();
        m.insert(0, "foo");
        m.insert(1, "bar");
        m.insert(2, "baz");
        m
    })
}

fn test_once_cell() {
    println!("Testing once_cell...");

    // First access to `HASHMAP` initializes it
    println!("The entry for `0` is \"{}\".", HASHMAP.get(&0).unwrap());

    // Any further access to `HASHMAP` just returns the computed value
    println!("The entry for `1` is \"{}\".", HASHMAP.get(&1).unwrap());

    // The same works for function-style:
    let foo_value = hashmap().get(&0).unwrap();
    let bar_value = hashmap().get(&1).unwrap();

    println!("Function-style: entry for `0` is \"{}\"", foo_value);
    println!("Function-style: entry for `1` is \"{}\"", bar_value);

    assert_eq!(hashmap().get(&0), Some(&"foo"));
    assert_eq!(hashmap().get(&1), Some(&"bar"));

    println!("once_cell test completed successfully!");
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_once_cell_main() {
    test_once_cell();
}
