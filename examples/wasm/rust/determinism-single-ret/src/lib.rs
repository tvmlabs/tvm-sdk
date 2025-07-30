//#![no_std]
#[allow(warnings)]
mod bindings;

// The comments that follow the `use` declaration below
// correlate the rust module path segments with their
// `world.wit` counterparts:
use bindings::exports::gosh::determinism::test_interface::Guest;
//            <- items bundled with `export` keyword
//                     <- package namespace
//                           <- package
//                                  <- interface name

struct Component;

impl Guest for Component {
    fn test(kwargs: Vec<u8>) -> Vec<u8> {
        determinism(
            ((usize::from(kwargs[0]) * 256 + usize::from(kwargs[1])) * 256
                + usize::from(kwargs[2]))
                * 256
                + usize::from(kwargs[3]),
        )
    }
}

// impl IntoBytes for f64 {
//     fn to_le_bytes(a: Self) -> Vec<u8> {
//         a.to_le_bytes().to_vec()
//     }
// }

fn determinism(its: usize) -> Vec<u8> {
    let iterations = its.into();
    let mut results = Vec::with_capacity(1);
    let mut r = [1f64];

    for _ in 0..iterations {
        // Use floating point arithmetic with tiny imprecisions
        let a = 0.1f64;
        let b = 0.2f64;
        let c = (a + b) * (std::f64::consts::PI / 3.0);
        let k = (c * a).sin() + (b * c).cos();
        r[0] = (k * r[0]) + k;

        // Perform some non-deterministic arithmetic operations
        results.push(r[0].clone());
    }
    let mut result_vec = Vec::<u8>::with_capacity(1000);
    for r in results {
        result_vec.append(&mut r.to_le_bytes().to_vec());
    }
    result_vec
}

bindings::export!(Component with_types_in bindings);
