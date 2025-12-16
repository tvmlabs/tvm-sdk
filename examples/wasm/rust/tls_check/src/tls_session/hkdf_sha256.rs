// use std::error::Error;
// use std::io::{self, Read};

const SIZE: usize = 32;

const BLOCK_SIZE: usize = 64;

const CHUNK: usize = 64;
const INIT_0: u32 = 0x6A09E667;
const INIT_1: u32 = 0xBB67AE85;
const INIT_2: u32 = 0x3C6EF372;
const INIT_3: u32 = 0xA54FF53A;
const INIT_4: u32 = 0x510E527F;
const INIT_5: u32 = 0x9B05688C;
const INIT_6: u32 = 0x1F83D9AB;
const INIT_7: u32 = 0x5BE0CD19;
// const INIT_0_224: u32 = 0xC1059ED8;
// const INIT_1_224: u32 = 0x367CD507;
// const INIT_2_224: u32 = 0x3070DD17;
// const INIT_3_224: u32 = 0xF70E5939;
// const INIT_4_224: u32 = 0xFFC00B31;
// const INIT_5_224: u32 = 0x68581511;
// const INIT_6_224: u32 = 0x64F98FA7;
// const INIT_7_224: u32 = 0xBEFA4FA4;

const MAGIC224: &[u8] = b"sha\x02";
const MAGIC256: &[u8] = b"sha\x03";
const MARSHALLED_SIZE: usize = MAGIC256.len() + 8 * 4 + CHUNK + 8;

fn append_uint32(b: &mut Vec<u8>, v: u32) {
    b.push((v >> 24) as u8);
    b.push((v >> 16) as u8);
    b.push((v >> 8) as u8);
    b.push(v as u8);
}

pub fn append_uint64(b: &mut Vec<u8>, v: u64) {
    b.push((v >> 56) as u8);
    b.push((v >> 48) as u8);
    b.push((v >> 40) as u8);
    b.push((v >> 32) as u8);
    b.push((v >> 24) as u8);
    b.push((v >> 16) as u8);
    b.push((v >> 8) as u8);
    b.push(v as u8);
}

// fn consume_uint64(b: &[u8]) -> (&[u8], u64) {
// let x = ((b[7] as u64) |
// ((b[6] as u64) << 8) |
// ((b[5] as u64) << 16) |
// ((b[4] as u64) << 24) |
// ((b[3] as u64) << 32) |
// ((b[2] as u64) << 40) |
// ((b[1] as u64) << 48) |
// ((b[0] as u64) << 56));
// (&b[8..], x)
// }
//
// fn consume_uint32(b: &[u8]) -> (&[u8], u32) {
// let x = ((b[3] as u32) | ((b[2] as u32) << 8) | ((b[1] as u32) << 16) |
// ((b[0] as u32) << 24)); (&b[4..], x)
// }

// fn consume_uint64(b: &mut &[u8]) -> Result<u64, Box<dyn Error>> {
fn consume_uint64(b: &mut &[u8]) -> u64 {
    if b.len() < 8 {
        // return Err("Not enough bytes for u64".into());
        panic!("Not enough bytes for u64");
    }
    let result = (b[7] as u64)
        | ((b[6] as u64) << 8)
        | ((b[5] as u64) << 16)
        | ((b[4] as u64) << 24)
        | ((b[3] as u64) << 32)
        | ((b[2] as u64) << 40)
        | ((b[1] as u64) << 48)
        | ((b[0] as u64) << 56);
    *b = &b[8..];
    // Ok(result)
    result
}

// fn consume_uint32(b: &mut &[u8]) -> Result<u32, Box<dyn Error>> {
fn consume_uint32(b: &mut &[u8]) -> u32 {
    if b.len() < 4 {
        // return Err("Not enough bytes for u32".into());
        panic!("Not enough bytes for u32")
    }
    let result =
        (b[3] as u32) | ((b[2] as u32) << 8) | ((b[1] as u32) << 16) | ((b[0] as u32) << 24);
    *b = &b[4..];
    // Ok(result)
    result
}

fn put_uint32(b: &mut [u8], v: u32) {
    b[0] = (v >> 24) as u8;
    b[1] = (v >> 16) as u8;
    b[2] = (v >> 8) as u8;
    b[3] = v as u8;
}

