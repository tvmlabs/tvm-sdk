#[allow(warnings)]
mod bindings;

use bindings::Guest;

struct Component;

impl Guest for Component {
    /// Say hello!
    fn hello_world() -> String {
        "Hello, World!".to_string()
    }

    fn answer() -> i32 {
        42i32
    }
}

bindings::export!(Component with_types_in bindings);
