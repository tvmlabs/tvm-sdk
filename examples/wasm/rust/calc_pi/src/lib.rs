//#![no_std]
#[allow(warnings)]
mod bindings;

// The comments that follow the `use` declaration below
// correlate the rust module path segments with their
// `world.wit` counterparts:
use bindings::exports::docs::calc_pi::calc_interface::Guest;
//            <- items bundled with `export` keyword
//                     <- package namespace
//                           <- package
//                                  <- interface name
mod calc;

struct Component;

impl Guest for Component {
    fn add(kwargs: Vec<u8>) -> Vec<u8> {
        // let digits: u64 = (kwargs[0]) << 2 + kwargs[1];
        let number = u64::from_be_bytes([0, 0, 0, 0, 0, 0, kwargs[0], kwargs[1]]);
        calc::calc::pi(number).as_bytes().to_vec()
        //[kwargs[0] + kwargs[1]].to_vec()
    }
}

bindings::export!(Component with_types_in bindings);
