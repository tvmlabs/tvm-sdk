mod ecdsa;
mod ed25519;
mod rsa;

use std::collections::HashMap;
use std::collections::hash_map;
use std::net::*;
use std::ops::Neg;
// use num_traits::One;
// use std::iter::FromIterator;
use std::str::FromStr;

// use std::ops::{AddAssign, Deref};
// use std::time::SystemTime;
use chrono::{DateTime, TimeZone, Utc};
use num_bigint::BigInt;
use num_bigint::Sign;
use num_bigint::ToBigInt;

use crate::tls_session::certs::ecdsa::Curve;
use crate::tls_session::format;
use crate::tls_session::hkdf_sha256;
use crate::tls_session::sha512;
// use crate::tls_session::certs::PublicKey::{ECDSA_PUBLIC_KEY,
// ED25519_PUBLIC_KEY, X25519_PUBLIC_KEY};

#[derive(Debug, Clone)]
struct IpNet {
    ip: Vec<u8>,
    mask: Vec<u8>,
}

// ASN.1 objects have metadata preceding them:
//   the tag: the type of the object
//   a flag denoting if this object is compound or not
//   the class type: the namespace of the tag
//   the length of the object, in bytes

// Here are some standard tags and classes

// ASN.1 tags represent the type of the following object.
const TAG_BOOLEAN: u8 = 1;
const TAG_INTEGER: u8 = 2;
const TAG_BIT_STRING: u8 = 3;
const TAG_OCTET_STRING: u8 = 4;
const TAG_NULL: u8 = 5;
const TAG_OID: u8 = 6;
const TAG_ENUM: u8 = 10;
const TAG_UTF8_STRING: u8 = 12;
const TAG_SEQUENCE: u8 = 16;
const TAG_SET: u8 = 17;
const TAG_NUMERIC_STRING: u8 = 18;
const TAG_PRINTABLE_STRING: u8 = 19;
const TAG_T61_STRING: u8 = 20;
const TAG_IA5_STRING: u8 = 22;
const TAG_UTC_TIME: u8 = 23;
const TAG_GENERALIZED_TIME: u8 = 24;
const TAG_GENERAL_STRING: u8 = 27;
const TAG_BMP_STRING: u8 = 30;

// ASN.1 class types represent the namespace of the tag.
// const CLASS_UNIVERSAL: u16       = 0;
// const CLASS_APPLICATION: u16     = 1;
// const CLASS_CONTEXTSPECIFIC: u16 = 2;
// const CLASS_PRIVATE: u16         = 3;

// NullRawValue is a [RawValue] with its Tag set to the ASN.1 NULL type tag (5).
// const NullRawValue: RawValue = RawValue{Tag: TagNull};

// NULL_BYTES contains bytes representing the DER-encoded ASN.1 NULL type.
const NULL_BYTES: [u8; 2] = [TAG_NULL, 0];

#[derive(Debug, PartialEq)]
pub enum SignatureAlgorithm {
    UnknownSignatureAlgorithm = 0,
    MD2WithRSA = 1, // Unsupported.
    MD5WithRSA = 2, // Only supported for signing, not verification.
    SHA1WithRSA = 3, /* Only supported for signing, and verification of CRLs, CSRs, and OCSP
                     * responses. */
    SHA256WithRSA = 4,
    SHA384WithRSA = 5,
    SHA512WithRSA = 6,
    DSAWithSHA1 = 7,   // Unsupported.
    DSAWithSHA256 = 8, // Unsupported.
    ECDSAWithSHA1 = 9, /* Only supported for signing, and verification of CRLs, CSRs, and OCSP
                        * responses. */
    ECDSAWithSHA256 = 10,
    ECDSAWithSHA384 = 11,
    ECDSAWithSHA512 = 12,
    SHA256WithRSAPSS = 13,
    SHA384WithRSAPSS = 14,
    SHA512WithRSAPSS = 15,
    PureEd25519 = 16,
}
// const UnknownSignatureAlgorithm: u16 = 0;
// const MD2WithRSA: u16 = 1;  // Unsupported.
// const MD5WithRSA: u16 = 2;  // Only supported for signing, not verification.
// const SHA1WithRSA: u16 = 3; // Only supported for signing, and verification
// of CRLs, CSRs, and OCSP responses. const SHA256WithRSA: u16 = 4;
// const SHA384WithRSA: u16 = 5;
// const SHA512WithRSA: u16 = 6;
// const DSAWithSHA1: u16 = 7;   // Unsupported.
// const DSAWithSHA256: u16 = 8; // Unsupported.
// const ECDSAWithSHA1: u16 = 9; // Only supported for signing, and verification
// of CRLs, CSRs, and OCSP responses. const ECDSAWithSHA256: u16 = 10;
// const ECDSAWithSHA384: u16 = 11;
// const ECDSAWithSHA512: u16 = 12;
// const SHA256WithRSAPSS: u16 = 13;
// const SHA384WithRSAPSS: u16 = 14;
// const SHA512WithRSAPSS: u16 = 15;
// const PureEd25519: u16 = 16;

impl SignatureAlgorithm {
    fn is_rsa_pss(&self) -> bool {
        match self {
            SignatureAlgorithm::SHA256WithRSAPSS => true,
            SignatureAlgorithm::SHA384WithRSAPSS => true,
            SignatureAlgorithm::SHA512WithRSAPSS => true,
            _ => false,
        }
    }
}

const OID_SIGNATURE_MD2_WITH_RSA: [i32; 7] = [1, 2, 840, 113549, 1, 1, 2];
const OID_SIGNATURE_MD5_WITH_RSA: [i32; 7] = [1, 2, 840, 113549, 1, 1, 4];
const OID_SIGNATURE_SHA1_WITH_RSA: [i32; 7] = [1, 2, 840, 113549, 1, 1, 5];
const OID_SIGNATURE_SHA256_WITH_RSA: [i32; 7] = [1, 2, 840, 113549, 1, 1, 11];
const OID_SIGNATURE_SHA384_WITH_RSA: [i32; 7] = [1, 2, 840, 113549, 1, 1, 12];
const OID_SIGNATURE_SHA512_WITH_RSA: [i32; 7] = [1, 2, 840, 113549, 1, 1, 13];
const OID_SIGNATURE_RSA_PSS: [i32; 7] = [1, 2, 840, 113549, 1, 1, 10];
const OID_SIGNATURE_DSA_WITH_SHA1: [i32; 6] = [1, 2, 840, 10040, 4, 3];
const OID_SIGNATURE_DSA_WITH_SHA256: [i32; 9] = [2, 16, 840, 1, 101, 3, 4, 3, 2];
const OID_SIGNATURE_ECDSA_WITH_SHA1: [i32; 6] = [1, 2, 840, 10045, 4, 1];
const OID_SIGNATURE_ECDSA_WITH_SHA256: [i32; 7] = [1, 2, 840, 10045, 4, 3, 2];
const OID_SIGNATURE_ECDSA_WITH_SHA384: [i32; 7] = [1, 2, 840, 10045, 4, 3, 3];
const OID_SIGNATURE_ECDSA_WITH_SHA512: [i32; 7] = [1, 2, 840, 10045, 4, 3, 4];
const OID_SIGNATURE_ED25519: [i32; 4] = [1, 3, 101, 112];

const OID_SHA256: [i32; 9] = [2, 16, 840, 1, 101, 3, 4, 2, 1];
const OID_SHA384: [i32; 9] = [2, 16, 840, 1, 101, 3, 4, 2, 2];
const OID_SHA512: [i32; 9] = [2, 16, 840, 1, 101, 3, 4, 2, 3];

const OID_MGF1: [i32; 7] = [1, 2, 840, 113549, 1, 1, 8];

// oidISOSignatureSHA1WithRSA means the same as oidSignatureSHA1WithRSA
// but it's specified by ISO. Microsoft's makecert.exe has been known
// to produce certificates with this OID.
const OID_ISO_SIGNATURE_SHA1_WITH_RSA: [i32; 6] = [1, 3, 14, 3, 2, 29];

// const oidExtensionSubjectKeyId: [i32; 4]          = [2, 5, 29, 14];
// const oidExtensionKeyUsage: [i32; 4]              = [2, 5, 29, 15];
// const oidExtensionExtendedKeyUsage: [i32; 4]      = [2, 5, 29, 37];
// const oidExtensionAuthorityKeyId: [i32; 4]        = [2, 5, 29, 35];
// const oidExtensionBasicConstraints: [i32; 4]      = [2, 5, 29, 19];
// const oidExtensionSubjectAltName: [i32; 4]        = [2, 5, 29, 17];
// const oidExtensionCertificatePolicies: [i32; 4]   = [2, 5, 29, 32];
// const oidExtensionNameConstraints: [i32; 4]       = [2, 5, 29, 30];
// const oidExtensionCRLDistributionPoints: [i32; 4] = [2, 5, 29, 31];
const OID_EXTENSION_AUTHORITY_INFO_ACCESS: [i32; 9] = [1, 3, 6, 1, 5, 5, 7, 1, 1];
// const oidExtensionCRLNumber: [i32; 4]             = [2, 5, 29, 20];
// const oidExtensionReasonCode: [i32; 4]            = [2, 5, 29, 21];

const OID_AUTHORITY_INFO_ACCESS_OCSP: [i32; 9] = [1, 3, 6, 1, 5, 5, 7, 48, 1];
const OID_AUTHORITY_INFO_ACCESS_ISSUERS: [i32; 9] = [1, 3, 6, 1, 5, 5, 7, 48, 2];

// const UnknownPublicKeyAlgorithm: u16 = 0;
// const RSA: u16 = 1;
// const DSA: u16 = 2; // Only supported for parsing.
// const ECDSA: u16 = 3;
// const ED25519: u16 = 4;

#[derive(Debug, PartialEq)]
pub enum PublicKeyAlgorithm {
    UnknownPublicKeyAlgorithm = 0,
    RSA = 1,
    DSA = 2,
    ECDSA = 3,
    Ed25519 = 4,
}

impl PublicKeyAlgorithm {
    pub fn to_string(&self) -> String {
        match self {
            PublicKeyAlgorithm::UnknownPublicKeyAlgorithm => {
                "UnknownPublicKeyAlgorithm".to_string()
            }
            PublicKeyAlgorithm::RSA => "RSA".to_string(),
            PublicKeyAlgorithm::DSA => "DSA".to_string(),
            PublicKeyAlgorithm::ECDSA => "ECDSA".to_string(),
            PublicKeyAlgorithm::Ed25519 => "Ed25519".to_string(),
        }
    }
}

#[derive(Debug)]
struct AlgorithmDetails {
    algo: SignatureAlgorithm,
    name: String,
    oid: Vec<i32>, // ObjectIdentifier,
    pub_key_algo: PublicKeyAlgorithm,
    hash: Option<String>, // One can use String for hashing algorithm
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct KeyUsage(i32);

impl KeyUsage {
    pub const CERT_SIGN: KeyUsage = KeyUsage(1 << 5);
    pub const CONTENT_COMMITMENT: KeyUsage = KeyUsage(1 << 1);
    pub const CRL_SIGN: KeyUsage = KeyUsage(1 << 6);
    pub const DATA_ENCIPHERMENT: KeyUsage = KeyUsage(1 << 3);
    pub const DECIPHER_ONLY: KeyUsage = KeyUsage(1 << 8);
    pub const DIGITAL_SIGNATURE: KeyUsage = KeyUsage(1 << 0);
    pub const ENCIPHER_ONLY: KeyUsage = KeyUsage(1 << 7);
    pub const KEY_AGREEMENT: KeyUsage = KeyUsage(1 << 4);
    pub const KEY_ENCIPHERMENT: KeyUsage = KeyUsage(1 << 2);
}

#[derive(Debug, Clone, Copy)]
pub struct ExtKeyUsage(i32);

impl ExtKeyUsage {
    pub const ANY: ExtKeyUsage = ExtKeyUsage(0);
    pub const CLIENT_AUTH: ExtKeyUsage = ExtKeyUsage(2);
    pub const CODE_SIGNING: ExtKeyUsage = ExtKeyUsage(3);
    pub const EMAIL_PROTECTION: ExtKeyUsage = ExtKeyUsage(4);
    pub const IPSEC_END_SYSTEM: ExtKeyUsage = ExtKeyUsage(5);
    pub const IPSEC_TUNNEL: ExtKeyUsage = ExtKeyUsage(6);
    pub const IPSEC_USER: ExtKeyUsage = ExtKeyUsage(7);
    pub const MICROSOFT_COMMERCIAL_CODE_SIGNING: ExtKeyUsage = ExtKeyUsage(12);
    pub const MICROSOFT_KERNEL_CODE_SIGNING: ExtKeyUsage = ExtKeyUsage(13);
    pub const MICROSOFT_SERVER_GATED_CRYPTO: ExtKeyUsage = ExtKeyUsage(10);
    pub const NETSCAPE_SERVER_GATED_CRYPTO: ExtKeyUsage = ExtKeyUsage(11);
    pub const OCSP_SIGNING: ExtKeyUsage = ExtKeyUsage(9);
    pub const SERVER_AUTH: ExtKeyUsage = ExtKeyUsage(1);
    pub const TIME_STAMPING: ExtKeyUsage = ExtKeyUsage(8);
}

#[derive(Debug)]
pub struct PssParameters {
    // The fields are not required as the default values
    // point to SHA-1 (which is no longer suitable for use in signatures).
    pub hash: AlgorithmIdentifier,
    pub mgf: AlgorithmIdentifier,
    pub salt_length: i32,
    pub trailer_field: Option<i32>, // Optional field with default value 1
}

// Tag represents an ASN.1 identifier octet, consisting of a tag number
// (indicating a type) and class (such as context-specific or constructed).
//
// Methods in the cryptobyte package only support the low-tag-number form, i.e.
// a single identifier octet with bits 7-8 encoding the class and bits 1-6
// encoding the tag number.
//#[derive(Clone, Copy, Debug)]
// pub struct Tag(u8);

const CLASS_CONSTRUCTED: u8 = 0x20;
const CLASS_CONTEXT_SPECIFIC: u8 = 0x80;

// Methods for Tag
// impl Tag {

// pub fn constructed(self) -> Tag {
// Tag(self.0 | CLASS_CONSTRUCTED)
//}

// Setting the bit of context specific class
// pub fn context_specific(self) -> Tag {
// Tag(self.0 | CLASS_CONTEXT_SPECIFIC)
//}

pub const fn context_specific(tag: u8) -> u8 {
    tag | CLASS_CONTEXT_SPECIFIC
}
pub const fn constructed(tag: u8) -> u8 {
    tag | CLASS_CONSTRUCTED
}

// Standard combinations of tags and classes
// pub const BOOLEAN: Tag = Tag(1);
// pub const INTEGER: Tag = Tag(2);
// pub const BIT_STRING: Tag = Tag(3);
// pub const OCTET_STRING: Tag = Tag(4);
// pub const NULL: Tag = Tag(5);
// pub const OBJECT_IDENTIFIER: Tag = Tag(6);
// pub const ENUM: Tag = Tag(10);
// pub const UTF8_STRING: Tag = Tag(12);
// pub const SEQUENCE: Tag = Tag(16 | CLASS_CONSTRUCTED);
// pub const SET: Tag = Tag(17 | CLASS_CONSTRUCTED);
// pub const PRINTABLE_STRING: Tag = Tag(19);
// pub const T61_STRING: Tag = Tag(20);
// pub const IA5_STRING: Tag = Tag(22);
// pub const UTC_TIME: Tag = Tag(23);
// pub const GENERALIZED_TIME: Tag = Tag(24);
// pub const GENERAL_STRING: Tag = Tag(27);

pub const BOOLEAN: u8 = 1u8;
pub const INTEGER: u8 = 2u8;
pub const BIT_STRING: u8 = 3u8;
pub const OCTET_STRING: u8 = 4u8;
pub const NULL: u8 = 5u8;
pub const OBJECT_IDENTIFIER: u8 = 6u8;
pub const ENUM: u8 = 10u8;
pub const UTF8_STRING: u8 = 12u8;
pub const SEQUENCE: u8 = 16u8 | CLASS_CONSTRUCTED;
pub const SET: u8 = 17u8 | CLASS_CONSTRUCTED;
pub const NUMERIC_STRING: u8 = 18u8;
pub const PRINTABLE_STRING: u8 = 19u8;
pub const T61_STRING: u8 = 20u8;
pub const IA5_STRING: u8 = 22u8;
pub const UTC_TIME: u8 = 23u8;
pub const GENERALIZED_TIME: u8 = 24u8;
pub const GENERAL_STRING: u8 = 27u8;
pub const BMP_STRING: u8 = 30u8;

struct BitString {
    pub bytes: Vec<u8>,
    pub bit_length: usize,
}

impl BitString {
    // at returns the bit at the given index. If the index is out of range it
    // returns 0.
    pub fn at(&self, i: usize) -> u8 {
        if i >= self.bit_length {
            return 0u8;
        }
        let x = i / 8;
        let y = 7 - (i % 8) as u8;
        (self.bytes[x] >> y) & 1
    }

