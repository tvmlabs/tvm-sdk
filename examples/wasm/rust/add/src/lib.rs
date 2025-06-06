//#![no_std]
#[allow(warnings)]
mod bindings;

// The comments that follow the `use` declaration below
// correlate the rust module path segments with their
// `world.wit` counterparts:
use bindings::exports::docs::adder::add::Guest;
//            <- items bundled with `export` keyword
//                     <- package namespace
//                           <- package
//                                  <- interface name

struct Component;

impl Guest for Component {
    fn add(kwargs: Vec<u8>) -> Vec<u8> {
        [kwargs[0] + kwargs[1]].to_vec()
    }
}

bindings::export!(Component with_types_in bindings);
