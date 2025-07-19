//#![no_std]
#[allow(warnings)]
mod bindings;

// The comments that follow the `use` declaration below
// correlate the rust module path segments with their
// `world.wit` counterparts:
//            <- items bundled with `export` keyword
//                     <- package namespace
//                           <- package
//                                  <- interface name
use std::collections::HashMap;

use bindings::exports::docs::adder::add_interface::Guest;
struct Component;

impl Guest for Component {
    fn add(kwargs: Vec<u8>) -> Vec<u8> {
        let mut seen_exts: HashMap<String, bool> = HashMap::new();
        let z = 3;
        seen_exts.insert(z.to_string(), false);
        seen_exts.insert(String::from("basic_constraints_is_valid"), true);
        let key = (kwargs[0] + kwargs[1]).to_string();
        if *seen_exts.get(&key).unwrap() { [10u8].to_vec() } else { [11u8].to_vec() }
    }

    fn substract(kwargs: Vec<u8>) -> Vec<u8> {
        [kwargs[0] - kwargs[1]].to_vec()
    }
}

bindings::export!(Component with_types_in bindings);