fn put_uint64(b: &mut [u8], v: u64) {
    b[0] = (v >> 56) as u8;
    b[1] = (v >> 48) as u8;
    b[2] = (v >> 40) as u8;
    b[3] = (v >> 32) as u8;
    b[4] = (v >> 24) as u8;
    b[5] = (v >> 16) as u8;
    b[6] = (v >> 8) as u8;
    b[7] = v as u8;
}

#[derive(Clone)]
pub struct Digest {
    h: [u32; 8],
    x: [u8; CHUNK],
    nx: usize,
    len: u64,
    // is224: bool,
}

impl Digest {
    pub fn marshal_binary(&self) -> Result<Vec<u8>, &'static str> {
        let mut b = Vec::with_capacity(MARSHALLED_SIZE);
        // if self.is224 {
        // b.extend_from_slice(MAGIC224);
        //} else {
        b.extend_from_slice(MAGIC256);
        //}

        for &hash in &self.h {
            append_uint32(&mut b, hash);
        }

        b.extend_from_slice(&self.x[..self.nx]);
        // b.truncate(b.len() + self.x.len() - self.nx); // already zero
        b.resize(b.len() + self.x.len() - self.nx, 0); // already zero
        append_uint64(&mut b, self.len);

        Ok(b)
    }

    // fn unmarshal_binary(&mut self, b: &[u8]) -> Result<(), Box<dyn Error>> {
    fn unmarshal_binary(&mut self, b: &[u8]) {
        // if &b[..MAGIC256.len()] != MAGIC256 //b.len() < MAGIC224.len()
        //|| (self.is224 && &b[..MAGIC224.len()] != MAGIC224)
        //|| (!self.is224 && &b[..MAGIC256.len()] != MAGIC256)
        //{
        // return Err("crypto/sha256: invalid hash state identifier".into());
        //}
        // if b.len() != MARSHALLED_SIZE {
        // return Err("crypto/sha256: invalid hash state size".into());
        //}
        let mut b = &b[MAGIC224.len()..];

        self.h[0] = consume_uint32(&mut b); // self.h[0] = consume_uint32(&mut b)?;
        self.h[1] = consume_uint32(&mut b); // self.h[1] = consume_uint32(&mut b)?;
        self.h[2] = consume_uint32(&mut b); // self.h[2] = consume_uint32(&mut b)?;
        self.h[3] = consume_uint32(&mut b); // self.h[3] = consume_uint32(&mut b)?;
        self.h[4] = consume_uint32(&mut b); // self.h[4] = consume_uint32(&mut b)?;
        self.h[5] = consume_uint32(&mut b); // self.h[5] = consume_uint32(&mut b)?;
        self.h[6] = consume_uint32(&mut b); // self.h[6] = consume_uint32(&mut b)?;
        self.h[7] = consume_uint32(&mut b); // self.h[7] = consume_uint32(&mut b)?;

        let copied = b.len().min(CHUNK);
        self.x[..copied].copy_from_slice(&b[..copied]);
        b = &b[copied..];

        self.len = consume_uint64(&mut b); // self.len = consume_uint64(&mut b)?;
        self.nx = (self.len % (CHUNK as u64)) as usize;

        // Ok(())
    }

    pub fn reset(&mut self) {
        self.h[0] = INIT_0;
        self.h[1] = INIT_1;
        self.h[2] = INIT_2;
        self.h[3] = INIT_3;
        self.h[4] = INIT_4;
        self.h[5] = INIT_5;
        self.h[6] = INIT_6;
        self.h[7] = INIT_7;

        self.nx = 0;
        self.len = 0;
    }

    pub fn new() -> Digest {
        let mut d = Digest {
            h: [0; 8],
            x: [0; CHUNK],
            nx: 0,
            len: 0,
            // is224: false,
        };
        d.reset();
        d
    }

    pub fn size(&self) -> usize {
        // if !self.is224 {
        SIZE
        //} else {
        // SIZE_224
        //}
    }

    pub fn block_size(&self) -> usize {
        BLOCK_SIZE
    }

    // fn write(&mut self, p: &[u8]) -> (usize, Option<std::io::Error>) {
    // fn write(&mut self, p: &[u8]) -> Result<usize, std::io::Error> {
    pub fn write(&mut self, p: &[u8]) -> usize {
        let nn = p.len();
        self.len += nn as u64;

        let mut remaining = p; // Remaining data to be written

        // If there is data in nx, terminate it
        if self.nx > 0 {
            let n = remaining.len().min(CHUNK - self.nx);
            self.x[self.nx..self.nx + n].copy_from_slice(&remaining[..n]);
            self.nx += n;
            if self.nx == CHUNK {
                // let selfx = &self.x.clone();//[0u8;CHUNK];
                // Processing a full block
                block(self, &self.x.clone()); // block(self, &self.x);
                self.nx = 0;
            }
            remaining = &remaining[n..];
        }

        if remaining.len() >= CHUNK {
            let n = remaining.len() & !(CHUNK - 1);
            block(self, &remaining[..n]);
            remaining = &remaining[n..];
        }

        if !remaining.is_empty() {
            self.nx = remaining.len();
            self.x[..self.nx].copy_from_slice(remaining);
        }

        nn // (nn, None)
    }

    pub fn sum(&self, in_bytes: &[u8]) -> Vec<u8> {
        // make a copy of self so that the caller can continue writing and summing.
        let mut d0 = self.clone();
        let hash = d0.check_sum();
        // if d0.is224 {
        //[in_bytes, &hash[..SIZE_224]].concat()
        //} else {
        [in_bytes, &hash].concat()
        //}
    }

    fn check_sum(&mut self) -> [u8; SIZE] {
        let len = self.len;
        // Padding. Add a 1 bit and 0 bits until 56 bytes mod 64.
        let mut tmp = [0u8; 64 + 8]; // padding + length buffer
        tmp[0] = 0x80;

        let t = if len % 64 < 56 { 56 - len % 64 } else { 64 + 56 - len % 64 };

        let len_in_bits = len << 3;
        let padlen = &mut tmp[..(t as usize) + 8];

        put_uint64(&mut padlen[(t as usize)..], len_in_bits);
        self.write(&padlen);

        if self.nx != 0 {
            panic!("d.nx != 0");
        }

        let mut digest = [0u8; SIZE];

        put_uint32(&mut digest[0..4], self.h[0]);
        put_uint32(&mut digest[4..8], self.h[1]);
        put_uint32(&mut digest[8..12], self.h[2]);
        put_uint32(&mut digest[12..16], self.h[3]);
        put_uint32(&mut digest[16..20], self.h[4]);
        put_uint32(&mut digest[20..24], self.h[5]);
        put_uint32(&mut digest[24..28], self.h[6]);
        // if !self.is224 {
        put_uint32(&mut digest[28..32], self.h[7]);
        //}

        digest
    }
}