    // right_align returns a slice where the padding bits are at the beginning. The
    // slice may share memory with the BitString.
    pub fn right_align(&self) -> Vec<u8> {
        let shift = (8 - (self.bit_length % 8)) as u8;
        if shift == 8 || self.bytes.is_empty() {
            return self.bytes.clone();
        }

        let mut a = vec![0u8; self.bytes.len()];
        a[0] = self.bytes[0] >> shift;
        for i in 1..self.bytes.len() {
            a[i] = self.bytes[i - 1] << (8 - shift);
            a[i] |= self.bytes[i] >> shift;
        }
        return a;
    }
}

#[derive(Debug, Clone)]
pub struct ASN1String(Vec<u8>); // Uses Vec<u8> to represent a string

impl ASN1String {
    pub fn read_asn1_bitstring(&mut self, out: &mut BitString) -> bool {
        let mut bytes = ASN1String { 0: Vec::new() };
        if !self.read_asn1(&mut bytes, BIT_STRING)
            || bytes.0.is_empty()
            || bytes.0.len() * 8 / 8 != bytes.0.len()
        {
            return false;
        }

        let padding_bits = bytes.0[0];
        bytes.0 = bytes.0[1..].to_vec();

        if padding_bits > 7
            || bytes.0.is_empty() && padding_bits != 0
            || bytes.0.len() > 0
                && (bytes.0[bytes.0.len() - 1] & ((1 << padding_bits) - 1u8)) != 0u8
        {
            return false;
        }
        out.bit_length = (bytes.0.len() * 8) as usize - (padding_bits as usize);
        out.bytes = bytes.0;
        return true;
    }

    pub fn read_asn1(&mut self, out: &mut ASN1String, tag: u8) -> bool {
        let mut t = 0u8;
        if !self.read_any_asn1(out, &mut t) || t != tag {
            return false;
        }
        true
    }

    // ReadASN1Element reads the contents of a DER-encoded ASN.1 element (including
    // tag and length bytes) into out, and advances. The element must match the
    // given tag. It reports whether the read was successful.
    //
    // Tags greater than 30 are not supported (i.e. low-tag-number format only).
    pub fn read_asn1_element(&mut self, out: &mut ASN1String, tag: u8) -> bool {
        let mut t = 0u8;

        if !self.read_any_asn1_element(out, &mut t) || t != tag {
            return false;
        }
        true
    }

    pub fn read_any_asn1(&mut self, out: &mut ASN1String, out_tag: &mut u8) -> bool {
        self.read_asn1_impl(out, out_tag, true)
    }

    // ReadAnyASN1Element reads the contents of a DER-encoded ASN.1 element
    // (including tag and length bytes) into out, sets outTag to is tag, and
    // advances. It reports whether the read was successful.
    //
    // Tags greater than 30 are not supported (i.e. low-tag-number format only).
    pub fn read_any_asn1_element(&mut self, out: &mut ASN1String, out_tag: &mut u8) -> bool {
        self.read_asn1_impl(out, out_tag, false)
    }

    // ASN.1 tag check
    pub fn peek_asn1_tag(&self, tag: u8) -> bool {
        if self.0.is_empty() {
            return false;
        }
        return self.0[0] == tag;
    }

    // Пропуск ASN.1
    pub fn skip_asn1(&mut self, tag: u8) -> bool {
        let mut unused = ASN1String(vec![]);
        self.read_asn1(&mut unused, tag)
    }

    pub fn read_optional_asn1(
        &mut self,
        out: &mut ASN1String,
        out_present: &mut bool,
        tag: u8,
    ) -> bool {
        let present = self.peek_asn1_tag(tag);
        // if let Some(ref mut p) = out_present {
        // *p = present;
        //}
        *out_present = present;
        if present && !self.read_asn1(out, tag) {
            return false;
        }
        true
    }

    pub fn skip_optional_asn1(&mut self, tag: u8) -> bool {
        if !self.peek_asn1_tag(tag) {
            return true;
        }
        let mut unused = ASN1String(vec![]);
        self.read_asn1(&mut unused, tag)
    }

    // pub fn read_optional_asn1_integer(&mut self, out: &mut dyn std::any::Any,
    // tag: Tag, default_value: &dyn std::any::Any) -> bool {
    pub fn read_optional_asn1_integer(
        &mut self,
        out: &mut i64,
        tag: u8,
        default_value: i64,
    ) -> bool {
        let mut present = false;
        let mut i = ASN1String(vec![]);

        if !self.read_optional_asn1(&mut i, &mut present, tag) {
            return false;
        }

        if !present {
            // match out.downcast_mut::<i32>() {
            // Some(o) => *o = *default_value.downcast_ref::<i32>().unwrap(),
            // None => panic!("invalid type"),
            //}
            *out = default_value;
            return true;
        }

        if !i.read_asn1_i64(out) || !i.0.is_empty() {
            return false;
        }

        true
    }

    // read_asn1_boolean decodes an ASN.1 BOOLEAN and converts it to a boolean
    // representation into out and advances. It reports whether the read
    // was successful.
    fn read_asn1_boolean(&mut self, out: &mut bool) -> bool {
        let mut bytes = ASN1String { 0: Vec::new() };
        if !self.read_asn1(&mut bytes, BOOLEAN) || bytes.0.len() != 1 {
            return false;
        }
        match bytes.0[0] {
            0 => *out = false,
            0xff => *out = true,
            _ => return false,
        }
        return true;
    }

    pub fn read_asn1_i64(&mut self, out: &mut i64) -> bool {
        let mut bytes = ASN1String { 0: Vec::new() };
        // if !self.read_asn1(&mut bytes, INTEGER) || !check_asn1_integer(bytes) ||
        // !asn1_signed(out, &bytes) {
        if !self.read_asn1(&mut bytes, INTEGER)
            || !check_asn1_integer(&bytes.0)
            || !asn1_signed(out, &bytes.0)
        {
            return false;
        }
        return true;
    }

    // Reading of ASN.1 INTEGER to out
    // pub fn read_asn1_integer(&mut self, out: &mut dyn std::any::Any) -> bool { //
    // pub fn read_asn1_integer(&mut self, out: &mut dyn std::any::Any) -> bool {
    // Tey to get pointer ob number type.
    // if let Some(out_int) = out.downcast_mut::<i64>() {
    // let mut i: i64 = 0;
    // if !self.read_asn1_int64(&mut i) {
    // return false;
    //}
    // *out_int = i; // Set the value
    // return true;
    //} else if let Some(out_uint) = out.downcast_mut::<u64>() {
    // let mut u: u64 = 0;
    // if !self.read_asn1_uint64(&mut u) {
    // return false;
    //}
    // *out_uint = u; // Set the value
    // return true;
    //} else if let Some(out_big) = out.downcast_mut::<BigInt>() {
    // return self.read_asn1_big_int(out_big);
    //} else if let Some(out_bytes) = out.downcast_mut::<Vec<u8>>() {
    // return self.read_asn1_bytes(out_bytes);
    //}

    // panic!("out does not point to an integer type");

    //}

    pub fn read_asn1_int64(&mut self, out: &mut i64) -> bool {
        let mut bytes = ASN1String { 0: Vec::new() };
        if !self.read_asn1(&mut bytes, INTEGER)
            || !check_asn1_integer(&bytes.0)
            || !asn1_signed(out, &bytes.0)
        {
            return false;
        }
        true
    }

    pub fn read_asn1_uint64(&mut self, out: &mut u64) -> bool {
        let mut bytes = ASN1String { 0: Vec::new() };
        if !self.read_asn1(&mut bytes, INTEGER)
            || !check_asn1_integer(&bytes.0)
            || !asn1_unsigned(out, &bytes.0)
        {
            return false;
        }
        true
    }

    // pub fn read_asn1_big_int(&mut self, out: &mut BigInt) -> bool {
    // let mut bytes = ASN1String{ 0: Vec::new()}; // It is assumed that there will
    // be processing to fill bytes if !self.read_asn1(&mut bytes, INTEGER) ||
    // !check_asn1_integer(&bytes.0) { return false;
    //}

    // if bytes.0[0] & 0x80 == 0x80 {
    // Negative number.
    // let mut neg = bytes.0.iter().map(|b| !b).collect::<Vec<u8>>();
    //*out = BigInt::from_bytes_be(Sign::Plus,&neg); //out.set_bytes(&neg);
    // out.add_assign(&BigInt::from(1));
    // out.neg(); // out.negate();
    //} else {
    //*out = BigInt::from_bytes_be(Sign::Plus,&bytes.0);//out.set_bytes(&bytes);
    //}
    // true
    //}

    pub fn read_asn1_big_int(&mut self) -> Option<BigInt> {
        let mut bytes = ASN1String { 0: Vec::new() }; // It is assumed that there will be processing to fill bytes
        if !self.read_asn1(&mut bytes, INTEGER) || !check_asn1_integer(&bytes.0) {
            return None;
        }

        let big_one = BigInt::from(1); //BigInt::one();

        if bytes.0[0] & 0x80 == 0x80 {
            // Отрицательное число
            let neg: Vec<u8> = bytes.0.iter().map(|&b| !b).collect();
            let mut out = BigInt::from_signed_bytes_be(&neg); // let mut out = BigInt::from_bytes_neg(&BigEndian, &neg);
            out += &big_one;
            Some(-out)
        } else {
            Some(BigInt::from_signed_bytes_be(&bytes.0)) // Some(BigInt::from_bytes_positive(&BigEndian, &bytes))
        }
    }

    pub fn read_asn1_object_identifier(&mut self, out: &mut Vec<i32>) -> bool {
        let mut bytes = ASN1String { 0: Vec::new() }; // Initializing a vector to store bytes
        if !self.read_asn1(&mut bytes, OBJECT_IDENTIFIER) || bytes.0.is_empty() {
            return false;
        }

        // In the worst case, we get two elements from the first byte (which is encoded
        // differently), and then each varint is one byte.
        let mut components = vec![0; bytes.0.len() + 1];

        // The first varint is 40*value1 + value2:
        // value1 can take values 0, 1 and 2.
        let mut v: i32 = 0;
        if !bytes.read_base128_int(&mut v) {
            return false;
        }
        if v < 80 {
            components[0] = v / 40;
            components[1] = v % 40;
        } else {
            components[0] = 2;
            components[1] = v - 80;
        }

        let mut i = 2;
        while !bytes.0.is_empty() {
            if !bytes.read_base128_int(&mut v) {
                return false;
            }
            components[i] = v;
            i += 1;
        }
        *out = components[..i].to_vec();
        true
    }

    pub fn read_asn1_bytes(&mut self, out: &mut Vec<u8>) -> bool {
        let mut bytes = ASN1String { 0: Vec::new() };
        if !self.read_asn1(&mut bytes, INTEGER) || !check_asn1_integer(&bytes.0) {
            return false;
        }
        if bytes.0[0] & 0x80 == 0x80 {
            return false;
        }
        while bytes.0.len() > 1 && bytes.0[0] == 0 {
            bytes.0.remove(0);
        }
        *out = bytes.0;
        true
    }

    pub fn read_base128_int(&mut self, out: &mut i32) -> bool {
        let mut ret = 0;
        for i in 0.. {
            if self.0.len() == 0 {
                return false; // Handling end of data
            }
            if i == 5 {
                return false; // Too many bytes
            }
            // Avoiding int overflow on 32-bit platform
            if ret >= 1 << (31 - 7) {
                return false;
            }
            ret <<= 7;
            let b: u8 = self.read(1).unwrap()[0]; // Reading one byte

            // ITU-T X.690, section 8.19.2:
            // The sub-identifier must be encoded in the minimum possible number of octets,
            // i.e. the leading octet of the sub-identifier must not have the value 0x80.
            if i == 0 && b == 0x80 {
                return false;
            }

            ret |= (b & 0x7f) as i32;
            if b & 0x80 == 0 {
                *out = ret;
                return true;
            }
        }
        false // truncated data
    }

    pub fn read_asn1_impl(
        &mut self,
        out: &mut ASN1String,
        out_tag: &mut u8,
        skip_header: bool,
    ) -> bool {
        if self.0.len() < 2 {
            return false;
        }

        let tag = self.0[0];
        let len_byte = self.0[1];

        if tag & 0x1f == 0x1f {
            return false;
        }

        // if let Some(out_t) = out_tag {
        // *out_t = Tag(tag);
        //}
        *out_tag = tag;

        // ITU-T X.690 section 8.1.3
        let (length, header_len) = if len_byte & 0x80 == 0 {
            (u32::from(len_byte) + 2, 2)
        } else {
            let len_len = len_byte & 0x7f;

            if len_len == 0 || len_len > 4 || self.0.len() < (2 + len_len as usize) {
                return false;
            }

            let mut len_bytes = ASN1String(self.0[2..2 + len_len as usize].to_vec());
            let mut len32 = 0u32;
            if !len_bytes.read_unsigned(&mut len32, len_len as usize) {
                return false;
            }

            if len32 < 128 {
                return false;
            }
            if (len32 >> ((len_len - 1) * 8)) == 0 {
                return false;
            }

            let header_len = 2 + len_len as u32;
            if header_len + len32 < len32 {
                return false;
            }
            (header_len + len32, header_len)
        };

        if length as usize > self.0.len() || !self.read_bytes(out, length as usize) {
            return false;
        }

        if skip_header && !out.skip(header_len as usize) {
            panic!("cryptobyte: internal error");
        }

        true
    }

    // fn read_asn1_utc_time(&mut self) -> Result<DateTime<Utc>, String> {
    fn read_asn1_utc_time(&mut self) -> Option<DateTime<Utc>> {
        let mut bytes = ASN1String { 0: Vec::new() };
        if !self.read_asn1(&mut bytes, TAG_UTC_TIME) {
            return None; //return Err("Malformed UTCTime".to_string());
        }

        let t = String::from_utf8_lossy(&bytes.0).into_owned();
        let format_str = "%y%m%d%H%M%SZ"; // Standard UTCTime format
        match Utc.datetime_from_str(&t, format_str) {
            Ok(res) => {
                // Applying additional logic for 2050 year
                // if res.year() >= 2050 {
                // let res = res - chrono::Duration::days(36525); // -100 years
                // Ok(res)
                //} else {
                // Ok(res)
                //}
                Some(res)
            }
            Err(_) => None, // Err("Failed to parse UTCTime".to_string()),
        }
    }

    // fn read_asn1_generalized_time(&mut self) -> Result<DateTime<Utc>, String> {
    fn read_asn1_generalized_time(&mut self) -> Option<DateTime<Utc>> {
        let mut bytes = ASN1String { 0: Vec::new() };
        if !self.read_asn1(&mut bytes, TAG_GENERALIZED_TIME) {
            return None; //Err("Malformed GeneralizedTime".to_string());
        }

        let t = String::from_utf8_lossy(&bytes.0).into_owned();
        let format_str = "%Y%m%d%H%M%S%.fZ"; // Standard GeneralizedTime format
        match Utc.datetime_from_str(&t, format_str) {
            Ok(res) => Some(res), // Ok(res),
            Err(_) => None,       // Err("Failed to parse GeneralizedTime".to_string()),
        }
    }

    // Implementation of reading an unsigned integer from ASN.1
    // To simplify the implementation of the function, we assume that
    // the string is of sufficient length and returns "true" on success.
    pub fn read_unsigned(&mut self, out: &mut u32, length: usize) -> bool {
        let v = self.read(length);
        if v.is_none() {
            return false;
        }

        let v = v.unwrap();
        let mut result: u32 = 0;

        for byte in v {
            result <<= 8;
            result |= byte as u32;
        }

        *out = result;
        true
    }

    // Read n bytes, advancing the line
    fn read(&mut self, n: usize) -> Option<Vec<u8>> {
        if self.0.len() < n || n == 0 {
            return None;
        }

        let v = self.0[..n].to_vec(); // We get a cut and copy it
        self.0.drain(..n); // Remove read bytes from the internal vector
        Some(v)
    }

    // Implementation of reading bytes from ASN.1
    // By analogy with other code, should be implemented.
    pub fn read_bytes(&mut self, out: &mut ASN1String, length: usize) -> bool {
        if let Some(v) = self.read(length) {
            *out = ASN1String { 0: v }; // Copy the read bytes to out
            true
        } else {
            false
        }
    }

