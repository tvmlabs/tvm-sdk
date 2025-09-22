const CHUNK: usize = 128;
const INIT_0: u64 = 0x6a09e667f3bcc908;
const INIT_1: u64 = 0xbb67ae8584caa73b;
const INIT_2: u64 = 0x3c6ef372fe94f82b;
const INIT_3: u64 = 0xa54ff53a5f1d36f1;
const INIT_4: u64 = 0x510e527fade682d1;
const INIT_5: u64 = 0x9b05688c2b3e6c1f;
const INIT_6: u64 = 0x1f83d9abfb41bd6b;
const INIT_7: u64 = 0x5be0cd19137e2179;

const INIT_0_224: u64 = 0x8c3d37c819544da2;
const INIT_1_224: u64 = 0x73e1996689dcd4d6;
const INIT_2_224: u64 = 0x1dfab7ae32ff9c82;
const INIT_3_224: u64 = 0x679dd514582f9fcf;
const INIT_4_224: u64 = 0x0f6d2b697bd44da8;
const INIT_5_224: u64 = 0x77e36f7304c48942;
const INIT_6_224: u64 = 0x3f9d85a86a1d36c8;
const INIT_7_224: u64 = 0x1112e6ad91d692a1;

const INIT_0_256: u64 = 0x22312194fc2bf72c;
const INIT_1_256: u64 = 0x9f555fa3c84c64c2;
const INIT_2_256: u64 = 0x2393b86b6f53b151;
const INIT_3_256: u64 = 0x963877195940eabd;
const INIT_4_256: u64 = 0x96283ee2a88effe3;
const INIT_5_256: u64 = 0xbe5e1e2553863992;
const INIT_6_256: u64 = 0x2b0199fc2c85b8aa;
const INIT_7_256: u64 = 0x0eb72ddc81c52ca2;

const INIT_0_384: u64 = 0xcbbb9d5dc1059ed8;
const INIT_1_384: u64 = 0x629a292a367cd507;
const INIT_2_384: u64 = 0x9159015a3070dd17;
const INIT_3_384: u64 = 0x152fecd8f70e5939;
const INIT_4_384: u64 = 0x67332667ffc00b31;
const INIT_5_384: u64 = 0x8eb44a8768581511;
const INIT_6_384: u64 = 0xdb0c2e0d64f98fa7;
const INIT_7_384: u64 = 0x47b5481dbefa4fa4;

const MAGIC384: &[u8] = b"sha\x04";
const MAGIC224: &[u8] = b"sha\x05";
const MAGIC256: &[u8] = b"sha\x06";
const MAGIC512: &[u8] = b"sha\x07";
const MARSHALLED_SIZE: usize = MAGIC512.len() + 8 * 8 + CHUNK + 8;

// Size is the size, in bytes, of a SHA-512 checksum.
const SIZE: usize = 64;

// Size224 is the size, in bytes, of a SHA-224 checksum.
const SIZE_224: usize = 28;

// Size256 is the size, in bytes, of a SHA-512/256 checksum.
const SIZE_256: usize = 32;

// Size384 is the size, in bytes, of a SHA-384 checksum.
const SIZE_384: usize = 48;

// BlockSize is the block size, in bytes, of the SHA-512/224,
// SHA-512/256, SHA-384 and SHA-512 hash functions.
const BLOCK_SIZE: usize = 128;

fn append_uint32(b: &mut Vec<u8>, v: u32) {
    b.push((v >> 24) as u8);
    b.push((v >> 16) as u8);
    b.push((v >> 8) as u8);
    b.push(v as u8);
}

fn append_uint64(b: &mut Vec<u8>, v: u64) {
    b.push((v >> 56) as u8);
    b.push((v >> 48) as u8);
    b.push((v >> 40) as u8);
    b.push((v >> 32) as u8);
    b.push((v >> 24) as u8);
    b.push((v >> 16) as u8);
    b.push((v >> 8) as u8);
    b.push(v as u8);
}

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
    h: [u64; 8],
    x: [u8; CHUNK],
    nx: usize,
    len: u64,
    kind: u8, // sha224 =1, sha256 = 2, sha384 = 3, sha512 = 4
}