// Sum256 returns the SHA256 checksum of the data.
pub fn sum256(data: &[u8]) -> [u8; SIZE] {
    let mut d = Digest::new();
    d.reset();
    d.write(data);
    d.check_sum()
}

const K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

fn rotate_left_32(x: u32, k: i32) -> u32 {
    const N: u32 = 32;
    let s = (k as u32) & (N - 1);
    (x << s) | (x >> (N - s))
}

fn block(dig: &mut Digest, p: &[u8]) {
    let mut w: [u32; 64] = [0; 64];
    let (mut h0, mut h1, mut h2, mut h3, mut h4, mut h5, mut h6, mut h7) =
        (dig.h[0], dig.h[1], dig.h[2], dig.h[3], dig.h[4], dig.h[5], dig.h[6], dig.h[7]);

    let chunk = CHUNK;
    let mut pos = 0;

    while pos + chunk <= p.len() {
        for i in 0..16 {
            let j = i * 4;
            w[i] = (p[pos + j] as u32) << 24
                | (p[pos + j + 1] as u32) << 16
                | (p[pos + j + 2] as u32) << 8
                | (p[pos + j + 3] as u32);
        }
        for i in 16..64 {
            let v1 = w[i - 2];
            let t1 = rotate_left_32(v1, -17) ^ rotate_left_32(v1, -19) ^ (v1 >> 10);
            let v2 = w[i - 15];
            let t2 = rotate_left_32(v2, -7) ^ rotate_left_32(v2, -18) ^ (v2 >> 3);
            w[i] = t1.wrapping_add(w[i - 7]).wrapping_add(t2).wrapping_add(w[i - 16]);
        }
        let (mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h) =
            (h0, h1, h2, h3, h4, h5, h6, h7);
        for i in 0..64 {
            let t1 = h
                .wrapping_add(
                    rotate_left_32(e, -6) ^ rotate_left_32(e, -11) ^ rotate_left_32(e, -25),
                )
                .wrapping_add((e & f) ^ (!e & g))
                .wrapping_add(K[i])
                .wrapping_add(w[i]);

            let t2 = (rotate_left_32(a, -2) ^ rotate_left_32(a, -13) ^ rotate_left_32(a, -22))
                .wrapping_add((a & b) ^ (a & c) ^ (b & c));

            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(t1);
            d = c;
            c = b;
            b = a;
            a = t1.wrapping_add(t2);
        }
        h0 = h0.wrapping_add(a);
        h1 = h1.wrapping_add(b);
        h2 = h2.wrapping_add(c);
        h3 = h3.wrapping_add(d);
        h4 = h4.wrapping_add(e);
        h5 = h5.wrapping_add(f);
        h6 = h6.wrapping_add(g);
        h7 = h7.wrapping_add(h);

        pos += chunk;
    }
    dig.h[0] = h0;
    dig.h[1] = h1;
    dig.h[2] = h2;
    dig.h[3] = h3;
    dig.h[4] = h4;
    dig.h[5] = h5;
    dig.h[6] = h6;
    dig.h[7] = h7;
}

