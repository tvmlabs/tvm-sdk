use num_bigint::BigInt;
use num_bigint::BigUint;
use num_bigint::Sign;
use num_bigint::ToBigInt;
use num_traits::Zero;

// use std::error::Error;
// use std::fmt;
use crate::tls_session::hkdf_sha256::Digest;

#[derive(Debug, PartialEq)]
pub struct PublicKey {
    pub n: BigInt, // modulus
    pub e: i64,    // public exponent
}

impl PublicKey {
    pub fn size(&self) -> usize {
        (self.n.bits() + 7) / 8
    }

    pub fn equal(&self, other: &PublicKey) -> bool {
        &self.n == &other.n && self.e == other.e
    }
}

#[derive(Debug)]
pub struct OAEPOptions {
    hash: String,     // Placeholder for crypto hash type
    mgf_hash: String, // Placeholder for MGF hash type
    label: Vec<u8>,
}

// pub fn check_pub(pub_key: &PublicKey) -> Result<(), Box<dyn Error>> {
pub fn check_pub(pub_key: &PublicKey) -> bool {
    if pub_key.n.is_zero() {
        return false; //return Err(Box::new(PublicModulusError));
    }
    if pub_key.e < 2 {
        return false; //return Err(Box::new(PublicExponentSmallError));
    }
    if pub_key.e > ((1u64 << 31) - 1) as i64 {
        return false; //return Err(Box::new(PublicExponentLargeError));
    }
    true //Ok(())
}

// fn encrypt(pubkey: *PublicKey, plaintext: &[u8]) -> ([]byte, error) {
fn encrypt(pubkey: &PublicKey, plaintext: &[u8]) -> Vec<u8> {
    // let n = Modulus::new_modulus_from_big(&pub_key.n)?; // N, err :=
    // bigmod.NewModulusFromBig(pub.N) if err != nil {
    // 		return nil, err
    // 	}
    // let m = Nat::new().set_bytes(plaintext, &n)?; // m, err :=
    // bigmod.NewNat().SetBytes(plaintext, N)

    // if err != nil {
    // 		return nil, err
    // 	}
    // let e = pub_key.e as u32; // e := uint(pub.E)

    // let result = Nat::new().exp_short_var_time(&m, e, &n);
    // result.bytes(&n) // Ok(result.bytes(&n)) //return
    // bigmod.NewNat().ExpShortVarTime(m, e, N).Bytes(N), nil

    let base = BigUint::from_bytes_be(&plaintext); //let base = BigInt::from_bytes_be(Sign::Plus, &plaintext);
    let modulus = &pubkey.n.to_biguint().unwrap(); //let modulus = &pubkey.n;
    let exponent = BigInt::from(pubkey.e.clone()).to_biguint().unwrap(); //let exponent = BigInt::from(pubkey.e.clone());

    let result = base.modpow(&exponent, &modulus);

    result.to_bytes_be() //result.to_signed_bytes_be()
}

// fn verify_pkcs1v15(pub_key: &PublicKey, hash: usize, hashed: &[u8], sig:
// &[u8]) -> Result<(), Box<dyn Error>> {
pub fn verify_pkcs1v15(pub_key: &PublicKey, hash: usize, hashed: &[u8], sig: &[u8]) -> bool {
    let (hash_len, prefix) = pkcs1v15_hash_info(hash, hashed.len()); // let (hash_len, prefix) = pkcs1v15_hash_info(hash, hashed.len())?;
    let t_len = &prefix.clone().unwrap().len() + hash_len / 8;
    let k = pub_key.size();

    if k < t_len + 11 {
        return false; //return Err("verification error".into());
    }

    if k != sig.len() {
        return false; // return Err("verification error".into());
    }

    let em = encrypt(pub_key, sig); // let em = encrypt(pub_key, sig)?;

    // let mut ok = em[0] == 0 && em[1] == 1;
    // ok &= &em[k - hash_len/8..k] == hashed;
    // ok &= &em[k - t_len..(k - hash_len/8)] == &prefix.unwrap();
    // ok &= em[k - t_len - 1] == 0;

    let mut ok = em[0] == 1;
    ok &= &em[k - 1 - hash_len / 8..k - 1] == hashed;
    ok &= &em[k - 1 - t_len..(k - 1 - hash_len / 8)] == &prefix.unwrap();
    ok &= em[k - 1 - t_len - 1] == 0;

    // for i in 2..(k - t_len - 1) {
    // ok &= em[i] == 0xff;
    //}

    for i in 2..(k - t_len - 1) {
        ok &= em[i - 1] == 0xff;
    }

    if !ok {
        return false; //return Err("verification error".into());
    }

    true //Ok(())
}