impl Digest {
    //
    pub fn reset(&mut self) {
        match self.kind {
            3 => {
                self.h[0] = INIT_0_384;
                self.h[1] = INIT_1_384;
                self.h[2] = INIT_2_384;
                self.h[3] = INIT_3_384;
                self.h[4] = INIT_4_384;
                self.h[5] = INIT_5_384;
                self.h[6] = INIT_6_384;
                self.h[7] = INIT_7_384;
            }
            2 => {
                self.h[0] = INIT_0_256;
                self.h[1] = INIT_1_256;
                self.h[2] = INIT_2_256;
                self.h[3] = INIT_3_256;
                self.h[4] = INIT_4_256;
                self.h[5] = INIT_5_256;
                self.h[6] = INIT_6_256;
                self.h[7] = INIT_7_256;
            }
            1 => {
                self.h[0] = INIT_0_224;
                self.h[1] = INIT_1_224;
                self.h[2] = INIT_2_224;
                self.h[3] = INIT_3_224;
                self.h[4] = INIT_4_224;
                self.h[5] = INIT_5_224;
                self.h[6] = INIT_6_224;
                self.h[7] = INIT_7_224;
            }
            _ => {
                self.h[0] = INIT_0;
                self.h[1] = INIT_1;
                self.h[2] = INIT_2;
                self.h[3] = INIT_3;
                self.h[4] = INIT_4;
                self.h[5] = INIT_5;
                self.h[6] = INIT_6;
                self.h[7] = INIT_7;
            }
        }

        self.nx = 0;
        self.len = 0;
    }

    pub fn marshal_binary(&self) -> Result<Vec<u8>, &'static str> {
        let mut b = Vec::with_capacity(MARSHALLED_SIZE);
        // if self.is224 {
        // b.extend_from_slice(MAGIC224);
        //} else {
        // b.extend_from_slice(MAGIC384);
        //}
        match self.kind {
            1 => b.extend_from_slice(MAGIC224),
            2 => b.extend_from_slice(MAGIC256),
            3 => b.extend_from_slice(MAGIC384),
            _ => b.extend_from_slice(MAGIC512),
        }

        for &hash in &self.h {
            append_uint64(&mut b, hash);
        }

        b.extend_from_slice(&self.x[..self.nx]);
        b.resize(b.len() + self.x.len() - self.nx, 0);
        append_uint64(&mut b, self.len);