    // Skip advances the String by n byte and reports whether it was successful.
    fn skip(&mut self, length: usize) -> bool {
        // if length <= self.0.len() {
        // self.0.drain(..length);
        // true
        //} else {
        // false
        //}
        match self.read(length) {
            Some(res) => true,
            None => false,
        }
        // return s.read(n) != nil
    }
}

pub fn asn1_signed(out: &mut i64, n: &[u8]) -> bool {
    let length = n.len();
    if length > 8 {
        return false;
    }
    for &byte in n {
        *out <<= 8;
        *out |= byte as i64;
    }
    // Shift to extend the sign of the result.
    *out <<= 64 - (length as u8 * 8);
    *out >>= 64 - (length as u8 * 8);
    true
}

pub fn asn1_unsigned(out: &mut u64, n: &[u8]) -> bool {
    let length = n.len();
    if length > 9 || (length == 9 && n[0] != 0) {
        // Too large for uint64.
        return false;
    }
    if n[0] & 0x80 != 0 {
        // Negative number.
        return false;
    }
    for &byte in n {
        *out <<= 8;
        *out |= byte as u64;
    }
    true
}

// Checking the correctness of an ASN.1 INTEGER
pub fn check_asn1_integer(bytes: &[u8]) -> bool {
    if bytes.is_empty() {
        // INTEGER is encoded by at least one octet
        return false;
    }
    if bytes.len() == 1 {
        return true;
    }
    if (bytes[0] == 0 && (bytes[1] & 0x80) == 0) || (bytes[0] == 0xff && (bytes[1] & 0x80) == 0x80)
    {
        // The value is not minimally encoded
        return false;
    }
    return true;
}

// Represents a set of AttributeTypeAndValue
//#[derive(Debug, Clone)]
// pub struct AttributeTypeAndValueSET {
// pub rtype: ObjectIdentifier,
// pub value: Vec<Vec<AttributeTypeAndValue>>, // Vector of vectors
//}

// Represents an extension
#[derive(Debug, Clone)]
pub struct Extension {
    pub id: Vec<i32>, // //pub id: ObjectIdentifier,
    pub critical: bool, /* pub critical: Option<bool>, // Use Option to indicate an non-obvious
                       * field */
    pub value: Vec<u8>,
}

// Represents the X.509 distinguished name
#[derive(Debug, Clone)]
pub struct Name {
    pub country: Vec<String>,
    pub organization: Vec<String>,
    pub organizational_unit: Vec<String>,
    pub locality: Vec<String>,
    pub province: Vec<String>,
    pub street_address: Vec<String>,
    pub postal_code: Vec<String>,
    pub serial_number: String,
    pub common_name: String,
    pub names: Vec<AttributeTypeAndValue>, // All attributes parsed
    pub extra_names: Vec<AttributeTypeAndValue>, // Attributes copied to any serialized names
}

impl Name {
    pub fn default() -> Self {
        Name {
            country: Vec::new(),
            organization: Vec::new(),
            organizational_unit: Vec::new(),
            locality: Vec::new(),
            province: Vec::new(),
            street_address: Vec::new(),
            postal_code: Vec::new(),
            serial_number: String::new(),
            common_name: String::new(),
            names: Vec::new(),
            extra_names: Vec::new(),
        }
    }