//============================================
// hmac
pub struct Hmac {
    opad: Vec<u8>,
    ipad: Vec<u8>,
    outer: Digest,
    inner: Digest,
    marshaled: bool,
}

impl Hmac {
    pub fn new(key: &[u8]) -> Hmac {
        let mut hmac = Hmac {
            opad: Vec::new(),
            ipad: Vec::new(),
            outer: Digest::new(), // outer: hash_function(),
            inner: Digest::new(), // inner: hash_function(),
            marshaled: false,
        };

        let blocksize = hmac.inner.block_size();
        hmac.ipad.resize(blocksize, 0);
        hmac.opad.resize(blocksize, 0);

        let mut key_copy = key.to_vec();
        if key.len() > blocksize {
            hmac.outer.write(key);
            key_copy = hmac.outer.sum(&[]);
        }

        hmac.ipad.splice(0..key_copy.len(), key_copy.iter().cloned());
        hmac.opad.splice(0..key_copy.len(), key_copy.iter().cloned());
        for byte in hmac.ipad.iter_mut() {
            *byte ^= 0x36;
        }
        for byte in hmac.opad.iter_mut() {
            *byte ^= 0x5c;
        }
        hmac.inner.write(&hmac.ipad);

        hmac
    }

    pub fn sum(&mut self, input: &[u8]) -> Vec<u8> {
        let orig_len = input.len();
        let mut result = Vec::with_capacity(orig_len + self.inner.size());
        result.extend_from_slice(&self.inner.sum(input));

        if self.marshaled {
            self.outer.unmarshal_binary(&self.opad); // self.outer = self.outer.unmarshal_binary(&self.opad);
        } else {
            self.outer.reset();
            self.outer.write(&self.opad);
        }

        self.outer.write(&result[orig_len..]);
        let result = self.outer.sum(&result[..orig_len]);
        result
    }

    // fn write(&mut self, input: &[u8]) -> Result<usize, std::io::Error> {
    pub fn write(&mut self, input: &[u8]) -> usize {
        self.inner.write(input)
    }

    pub fn size(&self) -> usize {
        self.outer.size()
    }

    pub fn block_size(&self) -> usize {
        self.inner.block_size()
    }

