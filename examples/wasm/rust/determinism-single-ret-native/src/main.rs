fn main() {
    let res = determinism(1000);
    let mut floats = Vec::new();
    for float in res.chunks(8) {
        floats.push(f64::from_le_bytes(float.try_into().unwrap()));
    }
    println!("{:?}", floats);
}

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