fn pkcs1v15_hash_info(hash: usize, in_len: usize) -> (usize, Option<Vec<u8>>) {
    // fn pkcs1v15_hash_info(hash: CryptoHash, in_len: usize) -> Result<(usize,
    // Option<Vec<u8>>), Box<dyn Error>> { Special case: hash 0 is used to
    // indicate that the data  is directly signed.
    if hash == 0 {
        // if hash.size() == 0 {
        return (in_len, None); // return Ok((in_len, None));
    }

    // let hash_len = hash.size();
    if in_len != hash / 8 {
        // if in_len != hash_len {
        panic!("crypto/rsa: input must be hashed message"); //return Err("crypto/rsa: input must be hashed message".into());
    }

    let prefix = get_hash_prefix(hash); // let prefix = get_hash_prefix(&hash)?;
    (hash, prefix) // Ok((hash_len, prefix))
}

// fn get_hash_prefix(hash: usize) -> Result<Vec<u8>, Box<dyn Error>> {
fn get_hash_prefix(hash: usize) -> Option<Vec<u8>> {
    match hash {
        224 => {
            // SHA224
            Some(vec![
                0x30, 0x2d, 0x30, 0x0d, 0x06, 0x09, 0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02,
                0x04, 0x05, 0x00, 0x04, 0x1c,
            ])
        }
        256 => {
            // SHA256
            Some(vec![
                0x30, 0x31, 0x30, 0x0d, 0x06, 0x09, 0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02,
                0x01, 0x05, 0x00, 0x04, 0x20,
            ])
        }
        384 => {
            // SHA384
            Some(vec![
                0x30, 0x41, 0x30, 0x0d, 0x06, 0x09, 0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02,
                0x02, 0x05, 0x00, 0x04, 0x30,
            ])
        }
        512 => {
            // SHA512
            Some(vec![
                0x30, 0x51, 0x30, 0x0d, 0x06, 0x09, 0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02,
                0x03, 0x05, 0x00, 0x04, 0x40,
            ])
        }
        _ => None, // Err("unsupported hash function".into()),
    }
}

// PSS_SALT_LENGTH_AUTO causes the salt in a PSS signature to be as large
// as possible when signing, and to be auto-detected when verifying.
pub const PSS_SALT_LENGTH_AUTO: isize = 0;

// PSS_SALT_LENGTH_EQUALS_HASH causes the salt length to equal the length
// of the hash used in the signature.
pub const PSS_SALT_LENGTH_EQUALS_HASH: isize = -1;

// PSSOptions contains options for creating and verifying PSS signatures.
pub struct PSSOptions {
    // salt_length controls the length of the salt used in the PSS signature. It
    // can either be a positive number of bytes, or one of the special
    // PSSSaltLength constants.
    pub salt_length: isize,

    // Hash is the hash function used to generate the message digest. If not
    // zero, it overrides the hash function passed to SignPSS. It's required
    // when using PrivateKey.Sign.
    pub hash: usize,
}

// verify_pss verifies a PSS signature.
//
// A valid signature is indicated by returning a nil error. digest must be the
// result of hashing the input message using the given hash function. The opts
// argument may be nil, in which case sensible defaults are used. opts.Hash is
// ignored.
pub fn verify_pss(
    pub_key: &PublicKey,
    hash: usize,
    digest: &[u8],
    sig: &[u8],
    opts: &PSSOptions,
) -> bool {
    if sig.len() != pub_key.size() {
        return false; // "ErrVerification"
    }

    if opts.salt_length < PSS_SALT_LENGTH_EQUALS_HASH {
        return false; // invalidSaltLenErr;
    }
    let em_bits = pub_key.n.bits() - 1;
    let em_len = (em_bits + 7) / 8;
    let mut em = encrypt(pub_key, sig);

    // Like in signPSSWithSalt, deal with mismatches between emLen and the size
    // of the modulus. The spec would have us wire emLen into the encoding
    // function, but we'd rather always encode to the size of the modulus and
    // then strip leading zeroes if necessary. This only happens for weird
    // modulus sizes anyway.

    for i in em_len..em.len() {
        // while em.len() > em_len && em.len() > 0 {
        if em[i] != 0u8 {
            return false; // ErrVerification
        }
        if i == em_len {
            break;
        }
    }

    return emsa_pss_verify(digest, &em, em_bits, opts.salt_length, 32); // return emsa_pss_verify(digest, &em, em_bits, opts.salt_length, hash.New());
}