    // FillFromRDNSequence populates Name from the provided [RDNSequence].
    // Multi-entry RDNs are flattened, all entries are added to the
    // relevant Name's fields, and the grouping is not preserved.
    pub fn fill_from_rdn_sequence(&mut self, rdns: &Vec<Vec<AttributeTypeAndValue>>) {
        for rdn in rdns {
            if rdn.is_empty() {
                continue;
            }

            for atv in rdn {
                self.names.push(atv.clone()); // Save the attribute

                // Checking the value
                let value = &atv.value;

                let t = &atv.atype;
                if t.len() == 4 && t[0] == 2 && t[1] == 5 && t[2] == 4 {
                    match t[3] {
                        3 => self.common_name = value.clone(),
                        5 => self.serial_number = value.clone(),
                        6 => self.country.push(value.clone()),
                        7 => self.locality.push(value.clone()),
                        8 => self.province.push(value.clone()),
                        9 => self.street_address.push(value.clone()),
                        10 => self.organization.push(value.clone()),
                        11 => self.organizational_unit.push(value.clone()),
                        17 => self.postal_code.push(value.clone()),
                        _ => {}
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
struct Certificate {
    raw: Vec<u8>,                         // Complete ASN.1 DER content
    raw_tbs_certificate: Vec<u8>,         // Certificate part of raw ASN.1 DER content
    raw_subject_public_key_info: Vec<u8>, // DER encoded SubjectPublicKeyInfo
    raw_subject: Vec<u8>,                 // DER encoded Subject
    raw_issuer: Vec<u8>,                  // DER encoded Issuer

    signature: Vec<u8>,
    signature_algorithm: SignatureAlgorithm,
    public_key_algorithm: PublicKeyAlgorithm,
    public_key: PublicKey, // Using trait object for dynamic dispatch

    version: i64,
    serial_number: BigInt, // serial_number: Option<BigInt>,          // Type for big integers
    issuer: Name,
    subject: Name,
    not_before: DateTime<Utc>, // i64,//SystemTime,
    not_after: DateTime<Utc>,  // i64,//SystemTime,
    key_usage: KeyUsage,

    extensions: Vec<Extension>, // Raw X.509 extensions
    extra_extensions: Vec<Extension>, /* Extensions to be copied raw into any
                                 * marshaled certificates */
    unhandled_critical_extensions: Vec<Vec<i32>>, // List of extension IDs not fully processed

    ext_key_usage: Vec<ExtKeyUsage>, // Sequence of extended key usages
    unknown_ext_key_usage: Vec<Vec<i32>>, /* unknown_ext_key_usage: Vec<asn1::ObjectIdentifier>,
                                      * // Encountered extended key usages unknown to this
                                      * package */

    basic_constraints_valid: bool, // Indicates if BasicConstraints are valid
    is_ca: bool,
    max_path_len: i32,       // MaxPathLen for BasicConstraints
    max_path_len_zero: bool, // Indicates if MaxPathLen is explicitly zero
    subject_key_id: Vec<u8>,
    authority_key_id: Vec<u8>,
    ocsp_server: Vec<String>, // Authority Information Access
    issuing_certificate_url: Vec<String>,

    dns_names: Vec<String>, // Subject Alternate Name values
    email_addresses: Vec<String>,
    ip_addresses: Vec<IpAddr>, // IP addresses
    uris: Vec<String>,         /* uris: Vec<url::Url>,                  // Assuming url is a
                                * module with Url struct */

    permitted_dns_domains_critical: bool,
    permitted_dns_domains: Vec<String>,
    excluded_dns_domains: Vec<String>,
    permitted_ip_ranges: Vec<IpNet>,
    excluded_ip_ranges: Vec<IpNet>,
    permitted_email_addresses: Vec<String>,
    excluded_email_addresses: Vec<String>,
    permitted_uri_domains: Vec<String>,
    excluded_uri_domains: Vec<String>,

    crl_distribution_points: Vec<String>,
    policy_identifiers: Vec<Vec<i32>>, // policy_identifiers: Vec<asn1::ObjectIdentifier>,
    policies: Vec<Vec<u8>>,            // policies: Vec<OID>, // Assuming OID is defined
}

impl Certificate {
    //
    fn check_signature_from(&mut self, parent: &Certificate) -> bool {
        // fn (c *Certificate) Check_signature_from(parent *Certificate) error
        // RFC 5280, 4.2.1.9:
        // "If the basic constraints extension is not present in a version 3
        // certificate, or the extension is present but the cA boolean is not
        // asserted, then the certified public key MUST NOT be used to verify
        // certificate signatures."

        if parent.version == 3 && !parent.basic_constraints_valid
            || parent.basic_constraints_valid && !parent.is_ca
        {
            return false; //return ConstraintViolationError{}
        }

        if parent.key_usage.0 != 0 && parent.key_usage.0 & KeyUsage::CERT_SIGN.0 == 0 {
            return false; // return ConstraintViolationError{}
        }

        if parent.public_key_algorithm == PublicKeyAlgorithm::UnknownPublicKeyAlgorithm {
            return false; //return ErrUnsupportedAlgorithm
        }

        // return checkSignature(c.SignatureAlgorithm, c.RawTBSCertificate, c.Signature,
        // parent.PublicKey, false);
        return check_signature(
            &self.signature_algorithm,
            &self.raw_tbs_certificate,
            &self.signature,
            &parent.public_key,
            false,
        );
    }
}

// CheckSignature verifies that signature is a valid signature over signed from
// c's public key.
//
// This is a low-level API that performs no validity checks on the certificate.
//
// [MD5WithRSA] signatures are rejected, while [SHA1WithRSA] and [ECDSAWithSHA1]
// signatures are currently accepted.
fn check_signature(
    algo: &SignatureAlgorithm,
    signed: &Vec<u8>,
    signature: &Vec<u8>,
    public_key: &PublicKey,
    allow_sha1: bool,
) -> bool {
    let signature_algorithm_details: Vec<AlgorithmDetails> = vec![
        AlgorithmDetails {
            algo: SignatureAlgorithm::MD2WithRSA,
            name: String::from("MD2-RSA"),
            oid: OID_SIGNATURE_MD2_WITH_RSA.to_vec(),
            pub_key_algo: PublicKeyAlgorithm::RSA,
            hash: None, // no value for MD2
        },
        AlgorithmDetails {
            algo: SignatureAlgorithm::MD5WithRSA,
            name: String::from("MD5-RSA"),
            oid: OID_SIGNATURE_MD5_WITH_RSA.to_vec(),
            pub_key_algo: PublicKeyAlgorithm::RSA,
            hash: Some(String::from("MD5")),
        },
        AlgorithmDetails {
            algo: SignatureAlgorithm::SHA1WithRSA,
            name: String::from("SHA1-RSA"),
            oid: OID_SIGNATURE_SHA1_WITH_RSA.to_vec(),
            pub_key_algo: PublicKeyAlgorithm::RSA,
            hash: Some(String::from("SHA1")),
        },
        AlgorithmDetails {
            algo: SignatureAlgorithm::SHA256WithRSA,
            name: String::from("SHA256-RSA"),
            oid: OID_SIGNATURE_SHA256_WITH_RSA.to_vec(),
            pub_key_algo: PublicKeyAlgorithm::RSA,
            hash: Some(String::from("SHA256")),
        },
        AlgorithmDetails {
            algo: SignatureAlgorithm::SHA384WithRSA,
            name: String::from("SHA384-RSA"),
            oid: OID_SIGNATURE_SHA384_WITH_RSA.to_vec(),
            pub_key_algo: PublicKeyAlgorithm::RSA,
            hash: Some(String::from("SHA384")),
        },
        AlgorithmDetails {
            algo: SignatureAlgorithm::SHA512WithRSA,
            name: String::from("SHA512-RSA"),
            oid: OID_SIGNATURE_SHA512_WITH_RSA.to_vec(),
            pub_key_algo: PublicKeyAlgorithm::RSA,
            hash: Some(String::from("SHA512")),
        },
        AlgorithmDetails {
            algo: SignatureAlgorithm::SHA256WithRSAPSS,
            name: String::from("SHA256-RSAPSS"),
            oid: OID_SIGNATURE_RSA_PSS.to_vec(),
            pub_key_algo: PublicKeyAlgorithm::RSA,
            hash: Some(String::from("SHA256")),
        },
        AlgorithmDetails {
            algo: SignatureAlgorithm::SHA384WithRSAPSS,
            name: String::from("SHA384-RSAPSS"),
            oid: OID_SIGNATURE_RSA_PSS.to_vec(),
            pub_key_algo: PublicKeyAlgorithm::RSA,
            hash: Some(String::from("SHA384")),
        },
        AlgorithmDetails {
            algo: SignatureAlgorithm::SHA512WithRSAPSS,
            name: String::from("SHA512-RSAPSS"),
            oid: OID_SIGNATURE_RSA_PSS.to_vec(),
            pub_key_algo: PublicKeyAlgorithm::RSA,
            hash: Some(String::from("SHA512")),
        },
        AlgorithmDetails {
            algo: SignatureAlgorithm::DSAWithSHA1,
            name: String::from("DSA-SHA1"),
            oid: OID_SIGNATURE_DSA_WITH_SHA1.to_vec(),
            pub_key_algo: PublicKeyAlgorithm::DSA,
            hash: Some(String::from("SHA1")),
        },
        AlgorithmDetails {
            algo: SignatureAlgorithm::DSAWithSHA256,
            name: String::from("DSA-SHA256"),
            oid: OID_SIGNATURE_DSA_WITH_SHA256.to_vec(),
            pub_key_algo: PublicKeyAlgorithm::DSA,
            hash: Some(String::from("SHA256")),
        },
        AlgorithmDetails {
            algo: SignatureAlgorithm::ECDSAWithSHA1,
            name: String::from("ECDSA-SHA1"),
            oid: OID_SIGNATURE_ECDSA_WITH_SHA1.to_vec(),
            pub_key_algo: PublicKeyAlgorithm::ECDSA,
            hash: Some(String::from("SHA1")),
        },
        AlgorithmDetails {
            algo: SignatureAlgorithm::ECDSAWithSHA256,
            name: String::from("ECDSA-SHA256"),
            oid: OID_SIGNATURE_ECDSA_WITH_SHA256.to_vec(),
            pub_key_algo: PublicKeyAlgorithm::ECDSA,
            hash: Some(String::from("SHA256")),
        },
        AlgorithmDetails {
            algo: SignatureAlgorithm::ECDSAWithSHA384,
            name: String::from("ECDSA-SHA384"),
            oid: OID_SIGNATURE_ECDSA_WITH_SHA384.to_vec(),
            pub_key_algo: PublicKeyAlgorithm::ECDSA,
            hash: Some(String::from("SHA384")),
        },
        AlgorithmDetails {
            algo: SignatureAlgorithm::ECDSAWithSHA512,
            name: String::from("ECDSA-SHA512"),
            oid: OID_SIGNATURE_ECDSA_WITH_SHA512.to_vec(),
            pub_key_algo: PublicKeyAlgorithm::ECDSA,
            hash: Some(String::from("SHA512")),
        },
        AlgorithmDetails {
            algo: SignatureAlgorithm::PureEd25519,
            name: String::from("Ed25519"),
            oid: OID_SIGNATURE_ED25519.to_vec(),
            pub_key_algo: PublicKeyAlgorithm::Ed25519,
            hash: Some(String::from("")),
        },
    ];

    let mut hash_type = None;
    let mut pub_key_algo = PublicKeyAlgorithm::UnknownPublicKeyAlgorithm;

    for details in signature_algorithm_details {
        if details.algo == *algo {
            hash_type = details.hash;
            pub_key_algo = details.pub_key_algo;
            break;
        }
    }

    // match hash_type.unwrap() {
    //"SHA256" => {
    // signed = hkdf_sha256::sum256(signed);
    //}
    let hash_type_str = hash_type.unwrap();
    let mut hash_len_in_bits: usize = 0;
    let hashed = match hash_type_str.as_str() {
        "SHA256" => {
            hash_len_in_bits = 256;
            hkdf_sha256::sum256(signed).to_vec()
        }
        "SHA384" => {
            hash_len_in_bits = 384;
            sha512::sum384(signed).to_vec()
        }
        "SHA512" => {
            hash_len_in_bits = 512;
            sha512::sum512(signed).to_vec()
        }
        _ => panic!("unknown hash type"),
    };

    // match hash_type.unwrap() {
    //"MD5" => ... ,
    //"SHA1" => ... ,
    //_ => ... ,
    //}

    match public_key {
        PublicKey::RsaPublicKey(rsa_pub_key) => {
            if pub_key_algo != PublicKeyAlgorithm::RSA {
                return false; // signaturePublicKeyAlgoMismatchError(pubKeyAlgo, pub)
            }
            if algo.is_rsa_pss() {
                let pss_options =
                    rsa::PSSOptions { salt_length: rsa::PSS_SALT_LENGTH_EQUALS_HASH, hash: 0 };
                return rsa::verify_pss(
                    rsa_pub_key,
                    hash_len_in_bits,
                    &hashed,
                    signature,
                    &pss_options,
                );
            } else {
                // return rsa::verify_pkcs1v15(rsa_pub_key, hash_type, signed, signature);
                return rsa::verify_pkcs1v15(rsa_pub_key, hash_len_in_bits, &hashed, signature);
            }
        }
        PublicKey::ECDSAPublicKey(ecdsa_pub_key) => {
            if pub_key_algo != PublicKeyAlgorithm::ECDSA {
                return false;
            }
            if !ecdsa_verify_asn1(ecdsa_pub_key, &hashed, signature) {
                return false; // "x509: ECDSA verification failure")
            }
            return true;
        }
        PublicKey::ED25519PublicKey(ed25519_pub_key) => {
            if pub_key_algo != PublicKeyAlgorithm::Ed25519 {
                return false;
            }
            if !ed25519::verify(ed25519_pub_key, &hashed, signature) {
                return false; // "x509: Ed25519 verification failure")
            }
            return true;
        }
        _ => return false,
    }
}

fn ecdsa_verify_asn1(pub_key: &ecdsa::PublicKey, signed: &[u8], signature: &Vec<u8>) -> bool {
    if let Some((r_bytes, s_bytes)) = parse_signature(&signature) {
        //
        let c = pub_key.curve.params();

        // let q = *c.point_from_affine();

        let r = BigInt::from_bytes_be(Sign::Plus, &r_bytes);
        let s = BigInt::from_bytes_be(Sign::Plus, &s_bytes);

        if !ecdsa::verify(&pub_key, &signed, &r, &s) {
            return false;
        }

        return true;
    } else {
        return false;
    }
}

fn parse_signature(sig: &Vec<u8>) -> Option<(Vec<u8>, Vec<u8>)> {
    // fn parse_signature(sig: &[u8]) -> (r, s []byte, err error) {
    let mut inner = ASN1String { 0: Vec::new() }; //var inner cryptobyte.String

    let mut r: Vec<u8> = Vec::new();
    let mut s: Vec<u8> = Vec::new();
    let mut input = ASN1String { 0: sig.clone() }; // input := cryptobyte.String(sig)
    if !input.read_asn1(&mut inner, SEQUENCE) ||   // if !input.ReadASN1(&inner, asn1.SEQUENCE) ||
		!input.0.is_empty() ||  //  !input.Empty() ||
		!inner.read_asn1_bytes(&mut r) || // !inner.ReadASN1Integer(&r) ||
		!inner.read_asn1_bytes(&mut s) || // !inner.ReadASN1Integer(&s) ||
		!inner.0.is_empty()
    {
        // !inner.Empty() {
        return None; //return nil, nil, errors.New("invalid ASN.1")
    }
    return Some((r, s)); //return r, s, nil
}

// pub fn parse_name_constraints_extension(e: &Extension) -> Result<(bool,
// Box<dyn Error>), Box<dyn Error>> {
pub fn parse_name_constraints_extension(out: &mut Certificate, e: &Extension) -> Option<bool> {
    // RFC 5280, 4.2.1.10

    // NameConstraints ::= SEQUENCE {
    //      permittedSubtrees       [0]     GeneralSubtrees OPTIONAL,
    //      excludedSubtrees        [1]     GeneralSubtrees OPTIONAL }
    //
    // GeneralSubtrees ::= SEQUENCE SIZE (1..MAX) OF GeneralSubtree
    //
    // GeneralSubtree ::= SEQUENCE {
    //      base                    GeneralName,
    //      minimum         [0]     BaseDistance DEFAULT 0,
    //      maximum         [1]     BaseDistance OPTIONAL }
    //
    // BaseDistance ::= INTEGER (0..MAX)
    let mut outer = ASN1String { 0: e.value.clone() };
    let mut toplevel = ASN1String { 0: Vec::new() };
    let mut permitted = ASN1String { 0: Vec::new() };
    let mut excluded = ASN1String { 0: Vec::new() };

    let mut have_permitted = false;
    let mut have_excluded = false;

    if !outer.read_asn1(&mut toplevel, SEQUENCE)
        || !outer.0.is_empty()
        || !toplevel.read_optional_asn1(
            &mut permitted,
            &mut have_permitted,
            constructed(context_specific(0u8)),
        )
        || !toplevel.read_optional_asn1(
            &mut excluded,
            &mut have_excluded,
            constructed(context_specific(0u8)),
        )
        || !toplevel.0.is_empty()
    {
        return None; //  "invalid NameConstraints extension"
    };

    if !have_permitted && !have_excluded && permitted.0.is_empty() && excluded.0.is_empty() {
        return None; //Ok((false, "empty name constraints extension".into()));
    };

    let mut unhandled = false;

    // let get_values = |subtrees: cryptobyte::String| -> Result<(Vec<String>,
    // Vec<IpNet>, Vec<String>, Vec<String>), Box<dyn Error>> {
    let mut get_values =
        |subtrees: &mut ASN1String| -> Option<(Vec<String>, Vec<IpNet>, Vec<String>, Vec<String>)> {
            let mut dns_names: Vec<String> = Vec::new();
            let mut emails: Vec<String> = Vec::new();
            let mut ips: Vec<IpNet> = Vec::new();
            let mut uri_domains: Vec<String> = Vec::new();
            while !subtrees.0.is_empty() {
                let mut seq = ASN1String { 0: Vec::new() };
                let mut value = ASN1String { 0: Vec::new() };
                let mut tag = 0u8;
                if !subtrees.read_asn1(&mut seq, SEQUENCE)
                    || seq.read_any_asn1(&mut value, &mut tag)
                {
                    return None; // "invalid NameConstraints extension".
                }

                const DNS_TAG: u8 = context_specific(2u8);
                const EMAIL_TAG: u8 = context_specific(1u8);
                const IP_TAG: u8 = context_specific(7u8);
                const URI_TAG: u8 = context_specific(6u8);

                match tag {
                    DNS_TAG => {
                        let domain = String::from_utf8_lossy(&value.0).to_string();
                        if !is_ia5_string(&domain) {
                            return None; // invalid constraint value: {}", domain
                        }
                        let trimmed_domain = domain.trim_start_matches('.');

                        if domain_to_reverse_labels(trimmed_domain).is_none() {
                            return None; // "x509: failed to parse dnsName constraint {}", domain
                        }

                        dns_names.push(domain);
                    }
                    IP_TAG => {
                        let l = value.0.len();
                        let mut ip: Vec<u8> = Vec::new();
                        let mut mask: Vec<u8> = Vec::new();
                        match l {
                            8 => {
                                ip = value.0[..4].to_vec();
                                mask = value.0[4..].to_vec();
                            }
                            32 => {
                                ip = value.0[..16].to_vec();
                                mask = value.0[16..].to_vec();
                            }
                            _ => return None, /* "x509: IP constraint contained value of length
                                               * {}", l */
                        }
                        if !is_valid_ip_mask(&mask) {
                            return None; //
                        }
                        ips.push(IpNet { ip, mask });
                    }
                    EMAIL_TAG => {
                        let constraint = String::from_utf8_lossy(&value.0).to_string();
                        if !is_ia5_string(&constraint) {
                            return None; // "x509: invalid constraint value: " + err.Error()
                        }

                        if constraint.contains('@') {
                            // if parseRFC2821Mailbox(constraint).is_none() {
                            // return None; "x509: failed to parse rfc822Name
                            // constraint {]", constraint
                            //}
                        } else {
                            // Otherwise it's a domain name.
                            let domain = constraint.trim_start_matches('.').to_string();
                            // let mut domain = constraint.clone();
                            // if domain.len() > 0 && domain.get(0).unwrap() == '.' {
                            // domain = domain[1..].to_string();
                            //}
                            if domain_to_reverse_labels(&domain).is_none() {
                                return None; //
                            }
                        }
                        emails.push(constraint);
                    }
                    URI_TAG => {
                        let domain = String::from_utf8_lossy(&value.0).to_string();
                        if !is_ia5_string(&domain) {
                            return None; // "x509: invalid constraint value: "
                        }

                        if IpAddr::from_str(&domain).is_err() {
                            return None; // "x509: failed to parse URI constraint {}: cannot be IP address", domain
                        }

                        let trimmed_domain = domain.trim_start_matches('.');

                        if domain_to_reverse_labels(trimmed_domain).is_none() {
                            return None; // "x509: failed to parse URI constraint %q", domain)
                        }

                        uri_domains.push(domain);
                    }
                    _ => unhandled = true,
                }
            }
            return Some((dns_names, ips, emails, uri_domains));
        };

    let result = get_values(&mut permitted);
    if result.is_none() {
        return None;
    } else {
        out.permitted_dns_domains = result.clone().unwrap().0;
        out.permitted_ip_ranges = result.clone().unwrap().1;
        out.permitted_email_addresses = result.clone().unwrap().2;
        out.permitted_uri_domains = result.clone().unwrap().3;
        // return Some(false); // unhandled = false
    }
    let result = get_values(&mut excluded);
    if result.is_none() {
        return None;
    } else {
        out.excluded_dns_domains = result.clone().unwrap().0;
        out.excluded_ip_ranges = result.clone().unwrap().1;
        out.excluded_email_addresses = result.clone().unwrap().2;
        out.excluded_uri_domains = result.clone().unwrap().3;
    }
    out.permitted_dns_domains_critical = e.critical; // out.permitted_dns_domains_critical = e.critical.unwrap();

    return Some(unhandled);
}

fn process_extensions(out: &mut Certificate) -> bool {
    for e in out.extensions.clone() {
        let mut unhandled = false;

        if e.id.len() == 4 && e.id[0] == 2 && e.id[1] == 5 && e.id[2] == 29 {
            match e.id[3] {
                15 => {
                    out.key_usage = parse_key_usage_extension(&e.value).unwrap(); //out.key_usage = Some(KeyUsage::parse(&e.value)?);
                }
                19 => {
                    out.is_ca = parse_basic_constraints_extension(&e.value).unwrap().0;
                    out.max_path_len = parse_basic_constraints_extension(&e.value).unwrap().1;
                    out.basic_constraints_valid = true;
                    out.max_path_len_zero = out.max_path_len == 0;
                }
                17 => {
                    let (dns_names, email_addresses, ip_addresses, uris) =
                        parse_san_extension(&e.value).unwrap();
                    out.dns_names = dns_names;
                    out.email_addresses = email_addresses;
                    out.ip_addresses = ip_addresses;
                    out.uris = uris;

                    if out.dns_names.is_empty()
                        && out.email_addresses.is_empty()
                        && out.ip_addresses.is_empty()
                        && out.uris.is_empty()
                    {
                        unhandled = true;
                    }
                }
                30 => {
                    unhandled = parse_name_constraints_extension(out, &e).unwrap(); //parse_name_constraints_extension(out, e)?;
                }
                31 => {
                    // Handle CRLDistributionPoints
                    // RFC 5280, 4.2.1.13

                    // CRLDistributionPoints ::= SEQUENCE SIZE (1..MAX) OF DistributionPoint
                    //
                    // DistributionPoint ::= SEQUENCE {
                    //     distributionPoint       [0]     DistributionPointName OPTIONAL,
                    //     reasons                 [1]     ReasonFlags OPTIONAL,
                    //     cRLIssuer               [2]     GeneralNames OPTIONAL }
                    //
                    // DistributionPointName ::= CHOICE {
                    //     fullName                [0]     GeneralNames,
                    //     nameRelativeToCRLIssuer [1]     RelativeDistinguishedName }
                    let mut val_ = ASN1String { 0: e.value.clone() }; // Convert to a suitable type for ASN.1 reading
                    let mut val = ASN1String { 0: Vec::new() };

                    if !val_.read_asn1(&mut val, SEQUENCE) {
                        return false; // "x509: invalid CRL distribution points"
                    }

                    while !val.0.is_empty() {
                        let mut dp_der = ASN1String { 0: Vec::new() }; // Read the next Sequence
                        if !val.read_asn1(&mut dp_der, SEQUENCE) {
                            return false; // "x509: invalid CRL distribution points"
                        }
                        let mut dp_name_der = ASN1String { 0: Vec::new() };
                        let mut dp_name_present = false;
                        if !dp_der.read_optional_asn1(
                            &mut dp_name_der,
                            &mut dp_name_present,
                            context_specific(constructed(0u8)),
                        ) {
                            return false; // "x509: invalid CRL distribution points"
                        }

                        if !dp_name_present {
                            continue;
                        }

                        let mut dp_name_der_ = ASN1String { 0: Vec::new() };
                        if !dp_name_der
                            .read_asn1(&mut dp_name_der_, context_specific(constructed(0u8)))
                        {
                            return false; // "x509: invalid CRL distribution points"
                        }

                        while !dp_name_der_.0.is_empty() {
                            if !dp_name_der_.peek_asn1_tag(context_specific(6u8)) {
                                break;
                            }

                            let mut uri = ASN1String { 0: Vec::new() };
                            if !dp_name_der_.read_asn1(&mut uri, context_specific(6u8)) {
                                return false; // "x509: invalid CRL distribution points"
                            }

                            out.crl_distribution_points
                                .push(String::from_utf8_lossy(&uri.0).to_string());
                        }
                    }
                }
                35 => {
                    // RFC 5280, 4.2.1.1
                    let mut val = ASN1String { 0: e.value.clone() };
                    let mut akid = ASN1String { 0: Vec::new() };
                    if !val.read_asn1(&mut akid, SEQUENCE) {
                        return false; // "x509: invalid authority key identifier"
                    }
                    if akid.peek_asn1_tag(context_specific(0u8)) {
                        let mut akid_ = ASN1String { 0: Vec::new() };
                        if !akid.read_asn1(&mut akid_, context_specific(0u8)) {
                            return false; // "x509: invalid authority key identifier"
                        }
                        out.authority_key_id = akid.0;
                    }
                }
                37 => {
                    let parse_result = parse_ext_key_usage_extension(&e.value);
                    if !parse_result.is_none() {
                        out.ext_key_usage = parse_result.clone().unwrap().0;
                        out.unknown_ext_key_usage = parse_result.clone().unwrap().1;
                    }
                    // out.ExtKeyUsage, out.UnknownExtKeyUsage, err =
                    // parse_ext_key_usage_extension(e.Value);
                }
                14 => {
                    // RFC 5280, 4.2.1.2
                    let mut val = ASN1String { 0: e.value.clone() };
                    let mut skid = ASN1String { 0: Vec::new() };
                    if !val.read_asn1(&mut skid, OCTET_STRING) {
                        return false; // "x509: invalid subject key identifier"
                    }
                    out.subject_key_id = skid.0;
                }
                32 => {
                    let parse_results = parse_certificate_policies_extension(&e.value);
                    if parse_results.is_none() {
                        return false; // or err from parse_result
                    }

                    // out.policy_identifiers =
                    // out.PolicyIdentifiers = make([]asn1.ObjectIdentifier, 0,
                    // len(out.Policies)) 				for _, oid :=
                    // range out.Policies { 					if oid, ok
                    // := oid.toASN1OID(); ok {
                    // 						out.PolicyIdentifiers =
                    // append(out.PolicyIdentifiers, oid)
                    // 					}
                    // 				}
                }
                _ => unhandled = true,
            }
        } else if e.id == OID_EXTENSION_AUTHORITY_INFO_ACCESS {
            // RFC 5280 4.2.2.1: Authority Information Access
            let mut val_ = ASN1String { 0: e.value.clone() };
            let mut val = ASN1String { 0: Vec::new() };
            if !val_.read_asn1(&mut val, SEQUENCE) {
                return false; // "x509: invalid authority info access"
            }
            while !val.0.is_empty() {
                let mut aia_der = ASN1String { 0: Vec::new() };
                if !val.read_asn1(&mut aia_der, SEQUENCE) {
                    return false; // "x509: invalid authority info access"
                }
                let mut method: Vec<i32> = Vec::new();
                if !aia_der.read_asn1_object_identifier(&mut method) {
                    return false; // "x509: invalid authority info access"
                }

                if !aia_der.peek_asn1_tag(context_specific(6u8)) {
                    continue;
                }

                let mut aia_der_ = ASN1String { 0: Vec::new() };
                if !aia_der.read_asn1(&mut aia_der_, context_specific(6u8)) {
                    return false; // "x509: invalid authority info access"
                }
                let method_slice: [i32; 9] = method.as_slice().try_into().unwrap();
                match method_slice {
                    OID_AUTHORITY_INFO_ACCESS_OCSP => {
                        out.ocsp_server.push(String::from_utf8_lossy(&aia_der_.0).to_string());
                    }
                    OID_AUTHORITY_INFO_ACCESS_ISSUERS => out
                        .issuing_certificate_url
                        .push(String::from_utf8_lossy(&aia_der_.0).to_string()),
                    _ => {}
                }
            }
        } else {
            // Unknown extensions are recorded if critical.
            unhandled = true;
        }

        if e.critical && unhandled {
            // if e.critical.unwrap() && unhandled {
            out.unhandled_critical_extensions.push(e.id.clone());
        }
    }
    return true;
}

fn parse_certificate(der: &[u8]) -> Certificate {
    // fn parse_certificate(der: &[u8]) -> Result<Certificate, Box<dyn Error>> {
    let mut cert = Certificate {
        raw: Vec::new(),
        raw_tbs_certificate: Vec::new(),
        raw_subject_public_key_info: Vec::new(),
        raw_subject: Vec::new(),
        raw_issuer: Vec::new(),
        signature: Vec::new(),
        signature_algorithm: SignatureAlgorithm::UnknownSignatureAlgorithm, // Default value
        public_key_algorithm: PublicKeyAlgorithm::UnknownPublicKeyAlgorithm, // Default value
        public_key: PublicKey::UnknownPubicKey,
        version: 0,
        serial_number: BigInt::default(),
        issuer: Name::default(),
        subject: Name::default(),
        not_before: DateTime::<Utc>::MIN_UTC, // 0i64,//SystemTime::now(),
        not_after: DateTime::<Utc>::MAX_UTC,  // SystemTime::now(),
        key_usage: KeyUsage::CERT_SIGN,
        extensions: Vec::new(),
        extra_extensions: Vec::new(),
        unhandled_critical_extensions: Vec::new(),
        ext_key_usage: Vec::new(),
        unknown_ext_key_usage: Vec::new(),
        basic_constraints_valid: false,
        is_ca: false,
        max_path_len: 0,
        max_path_len_zero: false,
        subject_key_id: Vec::new(),
        authority_key_id: Vec::new(),
        ocsp_server: Vec::new(),
        issuing_certificate_url: Vec::new(),
        dns_names: Vec::new(),
        email_addresses: Vec::new(),
        ip_addresses: Vec::new(),
        uris: Vec::new(),
        permitted_dns_domains_critical: false,
        permitted_dns_domains: Vec::new(),
        excluded_dns_domains: Vec::new(),
        permitted_ip_ranges: Vec::new(),
        excluded_ip_ranges: Vec::new(),
        permitted_email_addresses: Vec::new(),
        excluded_email_addresses: Vec::new(),
        permitted_uri_domains: Vec::new(),
        excluded_uri_domains: Vec::new(),
        crl_distribution_points: vec![],
        policy_identifiers: vec![],
        policies: vec![],
    };

    let mut input = ASN1String { 0: der.to_vec() };
    // we read the SEQUENCE including length and tag bytes so that
    // we can populate Certificate.Raw, before unwrapping the
    // SEQUENCE so it can be operated on

    let mut input1 = input.clone();

    if !input.read_asn1_element(&mut input1, SEQUENCE) {
        // return Err("x509: malformed certificate".into());
        panic!("x509: malformed certificate");
    }
    cert.raw = input1.0.clone();

    if !input1.read_asn1(&mut input, SEQUENCE) {
        // return Err("x509: malformed certificate".into());
        panic!("x509: malformed certificate");
    }

    let mut tbs = ASN1String { 0: Vec::new() }; // Suitable type for tbs

    if !input.read_asn1_element(&mut tbs, SEQUENCE) {
        // return Err("x509: malformed tbs certificate".into());
        panic!("x509: malformed tbs certificate");
    }

    cert.raw_tbs_certificate = tbs.0.clone();

    let mut tbs1 = tbs.clone();
    if !tbs.read_asn1(&mut tbs1, SEQUENCE) {
        // return Err("x509: malformed version".into());
        panic!("x509: malformed tbs certificate");
    }

    // if !tbs1.read_optional_asn1_integer(&mut cert.version,
    // Tag(0).constructed().context_specific(), 0) {
    if !tbs1.read_optional_asn1_integer(&mut cert.version, context_specific(constructed(0u8)), 0) {
        // return Err("x509: malformed version".into());
        panic!("x509: malformed tbs certificate");
    }

    if cert.version < 0 {
        // return Err("x509: malformed version".into());
        panic!("x509: malformed version");
    }

    cert.version += 1;
    if cert.version > 3 {
        // return Err("x509: invalid version".into());
        panic!("x509: invalid version");
    }

    match tbs1.read_asn1_big_int() {
        Some(serial) => cert.serial_number = serial,
        None => panic!("x509: malformed serial number"),
    }

    let mut sig_ai_seq = ASN1String { 0: Vec::new() };
    if !tbs1.read_asn1(&mut sig_ai_seq, SEQUENCE) {
        // return Err("x509: malformed signature algorithm identifier".into());
        panic!("x509: malformed signature algorithm identifier");
    }

    // Before parsing the inner algorithm identifier, extract
    // the outer algorithm identifier and make sure that they
    // match.
    let mut outer_sig_ai_seq = ASN1String { 0: Vec::new() };
    if !input.read_asn1(&mut outer_sig_ai_seq, SEQUENCE) {
        // return Err("x509: malformed algorithm identifier".into());
        panic!("x509: malformed algorithm identifier");
    }

    if outer_sig_ai_seq.0 != sig_ai_seq.0 {
        // if outer_sig_ai_seq != sig_ai_seq {
        // return Err("x509: inner and outer signature algorithm identifiers don't
        // match".into());
        panic!("x509: inner and outer signature algorithm identifiers don't match");
    }

    let sig_ai = parse_ai(&mut sig_ai_seq);
    cert.signature_algorithm = get_signature_algorithm_from_ai(sig_ai);

    let mut issuer_seq = ASN1String { 0: Vec::new() };
    if !tbs1.read_asn1_element(&mut issuer_seq, SEQUENCE) {
        // return Err("x509: malformed issuer".into());
        panic!("x509: malformed issuer");
    }
    cert.raw_issuer = issuer_seq.0.clone();
    let issuer_rdns = parse_name(&mut issuer_seq);
    cert.issuer.fill_from_rdn_sequence(&issuer_rdns);

    let mut validity = ASN1String { 0: Vec::new() };
    if !tbs1.read_asn1(&mut validity, SEQUENCE) {
        panic!("x509: malformed validity");
    }

    (cert.not_before, cert.not_after) = parse_validity(&mut validity).unwrap();

    let mut subject_seq = ASN1String { 0: Vec::new() };
    if !tbs1.read_asn1_element(&mut subject_seq, SEQUENCE) {
        panic!("x509: malformed issuer");
    }
    cert.raw_subject = subject_seq.0.clone();
    let subject_rdns = parse_name(&mut subject_seq);

    cert.subject.fill_from_rdn_sequence(&subject_rdns);

    let mut spki = ASN1String { 0: Vec::new() };
    if !tbs1.read_asn1_element(&mut spki, SEQUENCE) {
        panic!("x509: malformed spki");
    }
    cert.raw_subject_public_key_info = spki.0.clone();
    let mut spki1 = ASN1String { 0: Vec::new() };
    if !spki.read_asn1(&mut spki1, SEQUENCE) {
        panic!("x509: malformed spki"); //return nil, errors.New("x509: malformed spki")
    }
    let mut pk_ai_seq = ASN1String { 0: Vec::new() };
    if !spki1.read_asn1(&mut pk_ai_seq, SEQUENCE) {
        panic!("x509: malformed public key algorithm identifier");
    }

    let pk_ai = parse_ai(&mut pk_ai_seq); //pkAI, err := parseAI(pkAISeq)
    // if err != nil {
    // return nil, err
    //}
    cert.public_key_algorithm = get_public_key_algorithm_from_oid(&pk_ai.algorithm);
    let mut spk = BitString { bytes: Vec::new(), bit_length: 0 }; //var spk asn1.BitString
    if !spki1.read_asn1_bitstring(&mut spk) {
        panic!("x509: malformed subjectPublicKey");
    }
    if cert.public_key_algorithm != PublicKeyAlgorithm::UnknownPublicKeyAlgorithm {
        let public_key_info = PublicKeyInfo { raw: Vec::new(), algorithm: pk_ai, public_key: spk };
        cert.public_key = parse_public_key(&public_key_info);
        // cert.PublicKey, err = parsePublicKey(&publicKeyInfo{
        // Algorithm: pkAI,
        // PublicKey: spk,
        //})
        // if err != nil {
        // return nil, err
        //}
    }

    if cert.version > 1 {
        if !tbs1.skip_optional_asn1(context_specific(1u8)) {
            panic!("x509: malformed issuerUniqueID");
        }
        if !tbs1.skip_optional_asn1(context_specific(2u8)) {
            panic!("x509: malformed subjectUniqueID");
        }

        if cert.version == 3 {
            let mut extensions = ASN1String { 0: Vec::new() };
            let mut present = false;
            if !tbs1.read_optional_asn1(
                &mut extensions,
                &mut present,
                context_specific(constructed(3u8)),
            ) {
                panic!("x509: malformed extensions");
            }

            if present {
                let mut seen_exts: HashMap<String, bool> = HashMap::new(); // seenExts := make(map[string]bool)
                let mut extensions1 = ASN1String { 0: Vec::new() };
                if !extensions.read_asn1(&mut extensions1, SEQUENCE) {
                    panic!("x509: malformed extensions");
                }

                while !extensions1.0.is_empty() {
                    let mut extension = ASN1String { 0: Vec::new() };
                    if !extensions1.read_asn1(&mut extension, SEQUENCE) {
                        panic!("x509: malformed extension");
                    }
                    let ext = parse_extension(&mut extension); //ext, err := parseExtension(extension)
                    // if err != nil {
                    // return nil, err
                    //}

                    let oid_str = to_oid_string(&ext.id); //oidStr := ext.Id.String()
                    if !seen_exts.get(&oid_str).is_none() {
                        panic!("x509: certificate contains duplicate extensions");
                    }
                    // if seenExts[oidStr] {
                    // return nil, errors.New("x509: certificate contains duplicate extensions")
                    //}
                    seen_exts.insert(oid_str, true); //seenExts[oidStr] = true
                    cert.extensions.push(ext);
                }

                if !process_extensions(&mut cert) {
                    panic!("x509: malformed with certificate extensions");
                }
            }
        }
    }

    let mut signature = BitString { bytes: vec![], bit_length: 0 };
    if !input.read_asn1_bitstring(&mut signature) {
        panic!("x509: malformed signature");
    }
    cert.signature = signature.right_align();

    cert
}

fn to_oid_string(data: &Vec<i32>) -> String {
    let mut s = String::new();
    let mut first = true;

    for &v in data.iter() {
        if !first {
            s.push('.');
        }
        s.push_str(&v.to_string());
        first = false;
    }
    s
}

#[derive(Debug, Clone)]
struct AttributeTypeAndValue {
    atype: Vec<i32>,
    value: String,
}

// struct RDN_sequence {

//};// []RelativeDistinguishedNameSET

// struct RelativeDistinguishedNameSET (
// Vec<AttributeTypeAndValue>,
//);

// parseName parses a DER encoded Name as defined in RFC 5280. We may
// want to export this function in the future for use in crypto/tls.
pub fn parse_name(raw: &mut ASN1String) -> Vec<Vec<AttributeTypeAndValue>> {
    // pub fn parse_name(raw: &mut ASN1String) -> RDN_sequence {
    //
    let mut der = ASN1String { 0: Vec::new() };
    if !raw.read_asn1(&mut der, SEQUENCE) {
        panic!("x509: invalid RDNSequence");
    }

    let mut rdn_seq: Vec<Vec<AttributeTypeAndValue>> = Vec::new(); // let mut rdn_seq: RDNSequence;
    while !der.0.is_empty() {
        let mut rdn_set: Vec<AttributeTypeAndValue> = Vec::new(); // let mut rdn_set: RelativeDistinguishedNameSET;
        let mut set = ASN1String { 0: Vec::new() };
        if !der.read_asn1(&mut set, SET) {
            // return nil, errors.New("x509: invalid RDNSequence")
            panic!("x509: invalid RDNSequence");
        }

        while !set.0.is_empty() {
            let mut atav = ASN1String { 0: Vec::new() };
            if !set.read_asn1(&mut atav, SEQUENCE) {
                panic!("x509: invalid RDNSequence: invalid attribute");
            }

            let mut attr: AttributeTypeAndValue =
                AttributeTypeAndValue { atype: Vec::new(), value: String::new() };
            if !atav.read_asn1_object_identifier(&mut attr.atype) {
                panic!("x509: invalid RDNSequence: invalid attribute type");
            }

            let mut raw_value = ASN1String { 0: Vec::new() };
            let mut value_tag = 0u8;
            if !atav.read_any_asn1(&mut raw_value, &mut value_tag) {
                panic!("x509: invalid RDNSequence: invalid attribute value");
            }

            attr.value = parse_asn1_string(value_tag, &raw_value.0)
                .expect("x509: invalid RDNSequence: invalid attribute value: %s");

            rdn_set.push(attr);
        }
        rdn_seq.push(rdn_set);
    }

    return rdn_seq;
}

// pub fn parse_asn1_string(tag: u8, value: &[u8]) -> Result<String, ASN1Error>
// {
pub fn parse_asn1_string(tag: u8, value: &[u8]) -> Option<String> {
    match tag {
        T61_STRING => Some(String::from_utf8_lossy(value).to_string()), /* Ok(String::from_utf8_lossy(value).to_string()), */
        PRINTABLE_STRING => {
            for &b in value {
                if !is_printable(b) {
                    return None; //return Err(ASN1Error::InvalidPrintableString);
                }
            }
            Some(String::from_utf8_lossy(value).to_string()) // Ok(String::from_utf8_lossy(value).to_string())
        }
        UTF8_STRING => {
            if !is_valid_utf8(&value) {
                return None; //return Err(ASN1Error::InvalidUTF8String);
            }
            Some(String::from_utf8_lossy(value).to_string()) // Ok(String::from_utf8_lossy(value).to_string())
        }
        BMP_STRING => {
            if value.len() % 2 != 0 {
                return None; // return Err(ASN1Error::InvalidBMPString);
            }

            let mut decoded = Vec::new();
            let mut iter = value.chunks_exact(2);
            for chunk in iter {
                let code_unit = ((chunk[0] as u16) << 8) | (chunk[1] as u16);
                decoded.push(code_unit);
            }

            // Strip terminator if present.
            if decoded.len() >= 2
                && decoded[decoded.len() - 1] == 0
                && decoded[decoded.len() - 2] == 0
            {
                decoded.pop();
                decoded.pop();
            }

            // Convert u16 vector to String
            match String::from_utf16(&decoded) {
                Ok(string) => return Some(string),
                Err(_) => return None,
            }

            // return Some(String::from_utf16(&decoded)?); // return
            // Ok(String::from_utf16(&decoded).map_err(|_|
            // ASN1Error::InvalidBMPString)?);
        }
        IA5_STRING => {
            let s = String::from_utf8_lossy(value).to_string();
            if is_ia5_string(&s) {
                return None; //return Err(ASN1Error::InvalidIA5String);
            }
            Some(s) //Ok(s)
        }
        NUMERIC_STRING => {
            for &b in value {
                if !('0' as u8 <= b && b <= '9' as u8 || b == ' ' as u8) {
                    return None; // return Err(ASN1Error::InvalidNumericString);
                }
            }
            Some(String::from_utf8_lossy(value).to_string()) // Ok(String::from_utf8_lossy(value).to_string())
        }
        _ => None, // Err(ASN1Error::UnsupportedStringType),
    }
}

// Helper functions to validate characters and UTF-8
fn is_printable(b: u8) -> bool {
    (b'b' >= b'a' && b <= b'z')
        || (b >= b'A' && b <= b'Z')
        || (b >= b'0' && b <= b'9')
        || (b >= b'\'' && b <= b')')
        || (b >= b'+' && b <= b'/')
        || b == b' '
        || b == b':'
        || b == b'='
        || b == b'?'
        || b == b'*'
        || b == b'&'
}

fn is_valid_utf8(value: &[u8]) -> bool {
    // Fast path for ASCII
    for &byte in value {
        if byte >= 0x80 {
            return false;
        }
    }
    true
}

// const MAX_ASCII: char = 'u{007F}';

fn is_ia5_string(s: &str) -> bool {
    for r in s.chars() {
        // Per RFC5280 "IA5String is limited to the set of ASCII characters"
        if r > 127 as char {
            // if r > MAX_ASCII {
            return false;
        }
    }
    return true;
}

fn is_valid_ip_mask(mask: &Vec<u8>) -> bool {
    let mut seen_zero = false;

    for &b in mask {
        if seen_zero {
            if b != 0 {
                return false;
            }
            continue;
        }

        match b {
            0x00 | 0x80 | 0xc0 | 0xe0 | 0xf0 | 0xf8 | 0xfc | 0xfe => {
                seen_zero = true;
            }
            0xff => {}
            _ => {
                return false;
            }
        }
    }

    true
}

fn domain_to_reverse_labels(domain: &str) -> Option<Vec<String>> {
    // fn domain_to_reverse_labels(domain: &str) -> (Vec<String>, bool) {
    let mut reverse_labels = Vec::new();
    let mut current_domain = domain.to_string();

    while !current_domain.is_empty() {
        if let Some(i) = current_domain.rfind('.') {
            reverse_labels.push(current_domain[i + 1..].to_string());
            current_domain.truncate(i);
        } else {
            reverse_labels.push(current_domain.clone());
            current_domain.clear();
        }
    }

    if !reverse_labels.is_empty() && reverse_labels[0].is_empty() {
        // An empty label at the end indicates an absolute value.
        return None; // return (Vec::new(), false);
    }

    for label in &reverse_labels {
        if label.is_empty() {
            // Empty labels are otherwise invalid.
            return None; // return (Vec::new(), false);
        }

        for c in label.chars() {
            if c < '\x21' || c > '\x7E' {
                // Invalid character.
                return None; //return (Vec::new(), false);
            }
        }
    }

    Some(reverse_labels) //(reverse_labels, true)
}

// ASN.1 types

#[derive(Debug, Clone)]
pub struct RawValue {
    class: i32,        // ASN.1 class (e.g. Universal, Application, Context-specific, Private)
    tag: i32,          // ASN.1 tag
    is_compound: bool, // Indicates if the RawValue is a compound type
    bytes: Vec<u8>,    // Undecoded bytes of the ASN.1 object
    full_bytes: Vec<u8>, // Complete bytes including the tag and length
}

#[derive(Debug, Clone)]
struct AlgorithmIdentifier {
    algorithm: Vec<i32>,
    parameters: Option<RawValue>,
}

pub fn parse_ai(der: &mut ASN1String) -> AlgorithmIdentifier {
    let mut algorithm: Vec<i32> = Vec::new();

    let mut parameters = RawValue {
        class: 0,
        tag: 0,
        is_compound: false,
        bytes: Vec::new(),
        full_bytes: Vec::new(),
    };

    if !der.read_asn1_object_identifier(&mut algorithm) {
        panic!("x509: malformed OID");
    }

    if der.0.is_empty() {
        return AlgorithmIdentifier { algorithm, parameters: Some(parameters) };
    }

    let mut params = ASN1String { 0: Vec::new() };
    let mut tag = 0u8;

    if !der.read_any_asn1_element(&mut params, &mut tag) {
        panic!("x509: malformed parameters");
    }

    parameters.tag = tag as i32;
    parameters.full_bytes = params.0.to_vec();

    return AlgorithmIdentifier { algorithm, parameters: Some(parameters) };
}

fn get_signature_algorithm_from_ai(ai: AlgorithmIdentifier) -> SignatureAlgorithm {
    if ai.algorithm == OID_SIGNATURE_ED25519.to_vec() {
        // RFC 8410, Section 3
        // > For all of the OIDs, the parameters MUST be absent.
        if ai.parameters.unwrap().full_bytes.len() != 0 {
            return SignatureAlgorithm::UnknownSignatureAlgorithm;
        }
    }

    let signature_algorithm_details: Vec<AlgorithmDetails> = vec![
        AlgorithmDetails {
            algo: SignatureAlgorithm::MD2WithRSA,
            name: String::from("MD2-RSA"),
            oid: OID_SIGNATURE_MD2_WITH_RSA.to_vec(),
            pub_key_algo: PublicKeyAlgorithm::RSA,
            hash: None, // no value for MD2
        },
        AlgorithmDetails {
            algo: SignatureAlgorithm::MD5WithRSA,
            name: String::from("MD5-RSA"),
            oid: OID_SIGNATURE_MD5_WITH_RSA.to_vec(),
            pub_key_algo: PublicKeyAlgorithm::RSA,
            hash: Some(String::from("MD5")),
        },
        AlgorithmDetails {
            algo: SignatureAlgorithm::SHA1WithRSA,
            name: String::from("SHA1-RSA"),
            oid: OID_SIGNATURE_SHA1_WITH_RSA.to_vec(),
            pub_key_algo: PublicKeyAlgorithm::RSA,
            hash: Some(String::from("SHA1")),
        },
        AlgorithmDetails {
            algo: SignatureAlgorithm::SHA256WithRSA,
            name: String::from("SHA256-RSA"),
            oid: OID_SIGNATURE_SHA256_WITH_RSA.to_vec(),
            pub_key_algo: PublicKeyAlgorithm::RSA,
            hash: Some(String::from("SHA256")),
        },
        AlgorithmDetails {
            algo: SignatureAlgorithm::SHA384WithRSA,
            name: String::from("SHA384-RSA"),
            oid: OID_SIGNATURE_SHA384_WITH_RSA.to_vec(),
            pub_key_algo: PublicKeyAlgorithm::RSA,
            hash: Some(String::from("SHA384")),
        },
        AlgorithmDetails {
            algo: SignatureAlgorithm::SHA512WithRSA,
            name: String::from("SHA512-RSA"),
            oid: OID_SIGNATURE_SHA512_WITH_RSA.to_vec(),
            pub_key_algo: PublicKeyAlgorithm::RSA,
            hash: Some(String::from("SHA512")),
        },
        AlgorithmDetails {
            algo: SignatureAlgorithm::SHA256WithRSAPSS,
            name: String::from("SHA256-RSAPSS"),
            oid: OID_SIGNATURE_RSA_PSS.to_vec(),
            pub_key_algo: PublicKeyAlgorithm::RSA,
            hash: Some(String::from("SHA256")),
        },
        AlgorithmDetails {
            algo: SignatureAlgorithm::SHA384WithRSAPSS,
            name: String::from("SHA384-RSAPSS"),
            oid: OID_SIGNATURE_RSA_PSS.to_vec(),
            pub_key_algo: PublicKeyAlgorithm::RSA,
            hash: Some(String::from("SHA384")),
        },
        AlgorithmDetails {
            algo: SignatureAlgorithm::SHA512WithRSAPSS,
            name: String::from("SHA512-RSAPSS"),
            oid: OID_SIGNATURE_RSA_PSS.to_vec(),
            pub_key_algo: PublicKeyAlgorithm::RSA,
            hash: Some(String::from("SHA512")),
        },
        AlgorithmDetails {
            algo: SignatureAlgorithm::DSAWithSHA1,
            name: String::from("DSA-SHA1"),
            oid: OID_SIGNATURE_DSA_WITH_SHA1.to_vec(),
            pub_key_algo: PublicKeyAlgorithm::DSA,
            hash: Some(String::from("SHA1")),
        },
        AlgorithmDetails {
            algo: SignatureAlgorithm::DSAWithSHA256,
            name: String::from("DSA-SHA256"),
            oid: OID_SIGNATURE_DSA_WITH_SHA256.to_vec(),
            pub_key_algo: PublicKeyAlgorithm::DSA,
            hash: Some(String::from("SHA256")),
        },
        AlgorithmDetails {
            algo: SignatureAlgorithm::ECDSAWithSHA1,
            name: String::from("ECDSA-SHA1"),
            oid: OID_SIGNATURE_ECDSA_WITH_SHA1.to_vec(),
            pub_key_algo: PublicKeyAlgorithm::ECDSA,
            hash: Some(String::from("SHA1")),
        },
        AlgorithmDetails {
            algo: SignatureAlgorithm::ECDSAWithSHA256,
            name: String::from("ECDSA-SHA256"),
            oid: OID_SIGNATURE_ECDSA_WITH_SHA256.to_vec(),
            pub_key_algo: PublicKeyAlgorithm::ECDSA,
            hash: Some(String::from("SHA256")),
        },
        AlgorithmDetails {
            algo: SignatureAlgorithm::ECDSAWithSHA384,
            name: String::from("ECDSA-SHA384"),
            oid: OID_SIGNATURE_ECDSA_WITH_SHA384.to_vec(),
            pub_key_algo: PublicKeyAlgorithm::ECDSA,
            hash: Some(String::from("SHA384")),
        },
        AlgorithmDetails {
            algo: SignatureAlgorithm::ECDSAWithSHA512,
            name: String::from("ECDSA-SHA512"),
            oid: OID_SIGNATURE_ECDSA_WITH_SHA512.to_vec(),
            pub_key_algo: PublicKeyAlgorithm::ECDSA,
            hash: Some(String::from("SHA512")),
        },
        AlgorithmDetails {
            algo: SignatureAlgorithm::PureEd25519,
            name: String::from("Ed25519"),
            oid: OID_SIGNATURE_ED25519.to_vec(),
            pub_key_algo: PublicKeyAlgorithm::Ed25519,
            hash: Some(String::from("")),
        },
    ];

    if ai.algorithm != OID_SIGNATURE_RSA_PSS.to_vec() {
        for details in signature_algorithm_details {
            if ai.algorithm == details.oid {
                return details.algo;
            }
        }
        return SignatureAlgorithm::UnknownSignatureAlgorithm;
    }

    // RSA PSS is special because it encodes important parameters
    // in the Parameters.
    // let params: PssParameters;
    // if unmarshal(ai.parameters.full_bytes, &params){
    // return SignatureAlgorithm::UnknownSignatureAlgorithm;
    //}

    // let mgf1_hash_func: AlgorithmIdentifier;
    // if unmarshal(params.mgf.parameters.full_bytes, &mgf1_hash_func){
    // return SignatureAlgorithm::UnknownSignatureAlgorithm;
    //}

    // PSS is greatly overburdened with options. This code forces them into
    // three buckets by requiring that the MGF1 hash function always match the
    // message hash function (as recommended in RFC 3447, Section 8.1), that the
    // salt length matches the hash length, and that the trailer field has the
    // default value.

    // if (!params.hash.parameters.unwrap().full_bytes.is_empty() &&
    // !params.hash.parameters.unwrap().full_bytes.to_vec()==NULL_BYTES.to_vec() )
    // || params.mgf.algorithm != oidMGF1 ||
    // mgf1_hash_func.algorithm != params.hash.algorithm ||
    //( mgf1_hash_func.parameters.unwrap().full_bytes.len() != 0 &&
    //( mgf1_hash_func.parameters.unwrap().full_bytes != NULL_BYTES ) ||
    // params.TrailerField != 1 {
    // return SignatureAlgorithm::UnknownSignatureAlgorithm;
    //}

    // match (params.hash.algorithm, params.salt_length) {
    //(OID_SHA256, 32) => SignatureAlgorithm::SHA256WithRSAPSS,
    //(OID_SHA384, 48) => SignatureAlgorithm::SHA384WithRSAPSS,
    //(OID_SHA512, 64) => SignatureAlgorithm::SHA512WithRSAPSS,
    //_ => SignatureAlgorithm::UnknownSignatureAlgorithm,
    //}

    return SignatureAlgorithm::UnknownSignatureAlgorithm;
}

// RFC 3279, 2.3 Public Key Algorithms
//
// 	pkcs-1 OBJECT IDENTIFIER ::== { iso(1) member-body(2) us(840)
// 		rsadsi(113549) pkcs(1) 1 }
//
// rsaEncryption OBJECT IDENTIFIER ::== { pkcs1-1 1 }
//
// 	id-dsa OBJECT IDENTIFIER ::== { iso(1) member-body(2) us(840)
// 		x9-57(10040) x9cm(4) 1 }
const OID_PUBLIC_KEY_RSA: [i32; 7] = [1, 2, 840, 113549, 1, 1, 1];
const OID_PUBLIC_KEY_DSA: [i32; 6] = [1, 2, 840, 10040, 4, 1];
// RFC 5480, 2.1.1 Unrestricted Algorithm Identifier and Parameters
//
// 	id-ecPublicKey OBJECT IDENTIFIER ::= {
// 		iso(1) member-body(2) us(840) ansi-X9-62(10045) keyType(2) 1 }
const OID_PUBLIC_KEY_ECDSA: [i32; 6] = [1, 2, 840, 10045, 2, 1];
// RFC 8410, Section 3
//
// 	id-X25519    OBJECT IDENTIFIER ::= { 1 3 101 110 }
// 	id-Ed25519   OBJECT IDENTIFIER ::= { 1 3 101 112 }
const OID_PUBLIC_KEY_X25519: [i32; 4] = [1, 3, 101, 110];
const OID_PUBLIC_KEY_ED25519: [i32; 4] = [1, 3, 101, 112];

fn get_public_key_algorithm_from_oid(oid: &Vec<i32>) -> PublicKeyAlgorithm {
    let oid_slice = oid.as_slice();
    match oid_slice {
        val if val == OID_PUBLIC_KEY_RSA.as_slice() => PublicKeyAlgorithm::RSA,
        val if val == OID_PUBLIC_KEY_DSA.as_slice() => PublicKeyAlgorithm::DSA,
        val if val == OID_PUBLIC_KEY_ECDSA.as_slice() => PublicKeyAlgorithm::ECDSA,
        val if val == OID_PUBLIC_KEY_ED25519.as_slice() => PublicKeyAlgorithm::Ed25519,
        _ => PublicKeyAlgorithm::UnknownPublicKeyAlgorithm,
    }
}
// pub fn unmarshal(b: &[u8], val: &mut dyn std::any::Any) -> Option<&[u8]> {
// unmarshal_with_params(b, val, "")
// }
//
// pub fn unmarshal_with_params(b: &[u8], val: &mut dyn std::any::Any, params:
// &str) -> Option<&[u8]> { let v = val.downcast_mut::<&mut dyn std::any::Any>()
// .ok_or(InvalidUnmarshalError { typ: std::any::TypeId::of::<&mut dyn
// std::any::Any>() })?;
//
// let (offset, rest) = parse_field(val, b, 0, parse_field_parameters(params))?;
// Some(&b[offset..])
// }
//
// fn parse_field(v: &mut dyn std::any::Any, bytes: &[u8], init_offset: usize,
// params: FieldParameters) -> Option<(usize, &[u8])> { let mut offset =
// init_offset;
//
// if offset == bytes.len() {
// if !set_default_value(v, &params) {
// return Err(SyntaxError { message: "sequence truncated" });
// }
// return Some((offset, bytes));//return Ok((offset, bytes));
// }
//
// Handle the ANY type.
// if let Some(iface) = v.downcast_ref::<&dyn std::any::Any>() {
// let (t, new_offset, err) = parse_tag_and_length(bytes, offset)?;
// if err.is_some() {
// return None; // return Err(err.unwrap());
// }
//
// Check length and parse according to tag
// if is_invalid_length(new_offset, t.length, bytes.len()) {
// return None; //return Err(SyntaxError { message: "data truncated" });
// }
//
// let inner_bytes = &bytes[new_offset..new_offset + t.length];
// let result: Box<dyn std::any::Any> = match t.tag {
// Tag::PrintableString => parse_printable_string(inner_bytes)?,
// Tag::NumericString => parse_numeric_string(inner_bytes)?,
// Tag::IA5String => parse_ia5_string(inner_bytes)?,
// Tag::T61String => parse_t61_string(inner_bytes)?,
// Tag::UTF8String => parse_utf8_string(inner_bytes)?,
// Tag::Integer => parse_int64(inner_bytes)?,
// Tag::BitString => parse_bit_string(inner_bytes)?,
// Tag::OID => parse_object_identifier(inner_bytes)?,
// Tag::UTCTime => parse_utc_time(inner_bytes)?,
// Tag::GeneralizedTime => parse_generalized_time(inner_bytes)?,
// Tag::OctetString => inner_bytes.to_vec().into_boxed_slice(),
// Tag::BMPString => parse_bmp_string(inner_bytes)?,
// _ => Box::new(()), // Unknown type handling
// };
//
// Update the reference
// mem::swap(v, &mut result);
// offset += t.length;
// return Some((offset, bytes));//Ok((offset, bytes));
// }
//
// Normal case; parse according to the ASN.1 rules
// let (t, new_offset, err) = parse_tag_and_length(bytes, offset)?;
// if err.is_some() {
// return None;//return Err(err.unwrap());
// }
//
// Some((offset, bytes))
// }

// fn parse_validity(der: &mut ASN1String) -> Option<(u64, u64)> {
fn parse_validity(der: &mut ASN1String) -> Option<(DateTime<Utc>, DateTime<Utc>)> {
    let not_before = parse_time(der);
    if not_before.is_none() {
        return None;
    }
    let not_after = parse_time(der);
    if not_after.is_none() {
        return None;
    }

    return Some((not_before.unwrap(), not_after.unwrap()));
}

fn parse_time(der: &mut ASN1String) -> Option<DateTime<Utc>> {
    if der.peek_asn1_tag(TAG_UTC_TIME) {
        der.read_asn1_utc_time()
    } else if der.peek_asn1_tag(TAG_GENERALIZED_TIME) {
        der.read_asn1_generalized_time()
    } else {
        None //Err("Unsupported time format".to_string())
    }
}

fn parse_extension(der: &mut ASN1String) -> Extension {
    // fn parse_extension(der: &mut ASN1String) -> (pkix.Extension, error) {
    let mut ext: Extension = Extension { id: vec![], critical: false, value: vec![] };
    if !der.read_asn1_object_identifier(&mut ext.id) {
        panic!("x509: malformed extension OID field");
    }
    let mut ext_critical = false;
    if der.peek_asn1_tag(BOOLEAN) {
        if !der.read_asn1_boolean(&mut ext_critical) {
            panic!("x509: malformed extension critical field");
        }
    }
    ext.critical = ext_critical;
    let mut val = ASN1String { 0: Vec::new() };
    if !der.read_asn1(&mut val, OCTET_STRING) {
        panic!("x509: malformed extension value field");
    }
    ext.value = val.0;
    return ext;
}

struct PublicKeyInfo {
    raw: Vec<u8>,
    algorithm: AlgorithmIdentifier,
    public_key: BitString,
}

#[derive(Debug, PartialEq)]
enum PublicKey {
    RsaPublicKey(rsa::PublicKey),
    ECDSAPublicKey(ecdsa::PublicKey),
    ED25519PublicKey(ed25519::PublicKey),
    X25519PublicKey,
    DsaPublicKey,
    UnknownPubicKey,
}

// RFC 5480, 2.1.1.1. Named Curve
//
// 	secp224r1 OBJECT IDENTIFIER ::= {
// 	  iso(1) identified-organization(3) certicom(132) curve(0) 33 }
//
// 	secp256r1 OBJECT IDENTIFIER ::= {
// 	  iso(1) member-body(2) us(840) ansi-X9-62(10045) curves(3)
// 	  prime(1) 7 }
//
// 	secp384r1 OBJECT IDENTIFIER ::= {
// 	  iso(1) identified-organization(3) certicom(132) curve(0) 34 }
//
// 	secp521r1 OBJECT IDENTIFIER ::= {
// 	  iso(1) identified-organization(3) certicom(132) curve(0) 35 }
//
// NB: secp256r1 is equivalent to prime256v1
const OID_NAMED_CURVE_P224: [i32; 5] = [1, 3, 132, 0, 33];
const OID_NAMED_CURVE_P256: [i32; 7] = [1, 2, 840, 10045, 3, 1, 7];
const OID_NAMED_CURVE_P384: [i32; 5] = [1, 3, 132, 0, 34];
const OID_NAMED_CURVE_P521: [i32; 5] = [1, 3, 132, 0, 35];

fn named_curve_from_oid(oid: &Vec<i32>) -> Option<ecdsa::Curve> {
    match oid.as_slice() {
        val if val == OID_NAMED_CURVE_P224.as_slice() => Some(ecdsa::p224()),
        val if val == OID_NAMED_CURVE_P256.as_slice() => Some(ecdsa::p256()),
        val if val == OID_NAMED_CURVE_P384.as_slice() => Some(ecdsa::p384()),
        val if val == OID_NAMED_CURVE_P521.as_slice() => Some(ecdsa::p521()),
        _ => None,
    }
    // switch {
    // case oid.Equal(oidNamedCurveP224):
    // return elliptic.P224()
    // case oid.Equal(oidNamedCurveP256):
    // return elliptic.P256()
    // case oid.Equal(oidNamedCurveP384):
    // return elliptic.P384()
    // case oid.Equal(oidNamedCurveP521):
    // return elliptic.P521()
    //}
    // return nil
}

fn parse_public_key(key_data: &PublicKeyInfo) -> PublicKey {
    let oid = &key_data.algorithm.algorithm;
    let params = key_data.algorithm.parameters.clone().unwrap().clone();
    let mut der = ASN1String { 0: key_data.public_key.right_align() }; // der = cryptobyte.String(key_data.publicKey.right_align() );
    match oid.as_slice() {
        val if val == OID_PUBLIC_KEY_RSA.as_slice() => {
            // RSA public keys must have a NULL in the parameters.
            // See RFC 3279, Section 2.3.1.
            if params.full_bytes != NULL_BYTES.to_vec() {
                panic!("x509: RSA key missing NULL parameters");
            }

            let mut p = rsa::PublicKey { n: BigInt::from(0), e: 0i64 };
            let mut der1 = ASN1String { 0: Vec::new() };
            if !der.read_asn1(&mut der1, SEQUENCE) {
                panic!("x509: invalid RSA public key");
            }

            match der1.read_asn1_big_int() {
                Some(big_int) => p.n = big_int,
                None => panic!("x509: invalid RSA modulus"),
            }

            if !der1.read_asn1_i64(&mut p.e) {
                panic!("x509: invalid RSA public exponent");
            }

            if p.n.sign() == Sign::Minus {
                panic!("x509: RSA modulus is not a positive number");
            }
            if p.e <= 0i64 {
                panic!("x509: RSA public exponent is not a positive number");
            }

            return PublicKey::RsaPublicKey(p);
        }
        val if val == OID_PUBLIC_KEY_ECDSA.as_slice() => {
            let mut params_der = ASN1String { 0: params.full_bytes.clone() }; // cryptobyte.String(params.FullBytes)
            let mut named_curve_oid: Vec<i32> = Vec::new();
            if !params_der.read_asn1_object_identifier(&mut named_curve_oid) {
                panic!("x509: invalid ECDSA parameters");
            }
            // let named_curve = named_curve_from_oid(&named_curve_oid);
            match named_curve_from_oid(&named_curve_oid) {
                Some(named_curve) => {
                    //
                    let (x, y): (BigInt, BigInt) = ecdsa::unmarshal(&named_curve, &der.0);
                    // if x == nil {
                    // panic!("x509: failed to unmarshal elliptic curve point");
                    //}
                    PublicKey::ECDSAPublicKey(ecdsa::PublicKey { curve: named_curve, x, y })
                }
                None => panic!("x509: unsupported elliptic curve"),
            }
        }
        val if val == OID_PUBLIC_KEY_ED25519.as_slice() => {
            // RFC 8410, Section 3
            // > For all of the OIDs, the parameters MUST be absent.
            if !params.full_bytes.is_empty() {
                panic!("x509: Ed25519 key encoded with illegal parameters");
            }
            if der.0.len() != ed25519::PUBLIC_KEY_SIZE {
                panic!("x509: wrong Ed25519 public key size");
            }

            PublicKey::ED25519PublicKey(ed25519::PublicKey(der.0))
        }
        val if val == OID_PUBLIC_KEY_X25519.as_slice() => PublicKey::X25519PublicKey,
        val if val == OID_PUBLIC_KEY_DSA.as_slice() => PublicKey::DsaPublicKey,
        _ => panic!("x509: unknown public key algorithm"),
    }
}

pub fn parse_key_usage_extension(der: &Vec<u8>) -> Option<KeyUsage> {
    let mut usage_bits = BitString { bytes: vec![], bit_length: 0 };
    let mut asn_der = ASN1String { 0: der.clone() };
    if !asn_der.read_asn1_bitstring(&mut usage_bits) {
        return None; // "x509: invalid key usage"
    }

    let mut usage: i32 = 0;
    for i in 0..9 {
        if usage_bits.at(i) != 0u8 {
            usage |= 1 << i;
        }
    }
    Some(KeyUsage(usage))
}

pub fn parse_basic_constraints_extension(der_bytes: &Vec<u8>) -> Option<(bool, i32)> {
    // pub fn parse_basic_constraints_extension(der: &Vec<u8>) -> Result<(bool,
    // i32), Box<dyn Error>> {
    let mut is_ca = false;
    let mut der_ = ASN1String { 0: der_bytes.clone() };
    let mut der = ASN1String { 0: Vec::new() };
    if !der_.read_asn1(&mut der, SEQUENCE) {
        return None; // "invalid basic constraints"
    }

    if der.peek_asn1_tag(BOOLEAN) {
        if !der.read_asn1_boolean(&mut is_ca) {
            return None; // "invalid basic constraints"
        }
    }

    let mut max_path_len: i64 = -1;

    if der.peek_asn1_tag(INTEGER) {
        if !der.read_asn1_i64(&mut max_path_len) {
            return None; // "invalid basic constraints"
        }
    }

    // TODO: map max_path_len to 0 if it has the -1 default value? (Issue 19285)
    Some((is_ca, max_path_len as i32))
}

pub fn for_each_san<F>(der_bytes: &Vec<u8>, mut callback: F) -> bool
// pub fn for_each_san<F>(der: &[u8], callback: F) -> Result<(), Box<dyn Error>>
where
    F: FnMut(u8, &ASN1String) -> bool, // F: Fn(u8, &[u8]) -> Result<(), Box<dyn Error>>,
{
    let mut der_ = ASN1String { 0: der_bytes.clone() };
    let mut der = ASN1String { 0: Vec::new() };
    if !der_.read_asn1(&mut der, SEQUENCE) {
        return false; // "invalid subject alternative names"
    }
    while !der.0.is_empty() {
        let mut san = ASN1String { 0: Vec::new() };
        let mut tag = 0u8;

        if !der.read_any_asn1(&mut san, &mut tag) {
            return false; // "invalid subject alternative names"
        }
        if let success = callback(0x80, &san) {
            // if let Err(err) = callback(tag ^ 0x80, &san) {
            return success; // return Err(err);
        }
    }
    return true;
}

// pub fn parse_san_extension(der: &[u8]) -> Result<(Vec<String>, Vec<String>,
// Vec<IpAddr>, Vec<Url>), Box<dyn Error>> {
pub fn parse_san_extension(
    der: &Vec<u8>,
) -> Option<(Vec<String>, Vec<String>, Vec<IpAddr>, Vec<String>)> {
    let mut dns_names = Vec::new();
    let mut email_addresses = Vec::new();
    let mut ip_addresses = Vec::new();
    let mut uris = Vec::new();

    const NAME_TYPE_EMAIL: u8 = 1;
    const NAME_TYPE_DNS: u8 = 2;
    const NAME_TYPE_URI: u8 = 6;
    const NAME_TYPE_IP: u8 = 7;

    if for_each_san(&der, |tag, data| {
        match tag {
            NAME_TYPE_EMAIL => {
                let email = String::from_utf8_lossy(&data.0).to_string();
                if !is_ia5_string(&email) {
                    return false; // return Err("SAN rfc822Name is malformed".into());
                }
                email_addresses.push(email);
            }
            NAME_TYPE_DNS => {
                let name = String::from_utf8_lossy(&data.0).to_string();
                if !is_ia5_string(&name) {
                    return false; // return Err("SAN rfc822Name is malformed".into());
                }
                dns_names.push(name);
            }
            NAME_TYPE_URI => {
                let uri_str = String::from_utf8_lossy(&data.0).to_string();
                if !is_ia5_string(&uri_str) {
                    return false; // return Err("SAN uniformResourceIdentifier is malformed".into());
                }
                // let uri = Url::parse(&uri_str).map_err(|err| {
                // format!("cannot parse URI {}: {}", uri_str, err)
                //})?;
                // if !uri.host_str().map_or(false, |host|
                // domain_to_reverse_labels(host).is_ok()) { return false;
                // //return Err(format!("cannot parse URI {}: invalid domain", uri_str).into());
                //}
                // uris.push(uri);
                uris.push(uri_str);
            }
            NAME_TYPE_IP => {
                let ip = match data.0.len() {
                    4 => {
                        let ip_bytes: [u8; 4] = data.0.clone().try_into().unwrap();
                        IpAddr::from(ip_bytes)
                    }
                    16 => {
                        let ip_bytes: [u8; 16] = data.0.clone().try_into().unwrap();
                        IpAddr::from(ip_bytes)
                    }
                    _ => return false, /* return Err(format!("cannot parse IP address of length
                                        * {}", data.len()).into()), */
                };
                ip_addresses.push(ip);
            }
            _ => {}
        }
        return true;
    }) == false
    {
        return None;
    };

    Some((dns_names, email_addresses, ip_addresses, uris))
}

// fn parse_ext_key_usage_extension(der: &Vec<u8>) -> ([]ExtKeyUsage,
// []asn1.ObjectIdentifier, error) {
fn parse_ext_key_usage_extension(der_bytes: &Vec<u8>) -> Option<(Vec<ExtKeyUsage>, Vec<Vec<i32>>)> {
    let mut ext_key_usages: Vec<ExtKeyUsage> = Vec::new(); //var extKeyUsages []ExtKeyUsage
    let mut unknown_usages: Vec<Vec<i32>> = Vec::new(); // var unknownUsages []asn1.ObjectIdentifier

    let mut der_ = ASN1String { 0: der_bytes.clone() };
    let mut der = ASN1String { 0: Vec::new() };
    if !der_.read_asn1(&mut der, SEQUENCE) {
        return None; // "x509: invalid extended key usages"
    }

    while !der.0.is_empty() {
        let mut eku: Vec<i32> = Vec::new();
        if !der.read_asn1_object_identifier(&mut eku) {
            return None; // "x509: invalid extended key usages"
        }

        let ext_key_usage_result = ext_key_usage_from_oid(&eku[..]);
        if ext_key_usage_result.is_none() {
            unknown_usages.push(eku);
        } else {
            ext_key_usages.push(ext_key_usage_result.unwrap());
        }
    }
    return Some((ext_key_usages, unknown_usages));
}

// func parseCertificatePoliciesExtension(der cryptobyte.String) ([]OID, error)
// {
fn parse_certificate_policies_extension(der_bytes: &Vec<u8>) -> Option<Vec<Vec<u8>>> {
    let mut oids: Vec<Vec<u8>> = Vec::new(); // var oids []OID
    let mut der_ = ASN1String { 0: der_bytes.clone() };
    let mut der = ASN1String { 0: Vec::new() };
    if !der_.read_asn1(&mut der, SEQUENCE) {
        return None; // "x509: invalid certificate policies"
    }

    while !der.0.is_empty() {
        let mut cp = ASN1String { 0: Vec::new() };
        let mut oid_bytes = ASN1String { 0: Vec::new() };
        if !der.read_asn1(&mut cp, SEQUENCE) || !cp.read_asn1(&mut oid_bytes, OBJECT_IDENTIFIER) {
            return None; // "x509: invalid certificate policies"
        }
        let oid_wrapper = OID::new_oid_from_der(&oid_bytes.0); // oid, ok := newOIDFromDER(OIDBytes)
        if oid_wrapper.is_none() {
            return None; // "x509: invalid certificate policies"
        }
        oids.push(oid_wrapper.unwrap().der);
        // oids = append(oids, oid)
    }
    return Some(oids);
}

pub struct OID {
    der: Vec<u8>,
}

impl OID {
    pub fn new_oid_from_der(der: &[u8]) -> Option<Self> {
        if der.is_empty() || der[der.len() - 1] & 0x80 != 0 {
            return None;
        }

        let mut start = 0;
        for (i, &v) in der.iter().enumerate() {
            // ITU-T X.690, section 8.19.2:
            // The subidentifier shall be encoded in the fewest possible octets,
            // that is, the leading octet of the subidentifier shall not have the value
            // 0x80.
            if i == start && v == 0x80 {
                return None;
            }
            if v & 0x80 == 0 {
                start = i + 1;
            }
        }

        Some(OID { der: der.to_vec() })
    }
}

// #[derive(Debug, Clone, Copy)]
// pub enum KeyUsage {
// DigitalSignature = 1 << 0,
// ContentCommitment = 1 << 1,
// KeyEncipherment = 1 << 2,
// DataEncipherment = 1 << 3,
// KeyAgreement = 1 << 4,
// CertSign = 1 << 5,
// CRLSign = 1 << 6,
// EncipherOnly = 1 << 7,
// DecipherOnly = 1 << 8,
// }

pub const OID_EXT_KEY_USAGE_ANY: [i32; 5] = [2, 5, 29, 37, 0];
pub const OID_EXT_KEY_USAGE_SERVER_AUTH: [i32; 9] = [1, 3, 6, 1, 5, 5, 7, 3, 1];
pub const OID_EXT_KEY_USAGE_CLIENT_AUTH: [i32; 9] = [1, 3, 6, 1, 5, 5, 7, 3, 2];
pub const OID_EXT_KEY_USAGE_CODE_SIGNING: [i32; 9] = [1, 3, 6, 1, 5, 5, 7, 3, 3];
pub const OID_EXT_KEY_USAGE_EMAIL_PROTECTION: [i32; 9] = [1, 3, 6, 1, 5, 5, 7, 3, 4];
pub const OID_EXT_KEY_USAGE_IPSEC_END_SYSTEM: [i32; 9] = [1, 3, 6, 1, 5, 5, 7, 3, 5];
pub const OID_EXT_KEY_USAGE_IPSEC_TUNNEL: [i32; 9] = [1, 3, 6, 1, 5, 5, 7, 3, 6];
pub const OID_EXT_KEY_USAGE_IPSEC_USER: [i32; 9] = [1, 3, 6, 1, 5, 5, 7, 3, 7];
pub const OID_EXT_KEY_USAGE_TIME_STAMPING: [i32; 9] = [1, 3, 6, 1, 5, 5, 7, 3, 8];
pub const OID_EXT_KEY_USAGE_OCSP_SIGNING: [i32; 9] = [1, 3, 6, 1, 5, 5, 7, 3, 9];
pub const OID_EXT_KEY_USAGE_MICROSOFT_SERVER_GATED_CRYPTO: [i32; 10] =
    [1, 3, 6, 1, 4, 1, 311, 10, 3, 3];
pub const OID_EXT_KEY_USAGE_NETSCAPE_SERVER_GATED_CRYPTO: [i32; 7] = [2, 16, 840, 1, 113730, 4, 1];
pub const OID_EXT_KEY_USAGE_MICROSOFT_COMMERCIAL_CODE_SIGNING: [i32; 10] =
    [1, 3, 6, 1, 4, 1, 311, 2, 1, 22];
pub const OID_EXT_KEY_USAGE_MICROSOFT_KERNEL_CODE_SIGNING: [i32; 10] =
    [1, 3, 6, 1, 4, 1, 311, 61, 1, 1];

#[derive(Debug)]
pub struct ExtKeyUsageOID {
    pub ext_key_usage: ExtKeyUsage,
    pub oid: &'static [i32],
}

pub const EXT_KEY_USAGE_OIDS: [ExtKeyUsageOID; 14] = [
    ExtKeyUsageOID { ext_key_usage: ExtKeyUsage::ANY, oid: &OID_EXT_KEY_USAGE_ANY },
    ExtKeyUsageOID { ext_key_usage: ExtKeyUsage::SERVER_AUTH, oid: &OID_EXT_KEY_USAGE_SERVER_AUTH },
    ExtKeyUsageOID { ext_key_usage: ExtKeyUsage::CLIENT_AUTH, oid: &OID_EXT_KEY_USAGE_CLIENT_AUTH },
    ExtKeyUsageOID {
        ext_key_usage: ExtKeyUsage::CODE_SIGNING,
        oid: &OID_EXT_KEY_USAGE_CODE_SIGNING,
    },
    ExtKeyUsageOID {
        ext_key_usage: ExtKeyUsage::EMAIL_PROTECTION,
        oid: &OID_EXT_KEY_USAGE_EMAIL_PROTECTION,
    },
    ExtKeyUsageOID {
        ext_key_usage: ExtKeyUsage::IPSEC_END_SYSTEM,
        oid: &OID_EXT_KEY_USAGE_IPSEC_END_SYSTEM,
    },
    ExtKeyUsageOID {
        ext_key_usage: ExtKeyUsage::IPSEC_TUNNEL,
        oid: &OID_EXT_KEY_USAGE_IPSEC_TUNNEL,
    },
    ExtKeyUsageOID { ext_key_usage: ExtKeyUsage::IPSEC_USER, oid: &OID_EXT_KEY_USAGE_IPSEC_USER },
    ExtKeyUsageOID {
        ext_key_usage: ExtKeyUsage::TIME_STAMPING,
        oid: &OID_EXT_KEY_USAGE_TIME_STAMPING,
    },
    ExtKeyUsageOID {
        ext_key_usage: ExtKeyUsage::OCSP_SIGNING,
        oid: &OID_EXT_KEY_USAGE_OCSP_SIGNING,
    },
    ExtKeyUsageOID {
        ext_key_usage: ExtKeyUsage::MICROSOFT_SERVER_GATED_CRYPTO,
        oid: &OID_EXT_KEY_USAGE_MICROSOFT_SERVER_GATED_CRYPTO,
    },
    ExtKeyUsageOID {
        ext_key_usage: ExtKeyUsage::NETSCAPE_SERVER_GATED_CRYPTO,
        oid: &OID_EXT_KEY_USAGE_NETSCAPE_SERVER_GATED_CRYPTO,
    },
    ExtKeyUsageOID {
        ext_key_usage: ExtKeyUsage::MICROSOFT_COMMERCIAL_CODE_SIGNING,
        oid: &OID_EXT_KEY_USAGE_MICROSOFT_COMMERCIAL_CODE_SIGNING,
    },
    ExtKeyUsageOID {
        ext_key_usage: ExtKeyUsage::MICROSOFT_KERNEL_CODE_SIGNING,
        oid: &OID_EXT_KEY_USAGE_MICROSOFT_KERNEL_CODE_SIGNING,
    },
];

// fn ext_key_usage_from_oid(oid asn1.ObjectIdentifier) (eku ExtKeyUsage, ok
// bool) {
pub fn ext_key_usage_from_oid(oid: &[i32]) -> Option<ExtKeyUsage> {
    for pair in EXT_KEY_USAGE_OIDS.iter() {
        if pair.oid == oid {
            return Some(pair.ext_key_usage);
        }
    }
    None
}

pub fn check_certs(
    current_time: i64,
    issuer: &[u8],
    check_sum: &[u8],
    certs_chain: &[u8],
    signature: &[u8],
    spare_root_cert: &[u8],
) -> Result<PublicKey, Vec<u8> > { // -> Option<PublicKey> {
    // extract
    // divide input string into three slices

    let len_of_certs_chain = (certs_chain[0] as usize) * 65536
        + (certs_chain[1] as usize) * 256
        + (certs_chain[2] as usize);

    if len_of_certs_chain + 1 != certs_chain.len() {
        return Err(vec![0u8, 3u8, 75u8]); // "certs chain len does not match" // return None;
    }

    let len_of_leaf_cert = (certs_chain[3] as usize) * 65536
        + (certs_chain[4] as usize) * 256
        + (certs_chain[5] as usize);

    let leaf_cert_slice = &certs_chain[6..len_of_leaf_cert + 6];

    let mut leaf_cert = parse_certificate(leaf_cert_slice); // leafCert, err := x509.ParseCertificate(leafCertSlice)

    if leaf_cert.not_after.timestamp() < current_time
        || leaf_cert.not_before.timestamp() > current_time
    {
        return Err(vec![0u8, 3u8, 76u8]); // "leaf cert has expired"  // return None;
    }

    let start_index = len_of_leaf_cert + 8;
    let len_of_internal_cert = (certs_chain[start_index] as usize) * 65536
        + (certs_chain[start_index + 1] as usize) * 256
        + (certs_chain[start_index + 2] as usize);

    let internal_cert_slice = &certs_chain[start_index + 3..start_index + len_of_internal_cert + 3];

    let mut internal_cert = parse_certificate(internal_cert_slice); // internalCert, err := x509.ParseCertificate(internalCertSlice)

    if internal_cert.not_after.timestamp() < current_time
        || internal_cert.not_before.timestamp() > current_time
    {
        return Err(vec![0u8, 3u8, 77u8]); // "internal cert has expired" // return None;
    }

    let start_index = start_index + 3 + len_of_internal_cert + 2;

    let root_cert = if start_index + 2 < certs_chain.len() {
        let len_of_root_cert = (certs_chain[start_index] as usize) * 65536
            + (certs_chain[start_index + 1] as usize) * 256
            + (certs_chain[start_index + 2] as usize);
        let root_cert_slice = &certs_chain[start_index + 3..start_index + len_of_root_cert + 3];
        // let root_cert = parse_certificate(root_cert_slice);
        parse_certificate(root_cert_slice)
    } else {
        parse_certificate(&spare_root_cert) //parse_certificate(&ROOT_FACEBOOK_CERT)
    };

    if root_cert.not_after.timestamp() < current_time
        || root_cert.not_before.timestamp() > current_time
    {
        return Err(vec![0u8, 3u8, 80u8]); // "root cert has expired" // return None;
    }

    // let context: [u8; 98] = [32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32,
    // 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32,
    // 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32,
    // 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 84, 76,
    // 83, 32, 49, 46, 51, 44, 32, 115, 101, 114, 118, 101, 114, 32, 67, 101, 114,
    // 116, 105, 102, 105, 99, 97, 116, 101, 86, 101, 114, 105, 102, 121, 0];

    // let check_sum_extend = format::concatenate( &[ &context, &check_sum]);
    // let check_prepared = hkdf_sha256::sum256(&check_sum_extend).to_vec();

    // let check_prepared: [u8; 32] = [147, 105, 160, 151, 89, 237, 59, 225, 198,
    // 46, 54, 194, 177, 254, 192, 149, 23, 103, 164, 206, 97, 92, 159, 53, 75, 225,
    // 227, 148, 11, 232, 183, 107]; let sig: [u8; 256] =  [12, 8, 221, 42, 77,
    // 195, 200, 100, 161, 125, 104, 108, 13, 7, 154, 72, 251, 91, 96, 30, 221, 24,
    // 214, 117, 114, 47, 207, 151, 211, 81, 234, 75, 51, 247, 238, 9, 156, 164, 72,
    // 201, 33, 48, 85, 216, 212, 106, 160, 244, 136, 144, 43, 215, 98, 247, 190,
    // 125, 104, 205, 211, 204, 223, 200, 142, 145, 13, 129, 131, 123, 63, 170, 130,
    // 147, 238, 25, 143, 102, 81, 253, 114, 37, 42, 200, 208, 181, 156, 128,
    // 202, 82, 254, 58, 112, 7, 110, 255, 207, 111, 10, 221, 219, 33, 127, 153,
    // 107, 107, 171, 50, 119, 8, 76, 20, 78, 233, 60, 93, 136, 185, 107, 2,
    // 209, 105, 167, 206, 99, 242, 189, 51, 35, 203, 118, 129, 7, 214, 243, 240,
    // 87, 205, 13, 42, 43, 158, 133, 255, 62, 160, 83, 175, 55, 236, 160, 201, 19,
    // 121, 162, 238, 58, 202, 33, 192, 109, 64, 161, 24, 242, 150, 209, 15, 63, 13,
    // 68, 31, 123, 183, 220, 230, 83, 40, 110, 5, 186, 255, 203, 175, 139, 102, 24,
    // 17, 83, 117, 44, 246, 24, 82, 58, 131, 217, 136, 177, 140, 164, 234, 25,
    // 143, 182, 243, 41, 220, 83, 144, 225, 190, 120, 82, 102, 133, 95, 81, 122,
    // 246, 161, 162, 128, 189, 0, 195, 247, 2, 240, 210, 69, 17, 217, 92, 217, 12,
    // 86, 15, 46, 150, 233, 97, 35, 201, 235, 108, 194, 134, 245, 202, 164, 37,
    // 79, 5, 163, 96, 180, 148];

    if !leaf_cert.check_signature_from(&internal_cert) {
        return Err(vec![0u8, 3u8, 81u8]); // panic!("leaf_cert.check_signature_from(&internal_cert)"); //return None;
    }

    if !internal_cert.check_signature_from(&root_cert) {
        // return None;
        return Err(vec![0u8, 3u8, 82u8]); // panic!("internal_cert.check_signature_from(&root_cert)");
    }

    if leaf_cert.issuer.organization[0] != "DigiCert Inc" && leaf_cert.issuer.organization[0] != "Google Trust Services" {
        return Err(vec![0u8, 3u8, 83u8]); // "untrusted leaf cert issuer"
    }

    match leaf_cert.subject.common_name.as_str() {
        "upload.video.google.com" => {
            if internal_cert.subject.common_name!="WR2" && internal_cert.subject.common_name!="WE2" {
                return Err(vec![0u8, 3u8, 84u8]); // "untrusted internal cert common_name"
            }

            if issuer!=vec![6, 103, 111, 111, 103, 108, 101] {
                return Err(vec![0u8, 3u8, 85u8]); // "incorrect leaf_cert.subject.common_name"
            }
        },
        "*.kakao.com" => {
            if internal_cert.subject.common_name!="Thawte TLS RSA CA G1" {
                return Err(vec![0u8, 3u8, 84u8]); // "untrusted internal cert common_name"
            }
            if issuer!=vec![5, 107, 97, 107, 97, 111] {
                return Err(vec![0u8, 3u8, 85u8]); // "incorrect leaf_cert.subject.common_name"
            }
        },
        "*.facebook.com" => {
            if internal_cert.subject.common_name!="DigiCert Global G2 TLS RSA SHA256 2020 CA1" {
                return Err(vec![0u8, 3u8, 84u8]); // "untrusted internal cert common_name"
            }
            if issuer!=vec![8, 102, 97, 99, 101, 98, 111, 111, 107] {
                return Err(vec![0u8, 3u8, 85u8]); // "incorrect leaf_cert.subject.common_name"
            }
        },
        _ => return Err(vec![0u8, 3u8, 85u8]) // "incorrect leaf_cert.subject.common_name"
    }

    match leaf_cert.public_key_algorithm.to_string() {
        val if val == "RSA".to_string() => {
            // pubkey, ok := leafCert.PublicKey.(*rsa.PublicKey)
            if let PublicKey::RsaPublicKey(pub_key) = leaf_cert.public_key {
                //
                let pss_options =
                    rsa::PSSOptions { salt_length: rsa::PSS_SALT_LENGTH_EQUALS_HASH, hash: 0 };
                if !rsa::verify_pss(&pub_key, 256, check_sum, signature, &pss_options) {
                    // if !rsa::verify_pss(&pub_key, 256, &check_prepared, &sig, &pss_options) {
                    // return None;
                    return Err(vec![0u8, 3u8, 86u8]); // panic!("verify pss panic");
                }

                // if !rsa::verify_pkcs1v15(&pub_key,256, &check_prepared,
                // signature) {

                // panic!("verify pkcs panic"); //return None;
                //}
            } else {
                // return None; //ErrCertificateTypeMismatch
                return Err(vec![0u8, 3u8, 87u8]);  // panic!("certificate type mismatch panic");
            }
        }
        val if val == "ECDSA".to_string() => {
            //
            if let PublicKey::ECDSAPublicKey(pub_key) = leaf_cert.public_key {
                //
                let len_of_r = signature[3] as usize; //     lenOfr := signature[3]
                let r_data = &signature[4..4 + len_of_r]; //     rData := signature[4:4+lenOfr]
                let len_of_s = signature[4 + len_of_r + 1] as usize;
                let s_data = &signature[4 + len_of_r + 2..4 + len_of_r + 2 + len_of_s];
                let r = BigInt::from_bytes_be(Sign::Plus, r_data); //     r := new(big.Int).SetBytes(rData)
                let s = BigInt::from_bytes_be(Sign::Plus, s_data); //     s := new(big.Int).SetBytes(sData)
                if !ecdsa::verify(&pub_key, check_sum, &r, &s) {
                    // return None;
                    return Err(vec![0u8, 3u8, 88u8]); // panic!("ecds verify panic");
                }
            } else {
                // return None;  //ErrCertificateTypeMismatch
                return Err(vec![0u8, 3u8, 89u8]); // panic!("certificate type mismatch panic");
            }
        }
        _ => return Err(vec![0u8, 3u8, 90u8]) // panic!("Unknown signature algorithm"),
    }

    return Ok(root_cert.public_key); // Some(root_cert.public_key);
}

pub fn check_certs_with_fixed_root(
    current_time: i64,
    issuer: &[u8],
    check_sum: &[u8],
    certs_chain: &[u8],
    signature: &[u8],
    root_cert_bytes: &[u8],
) -> Result<(), Vec<u8> > { // -> bool {

    let check_certs_result =
        check_certs(current_time, issuer, check_sum, certs_chain, signature, root_cert_bytes);
    //if check_certs_result.is_none() {
        //return false;
    //}
    match check_certs_result {
        Ok(root_public_key) => {
            let proposed_root_cert = parse_certificate(&root_cert_bytes);
            if proposed_root_cert.public_key == root_public_key {
                return Ok(()); // return true;
            }
            return Err(vec![0u8, 3u8, 190u8]);// "root public key does not match with key from proposed root cert" // return false;
        },
        Err(e) => return Err(e)
    }
}