    pub fn reset(&mut self) {
        if self.marshaled {
            // if let Err(err) = self.inner.unmarshal_binary(&self.ipad) {
            // panic!("{}", err);
            //}
            self.inner.unmarshal_binary(&self.ipad);
            return;
        }

        self.inner.reset();
        self.inner.write(&self.ipad);

        // Attempt to marshal the inner and outer hashes
        if let Ok(inner_marshalable) = self.inner.marshal_binary() {
            self.outer.reset();
            self.outer.write(&self.opad);
            if let Ok(outer_marshalable) = self.outer.marshal_binary() {
                // Marshaling succeeded; save the marshaled state for later
                self.ipad = inner_marshalable;
                self.opad = outer_marshalable;
                self.marshaled = true;
            }
        }
    }

    pub fn equal(mac1: &[u8], mac2: &[u8]) -> bool {
        mac1.len() == mac2.len() && mac1.iter().zip(mac2).all(|(a, b)| a == b)
    }
}

//========================================================

pub fn extract(secret: &[u8; 32], salt: &[u8; 32]) -> Result<[u8; 32], Vec<u8>> {
    // let salt = salt.unwrap_or(&vec![0; 32]);
    let mut extractor = Hmac::new(salt);
    extractor.write(secret);
    Ok(extractor.sum(&[]).try_into().unwrap())
}

pub struct Hkdf {
    expander: Hmac, // expander: HmacSha256,
    size: usize,
    info: Vec<u8>,
    counter: u8,
    prev: Vec<u8>,
    buf: Vec<u8>,
}

impl Hkdf {
    // fn new(expander: HmacSha256, info: Vec<u8>) -> Self {
    pub fn new(expander: Hmac, info: Vec<u8>) -> Self {
        let size = expander.size(); // let size = expander.output_size();
        Self { expander, size, info, counter: 1, prev: Vec::new(), buf: Vec::new() }
    }

    //

    // fn read(&mut self, p: &mut [u8]) -> io::Result<usize> {
    // pub fn read(&mut self, p: &mut [u8]) -> usize {
    pub fn read(&mut self, need: usize) -> Result<Vec<u8>, Vec<u8>> {
        // Check whether enough data can be generated
        // let need = p.len();
        let remains = self.buf.len() + (255 - self.counter as usize + 1) * self.size;

        let mut p = vec![0u8; need];
        if remains < need {
            // return Err(io::Error::new(io::ErrorKind::Other, "hkdf: entropy limit
            // reached")); return 0;
            // return p;
            return Err(vec![0u8, 8u8, 1u8]);
        }

        let n = self.buf.len().min(p.len());
        p[..n].copy_from_slice(&self.buf[..n]);
        self.buf = self.buf[n..].to_vec();

        while p.len() > n {
            self.expander.reset();
            self.expander.write(&self.prev); //self.expander.update(&self.prev);
            self.expander.write(&self.info); //self.expander.update(&self.info);
            self.expander.write(&[self.counter]); //self.expander.update(&[self.counter]);

            self.prev = self.expander.sum(&self.prev[..]); //self.prev = self.expander.finalize_reset().into_bytes().to_vec();
            self.counter += 1;

            // Copy the new batch into p
            // let batch_len = self.prev.len().min(p.len() - n);
            // p[n..n + batch_len].copy_from_slice(&self.prev[..batch_len]);
            // n += batch_len;
            // self.buf = self.prev.clone();
            let new_size = self.prev.len().min(p.len());
            p[..new_size].copy_from_slice(&self.prev[..new_size]);
            // p = &mut p[new_size..];
            //*p = &p[new_size..];
            p.drain(..new_size);

            // Update buf for subsequent calls
            self.buf.clear(); // Don't forget to clear buf before refilling
            self.buf.extend_from_slice(&self.prev);
        }

        let mut result = self.prev.clone();
        result.truncate(need);
        Ok(result)
        // return p; //need//Ok(need)
    }
}

// fn expand(hash: fn() -> HmacSha256, pseudorandom_key: &[u8], info: &[u8]) ->
// Hkdf<io::Empty> {
pub fn expand(pseudorandom_key: &[u8], info: &[u8]) -> Hkdf {
    let mut expander = Hmac::new(pseudorandom_key); //let mut expander = hash();
    // expander.update(pseudorandom_key);
    Hkdf::new(expander, info.to_vec())
}