        Ok(b)
    }

    fn unmarshal_binary(&mut self, b: &[u8]) {
        // if &b[..MAGIC256.len()] != MAGIC256 //b.len() < MAGIC224.len()
        //|| (self.is224 && &b[..MAGIC224.len()] != MAGIC224)
        //|| (!self.is224 && &b[..MAGIC256.len()] != MAGIC256)
        //{
        // return Err("crypto/sha256: invalid hash state identifier".into());
        //}
        if b.len() != MARSHALLED_SIZE {
            panic!("crypto/sha256: invalid hash state size"); //return Err("crypto/sha256: invalid hash state size".into());
        }
        let mut b = &b[MAGIC512.len()..];

        self.h[0] = consume_uint64(&mut b);
        self.h[1] = consume_uint64(&mut b);
        self.h[2] = consume_uint64(&mut b);
        self.h[3] = consume_uint64(&mut b);
        self.h[4] = consume_uint64(&mut b);
        self.h[5] = consume_uint64(&mut b);
        self.h[6] = consume_uint64(&mut b);
        self.h[7] = consume_uint64(&mut b);

        let copied = b.len().min(CHUNK);
        self.x[..copied].copy_from_slice(&b[..copied]);
        b = &b[copied..];

        self.len = consume_uint64(&mut b); // self.len = consume_uint64(&mut b)?;
        self.nx = (self.len % (CHUNK as u64)) as usize;

        // Ok(())
    }

    pub fn new224() -> Digest {
        let mut d = Digest { h: [0; 8], x: [0; CHUNK], nx: 0, len: 0, kind: 1 };
        d.reset();
        d
    }

    pub fn new256() -> Digest {
        let mut d = Digest { h: [0; 8], x: [0; CHUNK], nx: 0, len: 0, kind: 2 };
        d.reset();
        d
    }

    pub fn new384() -> Digest {
        let mut d = Digest {
            h: [0; 8],
            x: [0; CHUNK],
            nx: 0,
            len: 0,
            kind: 3, // kind: 384,
        };
        d.reset();
        d
    }

    pub fn new512() -> Digest {
        let mut d = Digest { h: [0; 8], x: [0; CHUNK], nx: 0, len: 0, kind: 4 };
        d.reset();
        d
    }

    pub fn size(&self) -> usize {
        // if !self.is224 {
        match self.kind {
            1 => SIZE_224,
            2 => SIZE_256,
            3 => SIZE_384,
            _ => SIZE,
        }
    }

    pub fn block_size(&self) -> usize {
        BLOCK_SIZE
    }

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
                // The full block handling
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
        // Make a copy of self so the caller can continue writing and summing
        let mut d0 = self.clone();
        let hash = d0.check_sum();
        // if d0.is224 {
        //[in_bytes, &hash[..SIZE_224]].concat()
        //} else {
        //[in_bytes, &hash].concat()
        //}
        match self.kind {
            1 => [in_bytes, &hash[..SIZE_224]].concat(),
            2 => [in_bytes, &hash[..SIZE_256]].concat(),
            3 => [in_bytes, &hash[..SIZE_384]].concat(),
            _ => [in_bytes, &hash].concat(),
        }
    }

    fn check_sum(&mut self) -> [u8; SIZE] {
        let len = self.len;
        // Padding. Add a 1 bit and 0 bits until 56 bytes mod 64.
        let mut tmp = [0u8; 128 + 16]; // padding + length buffer
        tmp[0] = 0x80;

        let t = if len % 128 < 112 { 112 - len % 128 } else { 128 + 112 - len % 128 };

        let len_in_bits = len << 3;
        let padlen = &mut tmp[..(t as usize) + 16];

        // Upper 64 bits are always zero, because len variable has type uint64,
        // and tmp is already zeroed at that index, so we can skip updating it.
        // binary.BigEndian.PutUint64(padlen[t+0:], 0)
        put_uint64(&mut padlen[(t as usize) + 8..], len_in_bits);
        self.write(&padlen);

        if self.nx != 0 {
            panic!("d.nx != 0");
        }

        let mut digest = [0u8; SIZE];

        put_uint64(&mut digest[0..8], self.h[0]);
        put_uint64(&mut digest[8..16], self.h[1]);
        put_uint64(&mut digest[16..24], self.h[2]);
        put_uint64(&mut digest[24..32], self.h[3]);
        put_uint64(&mut digest[32..40], self.h[4]);
        put_uint64(&mut digest[40..48], self.h[5]);

        if self.kind != 3 {
            // if !sha384 {
            put_uint64(&mut digest[48..56], self.h[6]);
            put_uint64(&mut digest[56..64], self.h[7]);
        }

        digest
    }
}

pub fn sum256(data: &[u8]) -> [u8; SIZE_256] {
    let mut d = Digest::new256();
    d.reset();
    d.write(data);
    let sum = d.check_sum();
    let res: [u8; SIZE_256] = sum[..SIZE_256].try_into().unwrap();
    return res;
}

// Sum384 returns the SHA384 checksum of the data.
pub fn sum384(data: &[u8]) -> [u8; SIZE_384] {
    let mut d = Digest::new384();
    d.reset();
    d.write(data);
    let sum = d.check_sum();
    let res: [u8; SIZE_384] = sum[..SIZE_384].try_into().unwrap();
    return res;
}

// Sum512 returns the SHA512 checksum of the data.
pub fn sum512(data: &[u8]) -> [u8; SIZE] {
    let mut d = Digest::new512();
    d.reset();
    d.write(data);
    let sum = d.check_sum();
    let res: [u8; SIZE] = sum[..SIZE].try_into().unwrap();
    return res;
}