// fn emsa_pss_verify(m_hash: &[u8], em: &[u8], em_bits: usize, mut s_len:
// isize, hash: hash.Hash) -> bool {
fn emsa_pss_verify(m_hash: &[u8], em: &[u8], em_bits: usize, mut s_len: isize, hash: u16) -> bool {
    let h_len = hash as isize; //hash.Size();
    if s_len == PSS_SALT_LENGTH_EQUALS_HASH {
        s_len = h_len;
    }
    let em_len = (em_bits + 7) / 8;

    if em_len != em.len() {
        return false; // "rsa: internal error: inconsistent length"
    }

    // 1. If the length of M is greater than the input limitation for the hash
    //    function (2^61 - 1 octets for SHA-1), output "inconsistent" and stop.
    //
    // 2. Let mHash = Hash(M), an octet string of length hLen.
    if h_len != m_hash.len() as isize {
        return false; // ErrVerification
    }

    let h_len = h_len as usize;
    let mut s_len = s_len as usize;

    // 3. If emLen < hLen + sLen + 2, output "inconsistent" and stop.
    if em_len < h_len + s_len + 2 {
        return false; // ErrVerification
    }

    // 4. If the rightmost octet of EM does not have hexadecimal value 0xbc, output
    //    "inconsistent" and stop.
    if em[em_len - 1] != 0xbc {
        return false; // ErrVerification
    }

    // 5. Let maskedDB be the leftmost emLen - hLen - 1 octets of EM, and let H be
    //    the next hLen octets.
    let mut db: Vec<u8> = Vec::from(&em[..em_len - h_len - 1]);
    let h = &em[em_len - h_len - 1..em_len - 1];

    // 6. If the leftmost 8 * emLen - emBits bits of the leftmost octet in maskedDB
    //    are not all equal to zero, output "inconsistent" and stop.
    let bit_mask: u8 = 0xff >> (8 * em_len - em_bits);
    if em[0] & !bit_mask != 0 {
        return false; // ErrVerification
    }

    // 7. Let dbMask = MGF(H, emLen - hLen - 1).
    //
    // 8. Let DB = maskedDB \xor dbMask.
    let mut hash_ = Digest::new(); // sha256
    mgf1_xor(&mut db, &mut hash_, &h); // mgf1_xor(db, hash, h);

    // 9. Set the leftmost 8 * emLen - emBits bits of the leftmost octet in DB to
    //    zero.
    db[0] &= bit_mask;

    // If we don't know the salt length, look for the 0x01 delimiter.
    if s_len == PSS_SALT_LENGTH_AUTO as usize {
        // let ps_len = bytes.IndexByte(db, 0x01);
        let mut ps_len: isize = -1;
        for i in 0..db.len() {
            if db[i] == 0x01 {
                ps_len = i as isize;
                break;
            }
        }

        if ps_len < 0 {
            return false; // ErrVerification
        }
        s_len = db.len() - (ps_len as usize) - 1;
    }

    // 10. If the emLen - hLen - sLen - 2 leftmost octets of DB are not zero or if
    //     the octet at position emLen - hLen - sLen - 1 (the leftmost position is
    //     "position 1") does not have hexadecimal value 0x01, output "inconsistent"
    //     and stop.
    let ps_len = em_len - h_len - s_len - 2;
    for i in 0..ps_len {
        // for _, e := range db[:psLen] {
        if db[i] != 0x00 {
            return false; // ErrVerification
        }
    }
    if db[ps_len] != 0x01 {
        return false; // ErrVerification
    }

    // 11. Let salt be the last sLen octets of DB.
    let salt = &db[db.len() - s_len..];

    // 12. Let M' = (0x)00 00 00 00 00 00 00 00 || mHash || salt ; M' is an octet
    //     string of length 8 + hLen + sLen with eight initial zero octets.
    //
    // 13. Let H' = Hash(M'), an octet string of length hLen.
    let prefix = [0u8; 8];

    hash_.write(&prefix); // hash.Write(prefix[:]);
    hash_.write(m_hash); // hash.Write(mHash);
    hash_.write(&salt); // hash.Write(salt);

    let h0 = hash_.sum(&[]); // let h0 = hash.Sum(nil);

    // 14. If H = H', output "consistent." Otherwise, output "inconsistent."
    if h0 != h {
        return false; // ErrVerification
    }
    return true;
}

// mgf1XOR XORs the bytes in out with a mask generated using the MGF1 function
// specified in PKCS #1 v2.1.
fn mgf1_xor(out: &mut [u8], hash: &mut Digest, seed: &[u8]) {
    let mut counter: [u8; 4] = [0; 4];
    let mut done = 0;

    while done < out.len() {
        hash.write(seed);
        hash.write(&counter);
        let digest = hash.sum(&[]); // let digest = hash.finish().to_be_bytes(); // digest = hash.Sum(digest[:0])
        hash.reset();

        for i in 0..digest.len() {
            if done < out.len() {
                out[done] ^= digest[i];
                done += 1;
            } else {
                break;
            }
        }
        inc_counter(&mut counter);
    }
}

fn inc_counter(counter: &mut [u8; 4]) {
    if counter[3].wrapping_add(1) != 0 {
        counter[3] += 1;
        return;
    }
    if counter[2].wrapping_add(1) != 0 {
        counter[2] += 1;
        return;
    }
    if counter[1].wrapping_add(1) != 0 {
        counter[1] += 1;
        return;
    }
    counter[0] += 1;
}
