// use std::fs::File;
// use std::io::{self, BufRead, Read};
// use std::path::Path;
// use std::error::Error;

pub const BASE_POINT: [u8; 32] = [
    9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

// Field element representation:
//
// Field elements are written as an array of signed, 64-bit limbs, least
// significant first. The value of the field element is:
//   x[0] + 2^26·x[1] + x^51·x[2] + 2^102·x[3] + ...
//
// i.e. the limbs are 26, 25, 26, 25, ... bits wide.

// Sum two numbers: output += in
fn fsum(in1: &[i64; 19], in2: &[i64; 19]) -> [i64; 19] {
    // fn fsum(output: &mut [i64; 10], input: &[i64; 10]) {
    let mut output: [i64; 19] = [0; 19];
    for i in (0..10).step_by(2) {
        output[i] = in1[i] + in2[i];
        output[i + 1] = in1[i + 1] + in2[i + 1];
    }
    output
}

fn fdifference(output: &mut [i64; 19], input: &[i64; 19]) {
    // fn fdifference(output: &mut [i64; 10], input: &[i64; 10]) {
    for i in 0..10 {
        output[i] = input[i] - output[i];
    }
}

fn fscalar_product(output: &mut [i64; 19], input: &[i64; 19], scalar: i64) {
    for i in 0..10 {
        output[i] = input[i] * scalar;
    }
}

fn fproduct(output: &mut [i64; 19], in2: &[i64; 19], in1: &[i64; 19]) {
    output[0] = (in2[0] as i32 as i64) * (in1[0] as i32 as i64);
    output[1] = (in2[0] as i32 as i64) * (in1[1] as i32 as i64)
        + (in2[1] as i32 as i64) * (in1[0] as i32 as i64);
    output[2] = 2 * ((in2[1] as i32 as i64) * (in1[1] as i32) as i64)
        + (in2[0] as i32 as i64) * (in1[2] as i32 as i64)
        + (in2[2] as i32 as i64) * (in1[0] as i32 as i64);
    output[3] = (in2[1] as i32 as i64) * (in1[2] as i32 as i64)
        + (in2[2] as i32 as i64) * (in1[1] as i32 as i64)
        + (in2[0] as i32 as i64) * (in1[3] as i32 as i64)
        + (in2[3] as i32 as i64) * (in1[0] as i32 as i64);
    output[4] = (in2[2] as i32 as i64) * (in1[2] as i32) as i64
        + 2 * ((in2[1] as i32 as i64) * (in1[3] as i32) as i64
            + (in2[3] as i32 as i64) * (in1[1] as i32) as i64)
        + (in2[0] as i32 as i64) * (in1[4] as i32) as i64
        + (in2[4] as i32 as i64) * (in1[0] as i32) as i64;
    output[5] = (in2[2] as i32 as i64) * (in1[3] as i32) as i64
        + (in2[3] as i32 as i64) * (in1[2] as i32) as i64
        + (in2[1] as i32 as i64) * (in1[4] as i32) as i64
        + (in2[4] as i32 as i64) * (in1[1] as i32) as i64
        + (in2[0] as i32 as i64) * (in1[5] as i32) as i64
        + (in2[5] as i32 as i64) * (in1[0] as i32) as i64;
    output[6] = 2
        * ((in2[3] as i32 as i64) * (in1[3] as i32) as i64
            + (in2[1] as i32 as i64) * (in1[5] as i32) as i64
            + (in2[5] as i32 as i64) * (in1[1] as i32) as i64)
        + (in2[2] as i32 as i64) * (in1[4] as i32) as i64
        + (in2[4] as i32 as i64) * (in1[2] as i32) as i64
        + (in2[0] as i32 as i64) * (in1[6] as i32) as i64
        + (in2[6] as i32 as i64) * (in1[0] as i32) as i64;
    output[7] = (in2[3] as i32 as i64) * (in1[4] as i32) as i64
        + (in2[4] as i32 as i64) * (in1[3] as i32) as i64
        + (in2[2] as i32 as i64) * (in1[5] as i32) as i64
        + (in2[5] as i32 as i64) * (in1[2] as i32) as i64
        + (in2[1] as i32 as i64) * (in1[6] as i32) as i64
        + (in2[6] as i32 as i64) * (in1[1] as i32) as i64
        + (in2[0] as i32 as i64) * (in1[7] as i32) as i64
        + (in2[7] as i32 as i64) * (in1[0] as i32) as i64;
    output[8] = (in2[4] as i32 as i64) * (in1[4] as i32) as i64
        + 2 * ((in2[3] as i32 as i64) * (in1[5] as i32) as i64
            + (in2[5] as i32 as i64) * (in1[3] as i32) as i64
            + (in2[1] as i32 as i64) * (in1[7] as i32) as i64
            + (in2[7] as i32 as i64) * (in1[1] as i32) as i64)
        + (in2[2] as i32 as i64) * (in1[6] as i32) as i64
        + (in2[6] as i32 as i64) * (in1[2] as i32) as i64
        + (in2[0] as i32 as i64) * (in1[8] as i32) as i64
        + (in2[8] as i32 as i64) * (in1[0] as i32) as i64;
    output[9] = (in2[4] as i32 as i64) * (in1[5] as i32) as i64
        + (in2[5] as i32 as i64) * (in1[4] as i32) as i64
        + (in2[3] as i32 as i64) * (in1[6] as i32) as i64
        + (in2[6] as i32 as i64) * (in1[3] as i32) as i64
        + (in2[2] as i32 as i64) * (in1[7] as i32) as i64
        + (in2[7] as i32 as i64) * (in1[2] as i32) as i64
        + (in2[1] as i32 as i64) * (in1[8] as i32) as i64
        + (in2[8] as i32 as i64) * (in1[1] as i32) as i64
        + (in2[0] as i32 as i64) * (in1[9] as i32) as i64
        + (in2[9] as i32 as i64) * (in1[0] as i32) as i64;
    output[10] = 2
        * ((in2[5] as i32 as i64) * (in1[5] as i32) as i64
            + (in2[3] as i32 as i64) * (in1[7] as i32) as i64
            + (in2[7] as i32 as i64) * (in1[3] as i32) as i64
            + (in2[1] as i32 as i64) * (in1[9] as i32) as i64
            + (in2[9] as i32 as i64) * (in1[1] as i32) as i64)
        + (in2[4] as i32 as i64) * (in1[6] as i32) as i64
        + (in2[6] as i32 as i64) * (in1[4] as i32) as i64
        + (in2[2] as i32 as i64) * (in1[8] as i32) as i64
        + (in2[8] as i32 as i64) * (in1[2] as i32) as i64;
    output[11] = (in2[5] as i32 as i64) * (in1[6] as i32) as i64
        + (in2[6] as i32 as i64) * (in1[5] as i32) as i64
        + (in2[4] as i32 as i64) * (in1[7] as i32) as i64
        + (in2[7] as i32 as i64) * (in1[4] as i32) as i64
        + (in2[3] as i32 as i64) * (in1[8] as i32) as i64
        + (in2[8] as i32 as i64) * (in1[3] as i32) as i64
        + (in2[2] as i32 as i64) * (in1[9] as i32) as i64
        + (in2[9] as i32 as i64) * (in1[2] as i32) as i64;
    output[12] = (in2[6] as i32 as i64) * (in1[6] as i32) as i64
        + 2 * ((in2[5] as i32 as i64) * (in1[7] as i32) as i64
            + (in2[7] as i32 as i64) * (in1[5] as i32) as i64
            + (in2[3] as i32 as i64) * (in1[9] as i32) as i64
            + (in2[9] as i32 as i64) * (in1[3] as i32) as i64)
        + (in2[4] as i32 as i64) * (in1[8] as i32) as i64
        + (in2[8] as i32 as i64) * (in1[4] as i32) as i64;
    output[13] = (in2[6] as i32 as i64) * (in1[7] as i32) as i64
        + (in2[7] as i32 as i64) * (in1[6] as i32) as i64
        + (in2[5] as i32 as i64) * (in1[8] as i32) as i64
        + (in2[8] as i32 as i64) * (in1[5] as i32) as i64
        + (in2[4] as i32 as i64) * (in1[9] as i32) as i64
        + (in2[9] as i32 as i64) * (in1[4] as i32) as i64;
    output[14] = 2
        * ((in2[7] as i32 as i64) * (in1[7] as i32) as i64
            + (in2[5] as i32 as i64) * (in1[9] as i32) as i64
            + (in2[9] as i32 as i64) * (in1[5] as i32) as i64)
        + (in2[6] as i32 as i64) * (in1[8] as i32) as i64
        + (in2[8] as i32 as i64) * (in1[6] as i32) as i64;
    output[15] = (in2[7] as i32 as i64) * (in1[8] as i32) as i64
        + (in2[8] as i32 as i64) * (in1[7] as i32) as i64
        + (in2[6] as i32 as i64) * (in1[9] as i32) as i64
        + (in2[9] as i32 as i64) * (in1[6] as i32) as i64;
    output[16] = (in2[8] as i32 as i64) * (in1[8] as i32) as i64
        + 2 * ((in2[7] as i32 as i64) * (in1[9] as i32) as i64
            + (in2[9] as i32 as i64) * (in1[7] as i32) as i64);
    output[17] = (in2[8] as i32 as i64) * (in1[9] as i32) as i64
        + (in2[9] as i32 as i64) * (in1[8] as i32) as i64;
    output[18] = 2 * ((in2[9] as i32 as i64) * (in1[9] as i32) as i64);
}

fn freduce_degree(output: &mut [i64; 19]) {
    output[8] += output[18] << 4;
    output[8] += output[18] << 1;
    output[8] += output[18];
    output[7] += output[17] << 4;
    output[7] += output[17] << 1;
    output[7] += output[17];
    output[6] += output[16] << 4;
    output[6] += output[16] << 1;
    output[6] += output[16];
    output[5] += output[15] << 4;
    output[5] += output[15] << 1;
    output[5] += output[15];
    output[4] += output[14] << 4;
    output[4] += output[14] << 1;
    output[4] += output[14];
    output[3] += output[13] << 4;
    output[3] += output[13] << 1;
    output[3] += output[13];
    output[2] += output[12] << 4;
    output[2] += output[12] << 1;
    output[2] += output[12];
    output[1] += output[11] << 4;
    output[1] += output[11] << 1;
    output[1] += output[11];
    output[0] += output[10] << 4;
    output[0] += output[10] << 1;
    output[0] += output[10];
}

// return v / 2^26, using only shifts and adds.
fn div_by_2_26(v: i64) -> i64 {
    // High word of v; no shift needed
    let highword = ((v as u64) >> 32) as u32;
    // Set to all 1s if v was negative; else set to 0s.
    let sign = (highword as i32) >> 31;
    // Set to 0x3ffffff if v was negative; else set to 0.
    let roundoff = (sign as u32) >> 6;
    // Should return v / (1<<26)
    (v + roundoff as i64) >> 26
}

// return v / (2^25), using only shifts and adds.
fn div_by_2_25(v: i64) -> i64 {
    let highword = ((v as u64) >> 32) as u32;
    let sign = (highword as i32) >> 31;
    let roundoff = (sign as u32) >> 7;
    (v + roundoff as i64) >> 25
}

fn div_s32_by_2_25(v: i32) -> i32 {
    let roundoff = (v >> 31) as u32 >> 7;
    (v + roundoff as i32) >> 25
}

// Reduce all coefficients of the short form input so that |x| < 2^26.
// On entry: |output[i]| < 2^62
fn freduce_coefficients(output: &mut [i64; 19]) {
    // fn freduce_coefficients(output: &mut [i64; 11]) {
    output[10] = 0;

    for i in (0..10).step_by(2) {
        let over = div_by_2_26(output[i]);
        output[i] -= over << 26;
        output[i + 1] += over;

        let over = div_by_2_25(output[i + 1]);
        output[i + 1] -= over << 25;
        output[i + 2] += over;
    }
    // Now |output[10]| < 2 ^ 38 and all other coefficients are reduced.
    output[0] += output[10] << 4;
    output[0] += output[10] << 1;
    output[0] += output[10];
    output[10] = 0;

    // Now output[1..9] are reduced, and |output[0]| < 2^26 + 19 * 2^38
    // So |over| will be no more than 77825
    let over = div_by_2_26(output[0]);
    output[0] -= over << 26;
    output[1] += over;

    // Now output[0,2..9] are reduced, and |output[1]| < 2^25 + 77825
    // So |over| will be no more than 1.
    let over32 = div_s32_by_2_25(output[1] as i32);
    output[1] -= (over32 as i64) << 25;
    output[2] += over32 as i64;

    // Finally, output[0,1,3..9] are reduced, and output[2] is "nearly reduced":
    // we have |output[2]| <= 2^26.  This is good enough for all of our math,
    // but it will require an extra freduce_coefficients before fcontract.
}

// A helpful wrapper around fproduct: output = in * in2.
//
// output must be distinct to both inputs. The output is reduced degree and
// reduced coefficient.
fn fmul(output: &mut [i64; 19], in_data: &[i64; 19], in2: &[i64; 19]) {
    let mut t = [0i64; 19];
    fproduct(&mut t, in_data, in2);
    freduce_degree(&mut t);
    freduce_coefficients(&mut t);
    output.copy_from_slice(&t); // output.copy_from_slice(&t[..10]);
}

fn fsquare_inner(output: &mut [i64; 19], input: &[i64; 19]) {
    output[0] = ((input[0] as i32) as i64) * ((input[0] as i32) as i64);
    output[1] = 2 * (input[0] as i32 as i64) * (input[1] as i32 as i64);
    output[2] = 2
        * (((input[1] as i32 as i64) * (input[1] as i32 as i64))
            + ((input[0] as i32 as i64) * (input[2] as i32 as i64)));
    output[3] = 2
        * (((input[1] as i32 as i64) * (input[2] as i32 as i64))
            + ((input[0] as i32 as i64) * (input[3] as i32 as i64)));
    output[4] = ((input[2] as i32 as i64) * (input[2] as i32 as i64))
        + 4 * ((input[1] as i32 as i64) * (input[3] as i32 as i64))
        + 2 * ((input[0] as i32 as i64) * (input[4] as i32 as i64));
    output[5] = 2
        * (((input[2] as i32 as i64) * (input[3] as i32 as i64))
            + ((input[1] as i32 as i64) * (input[4] as i32 as i64))
            + ((input[0] as i32 as i64) * (input[5] as i32 as i64)));
    output[6] = 2
        * (((input[3] as i32 as i64) * (input[3] as i32 as i64))
            + ((input[2] as i32 as i64) * (input[4] as i32 as i64))
            + ((input[0] as i32 as i64) * (input[6] as i32 as i64))
            + 2 * ((input[1] as i32 as i64) * (input[5] as i32 as i64)));
    output[7] = 2
        * (((input[3] as i32 as i64) * (input[4] as i32 as i64))
            + ((input[2] as i32 as i64) * (input[5] as i32 as i64))
            + ((input[1] as i32 as i64) * (input[6] as i32 as i64))
            + ((input[0] as i32 as i64) * (input[7] as i32 as i64)));
    output[8] = ((input[4] as i32 as i64) * (input[4] as i32 as i64))
        + 2 * (((input[2] as i32 as i64) * (input[6] as i32 as i64))
            + ((input[0] as i32 as i64) * (input[8] as i32 as i64))
            + 2 * (((input[1] as i32 as i64) * (input[7] as i32 as i64))
                + ((input[3] as i32 as i64) * (input[5] as i32 as i64))));
    output[9] = 2
        * (((input[4] as i32 as i64) * (input[5] as i32 as i64))
            + ((input[3] as i32 as i64) * (input[6] as i32 as i64))
            + ((input[2] as i32 as i64) * (input[7] as i32 as i64))
            + ((input[1] as i32 as i64) * (input[8] as i32 as i64))
            + ((input[0] as i32 as i64) * (input[9] as i32 as i64)));
    output[10] = 2
        * (((input[5] as i32 as i64) * (input[5] as i32 as i64))
            + ((input[4] as i32 as i64) * (input[6] as i32 as i64))
            + ((input[2] as i32 as i64) * (input[8] as i32 as i64))
            + 2 * (((input[3] as i32 as i64) * (input[7] as i32 as i64))
                + ((input[1] as i32 as i64) * (input[9] as i32 as i64))));
    output[11] = 2
        * (((input[5] as i32 as i64) * (input[6] as i32 as i64))
            + ((input[4] as i32 as i64) * (input[7] as i32 as i64))
            + ((input[3] as i32 as i64) * (input[8] as i32 as i64))
            + ((input[2] as i32 as i64) * (input[9] as i32 as i64)));
    output[12] = ((input[6] as i32 as i64) * (input[6] as i32 as i64))
        + 2 * (((input[4] as i32 as i64) * (input[8] as i32 as i64))
            + 2 * (((input[5] as i32 as i64) * (input[7] as i32 as i64))
                + ((input[3] as i32 as i64) * (input[9] as i32 as i64))));
    output[13] = 2
        * (((input[6] as i32 as i64) * (input[7] as i32 as i64))
            + ((input[5] as i32 as i64) * (input[8] as i32 as i64))
            + ((input[4] as i32 as i64) * (input[9] as i32 as i64)));
    output[14] = 2
        * (((input[7] as i32 as i64) * (input[7] as i32 as i64))
            + ((input[6] as i32 as i64) * (input[8] as i32 as i64))
            + 2 * ((input[5] as i32 as i64) * (input[9] as i32 as i64)));
    output[15] = 2
        * (((input[7] as i32 as i64) * (input[8] as i32 as i64))
            + ((input[6] as i32 as i64) * (input[9] as i32 as i64)));
    output[16] = ((input[8] as i32 as i64) * (input[8] as i32 as i64))
        + 4 * ((input[7] as i32 as i64) * (input[9] as i32 as i64));
    output[17] = 2 * ((input[8] as i32 as i64) * (input[9] as i32 as i64));
    output[18] = 2 * ((input[9] as i32 as i64) * (input[9] as i32 as i64));
}

fn fsquare(output: &mut [i64; 19], input: &[i64; 19]) {
    let mut t = [0i64; 19];
    fsquare_inner(&mut t, input);
    freduce_degree(&mut t);
    freduce_coefficients(&mut t);
    for i in 0..10 {
        output[i] = t[i];
    }
    // output.copy_from_slice(&t);//output.copy_from_slice(&t[..10]);
}

// Take a little-endian, 32-byte number and expand it into polynomial form
fn fexpand(output: &mut [i64; 19], input: &[u8; 32]) {
    macro_rules! F {
        ($n:expr, $start:expr, $shift:expr, $mask:expr) => {
            output[$n] = (((input[$start + 0] as i64)
                | ((input[$start + 1] as i64) << 8)
                | ((input[$start + 2] as i64) << 16)
                | ((input[$start + 3] as i64) << 24))
                >> $shift)
                & $mask;
        };
    }

    F!(0, 0, 0, 0x3ffffff);
    F!(1, 3, 2, 0x1ffffff);
    F!(2, 6, 3, 0x3ffffff);
    F!(3, 9, 5, 0x1ffffff);
    F!(4, 12, 6, 0x3ffffff);
    F!(5, 16, 0, 0x1ffffff);
    F!(6, 19, 1, 0x3ffffff);
    F!(7, 22, 3, 0x1ffffff);
    F!(8, 25, 4, 0x3ffffff);
    F!(9, 28, 6, 0x3ffffff);
}

fn fcontract(output: &mut [u8; 32], input: &mut [i64; 19]) {
    for _ in 0..2 {
        for i in 0..9 {
            if i % 2 == 1 {
                let mask = (input[i] as i32) >> 31;
                let carry = -(((input[i] as i32) & mask) >> 25);
                input[i] = (input[i] as i32 + (carry << 25)) as i64;
                input[i + 1] -= carry as i64;
            } else {
                let mask = (input[i] as i32) >> 31;
                let carry = -(((input[i] as i32) & mask) >> 26);
                input[i] = (input[i] as i32 + (carry << 26)) as i64;
                input[i + 1] = (input[i + 1] as i32 - carry) as i64;
            }
        }
        let mask = (input[9] as i32) >> 31;
        let carry = -(((input[9] as i32) & mask) >> 25);
        input[9] = (input[9] as i32 + (carry << 25)) as i64;
        input[0] -= (carry * 19) as i64;
    }

    let mask = (input[0] as i32) >> 31;
    let carry = -(((input[0] as i32) & mask) >> 26);
    input[0] = (input[0] as i32 + (carry << 26)) as i64;
    input[1] -= carry as i64;

    input[1] <<= 2;
    input[2] <<= 3;
    input[3] <<= 5;
    input[4] <<= 6;
    input[6] <<= 1;
    input[7] <<= 3;
    input[8] <<= 4;
    input[9] <<= 6;

    macro_rules! F {
        ($i:expr, $s:expr) => {
            output[$s + 0] |= input[$i] as u8 & 0xff;
            output[$s + 1] = (input[$i] >> 8) as u8 & 0xff;
            output[$s + 2] = (input[$i] >> 16) as u8 & 0xff;
            output[$s + 3] = (input[$i] >> 24) as u8 & 0xff;
        };
    }

    output[0] = 0;
    output[16] = 0;
    F!(0, 0);
    F!(1, 3);
    F!(2, 6);
    F!(3, 9);
    F!(4, 12);
    F!(5, 16);
    F!(6, 19);
    F!(7, 22);
    F!(8, 25);
    F!(9, 28);
}

fn fmonty(
    x2: &mut [i64; 19],
    z2: &mut [i64; 19], // output 2Q
    x3: &mut [i64; 19],
    z3: &mut [i64; 19], // output Q + Q'
    x: &mut [i64; 19],
    z: &mut [i64; 19], // input Q
    xprime: &mut [i64; 19],
    zprime: &mut [i64; 19], // input Q'
    qmqp: &[i64; 19],
) {
    //  input Q - Q'
    let mut origx = [0; 19]; // let mut origx = [0; 10];
    let mut origxprime = [0; 19]; // let mut origxprime = [0; 10];
    let mut zzz = [0; 19];
    let mut xx = [0; 19];
    let mut zz = [0; 19];
    let mut xxprime = [0; 19];
    let mut zzprime = [0; 19];
    let mut zzzprime = [0i64; 19];
    let mut xxxprime = [0; 19];

    origx.copy_from_slice(x); // copy only 10 items!!!
    let x = fsum(x, z); //fsum(x, z);
    fdifference(z, &origx); // z becomes x - z

    origxprime.copy_from_slice(xprime); // Copy xprime to origxprime
    let xprime = fsum(xprime, zprime); //fsum(xprime, zprime);
    fdifference(zprime, &origxprime); // zprime becomes xprime - zprime
    fproduct(&mut xxprime, &xprime, z);
    fproduct(&mut zzprime, &x, zprime);
    freduce_degree(&mut xxprime);
    freduce_coefficients(&mut xxprime);

    freduce_degree(&mut zzprime);
    freduce_coefficients(&mut zzprime);
    origxprime.copy_from_slice(&xxprime); // Copy xxprime to origxprime
    let xxprime = fsum(&xxprime, &zzprime); //fsum(&mut xxprime, &zzprime);

    fdifference(&mut zzprime, &origxprime); // zzprime becomes xxprime - zzprime
    fsquare(&mut xxxprime, &xxprime);
    fsquare(&mut zzzprime, &zzprime);
    fproduct(&mut zzprime, &zzzprime, qmqp);
    freduce_degree(&mut zzprime);
    freduce_coefficients(&mut zzprime);

    x3.copy_from_slice(&xxxprime); // Copy xxxprime to x3
    z3.copy_from_slice(&zzprime); // Copy zzprime to z3

    // Squares x and z
    fsquare(&mut xx, &x);
    fsquare(&mut zz, z);
    fproduct(x2, &xx, &zz);
    freduce_degree(x2);
    freduce_coefficients(x2);
    fdifference(&mut zz, &xx); // does zz = xx - zz
    zzz[10..19].fill(0); // Reset the last 9 elements of zzz //memset(zzz + 10, 0, sizeof(limb) * 9);
    fscalar_product(&mut zzz, &zz, 121665);

    // No need to call freduce_degree here:
    // fscalar_product doesn't increase the degree of its input.
    freduce_coefficients(&mut zzz);
    let zzz = fsum(&zzz, &xx); //fsum(&mut zzz, &xx);
    fproduct(z2, &zz, &zzz);
    freduce_degree(z2);
    freduce_coefficients(z2);
}

fn swap_conditional(a: &mut [i64; 19], b: &mut [i64; 19], iswap: i64) {
    let swap = (-iswap) as i32;
    for i in 0..10 {
        let x = swap & ((a[i] as i32) ^ (b[i] as i32));
        // let aias = (a[i] as i32);
        // let bias = (b[i] as i32);

        a[i] = (a[i] as i32 ^ x) as i64;
        b[i] = (b[i] as i32 ^ x) as i64;
    }
}

fn cmult(resultx: &mut [i64; 19], resultz: &mut [i64; 19], n: &[u8; 32], q: &[i64; 19]) {
    let mut a = [0; 19];
    let mut b = [0; 19];
    b[0] = 1; //[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut c = [0; 19];
    c[0] = 1; //[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut d = [0; 19];

    let mut nqpqx = &mut a;
    let mut nqpqz = &mut b;
    let mut nqx = &mut c;
    let mut nqz = &mut d;

    let mut e = [0; 19];
    let mut f = [0; 19];
    f[0] = 1; //[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut g = [0; 19];
    let mut h = [0; 19];
    h[0] = 1; //[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    let mut nqpqx2 = &mut e;
    let mut nqpqz2 = &mut f;
    let mut nqx2 = &mut g;
    let mut nqz2 = &mut h;

    let mut byte: u8;

    // Copy q to nqpqx
    nqpqx.copy_from_slice(q);

    for i in 0..32 {
        byte = n[31 - i];
        for _ in 0..8 {
            let bit = (byte >> 7) as i64;

            swap_conditional(nqx, nqpqx, bit);

            swap_conditional(nqz, nqpqz, bit);

            fmonty(nqx2, nqz2, nqpqx2, nqpqz2, nqx, nqz, nqpqx, nqpqz, q);

            // let _ = io::stdin().read_line(&mut String::new());

            swap_conditional(nqx2, nqpqx2, bit);
            swap_conditional(nqz2, nqpqz2, bit);

            // Swap references
            std::mem::swap(&mut nqx, &mut nqx2);
            std::mem::swap(&mut nqz, &mut nqz2);
            std::mem::swap(&mut nqpqx, &mut nqpqx2);
            std::mem::swap(&mut nqpqz, &mut nqpqz2);

            byte <<= 1;
        }
    }

    resultx.copy_from_slice(nqx); // need copy only 10 items!!!
    resultz.copy_from_slice(nqz); // need copy only 10 items!!!
}

fn crecip(out: &mut [i64; 19], z: &[i64; 19]) {
    let mut z2 = [0; 19];
    let mut z9 = [0; 19];
    let mut z11 = [0; 19];
    let mut z2_5_0 = [0; 19];
    let mut z2_10_0 = [0; 19];
    let mut z2_20_0 = [0; 19];
    let mut z2_50_0 = [0; 19];
    let mut z2_100_0 = [0; 19];
    let mut t0 = [0; 19];
    let mut t1 = [0; 19];

    fsquare(&mut z2, z); // 2
    fsquare(&mut t1, &z2); // 4
    fsquare(&mut t0, &t1); // 8
    fmul(&mut z9, &t0, z); // 9
    fmul(&mut z11, &z9, &z2); // 11
    fsquare(&mut t0, &z11); // 22
    fmul(&mut z2_5_0, &t0, &z9); // 2^5 - 2^0 = 31

    fsquare(&mut t0, &z2_5_0); //  2^6 - 2^1
    fsquare(&mut t1, &t0);
    fsquare(&mut t0, &t1);
    fsquare(&mut t1, &t0);
    fsquare(&mut t0, &t1);
    fmul(&mut z2_10_0, &t0, &z2_5_0);

    fsquare(&mut t0, &z2_10_0);
    fsquare(&mut t1, &t0);
    for _ in (2..10).step_by(2) {
        fsquare(&mut t0, &t1);
        fsquare(&mut t1, &t0);
    }
    fmul(&mut z2_20_0, &t1, &z2_10_0);

    fsquare(&mut t0, &z2_20_0);
    fsquare(&mut t1, &t0);
    for _ in (2..20).step_by(2) {
        fsquare(&mut t0, &t1);
        fsquare(&mut t1, &t0);
    }
    fmul(&mut t0, &t1, &z2_20_0);

    fsquare(&mut t1, &t0);
    fsquare(&mut t0, &t1);
    for _ in (2..10).step_by(2) {
        fsquare(&mut t1, &t0);
        fsquare(&mut t0, &t1);
    }
    fmul(&mut z2_50_0, &t0, &z2_10_0);

    fsquare(&mut t0, &z2_50_0);
    fsquare(&mut t1, &t0);
    for _ in (2..50).step_by(2) {
        fsquare(&mut t0, &t1);
        fsquare(&mut t1, &t0);
    }
    fmul(&mut z2_100_0, &t1, &z2_50_0);

    fsquare(&mut t1, &z2_100_0);
    fsquare(&mut t0, &t1);
    for _ in (2..100).step_by(2) {
        fsquare(&mut t1, &t0);
        fsquare(&mut t0, &t1);
    }
    fmul(&mut t1, &t0, &z2_100_0);

    fsquare(&mut t0, &t1);
    fsquare(&mut t1, &t0);
    for _ in (2..50).step_by(2) {
        fsquare(&mut t0, &t1);
        fsquare(&mut t1, &t0);
    }
    fmul(&mut t0, &t1, &z2_50_0);
    fsquare(&mut t1, &t0);
    fsquare(&mut t0, &t1); // 2^252 - 2^2
    fsquare(&mut t1, &t0); // 2^253 - 2^3
    fsquare(&mut t0, &t1); // 2^254 - 2^4
    fsquare(&mut t1, &t0); // 2^255 - 2^5

    fmul(out, &t1, &z11); /* 2^255 - 21 */
}

// pub fn curve25519_donna(mypublic: &mut [u8; 32], secret: &[u8; 32],
// basepoint: &[u8; 32]) -> i32 { let mut bp = [0; 10];
// let mut x = [0; 10];
// let mut z = [0; 10];
// let mut zmone = [0; 10];
// let mut e = [0; 32];
//
// e.copy_from_slice(secret);
// e[0] &= 248;
// e[31] &= 127;
// e[31] |= 64;
//
// fexpand(&mut bp, basepoint);
// cmult(&mut x, &mut z, &e, &bp);
// crecip(&mut zmone, &z);
// fmul(&mut z, &x, &zmone);
// freduce_coefficients(&mut z);
// fcontract(mypublic, &z);
// return 0;
// }

pub fn curve25519_donna(secret: &[u8; 32], basepoint: &[u8; 32]) -> Result<[u8; 32], Vec<u8>> {
    // error codes [4][1]...[4][255]
    let mut bp = [0; 19];
    let mut x = [0; 19];
    let mut z = [0; 19];
    let mut zmone = [0; 19];
    let mut e = [0; 32];

    e.copy_from_slice(secret);
    e[0] &= 248;
    e[31] &= 127;
    e[31] |= 64;

    let mut mypublic: [u8; 32] = [0u8; 32];
    fexpand(&mut bp, basepoint);

    cmult(&mut x, &mut z, &e, &bp);
    crecip(&mut zmone, &z);

    fmul(&mut z, &x, &zmone);
    freduce_coefficients(&mut z);
    fcontract(&mut mypublic, &mut z);
    Ok(mypublic)
}

fn base64_value(c: char) -> u16 {
    if c >= 'A' && c <= 'Z' {
        return ((c as u8) - b'A') as u16;
    }
    if c >= 'a' && c <= 'z' {
        return (26 + (c as u8) - b'a') as u16;
    }
    if c >= '0' && c <= '9' {
        return (52 + (c as u8) - b'0') as u16;
    }
    if c == '+' {
        return 62 as u16;
    }
    if c == '/' {
        return 63 as u16;
    }
    return 0x1000; // Error value
}

fn base64_decode(data: &mut Vec<u8>, len: &mut usize) {
    let mut read = 0;
    let mut write = 0;
    let mut state = [0u16; 4]; // State buffer

    while read < *len {
        state[read % 4] = base64_value(data[read] as char);
        if state[read % 4] == 0x1000 {
            break;
        }
        if (read % 4) == 3 {
            data[write] = ((state[0] << 2) | (state[1] >> 4)) as u8;
            write += 1;
            data[write] = ((state[1] << 4) | (state[2] >> 2)) as u8;
            write += 1;
            data[write] = ((state[2] << 6) | state[3]) as u8;
            write += 1;
        }
        read += 1;
    }

    match read % 4 {
        2 => {
            data[write] = ((state[0] << 2) | (state[1] >> 4)) as u8;
            write += 1;
        }
        3 => {
            data[write] = ((state[0] << 2) | (state[1] >> 4)) as u8;
            write += 1;
            data[write] = ((state[1] << 4) | (state[2] >> 2)) as u8;
            write += 1;
        }
        _ => {}
    }

    *len = write;
}

// fn read_key(filename: &str, key: &mut [u8; 32]) -> Result<(), Box<dyn Error>>
// { let file = File::open(filename)?;
// let reader = io::BufReader::new(file);
//
// let mut lines = reader.lines();
//
// Read the first line
// if let Some(line) = lines.next() {
// let line = line?;
// if !line.starts_with("-----BEGIN ") {
// return Err(format!("File {} is not a PEM file", filename).into());
// }
// }
//
// Read the second line
// if let Some(line) = lines.next() {
// let line = line?;
// let line = line.trim(); // Remove new line characters
// let mut base64_data: Vec<u8> = line.bytes().collect();
//
// Decode base64
// let mut len = base64_data.len();
// base64_decode(&mut base64_data, &mut len);
//
// if len < 32 {
// return Err(format!("Short read from {}", filename).into());
// }
//
// Copy the last 32 bytes to the key
// key.copy_from_slice(&base64_data[(len - 32)..len]);
// }
//
// Ok(())
// }

//
//$ cc -o curve25519-mult curve25519-mult.c
// $ ./curve25519-mult server-ephemeral-private.key \
//                     client-ephemeral-public.key | hexdump
//
// 0000000 df 4a 29 1b aa 1e b7 cf a6 93 4b 29 b4 74 ba ad
// 0000010 26 97 e2 9f 1f 92 0d cc 77 c8 a0 a0 88 44 76 24
//
// df4a291baa1eb7cfa6934b29b474baad2697e29f1f920dcc77c8a0a088447624
