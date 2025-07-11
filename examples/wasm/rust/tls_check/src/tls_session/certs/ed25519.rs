
//use rand::{CryptoRng, RngCore, Rng};
//use sha2::{Digest, Sha512};
//use subtle::ConstantTimeEq;
//use std::error::Error;
//use std::fmt;

pub const PUBLIC_KEY_SIZE: usize = 32;
pub const PRIVATE_KEY_SIZE: usize = 64;
pub const SIGNATURE_SIZE: usize = 64;
pub const SEED_SIZE: usize = 32;

#[derive(Clone, Debug, PartialEq)]
pub struct PublicKey(pub Vec<u8>);

impl PublicKey {

    pub fn equal(&self, other: &PublicKey) -> bool {
        //self.0.ct_eq(&other.0).into()
        self.0 == other.0
    }
}

#[derive(Clone)]
pub struct PrivateKey(pub Vec<u8>);

impl PrivateKey {
    pub fn public(&self) -> PublicKey {
        PublicKey(self.0[32..].to_vec())
    }

    pub fn equal(&self, other: &PrivateKey) -> bool {
        //self.0.ct_eq(&other.0).into()
        self.0 == other.0
    }

    pub fn seed(&self) -> Vec<u8> {
        self.0[0..SEED_SIZE].to_vec()
    }
/*
    pub fn sign(&self,
        rng: &mut (impl RngCore + CryptoRng),
        message: &[u8],
        opts: &Options,
    ) -> Result<Vec<u8>, Box<dyn Error>> {
        let hash = opts.hash.clone();
        let context = &opts.context;

        match hash {
            Hash::SHA512 => {
                if message.len() != 64 {
                    return Err(Box::new(SignatureError::BadMessageHashLength));
                }
                if context.len() > 255 {
                    return Err(Box::new(SignatureError::BadContextLength));
                }

                let mut signature = vec![0u8; SIGNATURE_SIZE];
                sign(&mut signature, &self.0, message, DOM_PREFIX_PH, context);
                Ok(signature)
            }
            Hash::None if !context.is_empty() => {
                if context.len() > 255 {
                    return Err(Box::new(SignatureError::BadContextLength));
                }

                let mut signature = vec![0u8; SIGNATURE_SIZE];
                sign(&mut signature, &self.0, message, DOM_PREFIX_CTX, context);
                Ok(signature)
            }
            Hash::None => {
                Ok(sign_pure(&self.0, message))
            }
            _ => {
                Err(Box::new(SignatureError::BadHashFunction))
            }
        }
    }*/
}

#[derive(Clone)]
pub struct Options {
    pub hash: Hash,
    pub context: String,
}

#[derive(Clone)]
pub enum Hash {
    SHA512,
    None,
}

//pub fn generate_key<R: RngCore + CryptoRng>(rng: &mut R) -> Result<(PublicKey, PrivateKey), Box<dyn Error>> {
    //let mut seed = vec![0u8; SEED_SIZE];
    //rng.fill_bytes(&mut seed);

    //let private_key = new_key_from_seed(&seed)?;
    //let public_key = PublicKey(private_key.0[32..].to_vec());

    //Ok((public_key, private_key))
//}

//fn new_key_from_seed(seed: &[u8]) -> Result<PrivateKey, Box<dyn Error>> {
    //if seed.len() != SEED_SIZE {
        //return Err(Box::new(SignatureError::BadSeedLength));
    //}

    //let mut private_key = vec![0u8; PRIVATE_KEY_SIZE];
    //new_key_from_seed_impl(&mut private_key, seed)?;
    //Ok(PrivateKey(private_key))
//}

//fn new_key_from_seed_impl(private_key: &mut [u8], seed: &[u8]) -> Result<(), Box<dyn Error>> {
    //let hash = Sha512::digest(seed);
    //let s = scalar_from_hash(&hash[0..32])?;
    //let public_key = scalar_base_mult(&s)?;

    //private_key[..SEED_SIZE].copy_from_slice(seed);
    //private_key[SEED_SIZE..].copy_from_slice(&public_key);
    //Ok(())
//}

// Verify reports whether sig is a valid signature of message by publicKey. It
// will panic if len(pub_key) is not [PublicKeySize].
pub fn verify(pub_key: &PublicKey, message: &[u8], sig: &[u8]) -> bool {
	return false; //return verify_inner(publicKey, message, sig, domPrefixPure, "")
}
/*
fn verify_inner(publicKey PublicKey, message, sig []byte, domPrefix, context string) -> bool {
	if l := len(publicKey); l != PublicKeySize {
		panic("ed25519: bad public key length: " + strconv.Itoa(l))
	}

	if len(sig) != SignatureSize || sig[63]&224 != 0 {
		return false
	}

	A, err := (&edwards25519.Point{}).SetBytes(publicKey)
	if err != nil {
		return false
	}

	kh := sha512.New()
	if domPrefix != domPrefixPure {
		kh.Write([]byte(domPrefix))
		kh.Write([]byte{byte(len(context))})
		kh.Write([]byte(context))
	}
	kh.Write(sig[:32])
	kh.Write(publicKey)
	kh.Write(message)
	hramDigest := make([]byte, 0, sha512.Size)
	hramDigest = kh.Sum(hramDigest)
	k, err := edwards25519.NewScalar().SetUniformBytes(hramDigest)
	if err != nil {
		panic("ed25519: internal error: setting scalar failed")
	}

	S, err := edwards25519.NewScalar().SetCanonicalBytes(sig[32:])
	if err != nil {
		return false
	}

	// [S]B = R + [k]A --> [k](-A) + [S]B = R
	minusA := (&edwards25519.Point{}).Negate(A)
	R := (&edwards25519.Point{}).VarTimeDoubleScalarBaseMult(k, minusA, S)

	return bytes.Equal(sig[:32], R.Bytes())
}*/