const K: [u64; 80] = [
    0x428a2f98d728ae22,
    0x7137449123ef65cd,
    0xb5c0fbcfec4d3b2f,
    0xe9b5dba58189dbbc,
    0x3956c25bf348b538,
    0x59f111f1b605d019,
    0x923f82a4af194f9b,
    0xab1c5ed5da6d8118,
    0xd807aa98a3030242,
    0x12835b0145706fbe,
    0x243185be4ee4b28c,
    0x550c7dc3d5ffb4e2,
    0x72be5d74f27b896f,
    0x80deb1fe3b1696b1,
    0x9bdc06a725c71235,
    0xc19bf174cf692694,
    0xe49b69c19ef14ad2,
    0xefbe4786384f25e3,
    0x0fc19dc68b8cd5b5,
    0x240ca1cc77ac9c65,
    0x2de92c6f592b0275,
    0x4a7484aa6ea6e483,
    0x5cb0a9dcbd41fbd4,
    0x76f988da831153b5,
    0x983e5152ee66dfab,
    0xa831c66d2db43210,
    0xb00327c898fb213f,
    0xbf597fc7beef0ee4,
    0xc6e00bf33da88fc2,
    0xd5a79147930aa725,
    0x06ca6351e003826f,
    0x142929670a0e6e70,
    0x27b70a8546d22ffc,
    0x2e1b21385c26c926,
    0x4d2c6dfc5ac42aed,
    0x53380d139d95b3df,
    0x650a73548baf63de,
    0x766a0abb3c77b2a8,
    0x81c2c92e47edaee6,
    0x92722c851482353b,
    0xa2bfe8a14cf10364,
    0xa81a664bbc423001,
    0xc24b8b70d0f89791,
    0xc76c51a30654be30,
    0xd192e819d6ef5218,
    0xd69906245565a910,
    0xf40e35855771202a,
    0x106aa07032bbd1b8,
    0x19a4c116b8d2d0c8,
    0x1e376c085141ab53,
    0x2748774cdf8eeb99,
    0x34b0bcb5e19b48a8,
    0x391c0cb3c5c95a63,
    0x4ed8aa4ae3418acb,
    0x5b9cca4f7763e373,
    0x682e6ff3d6b2b8a3,
    0x748f82ee5defb2fc,
    0x78a5636f43172f60,
    0x84c87814a1f0ab72,
    0x8cc702081a6439ec,
    0x90befffa23631e28,
    0xa4506cebde82bde9,
    0xbef9a3f7b2c67915,
    0xc67178f2e372532b,
    0xca273eceea26619c,
    0xd186b8c721c0c207,
    0xeada7dd6cde0eb1e,
    0xf57d4f7fee6ed178,
    0x06f067aa72176fba,
    0x0a637dc5a2c898a6,
    0x113f9804bef90dae,
    0x1b710b35131c471b,
    0x28db77f523047d84,
    0x32caab7b40c72493,
    0x3c9ebe0a15c9bebc,
    0x431d67c49c100d4c,
    0x4cc5d4becb3e42b6,
    0x597f299cfc657e2a,
    0x5fcb6fab3ad6faec,
    0x6c44198c4a475817,
];

fn rotate_left_64(x: u64, k: i32) -> u64 {
    const N: u64 = 64;
    let s = (k as u64) & (N - 1);
    (x << s) | (x >> (N - s))
}

fn block(dig: &mut Digest, p: &[u8]) {
    let mut w: [u64; 80] = [0; 80];
    let (mut h0, mut h1, mut h2, mut h3, mut h4, mut h5, mut h6, mut h7) =
        (dig.h[0], dig.h[1], dig.h[2], dig.h[3], dig.h[4], dig.h[5], dig.h[6], dig.h[7]);

    let chunk = CHUNK;
    let mut pos = 0;

    while pos + chunk <= p.len() {
        for i in 0..16 {
            let j = i * 8;
            w[i] = (p[pos + j] as u64) << 56
                | (p[pos + j + 1] as u64) << 48
                | (p[pos + j + 2] as u64) << 40
                | (p[pos + j + 3] as u64) << 32
                | (p[pos + j + 4] as u64) << 24
                | (p[pos + j + 5] as u64) << 16
                | (p[pos + j + 6] as u64) << 8
                | (p[pos + j + 7] as u64);
        }
        for i in 16..80 {
            let v1 = w[i - 2];
            let t1 = rotate_left_64(v1, -19) ^ rotate_left_64(v1, -61) ^ (v1 >> 6);
            let v2 = w[i - 15];
            let t2 = rotate_left_64(v2, -1) ^ rotate_left_64(v2, -8) ^ (v2 >> 7);
            w[i] = t1.wrapping_add(w[i - 7]).wrapping_add(t2).wrapping_add(w[i - 16]);
        }
        let (mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h) =
            (h0, h1, h2, h3, h4, h5, h6, h7);
        for i in 0..80 {
            let t1 = h
                .wrapping_add(
                    rotate_left_64(e, -14) ^ rotate_left_64(e, -18) ^ rotate_left_64(e, -41),
                )
                .wrapping_add((e & f) ^ (!e & g))
                .wrapping_add(K[i])
                .wrapping_add(w[i]);

            let t2 = (rotate_left_64(a, -28) ^ rotate_left_64(a, -34) ^ rotate_left_64(a, -39))
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
