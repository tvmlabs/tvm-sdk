mod rsa;
mod ecdsa;
mod ed25519;

///mod hkdf_sha256;

use crate::tls_session::hkdf_sha256;

//use core::slice::SlicePattern;
use std::net;
//use std::ops::{AddAssign, Deref};
//use std::time::SystemTime;
use chrono::{DateTime, Utc, TimeZone, ParseError};
use std::ops::Neg;

use num_bigint::{BigInt, ToBigInt, Sign};
//use num_traits::One;
//use std::iter::FromIterator;
//use std::str::FromStr;

use std::collections::{hash_map, HashMap};
use crate::tls_session::certs::ecdsa::Curve;
//use crate::tls_session::certs::PublicKey::{ECDSA_PUBLIC_KEY, ED25519_PUBLIC_KEY, X25519_PUBLIC_KEY};

// ASN.1 objects have metadata preceding them:
//   the tag: the type of the object
//   a flag denoting if this object is compound or not
//   the class type: the namespace of the tag
//   the length of the object, in bytes

// Here are some standard tags and classes

// ASN.1 tags represent the type of the following object.
const TagBoolean: u8         = 1;
const TagInteger: u8         = 2;
const TagBitString: u8       = 3;
const TagOctetString: u8     = 4;
const TagNull: u8            = 5;
const TagOID: u8             = 6;
const TagEnum: u8            = 10;
const TagUTF8String: u8      = 12;
const TagSequence: u8        = 16;
const TagSet: u8             = 17;
const TagNumericString: u8   = 18;
const TagPrintableString: u8 = 19;
const TagT61String: u8       = 20;
const TagIA5String: u8       = 22;
const TagUTCTime: u8         = 23;
const TagGeneralizedTime: u8 = 24;
const TagGeneralString: u8   = 27;
const TagBMPString: u8       = 30;

// ASN.1 class types represent the namespace of the tag.
const ClassUniversal: u16       = 0;
const ClassApplication: u16     = 1;
const ClassContextSpecific: u16 = 2;
const ClassPrivate: u16         = 3;

// NullRawValue is a [RawValue] with its Tag set to the ASN.1 NULL type tag (5).
//const NullRawValue: RawValue = RawValue{Tag: TagNull};

// NullBytes contains bytes representing the DER-encoded ASN.1 NULL type.
const NullBytes: [u8; 2] = [TagNull, 0];

#[derive(Debug, PartialEq)]
pub enum SignatureAlgorithm {
    UnknownSignatureAlgorithm = 0,
    MD2WithRSA = 1,   // Unsupported.
    MD5WithRSA = 2,   // Only supported for signing, not verification.
    SHA1WithRSA = 3,  // Only supported for signing, and verification of CRLs, CSRs, and OCSP responses.
    SHA256WithRSA = 4,
    SHA384WithRSA = 5,
    SHA512WithRSA = 6,
    DSAWithSHA1 = 7, // Unsupported.
    DSAWithSHA256 = 8, // Unsupported.
    ECDSAWithSHA1 = 9, // Only supported for signing, and verification of CRLs, CSRs, and OCSP responses.
    ECDSAWithSHA256 = 10,
    ECDSAWithSHA384 = 11,
    ECDSAWithSHA512 = 12,
    SHA256WithRSAPSS = 13,
    SHA384WithRSAPSS = 14,
    SHA512WithRSAPSS = 15,
    PureEd25519 = 16,
}
/*
const UnknownSignatureAlgorithm: u16 = 0;
const MD2WithRSA: u16 = 1;  // Unsupported.
const MD5WithRSA: u16 = 2;  // Only supported for signing, not verification.
const SHA1WithRSA: u16 = 3; // Only supported for signing, and verification of CRLs, CSRs, and OCSP responses.
const SHA256WithRSA: u16 = 4;
const SHA384WithRSA: u16 = 5;
const SHA512WithRSA: u16 = 6;
const DSAWithSHA1: u16 = 7;   // Unsupported.
const DSAWithSHA256: u16 = 8; // Unsupported.
const ECDSAWithSHA1: u16 = 9; // Only supported for signing, and verification of CRLs, CSRs, and OCSP responses.
const ECDSAWithSHA256: u16 = 10;
const ECDSAWithSHA384: u16 = 11;
const ECDSAWithSHA512: u16 = 12;
const SHA256WithRSAPSS: u16 = 13;
const SHA384WithRSAPSS: u16 = 14;
const SHA512WithRSAPSS: u16 = 15;
const PureEd25519: u16 = 16;*/

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

pub const ROOT_GOOGLE_CERT_G1: [u8;1371] = [48, 130, 5, 87, 48, 130, 3, 63, 160, 3, 2, 1, 2, 2, 13, 2, 3, 229, 147, 111, 49, 176, 19, 73,
    136, 107, 162, 23, 48, 13, 6, 9, 42, 134, 72, 134, 247, 13, 1, 1, 12, 5, 0, 48, 71, 49, 11, 48, 9, 6, 3, 85, 4, 6, 19, 2, 85, 83, 49,
    34, 48, 32, 6, 3, 85, 4, 10, 19, 25, 71, 111, 111, 103, 108, 101, 32, 84, 114, 117, 115, 116, 32, 83, 101, 114, 118, 105, 99, 101,
    115, 32, 76, 76, 67, 49, 20, 48, 18, 6, 3, 85, 4, 3, 19, 11, 71, 84, 83, 32, 82, 111, 111, 116, 32, 82, 49, 48, 30, 23, 13, 49, 54,
    48, 54, 50, 50, 48, 48, 48, 48, 48, 48, 90, 23, 13, 51, 54, 48, 54, 50, 50, 48, 48, 48, 48, 48, 48, 90, 48, 71, 49, 11, 48, 9, 6, 3,
    85, 4, 6, 19, 2, 85, 83, 49, 34, 48, 32, 6, 3, 85, 4, 10, 19, 25, 71, 111, 111, 103, 108, 101, 32, 84, 114, 117, 115, 116, 32, 83,
    101, 114, 118, 105, 99, 101, 115, 32, 76, 76, 67, 49, 20, 48, 18, 6, 3, 85, 4, 3, 19, 11, 71, 84, 83, 32, 82, 111, 111, 116, 32, 82,
    49, 48, 130, 2, 34, 48, 13, 6, 9, 42, 134, 72, 134, 247, 13, 1, 1, 1, 5, 0, 3, 130, 2, 15, 0, 48, 130, 2, 10, 2, 130, 2, 1, 0, 182,
    17, 2, 139, 30, 227, 161, 119, 155, 59, 220, 191, 148, 62, 183, 149, 167, 64, 60, 161, 253, 130, 249, 125, 50, 6, 130, 113, 246, 246,
    140, 127, 251, 232, 219, 188, 106, 46, 151, 151, 163, 140, 75, 249, 43, 246, 177, 249, 206, 132, 29, 177, 249, 197, 151, 222, 239,
    185, 242, 163, 233, 188, 18, 137, 94, 167, 170, 82, 171, 248, 35, 39, 203, 164, 177, 156, 99, 219, 215, 153, 126, 240, 10, 94, 235,
    104, 166, 244, 198, 90, 71, 13, 77, 16, 51, 227, 78, 177, 19, 163, 200, 24, 108, 75, 236, 252, 9, 144, 223, 157, 100, 41, 37, 35, 7,
    161, 180, 210, 61, 46, 96, 224, 207, 210, 9, 135, 187, 205, 72, 240, 77, 194, 194, 122, 136, 138, 187, 186, 207, 89, 25, 214, 175,
    143, 176, 7, 176, 158, 49, 241, 130, 193, 192, 223, 46, 166, 109, 108, 25, 14, 181, 216, 126, 38, 26, 69, 3, 61, 176, 121, 164, 148,
    40, 173, 15, 127, 38, 229, 168, 8, 254, 150, 232, 60, 104, 148, 83, 238, 131, 58, 136, 43, 21, 150, 9, 178, 224, 122, 140, 46, 117,
    214, 156, 235, 167, 86, 100, 143, 150, 79, 104, 174, 61, 151, 194, 132, 143, 192, 188, 64, 192, 11, 92, 189, 246, 135, 179, 53, 108,
    172, 24, 80, 127, 132, 224, 76, 205, 146, 211, 32, 233, 51, 188, 82, 153, 175, 50, 181, 41, 179, 37, 42, 180, 72, 249, 114, 225, 202,
    100, 247, 230, 130, 16, 141, 232, 157, 194, 138, 136, 250, 56, 102, 138, 252, 99, 249, 1, 249, 120, 253, 123, 92, 119, 250, 118, 135,
    250, 236, 223, 177, 14, 121, 149, 87, 180, 189, 38, 239, 214, 1, 209, 235, 22, 10, 187, 142, 11, 181, 197, 197, 138, 85, 171, 211,
    172, 234, 145, 75, 41, 204, 25, 164, 50, 37, 78, 42, 241, 101, 68, 208, 2, 206, 170, 206, 73, 180, 234, 159, 124, 131, 176, 64, 123,
    231, 67, 171, 167, 108, 163, 143, 125, 137, 129, 250, 76, 165, 255, 213, 142, 195, 206, 75, 224, 181, 216, 179, 142, 69, 207, 118,
    192, 237, 64, 43, 253, 83, 15, 176, 167, 213, 59, 13, 177, 138, 162, 3, 222, 49, 173, 204, 119, 234, 111, 123, 62, 214, 223, 145, 34,
    18, 230, 190, 250, 216, 50, 252, 16, 99, 20, 81, 114, 222, 93, 214, 22, 147, 189, 41, 104, 51, 239, 58, 102, 236, 7, 138, 38, 223, 19,
    215, 87, 101, 120, 39, 222, 94, 73, 20, 0, 162, 0, 127, 154, 168, 33, 182, 169, 177, 149, 176, 165, 185, 13, 22, 17, 218, 199, 108,
    72, 60, 64, 224, 126, 13, 90, 205, 86, 60, 209, 151, 5, 185, 203, 75, 237, 57, 75, 156, 196, 63, 210, 85, 19, 110, 36, 176, 214, 113,
    250, 244, 193, 186, 204, 237, 27, 245, 254, 129, 65, 216, 0, 152, 61, 58, 200, 174, 122, 152, 55, 24, 5, 149, 2, 3, 1, 0, 1, 163, 66,
    48, 64, 48, 14, 6, 3, 85, 29, 15, 1, 1, 255, 4, 4, 3, 2, 1, 134, 48, 15, 6, 3, 85, 29, 19, 1, 1, 255, 4, 5, 48, 3, 1, 1, 255, 48, 29,
    6, 3, 85, 29, 14, 4, 22, 4, 20, 228, 175, 43, 38, 113, 26, 43, 72, 39, 133, 47, 82, 102, 44, 239, 240, 137, 19, 113, 62, 48, 13, 6, 9,
    42, 134, 72, 134, 247, 13, 1, 1, 12, 5, 0, 3, 130, 2, 1, 0, 159, 170, 66, 38, 219, 11, 155, 190, 255, 30, 150, 146, 46, 62, 162, 101,
    74, 106, 152, 186, 34, 203, 125, 193, 58, 216, 130, 10, 6, 198, 246, 165, 222, 192, 78, 135, 102, 121, 161, 249, 166, 88, 156, 170,
    249, 181, 230, 96, 231, 224, 232, 177, 30, 66, 65, 51, 11, 55, 61, 206, 137, 112, 21, 202, 181, 36, 168, 207, 107, 181, 210, 64, 33,
    152, 207, 34, 52, 207, 59, 197, 34, 132, 224, 197, 14, 138, 124, 93, 136, 228, 53, 36, 206, 155, 62, 26, 84, 30, 110, 219, 178, 135,
    167, 252, 243, 250, 129, 85, 20, 98, 10, 89, 169, 34, 5, 49, 62, 130, 214, 238, 219, 87, 52, 188, 51, 149, 211, 23, 27, 232, 39, 162,
    139, 123, 78, 38, 26, 122, 90, 100, 182, 209, 172, 55, 241, 253, 160, 243, 56, 236, 114, 240, 17, 117, 157, 203, 52, 82, 141, 230, 118,
    107, 23, 198, 223, 134, 171, 39, 142, 73, 43, 117, 102, 129, 16, 33, 166, 234, 62, 244, 174, 37, 255, 124, 21, 222, 206, 140, 37, 63,
    202, 98, 112, 10, 247, 47, 9, 102, 7, 200, 63, 28, 252, 240, 219, 69, 48, 223, 98, 136, 193, 181, 15, 157, 195, 159, 74, 222, 89, 89,
    71, 197, 135, 34, 54, 230, 130, 167, 237, 10, 185, 226, 7, 160, 141, 123, 122, 74, 60, 113, 210, 226, 3, 161, 31, 50, 7, 221, 27, 228,
    66, 206, 12, 0, 69, 97, 128, 181, 11, 32, 89, 41, 120, 189, 249, 85, 203, 99, 197, 60, 76, 244, 182, 255, 219, 106, 95, 49, 107, 153,
    158, 44, 193, 107, 80, 164, 215, 230, 24, 20, 189, 133, 63, 103, 171, 70, 159, 160, 255, 66, 167, 58, 127, 92, 203, 93, 176, 112, 29,
    43, 52, 245, 212, 118, 9, 12, 235, 120, 76, 89, 5, 243, 51, 66, 195, 97, 21, 16, 27, 119, 77, 206, 34, 140, 212, 133, 242, 69, 125,
    183, 83, 234, 239, 64, 90, 148, 10, 92, 32, 95, 78, 64, 93, 98, 34, 118, 223, 255, 206, 97, 189, 140, 35, 120, 210, 55, 2, 224, 142,
    222, 209, 17, 55, 137, 246, 191, 237, 73, 7, 98, 174, 146, 236, 64, 26, 175, 20, 9, 217, 208, 78, 178, 162, 247, 190, 238, 238, 216,
    255, 220, 26, 45, 222, 184, 54, 113, 226, 252, 121, 183, 148, 37, 209, 72, 115, 91, 161, 53, 231, 179, 153, 103, 117, 193, 25, 58, 43,
    71, 78, 211, 66, 142, 253, 49, 200, 22, 102, 218, 210, 12, 60, 219, 179, 142, 201, 161, 13, 128, 15, 123, 22, 119, 20, 191, 255, 219,
    9, 148, 178, 147, 188, 32, 88, 21, 233, 219, 113, 67, 243, 222, 16, 195, 0, 220, 168, 42, 149, 182, 194, 214, 63, 144, 107, 118, 219,
    108, 254, 140, 188, 242, 112, 53, 12, 220, 153, 25, 53, 220, 215, 200, 70, 99, 213, 54, 113, 174, 87, 251, 183, 130, 109, 220];


pub const ROOT_GOOGLE_CERT_G2: [u8;1371] = [48, 130, 5, 87, 48, 130, 3, 63, 160, 3, 2, 1, 2, 2, 13, 2, 3, 229, 174, 197, 141, 4, 37, 26,
    171, 17, 37, 170, 48, 13, 6, 9, 42, 134, 72, 134, 247, 13, 1, 1, 12, 5, 0, 48, 71, 49, 11, 48, 9, 6, 3, 85, 4, 6, 19, 2, 85, 83, 49,
    34, 48, 32, 6, 3, 85, 4, 10, 19, 25, 71, 111, 111, 103, 108, 101, 32, 84, 114, 117, 115, 116, 32, 83, 101, 114, 118, 105, 99, 101,
    115, 32, 76, 76, 67, 49, 20, 48, 18, 6, 3, 85, 4, 3, 19, 11, 71, 84, 83, 32, 82, 111, 111, 116, 32, 82, 50, 48, 30, 23, 13, 49, 54,
    48, 54, 50, 50, 48, 48, 48, 48, 48, 48, 90, 23, 13, 51, 54, 48, 54, 50, 50, 48, 48, 48, 48, 48, 48, 90, 48, 71, 49, 11, 48, 9, 6, 3,
    85, 4, 6, 19, 2, 85, 83, 49, 34, 48, 32, 6, 3, 85, 4, 10, 19, 25, 71, 111, 111, 103, 108, 101, 32, 84, 114, 117, 115, 116, 32, 83,
    101, 114, 118, 105, 99, 101, 115, 32, 76, 76, 67, 49, 20, 48, 18, 6, 3, 85, 4, 3, 19, 11, 71, 84, 83, 32, 82, 111, 111, 116, 32, 82,
    50, 48, 130, 2, 34, 48, 13, 6, 9, 42, 134, 72, 134, 247, 13, 1, 1, 1, 5, 0, 3, 130, 2, 15, 0, 48, 130, 2, 10, 2, 130, 2, 1, 0, 206,
    222, 253, 166, 251, 236, 236, 20, 52, 60, 7, 6, 90, 108, 89, 247, 25, 53, 221, 247, 193, 157, 85, 170, 211, 205, 59, 164, 147, 114,
    239, 10, 250, 109, 157, 246, 240, 133, 128, 91, 161, 72, 82, 159, 57, 197, 183, 238, 40, 172, 239, 203, 118, 104, 20, 185, 223, 173,
    1, 108, 153, 31, 196, 34, 29, 159, 254, 114, 119, 224, 44, 91, 175, 228, 4, 191, 79, 114, 160, 26, 52, 152, 232, 57, 104, 236, 149,
    37, 123, 118, 161, 230, 105, 185, 133, 25, 189, 137, 140, 254, 173, 237, 54, 234, 115, 188, 255, 131, 226, 203, 125, 193, 210, 206,
    74, 179, 141, 5, 158, 139, 73, 147, 223, 193, 91, 208, 110, 94, 240, 46, 48, 46, 130, 252, 250, 188, 180, 23, 10, 72, 229, 136, 155,
    197, 155, 107, 222, 176, 202, 180, 3, 240, 218, 244, 144, 184, 101, 100, 247, 92, 76, 173, 232, 126, 102, 94, 153, 215, 184, 194, 62,
    200, 208, 19, 157, 173, 238, 228, 69, 123, 137, 85, 247, 138, 31, 98, 82, 132, 18, 179, 194, 64, 151, 227, 138, 31, 71, 145, 166, 116,
    90, 210, 248, 177, 99, 40, 16, 184, 179, 9, 184, 86, 119, 64, 162, 38, 152, 121, 198, 254, 223, 37, 238, 62, 229, 160, 127, 212, 97,
    15, 81, 75, 60, 63, 140, 218, 225, 112, 116, 216, 194, 104, 161, 249, 193, 12, 233, 161, 226, 127, 187, 85, 60, 118, 6, 238, 106, 78,
    204, 146, 136, 48, 77, 154, 189, 79, 11, 72, 154, 132, 181, 152, 163, 213, 251, 115, 193, 87, 97, 221, 40, 86, 117, 19, 174, 135, 142,
    231, 12, 81, 9, 16, 117, 136, 76, 188, 141, 249, 123, 60, 212, 34, 72, 31, 42, 220, 235, 107, 187, 68, 177, 203, 51, 113, 50, 70, 175,
    173, 74, 241, 140, 232, 116, 58, 172, 231, 26, 34, 115, 128, 210, 48, 247, 37, 66, 199, 34, 59, 59, 18, 173, 150, 46, 198, 195, 118, 7,
    170, 32, 183, 53, 73, 87, 233, 146, 73, 232, 118, 22, 114, 49, 103, 43, 150, 126, 138, 163, 199, 148, 86, 34, 191, 106, 75, 126, 1, 33,
    178, 35, 50, 223, 228, 154, 68, 109, 89, 91, 93, 245, 0, 160, 28, 155, 198, 120, 151, 141, 144, 255, 155, 200, 170, 180, 175, 17, 81,
    57, 94, 217, 251, 103, 173, 213, 91, 17, 157, 50, 154, 27, 189, 213, 186, 91, 165, 201, 203, 37, 105, 83, 85, 39, 92, 224, 202, 54, 203,
    136, 97, 251, 30, 183, 208, 203, 238, 22, 251, 211, 166, 76, 222, 146, 165, 212, 226, 223, 245, 6, 84, 222, 46, 157, 75, 180, 147, 48,
    170, 129, 206, 221, 26, 220, 81, 115, 13, 79, 112, 233, 229, 182, 22, 33, 25, 121, 178, 230, 137, 11, 117, 100, 202, 213, 171, 188, 9,
    193, 24, 161, 255, 212, 84, 161, 133, 60, 253, 20, 36, 3, 178, 135, 211, 164, 183, 2, 3, 1, 0, 1, 163, 66, 48, 64, 48, 14, 6, 3, 85, 29,
    15, 1, 1, 255, 4, 4, 3, 2, 1, 134, 48, 15, 6, 3, 85, 29, 19, 1, 1, 255, 4, 5, 48, 3, 1, 1, 255, 48, 29, 6, 3, 85, 29, 14, 4, 22, 4, 20,
    187, 255, 202, 142, 35, 159, 79, 153, 202, 219, 226, 104, 166, 165, 21, 39, 23, 30, 217, 14, 48, 13, 6, 9, 42, 134, 72, 134, 247, 13, 1,
    1, 12, 5, 0, 3, 130, 2, 1, 0, 31, 202, 206, 221, 199, 190, 161, 159, 217, 39, 76, 11, 220, 23, 152, 17, 106, 136, 222, 61, 230, 113, 86,
    114, 178, 158, 26, 78, 156, 213, 43, 152, 36, 93, 155, 107, 123, 176, 51, 130, 9, 189, 223, 37, 70, 234, 152, 158, 182, 27, 254, 131,
    60, 210, 98, 97, 193, 4, 237, 206, 224, 197, 201, 200, 19, 19, 85, 231, 168, 99, 173, 140, 123, 1, 254, 119, 48, 225, 206, 104, 155, 5,
    248, 18, 238, 121, 49, 160, 65, 69, 53, 40, 10, 113, 164, 36, 79, 140, 220, 60, 130, 7, 95, 102, 220, 125, 16, 254, 12, 97, 179, 5, 149,
    238, 225, 174, 129, 15, 168, 248, 199, 143, 77, 168, 35, 2, 38, 107, 29, 131, 82, 85, 206, 181, 47, 0, 202, 128, 64, 224, 225, 116, 172,
    96, 245, 135, 128, 157, 174, 54, 100, 145, 93, 176, 104, 24, 234, 138, 97, 201, 119, 168, 151, 196, 201, 199, 165, 252, 85, 75, 243,
    240, 127, 185, 101, 61, 39, 104, 208, 204, 107, 250, 83, 157, 225, 145, 26, 201, 93, 26, 150, 109, 50, 135, 237, 3, 32, 200, 2, 206,
    90, 190, 217, 234, 253, 178, 77, 196, 47, 27, 223, 95, 122, 245, 248, 139, 198, 238, 49, 58, 37, 81, 85, 103, 141, 100, 50, 123, 233,
    158, 195, 130, 186, 42, 45, 233, 30, 180, 224, 72, 6, 162, 252, 103, 175, 31, 34, 2, 115, 251, 32, 10, 175, 157, 84, 75, 161, 205, 255,
    96, 71, 176, 63, 93, 239, 27, 86, 189, 151, 33, 150, 45, 10, 209, 94, 157, 56, 2, 71, 108, 185, 244, 246, 35, 37, 184, 160, 106, 154,
    43, 119, 8, 250, 196, 177, 40, 144, 38, 88, 8, 60, 226, 126, 170, 215, 61, 111, 186, 49, 136, 10, 5, 235, 39, 181, 161, 73, 238, 160,
    69, 84, 123, 230, 39, 101, 153, 32, 33, 168, 163, 188, 251, 24, 150, 187, 82, 111, 12, 237, 131, 81, 76, 233, 89, 226, 32, 96, 197,
    194, 101, 146, 130, 140, 243, 16, 31, 14, 138, 151, 190, 119, 130, 109, 63, 143, 29, 93, 188, 73, 39, 189, 204, 79, 15, 225, 206, 118,
    134, 4, 35, 197, 192, 140, 18, 91, 253, 219, 132, 160, 36, 241, 72, 255, 100, 124, 208, 190, 92, 22, 209, 239, 153, 173, 192, 31, 251,
    203, 174, 188, 56, 34, 6, 38, 100, 218, 218, 151, 14, 63, 40, 21, 68, 168, 79, 0, 202, 240, 154, 204, 207, 116, 106, 180, 62, 60, 235,
    149, 236, 181, 211, 90, 216, 129, 153, 233, 67, 24, 55, 235, 179, 187, 209, 88, 98, 65, 243, 102, 210, 143, 170, 120, 149, 84, 32, 195,
    90, 46, 116, 43, 213, 209, 190, 24, 105, 192, 172, 213, 164, 207, 57, 186, 81, 132, 3, 101, 233, 98, 192, 98, 254, 216, 77, 85, 150,
    226, 208, 17, 250, 72, 52, 17, 236, 158, 237, 5, 29, 228, 200, 214, 29, 134, 203];

pub const ROOT_GOOGLE_CERT_G3: [u8; 525] = [48, 130, 2, 9, 48, 130, 1, 142, 160, 3, 2, 1, 2, 2, 13, 2, 3, 229, 184, 130, 235, 32, 248, 37,
    39, 109, 61, 102, 48, 10, 6, 8, 42, 134, 72, 206, 61, 4, 3, 3, 48, 71, 49, 11, 48, 9, 6, 3, 85, 4, 6, 19, 2, 85, 83, 49, 34, 48, 32, 6,
    3, 85, 4, 10, 19, 25, 71, 111, 111, 103, 108, 101, 32, 84, 114, 117, 115, 116, 32, 83, 101, 114, 118, 105, 99, 101, 115, 32, 76, 76, 67,
    49, 20, 48, 18, 6, 3, 85, 4, 3, 19, 11, 71, 84, 83, 32, 82, 111, 111, 116, 32, 82, 51, 48, 30, 23, 13, 49, 54, 48, 54, 50, 50, 48, 48,
    48, 48, 48, 48, 90, 23, 13, 51, 54, 48, 54, 50, 50, 48, 48, 48, 48, 48, 48, 90, 48, 71, 49, 11, 48, 9, 6, 3, 85, 4, 6, 19, 2, 85, 83,
    49, 34, 48, 32, 6, 3, 85, 4, 10, 19, 25, 71, 111, 111, 103, 108, 101, 32, 84, 114, 117, 115, 116, 32, 83, 101, 114, 118, 105, 99, 101,
    115, 32, 76, 76, 67, 49, 20, 48, 18, 6, 3, 85, 4, 3, 19, 11, 71, 84, 83, 32, 82, 111, 111, 116, 32, 82, 51, 48, 118, 48, 16, 6, 7, 42,
    134, 72, 206, 61, 2, 1, 6, 5, 43, 129, 4, 0, 34, 3, 98, 0, 4, 31, 79, 51, 135, 51, 41, 138, 161, 132, 222, 203, 199, 33, 88, 65, 137,
    234, 86, 157, 43, 75, 133, 198, 29, 76, 39, 188, 127, 38, 81, 114, 111, 226, 159, 214, 163, 202, 204, 69, 20, 70, 139, 173, 239, 126,
    134, 140, 236, 177, 126, 47, 255, 169, 113, 157, 24, 132, 69, 4, 65, 85, 110, 43, 234, 38, 127, 187, 144, 1, 227, 75, 25, 186, 228, 84,
    150, 69, 9, 177, 213, 108, 145, 68, 173, 132, 19, 142, 154, 140, 13, 128, 12, 50, 246, 224, 39, 163, 66, 48, 64, 48, 14, 6, 3, 85, 29,
    15, 1, 1, 255, 4, 4, 3, 2, 1, 134, 48, 15, 6, 3, 85, 29, 19, 1, 1, 255, 4, 5, 48, 3, 1, 1, 255, 48, 29, 6, 3, 85, 29, 14, 4, 22, 4, 20,
    193, 241, 38, 186, 160, 45, 174, 133, 129, 207, 211, 241, 42, 18, 189, 184, 10, 103, 253, 188, 48, 10, 6, 8, 42, 134, 72, 206, 61, 4,
    3, 3, 3, 105, 0, 48, 102, 2, 49, 0, 246, 225, 32, 149, 20, 123, 84, 163, 144, 22, 17, 191, 132, 200, 234, 111, 107, 23, 158, 30, 70,
    152, 32, 155, 159, 211, 13, 217, 172, 211, 47, 205, 124, 248, 91, 46, 85, 187, 191, 221, 146, 247, 164, 12, 220, 49, 225, 162, 2, 49,
    0, 252, 151, 102, 102, 229, 67, 22, 19, 131, 221, 199, 223, 47, 190, 20, 56, 237, 1, 206, 177, 23, 26, 17, 117, 233, 189, 3, 143, 38,
    126, 132, 229, 201, 96, 166, 149, 215, 84, 89, 183, 231, 17, 44, 137, 212, 185, 238, 23];

pub const ROOT_GOOGLE_CERT_G4: [u8; 525] = [48, 130, 2, 9, 48, 130, 1, 142, 160, 3, 2, 1, 2, 2, 13, 2, 3, 229, 192, 104, 239, 99, 26, 156,
    114, 144, 80, 82, 48, 10, 6, 8, 42, 134, 72, 206, 61, 4, 3, 3, 48, 71, 49, 11, 48, 9, 6, 3, 85, 4, 6, 19, 2, 85, 83, 49, 34, 48, 32, 6,
    3, 85, 4, 10, 19, 25, 71, 111, 111, 103, 108, 101, 32, 84, 114, 117, 115, 116, 32, 83, 101, 114, 118, 105, 99, 101, 115, 32, 76, 76, 67,
    49, 20, 48, 18, 6, 3, 85, 4, 3, 19, 11, 71, 84, 83, 32, 82, 111, 111, 116, 32, 82, 52, 48, 30, 23, 13, 49, 54, 48, 54, 50, 50, 48, 48,
    48, 48, 48, 48, 90, 23, 13, 51, 54, 48, 54, 50, 50, 48, 48, 48, 48, 48, 48, 90, 48, 71, 49, 11, 48, 9, 6, 3, 85, 4, 6, 19, 2, 85, 83,
    49, 34, 48, 32, 6, 3, 85, 4, 10, 19, 25, 71, 111, 111, 103, 108, 101, 32, 84, 114, 117, 115, 116, 32, 83, 101, 114, 118, 105, 99, 101,
    115, 32, 76, 76, 67, 49, 20, 48, 18, 6, 3, 85, 4, 3, 19, 11, 71, 84, 83, 32, 82, 111, 111, 116, 32, 82, 52, 48, 118, 48, 16, 6, 7, 42,
    134, 72, 206, 61, 2, 1, 6, 5, 43, 129, 4, 0, 34, 3, 98, 0, 4, 243, 116, 115, 167, 104, 139, 96, 174, 67, 184, 53, 197, 129, 48, 123,
    75, 73, 157, 251, 193, 97, 206, 230, 222, 70, 189, 107, 213, 97, 24, 53, 174, 64, 221, 115, 247, 137, 145, 48, 90, 235, 60, 238, 133,
    124, 162, 64, 118, 59, 169, 198, 184, 71, 216, 42, 231, 146, 145, 106, 115, 233, 177, 114, 57, 159, 41, 159, 162, 152, 211, 95, 94, 88, 134, 101, 15, 161, 132, 101, 6, 209, 220, 139, 201, 199, 115, 200, 140, 106, 47, 229, 196, 171, 209, 29, 138, 163, 66, 48, 64, 48, 14, 6, 3, 85, 29, 15, 1, 1, 255, 4, 4, 3, 2, 1, 134, 48, 15, 6, 3, 85, 29, 19, 1, 1, 255, 4, 5, 48, 3, 1, 1, 255, 48, 29, 6, 3, 85, 29, 14, 4, 22, 4, 20, 128, 76, 214, 235, 116, 255, 73, 54, 163, 213, 216, 252, 181, 62, 197, 106, 240, 148, 29, 140, 48, 10, 6, 8, 42, 134, 72, 206, 61, 4, 3, 3, 3, 105, 0, 48, 102, 2, 49, 0, 232, 64, 255, 131, 222, 3, 244, 159, 174, 29, 122, 167, 46, 185, 175, 79, 246, 131, 29, 14, 45, 133, 1, 29, 209, 217, 106, 236, 15, 194, 175, 199, 94, 86, 94, 92, 213, 28, 88, 34, 40, 11, 247, 48, 182, 47, 177, 124, 2, 49, 0, 240, 97, 60, 167, 244, 160, 130, 227, 33, 213, 132, 29, 115, 134, 156, 45, 175, 202, 52, 155, 241, 159, 185, 35, 54, 226, 188, 96, 3, 157, 128, 179, 154, 86, 200, 225, 226, 187, 20, 121, 202, 205, 33, 212, 148, 181, 73, 67];

pub const ROOTS_CERTS: [&str; 4] = [
    "308205573082033fa003020102020d0203e5936f31b01349886ba217300d06092a864886f70d01010c05003047310b300906035504061302555331223020060355040a1319476f6f676c65205472757374205365727669636573204c4c43311430120603550403130b47545320526f6f74205231301e170d3136303632323030303030305a170d3336303632323030303030305a3047310b300906035504061302555331223020060355040a1319476f6f676c65205472757374205365727669636573204c4c43311430120603550403130b47545320526f6f7420523130820222300d06092a864886f70d01010105000382020f003082020a0282020100b611028b1ee3a1779b3bdcbf943eb795a7403ca1fd82f97d32068271f6f68c7ffbe8dbbc6a2e9797a38c4bf92bf6b1f9ce841db1f9c597deefb9f2a3e9bc12895ea7aa52abf82327cba4b19c63dbd7997ef00a5eeb68a6f4c65a470d4d1033e34eb113a3c8186c4becfc0990df9d6429252307a1b4d23d2e60e0cfd20987bbcd48f04dc2c27a888abbbacf5919d6af8fb007b09e31f182c1c0df2ea66d6c190eb5d87e261a45033db079a49428ad0f7f26e5a808fe96e83c689453ee833a882b159609b2e07a8c2e75d69ceba756648f964f68ae3d97c2848fc0bc40c00b5cbdf687b3356cac18507f84e04ccd92d320e933bc5299af32b529b3252ab448f972e1ca64f7e682108de89dc28a88fa38668afc63f901f978fd7b5c77fa7687faecdfb10e799557b4bd26efd601d1eb160abb8e0bb5c5c58a55abd3acea914b29cc19a432254e2af16544d002ceaace49b4ea9f7c83b0407be743aba76ca38f7d8981fa4ca5ffd58ec3ce4be0b5d8b38e45cf76c0ed402bfd530fb0a7d53b0db18aa203de31adcc77ea6f7b3ed6df912212e6befad832fc1063145172de5dd61693bd296833ef3a66ec078a26df13d757657827de5e491400a2007f9aa821b6a9b195b0a5b90d1611dac76c483c40e07e0d5acd563cd19705b9cb4bed394b9cc43fd255136e24b0d671faf4c1bacced1bf5fe8141d800983d3ac8ae7a98371805950203010001a3423040300e0603551d0f0101ff040403020186300f0603551d130101ff040530030101ff301d0603551d0e04160414e4af2b26711a2b4827852f52662ceff08913713e300d06092a864886f70d01010c050003820201009faa4226db0b9bbeff1e96922e3ea2654a6a98ba22cb7dc13ad8820a06c6f6a5dec04e876679a1f9a6589caaf9b5e660e7e0e8b11e4241330b373dce897015cab524a8cf6bb5d2402198cf2234cf3bc52284e0c50e8a7c5d88e43524ce9b3e1a541e6edbb287a7fcf3fa815514620a59a92205313e82d6eedb5734bc3395d3171be827a28b7b4e261a7a5a64b6d1ac37f1fda0f338ec72f011759dcb34528de6766b17c6df86ab278e492b7566811021a6ea3ef4ae25ff7c15dece8c253fca62700af72f096607c83f1cfcf0db4530df6288c1b50f9dc39f4ade595947c5872236e682a7ed0ab9e207a08d7b7a4a3c71d2e203a11f3207dd1be442ce0c00456180b50b20592978bdf955cb63c53c4cf4b6ffdb6a5f316b999e2cc16b50a4d7e61814bd853f67ab469fa0ff42a73a7f5ccb5db0701d2b34f5d476090ceb784c5905f33342c36115101b774dce228cd485f2457db753eaef405a940a5c205f4e405d622276dfffce61bd8c2378d23702e08eded1113789f6bfed490762ae92ec401aaf1409d9d04eb2a2f7beeeeed8ffdc1a2ddeb83671e2fc79b79425d148735ba135e7b3996775c1193a2b474ed3428efd31c81666dad20c3cdbb38ec9a10d800f7b167714bfffdb0994b293bc205815e9db7143f3de10c300dca82a95b6c2d63f906b76db6cfe8cbcf270350cdc991935dcd7c84663d53671ae57fbb7826ddc",
    "308205573082033fa003020102020d0203e5aec58d04251aab1125aa300d06092a864886f70d01010c05003047310b300906035504061302555331223020060355040a1319476f6f676c65205472757374205365727669636573204c4c43311430120603550403130b47545320526f6f74205232301e170d3136303632323030303030305a170d3336303632323030303030305a3047310b300906035504061302555331223020060355040a1319476f6f676c65205472757374205365727669636573204c4c43311430120603550403130b47545320526f6f7420523230820222300d06092a864886f70d01010105000382020f003082020a0282020100cedefda6fbecec14343c07065a6c59f71935ddf7c19d55aad3cd3ba49372ef0afa6d9df6f085805ba148529f39c5b7ee28acefcb766814b9dfad016c991fc4221d9ffe7277e02c5bafe404bf4f72a01a3498e83968ec95257b76a1e669b98519bd898cfeaded36ea73bcff83e2cb7dc1d2ce4ab38d059e8b4993dfc15bd06e5ef02e302e82fcfabcb4170a48e5889bc59b6bdeb0cab403f0daf490b86564f75c4cade87e665e99d7b8c23ec8d0139dadeee4457b8955f78a1f62528412b3c24097e38a1f4791a6745ad2f8b1632810b8b309b8567740a2269879c6fedf25ee3ee5a07fd4610f514b3c3f8cdae17074d8c268a1f9c10ce9a1e27fbb553c7606ee6a4ecc9288304d9abd4f0b489a84b598a3d5fb73c15761dd28567513ae878ee70c51091075884cbc8df97b3cd422481f2adceb6bbb44b1cb33713246afad4af18ce8743aace71a227380d230f72542c7223b3b12ad962ec6c37607aa20b7354957e99249e876167231672b967e8aa3c7945622bf6a4b7e0121b22332dfe49a446d595b5df500a01c9bc678978d90ff9bc8aab4af1151395ed9fb67add55b119d329a1bbdd5ba5ba5c9cb25695355275ce0ca36cb8861fb1eb7d0cbee16fbd3a64cde92a5d4e2dff50654de2e9d4bb49330aa81cedd1adc51730d4f70e9e5b616211979b2e6890b7564cad5abbc09c118a1ffd454a1853cfd142403b287d3a4b70203010001a3423040300e0603551d0f0101ff040403020186300f0603551d130101ff040530030101ff301d0603551d0e04160414bbffca8e239f4f99cadbe268a6a51527171ed90e300d06092a864886f70d01010c050003820201001fcaceddc7bea19fd9274c0bdc1798116a88de3de6715672b29e1a4e9cd52b98245d9b6b7bb0338209bddf2546ea989eb61bfe833cd26261c104edcee0c5c9c8131355e7a863ad8c7b01fe7730e1ce689b05f812ee7931a0414535280a71a4244f8cdc3c82075f66dc7d10fe0c61b30595eee1ae810fa8f8c78f4da82302266b1d835255ceb52f00ca8040e0e174ac60f587809dae3664915db06818ea8a61c977a897c4c9c7a5fc554bf3f07fb9653d2768d0cc6bfa539de1911ac95d1a966d3287ed0320c802ce5abed9eafdb24dc42f1bdf5f7af5f88bc6ee313a255155678d64327be99ec382ba2a2de91eb4e04806a2fc67af1f220273fb200aaf9d544ba1cdff6047b03f5def1b56bd9721962d0ad15e9d3802476cb9f4f62325b8a06a9a2b7708fac4b128902658083ce27eaad73d6fba31880a05eb27b5a149eea045547be62765992021a8a3bcfb1896bb526f0ced83514ce959e22060c5c26592828cf3101f0e8a97be77826d3f8f1d5dbc4927bdcc4f0fe1ce76860423c5c08c125bfddb84a024f148ff647cd0be5c16d1ef99adc01ffbcbaebc3822062664dada970e3f281544a84f00caf09acccf746ab43e3ceb95ecb5d35ad88199e9431837ebb3bbd1586241f366d28faa78955420c35a2e742bd5d1be1869c0acd5a4cf39ba51840365e962c062fed84d5596e2d011fa483411ec9eed051de4c8d61d86cb",
    "308202093082018ea003020102020d0203e5b882eb20f825276d3d66300a06082a8648ce3d0403033047310b300906035504061302555331223020060355040a1319476f6f676c65205472757374205365727669636573204c4c43311430120603550403130b47545320526f6f74205233301e170d3136303632323030303030305a170d3336303632323030303030305a3047310b300906035504061302555331223020060355040a1319476f6f676c65205472757374205365727669636573204c4c43311430120603550403130b47545320526f6f742052333076301006072a8648ce3d020106052b81040022036200041f4f338733298aa184decbc721584189ea569d2b4b85c61d4c27bc7f2651726fe29fd6a3cacc4514468badef7e868cecb17e2fffa9719d1884450441556e2bea267fbb9001e34b19bae454964509b1d56c9144ad84138e9a8c0d800c32f6e027a3423040300e0603551d0f0101ff040403020186300f0603551d130101ff040530030101ff301d0603551d0e04160414c1f126baa02dae8581cfd3f12a12bdb80a67fdbc300a06082a8648ce3d0403030369003066023100f6e12095147b54a3901611bf84c8ea6f6b179e1e4698209b9fd30dd9acd32fcd7cf85b2e55bbbfdd92f7a40cdc31e1a2023100fc976666e543161383ddc7df2fbe1438ed01ceb1171a1175e9bd038f267e84e5c960a695d75459b7e7112c89d4b9ee17",
    "308202093082018ea003020102020d0203e5c068ef631a9c72905052300a06082a8648ce3d0403033047310b300906035504061302555331223020060355040a1319476f6f676c65205472757374205365727669636573204c4c43311430120603550403130b47545320526f6f74205234301e170d3136303632323030303030305a170d3336303632323030303030305a3047310b300906035504061302555331223020060355040a1319476f6f676c65205472757374205365727669636573204c4c43311430120603550403130b47545320526f6f742052343076301006072a8648ce3d020106052b8104002203620004f37473a7688b60ae43b835c581307b4b499dfbc161cee6de46bd6bd5611835ae40dd73f78991305aeb3cee857ca240763ba9c6b847d82ae792916a73e9b172399f299fa298d35f5e5886650fa1846506d1dc8bc9c773c88c6a2fe5c4abd11d8aa3423040300e0603551d0f0101ff040403020186300f0603551d130101ff040530030101ff301d0603551d0e04160414804cd6eb74ff4936a3d5d8fcb53ec56af0941d8c300a06082a8648ce3d0403030369003066023100e840ff83de03f49fae1d7aa72eb9af4ff6831d0e2d85011dd1d96aec0fc2afc75e565e5cd51c5822280bf730b62fb17c023100f0613ca7f4a082e321d5841d73869c2dafca349bf19fb92336e2bc60039d80b39a56c8e1e2bb1479cacd21d494b54943"
]; //der in hex



const OID_SIGNATURE_MD2_WITH_RSA: [i32; 7]      = [1, 2, 840, 113549, 1, 1, 2];
const OID_SIGNATURE_MD5_WITH_RSA: [i32; 7]      = [1, 2, 840, 113549, 1, 1, 4];
const OID_SIGNATURE_SHA1_WITH_RSA: [i32; 7]     = [1, 2, 840, 113549, 1, 1, 5];
const OID_SIGNATURE_SHA256_WITH_RSA: [i32; 7]   = [1, 2, 840, 113549, 1, 1, 11];
const OID_SIGNATURE_SHA384_WITH_RSA: [i32; 7]   = [1, 2, 840, 113549, 1, 1, 12];
const OID_SIGNATURE_SHA512_WITH_RSA: [i32; 7]   = [1, 2, 840, 113549, 1, 1, 13];
const OID_SIGNATURE_RSA_PSS: [i32; 7]          = [1, 2, 840, 113549, 1, 1, 10];
const OID_SIGNATURE_DSA_WITH_SHA1: [i32; 6]     = [1, 2, 840, 10040, 4, 3];
const OID_SIGNATURE_DSA_WITH_SHA256: [i32; 9]   = [2, 16, 840, 1, 101, 3, 4, 3, 2];
const OID_SIGNATURE_ECDSA_WITH_SHA1: [i32; 6]   = [1, 2, 840, 10045, 4, 1];
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
const oidISOSignatureSHA1WithRSA: [i32; 6] = [1, 3, 14, 3, 2, 29];


const UnknownPublicKeyAlgorithm: u16 = 0;
const RSA: u16 = 1;
const DSA: u16 = 2; // Only supported for parsing.
const ECDSA: u16 = 3;
const Ed25519: u16 = 4;

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
            PublicKeyAlgorithm::UnknownPublicKeyAlgorithm => "UnknownPublicKeyAlgorithm".to_string(),
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
    oid: Vec<i32>, //ObjectIdentifier,
    pub_key_algo: PublicKeyAlgorithm,
    hash: Option<String>, // Можно использовать String для хеш-алгоритма
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct KeyUsage(i32);

impl KeyUsage {
    pub const DIGITAL_SIGNATURE: KeyUsage = KeyUsage(1 << 0);
    pub const CONTENT_COMMITMENT: KeyUsage = KeyUsage(1 << 1);
    pub const KEY_ENCIPHERMENT: KeyUsage = KeyUsage(1 << 2);
    pub const DATA_ENCIPHERMENT: KeyUsage = KeyUsage(1 << 3);
    pub const KEY_AGREEMENT: KeyUsage = KeyUsage(1 << 4);
    pub const CERT_SIGN: KeyUsage = KeyUsage(1 << 5);
    pub const CRL_SIGN: KeyUsage = KeyUsage(1 << 6);
    pub const ENCIPHER_ONLY: KeyUsage = KeyUsage(1 << 7);
    pub const DECIPHER_ONLY: KeyUsage = KeyUsage(1 << 8);
}

#[derive(Debug, Clone, Copy)]
pub struct ExtKeyUsage(i32);

impl ExtKeyUsage {
    pub const ANY: ExtKeyUsage = ExtKeyUsage(0);
    pub const SERVER_AUTH: ExtKeyUsage = ExtKeyUsage(1);
    pub const CLIENT_AUTH: ExtKeyUsage = ExtKeyUsage(2);
    pub const CODE_SIGNING: ExtKeyUsage = ExtKeyUsage(3);
    pub const EMAIL_PROTECTION: ExtKeyUsage = ExtKeyUsage(4);
    pub const IPSEC_END_SYSTEM: ExtKeyUsage = ExtKeyUsage(5);
    pub const IPSEC_TUNNEL: ExtKeyUsage = ExtKeyUsage(6);
    pub const IPSEC_USER: ExtKeyUsage = ExtKeyUsage(7);
    pub const TIME_STAMPING: ExtKeyUsage = ExtKeyUsage(8);
    pub const OCSP_SIGNING: ExtKeyUsage = ExtKeyUsage(9);
    pub const MICROSOFT_SERVER_GATED_CRYPTO: ExtKeyUsage = ExtKeyUsage(10);
    pub const NETSCAPE_SERVER_GATED_CRYPTO: ExtKeyUsage = ExtKeyUsage(11);
    pub const MICROSOFT_COMMERCIAL_CODE_SIGNING: ExtKeyUsage = ExtKeyUsage(12);
    pub const MICROSOFT_KERNEL_CODE_SIGNING: ExtKeyUsage = ExtKeyUsage(13);
}


#[derive(Debug)]
pub struct PssParameters {
   
    pub hash: AlgorithmIdentifier,
    pub mgf: AlgorithmIdentifier,
    pub salt_length: i32,
    pub trailer_field: Option<i32>, 
}



// Tag represents an ASN.1 identifier octet, consisting of a tag number
// (indicating a type) and class (such as context-specific or constructed).
//
// Methods in the cryptobyte package only support the low-tag-number form, i.e.
// a single identifier octet with bits 7-8 encoding the class and bits 1-6
// encoding the tag number.
//#[derive(Clone, Copy, Debug)]
//pub struct Tag(u8);

const CLASS_CONSTRUCTED: u8 = 0x20;
const CLASS_CONTEXT_SPECIFIC: u8 = 0x80;

//impl Tag {

    //pub fn constructed(self) -> Tag {
        //Tag(self.0 | CLASS_CONSTRUCTED)
    //}

    //pub fn context_specific(self) -> Tag {
        //Tag(self.0 | CLASS_CONTEXT_SPECIFIC)
    //}
//}

pub fn context_specific(tag: u8) -> u8 {
    tag | CLASS_CONTEXT_SPECIFIC
}
pub fn constructed(tag: u8) -> u8 {
    tag | CLASS_CONSTRUCTED
}

//pub const BOOLEAN: Tag = Tag(1);
//pub const INTEGER: Tag = Tag(2);
//pub const BIT_STRING: Tag = Tag(3);
//pub const OCTET_STRING: Tag = Tag(4);
//pub const NULL: Tag = Tag(5);
//pub const OBJECT_IDENTIFIER: Tag = Tag(6);
//pub const ENUM: Tag = Tag(10);
//pub const UTF8_STRING: Tag = Tag(12);
//pub const SEQUENCE: Tag = Tag(16 | CLASS_CONSTRUCTED);
//pub const SET: Tag = Tag(17 | CLASS_CONSTRUCTED);
//pub const PRINTABLE_STRING: Tag = Tag(19);
//pub const T61_STRING: Tag = Tag(20);
//pub const IA5_STRING: Tag = Tag(22);
//pub const UTC_TIME: Tag = Tag(23);
//pub const GENERALIZED_TIME: Tag = Tag(24);
//pub const GENERAL_STRING: Tag = Tag(27);

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
        let y = 7 - (i%8) as u8;
        (self.bytes[x]>>y) & 1
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
            a[i] = self.bytes[i-1] << (8 - shift);
            a[i] |= self.bytes[i] >> shift;
        }
        return a;
    }
}

#[derive(Debug, Clone)]
pub struct ASN1String(Vec<u8>); 

impl ASN1String {

    pub fn read_asn1_bitstring(&mut self, out: &mut BitString) -> bool {
        let mut bytes = ASN1String{ 0: Vec::new()};
        if !self.read_asn1(&mut bytes, BIT_STRING) || bytes.0.is_empty() || bytes.0.len()*8/8 != bytes.0.len() {
            return false;
        }

        let padding_bits = bytes.0[0];
        bytes.0 = bytes.0[1..].to_vec();

        if padding_bits > 7 ||
		bytes.0.is_empty() && padding_bits != 0 ||
		bytes.0.len() > 0 && (bytes.0[bytes.0.len()-1] & (((1<<padding_bits) as i8 - 1i8) != 0i8) as u8) != 0u8 {
            return false;
        }
        out.bit_length = (bytes.0.len()*8) as usize - (padding_bits as usize);
        out.bytes = bytes.0;
        return true
    }


    pub fn read_asn1(&mut self, out: &mut ASN1String, tag: u8) -> bool {
        let mut t = 0u8;
        if !self.read_any_asn1(out, &mut t) || t != tag {
            return false;
        }
        true
    }


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

    pub fn peek_asn1_tag(&self, tag: u8) -> bool {
        self.0.is_empty().then(|| false).unwrap_or(self.0[0] == tag)
    }

    pub fn skip_asn1(&mut self, tag: u8) -> bool {
        let mut unused = ASN1String(vec![]);
        self.read_asn1(&mut unused, tag)
    }

    pub fn read_optional_asn1(&mut self, out: &mut ASN1String, out_present: &mut bool, tag: u8) -> bool {
        let present = self.peek_asn1_tag(tag);
        //if let Some(ref mut p) = out_present {
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

    // pub fn read_optional_asn1_integer(&mut self, out: &mut dyn std::any::Any, tag: Tag, default_value: &dyn std::any::Any) -> bool {
    pub fn read_optional_asn1_integer(&mut self, out: &mut i64, tag: u8, default_value: i64) -> bool {
        let mut present = false;
        let mut i = ASN1String(vec![]);

        if !self.read_optional_asn1(&mut i, &mut present, tag) {
            return false;
        }

        if !present {
            //match out.downcast_mut::<i32>() {
                //Some(o) => *o = *default_value.downcast_ref::<i32>().unwrap(),
                //None => panic!("invalid type"),
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
        let mut bytes = ASN1String{0: Vec::new()};
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
        let mut bytes = ASN1String{ 0: Vec::new()};
        // if !self.read_asn1(&mut bytes, INTEGER) || !check_asn1_integer(bytes) || !asn1_signed(out, &bytes) {
        if !self.read_asn1(&mut bytes, INTEGER) || !check_asn1_integer(&bytes.0) || !asn1_signed(out, &bytes.0) {
            return false;
        }
        return true;
    }





    pub fn read_asn1_int64(&mut self, out: &mut i64) -> bool {
        let mut bytes = ASN1String{ 0: Vec::new()};
        if !self.read_asn1(&mut bytes, INTEGER) || !check_asn1_integer(&bytes.0) || !asn1_signed(out, &bytes.0) {
            return false;
        }
        true
    }

    pub fn read_asn1_uint64(&mut self, out: &mut u64) -> bool {
        let mut bytes = ASN1String{ 0: Vec::new()}; 
        if !self.read_asn1(&mut bytes, INTEGER) || !check_asn1_integer(&bytes.0) || !asn1_unsigned(out, &bytes.0) {
            return false;
        }
        true
    }

  
    pub fn read_asn1_big_int(&mut self) -> Option<BigInt> {
        let mut bytes = ASN1String{ 0: Vec::new()}; 
        if !self.read_asn1(&mut bytes, INTEGER) || !check_asn1_integer(&bytes.0) {
            return None;
        }

        let big_one = BigInt::from(1);//BigInt::one();

        if bytes.0[0] & 0x80 == 0x80 {
            let neg: Vec<u8> = bytes.0.iter().map(|&b| !b).collect();
            let mut out = BigInt::from_signed_bytes_be(&neg); // let mut out = BigInt::from_bytes_neg(&BigEndian, &neg);
            out += &big_one;
            Some(-out)
        } else {
            Some(BigInt::from_signed_bytes_be(&bytes.0)) // Some(BigInt::from_bytes_positive(&BigEndian, &bytes))
        }
    }

    pub fn read_asn1_object_identifier(&mut self, out: &mut Vec<i32>) -> bool {
        let mut bytes = ASN1String{ 0: Vec::new() }; 
        if !self.read_asn1(&mut bytes, OBJECT_IDENTIFIER) || bytes.0.is_empty() {
            return false;
        }

    
        let mut components = vec![0; bytes.0.len() + 1];

 
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
        let mut bytes = ASN1String{ 0: Vec::new()};  
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
                return false; 
            }
            if i == 5 {
                return false; 
            }
            if ret >= 1 << (31 - 7) {
                return false;
            }
            ret <<= 7;
            let b: u8 = self.read(1).unwrap()[0]; // Чтение одного байта

            if i == 0 && b == 0x80 {
                return false;
            }

            ret |= (b & 0x7f) as i32;
            if b & 0x80 == 0 {
                *out = ret;
                return true;
            }
        }
        false 
    }

    pub fn read_asn1_impl(&mut self, out: &mut ASN1String, out_tag: &mut u8, skip_header: bool) -> bool {
        if self.0.len() < 2 {
            return false;
        }

        let tag = self.0[0];
        let len_byte = self.0[1];

        if tag & 0x1f == 0x1f {

            return false;
        }

        //if let Some(out_t) = out_tag {
            // *out_t = Tag(tag);
        //}
        *out_tag = tag;

        // ITU-T X.690 section 8.1.3
        let (length, header_len) = if len_byte & 0x80 == 0 {

            (u32::from(len_byte) + 2, 2)
        } else {
            let len_len = len_byte & 0x7f;

            if len_len == 0 || len_len > 4 || self.0.len() < (2 + len_len as usize) {
                //println!("read_asn1_impl long len false");
                return false;
            }

            let mut len_bytes = ASN1String(self.0[2..2 + len_len as usize].to_vec());
            let mut len32 = 0u32;
            //println!("read_asn1_impl len_bytes before is : {:?}", &len_bytes);
            if !len_bytes.read_unsigned(&mut len32, len_len as usize) {
                return false;
            }
            //println!("read_asn1_impl len_bytes after is : {:?}", &len_bytes);
            //println!("read_asn1_impl len32 is : {:?}", &len32);

     
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
        //println!("read_asn1_impl length is : {:?}", &length);
        //println!("read_asn1_impl header_len is : {:?}", &header_len);

        //println!("self.0 before !self.read_bytes(out, length as usize) is : {:?}", &self.0);
        if length as usize > self.0.len() || !self.read_bytes(out, length as usize) {
            return false;
        }
        //println!("read_asn1_impl out is : {:?}", &out);
        if skip_header && !out.skip(header_len as usize) {
            panic!("cryptobyte: internal error");
        }
        //println!("out (not panic) is : {:?}", &out);
        //println!("self.0 (not panic) is : {:?}", &self.0);

        true
    }

    // fn read_asn1_utc_time(&mut self) -> Result<DateTime<Utc>, String> {
    fn read_asn1_utc_time(&mut self) -> Option<DateTime<Utc>> {
        let mut bytes = ASN1String{ 0: Vec::new()};
        if !self.read_asn1(&mut bytes, TagUTCTime) {
            return None;//return Err("Malformed UTCTime".to_string());
        }

        let t = String::from_utf8_lossy(&bytes.0).into_owned();
        let format_str = "%y%m%d%H%M%SZ"; 
        match Utc.datetime_from_str(&t, format_str) {
            Ok(res) => {
                
                Some(res)
            }
            Err(_) => None,//Err("Failed to parse UTCTime".to_string()),
        }
    }

    // fn read_asn1_generalized_time(&mut self) -> Result<DateTime<Utc>, String> {
    fn read_asn1_generalized_time(&mut self) -> Option<DateTime<Utc>> {
        let mut bytes = ASN1String{ 0: Vec::new()};
        if !self.read_asn1(&mut bytes, TagGeneralizedTime) {
            return None;//Err("Malformed GeneralizedTime".to_string());
        }

        let t = String::from_utf8_lossy(&bytes.0).into_owned();
        let format_str = "%Y%m%d%H%M%S%.fZ"; 
        match Utc.datetime_from_str(&t, format_str) {
            Ok(res) => Some(res),//Ok(res),
            Err(_) => None,//Err("Failed to parse GeneralizedTime".to_string()),
        }
    }

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

    fn read(&mut self, n: usize) -> Option<Vec<u8>> {
        if self.0.len() < n || n == 0 {
            return None;
        }

        let v = self.0[..n].to_vec(); 
        self.0.drain(..n); 
        Some(v)
    }


    pub fn read_bytes(&mut self, out: &mut ASN1String, length: usize) -> bool {
        if let Some(v) = self.read(length) {
            *out = ASN1String{0:v};
            true
        } else {
            false
        }
    }

    // Skip advances the String by n byte and reports whether it was successful.
    fn skip(&mut self, length: usize) -> bool {
        
        match self.read(length){
            Some(res) => true,
            None => false
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
    *out <<= 64 - (length as u8 * 8);
    *out >>= 64 - (length as u8 * 8);
    true
}

pub fn asn1_unsigned(out: &mut u64, n: &[u8]) -> bool {
    let length = n.len();
    if length > 9 || (length == 9 && n[0] != 0) {
        return false;
    }
    if n[0] & 0x80 != 0 {
        return false;
    }
    for &byte in n {
        *out <<= 8;
        *out |= byte as u64;
    }
    true
}

pub fn check_asn1_integer(bytes: &[u8]) -> bool {
    if bytes.is_empty() {
        return false;
    }
    if bytes.len() == 1 {
        return true;
    }
    if (bytes[0] == 0 && (bytes[1] & 0x80) == 0) || (bytes[0] == 0xff && (bytes[1] & 0x80) == 0x80) {
        return false;
    }
    return true;
}


#[derive(Debug, Clone)]
pub struct Extension {
    pub id: Vec<i32>, // //pub id: ObjectIdentifier,
    pub critical: Option<bool>, 
    pub value: Vec<u8>,
}


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
    pub names: Vec<AttributeTypeAndValue>,
    pub extra_names: Vec<AttributeTypeAndValue>,
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

        //println!("fill_from_rdn_sequence rdn starts");
        //println!("fill_from_rdn_sequence rdns.len() is : {:?}", &rdns.len());

        for rdn in rdns {
            //println!("fill_from_rdn_sequence rdn is : {:?}", &rdn);
            if rdn.is_empty() {
                continue;
            }

            for atv in rdn {
                //println!("fill_from_rdn_sequence atv is : {:?}", &atv);
                self.names.push(atv.clone()); 

                
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
pub struct Certificate {
    raw: Vec<u8>,                             // Complete ASN.1 DER content
    raw_tbs_certificate: Vec<u8>,             // Certificate part of raw ASN.1 DER content
    raw_subject_public_key_info: Vec<u8>,    // DER encoded SubjectPublicKeyInfo
    raw_subject: Vec<u8>,                     // DER encoded Subject
    raw_issuer: Vec<u8>,                      // DER encoded Issuer

    signature: Vec<u8>,
    signature_algorithm: SignatureAlgorithm,
    public_key_algorithm: PublicKeyAlgorithm,
    public_key: PublicKey,    // Using trait object for dynamic dispatch

    version: i64,
    pub serial_number: BigInt,     // serial_number: Option<BigInt>,          // Type for big integers
    issuer: Name,
    subject: Name,
    not_before: DateTime<Utc>,//i64,//SystemTime,                    // Using SystemTime for time representation
    not_after: DateTime<Utc>,//i64,//SystemTime,
    key_usage: KeyUsage,

    extensions: Vec<Extension>,          // Raw X.509 extensions
    extra_extensions: Vec<Extension>,    // Extensions to be copied raw into any marshaled certificates
    //unhandled_critical_extensions: Vec<asn1::ObjectIdentifier>, // List of extension IDs not fully processed

    ext_key_usage: Vec<ExtKeyUsage>,           // Sequence of extended key usages
    unknown_ext_key_usage: Vec<Vec<i32>>,//unknown_ext_key_usage: Vec<asn1::ObjectIdentifier>, // Encountered extended key usages unknown to this package

    basic_constraints_valid: bool,              // Indicates if BasicConstraints are valid
    is_ca: bool,
    max_path_len: i32,                         // MaxPathLen for BasicConstraints
    max_path_len_zero: bool,                   // Indicates if MaxPathLen is explicitly zero
    subject_key_id: Vec<u8>,
    authority_key_id: Vec<u8>,
    ocsp_server: Vec<String>,                   // Authority Information Access
    issuing_certificate_url: Vec<String>,

    dns_names: Vec<String>,                     // Subject Alternate Name values
    email_addresses: Vec<String>,
    ip_addresses: Vec<net::IpAddr>,            // IP addresses
    //uris: Vec<url::Url>,                       // Assuming url is a module with Url struct

    permitted_dns_domains_critical: bool,
    permitted_dns_domains: Vec<String>,
    excluded_dns_domains: Vec<String>,
    //permitted_ip_ranges: Vec<IpNet>, // Assuming IpNet is defined
    //excluded_ip_ranges: Vec<IpNet>,
    permitted_email_addresses: Vec<String>,
    excluded_email_addresses: Vec<String>,
    permitted_uri_domains: Vec<String>,
    excluded_uri_domains: Vec<String>,

    crl_distribution_points: Vec<String>,
    policy_identifiers: Vec<Vec<i32>>, // policy_identifiers: Vec<asn1::ObjectIdentifier>,
    //policies: Vec<OID>, // Assuming OID is defined
}

impl Certificate {
    //
    fn check_signature_from(&mut self, parent: &Certificate) -> bool { // fn (c *Certificate) Check_signature_from(parent *Certificate) error
        // RFC 5280, 4.2.1.9:
        // "If the basic constraints extension is not present in a version 3
        // certificate, or the extension is present but the cA boolean is not
        // asserted, then the certified public key MUST NOT be used to verify
        // certificate signatures."
        if parent.version == 3 && !parent.basic_constraints_valid ||
		parent.basic_constraints_valid && !parent.is_ca {
            return false; //return ConstraintViolationError{}
        }

        if parent.key_usage.0 != 0 && parent.key_usage.0 & KeyUsage::CERT_SIGN.0 == 0 {
            return false; // return ConstraintViolationError{}
        }

        if parent.public_key_algorithm == PublicKeyAlgorithm::UnknownPublicKeyAlgorithm {
            return false; //return ErrUnsupportedAlgorithm
        }

        // return checkSignature(c.SignatureAlgorithm, c.RawTBSCertificate, c.Signature, parent.PublicKey, false);
        return check_signature(&self.signature_algorithm, &self.raw_tbs_certificate, &self.signature, &parent.public_key, false);
    }
}

// CheckSignature verifies that signature is a valid signature over signed from
// c's public key.
//
// This is a low-level API that performs no validity checks on the certificate.
//
// [MD5WithRSA] signatures are rejected, while [SHA1WithRSA] and [ECDSAWithSHA1]
// signatures are currently accepted.
fn check_signature(algo: &SignatureAlgorithm, signed: &Vec<u8>, signature: &Vec<u8>, public_key: &PublicKey, allow_SHA1: bool) -> bool {

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

    //match hash_type.unwrap() {
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
                let pss_options = rsa::PSSOptions{salt_length: rsa::PSSSaltLengthEqualsHash, hash: 0 };
                return rsa::verify_pss(rsa_pub_key, 256, signed, signature, &pss_options);
            } else {
                // return rsa::verify_pkcs1v15(rsa_pub_key, hash_type, signed, signature);
                return rsa::verify_pkcs1v15(rsa_pub_key, 256, signed, signature);
            }
        },
        PublicKey::ECDSAPublicKey(ecdsa_pub_key) => {
            if pub_key_algo != PublicKeyAlgorithm::ECDSA {
                return false;
            }
            if !ecdsa_verify_asn1(ecdsa_pub_key, signed, signature) {
                return false; // "x509: ECDSA verification failure")
            }
            return true;
        },
        PublicKey::ED25519PublicKey(ed25519_pub_key) => {
            if pub_key_algo != PublicKeyAlgorithm::Ed25519 {
                return false;
            }
            if !ed25519::verify(ed25519_pub_key, signed, signature) {
                return false; // "x509: Ed25519 verification failure")
            }
            return true;
        },
        _ => return false,
    }
}

fn ecdsa_verify_asn1(pub_key: &ecdsa::PublicKey, signed: &[u8], signature: &Vec<u8>) -> bool {
    if let Some((r_bytes, s_bytes)) = parse_signature(&signature) {
        //
        let c = pub_key.curve.params();

        //let q = *c.point_from_affine();

        let r = BigInt::from_bytes_be(Sign::Plus, &r_bytes);
        let s = BigInt::from_bytes_be(Sign::Plus, &s_bytes);

        if !ecdsa::verify(&pub_key, &signed, &r, &s) {
            return false;
        }

        return true;
    }
    else {
        return false;
    }
}

fn parse_signature(sig: &Vec<u8>) -> Option<(Vec<u8>, Vec<u8>)> { // fn parse_signature(sig: &[u8]) -> (r, s []byte, err error) {
	let mut inner = ASN1String{ 0: Vec::new()}; //var inner cryptobyte.String

    let mut r: Vec<u8> = Vec::new();
    let mut s: Vec<u8> = Vec::new();
	let mut input = ASN1String{ 0: sig.clone()}; // input := cryptobyte.String(sig)
	if !input.read_asn1(&mut inner, SEQUENCE) ||   // if !input.ReadASN1(&inner, asn1.SEQUENCE) ||
		!input.0.is_empty() ||  //  !input.Empty() ||
		!inner.read_asn1_bytes(&mut r) || // !inner.ReadASN1Integer(&r) ||
		!inner.read_asn1_bytes(&mut s) || // !inner.ReadASN1Integer(&s) ||
		!inner.0.is_empty() { // !inner.Empty() {
		return None; //return nil, nil, errors.New("invalid ASN.1")
	}
	return Some((r, s)); //return r, s, nil
}

pub fn parse_certificate(der: &[u8]) -> Certificate { // fn parse_certificate(der: &[u8]) -> Result<Certificate, Box<dyn Error>> {
    //
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
        issuer: Name::default(), // Assuming a default implementation
        subject: Name::default(),
        not_before: DateTime::<Utc>::MIN_UTC, //0i64,//SystemTime::now(),
        not_after: DateTime::<Utc>::MAX_UTC,//SystemTime::now(),
        key_usage: KeyUsage::CERT_SIGN,
        extensions: Vec::new(),
        extra_extensions: Vec::new(),
        //unhandled_critical_extensions: Vec::new(),
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
        //uris: Vec::new(),
        permitted_dns_domains_critical: false,
        permitted_dns_domains: Vec::new(),
        excluded_dns_domains: Vec::new(),
        //permitted_ip_ranges: Vec::new(),
        //excluded_ip_ranges: Vec::new(),
        permitted_email_addresses: Vec::new(),
        excluded_email_addresses: Vec::new(),
        permitted_uri_domains: Vec::new(),
        excluded_uri_domains: Vec::new(),
        crl_distribution_points: vec![],
        policy_identifiers: vec![],
        //policies: vec![]
    };

   let mut input = ASN1String{ 0: der.to_vec()};
    // we read the SEQUENCE including length and tag bytes so that
	// we can populate Certificate.Raw, before unwrapping the
	// SEQUENCE so it can be operated on

    let mut input1 = input.clone();

    //println!("parseCertificate input before read_asn1_element is : {:?}", &input);

    if !input.read_asn1_element(&mut input1, SEQUENCE) {
        //return Err("x509: malformed certificate".into());
        panic!("x509: malformed certificate");
    }
    //println!("parseCertificate input after read_asn1_element is : {:?}", &input1);
    cert.raw = input1.0.clone();

    if !input1.read_asn1(&mut input, SEQUENCE) {
        //return Err("x509: malformed certificate".into());
        panic!("x509: malformed certificate");
    }
    //println!("parseCertificate input after read_asn1 is : {:?}", &input);

     let mut tbs = ASN1String{ 0: Vec::new()}; // Подходящий тип для tbs

    if !input.read_asn1_element(&mut tbs, SEQUENCE) {
        //return Err("x509: malformed tbs certificate".into());
        panic!("x509: malformed tbs certificate");
    }
    //println!("parseCertificate tbs after read_asn1 is : {:?}", &tbs);
    cert.raw_tbs_certificate = tbs.0.clone();

    let mut tbs1 = tbs.clone();
    if !tbs.read_asn1(&mut tbs1, SEQUENCE) {
        //return Err("x509: malformed version".into());
        panic!("x509: malformed tbs certificate");
    }

    //println!("parseCertificate tbs1 after read_asn1 is : {:?}", &tbs1);

    // if !tbs1.read_optional_asn1_integer(&mut cert.version, Tag(0).constructed().context_specific(), 0) {
    if !tbs1.read_optional_asn1_integer(&mut cert.version, context_specific(constructed(0u8)), 0) {
        //return Err("x509: malformed version".into());
        panic!("x509: malformed tbs certificate");
    }

    if cert.version < 0 {
        //return Err("x509: malformed version".into());
        panic!("x509: malformed version");
    }

    cert.version += 1;
    //println!("parseCertificate cert.version is : {:?}", &cert.version);
    if cert.version > 3 {
        //return Err("x509: invalid version".into());
        panic!("x509: invalid version");
    }

    //let mut serial = BigInt::default(); // Эквивалент создания нового большого числа
    //if !tbs1.read_asn1_big_int(&mut serial) { // if !tbs1.read_asn1_integer(&mut tbs, &serial) {
        //return Err("x509: malformed serial number".into());
        //panic!("x509: malformed serial number");
    //}
    match tbs1.read_asn1_big_int(){
        Some(serial) => cert.serial_number = serial,
        None => panic!("x509: malformed serial number"),
    }
    //cert.serial_number = serial;

    let mut sig_ai_seq = ASN1String{ 0: Vec::new()};
    if !tbs1.read_asn1(&mut sig_ai_seq, SEQUENCE) {
        //return Err("x509: malformed signature algorithm identifier".into());
        panic!("x509: malformed signature algorithm identifier");
    }

    // Before parsing the inner algorithm identifier, extract
	// the outer algorithm identifier and make sure that they
	// match.
    let mut outer_sig_ai_seq = ASN1String{ 0: Vec::new()};
    if !input.read_asn1(&mut outer_sig_ai_seq, SEQUENCE) {
        //return Err("x509: malformed algorithm identifier".into());
        panic!("x509: malformed algorithm identifier");
    }
    //println!("parseCertificate sig_ai_seq.0 is : {:?}", &sig_ai_seq.0);
    //println!("parseCertificate outer_sig_ai_seq.0 is : {:?}", &outer_sig_ai_seq.0);
    if outer_sig_ai_seq.0 != sig_ai_seq.0 { // if outer_sig_ai_seq != sig_ai_seq {
        //return Err("x509: inner and outer signature algorithm identifiers don't match".into());
        panic!("x509: inner and outer signature algorithm identifiers don't match");
    }

    let sig_ai = parse_ai(&mut sig_ai_seq); 
    cert.signature_algorithm = get_signature_algorithm_from_ai(sig_ai);
    //println!("parseCertificate cert.signature_algorithm is : {:?}", &cert.signature_algorithm);

    let mut issuer_seq = ASN1String{ 0: Vec::new()};
    if !tbs1.read_asn1_element(&mut issuer_seq, SEQUENCE) {
        //return Err("x509: malformed issuer".into());
        panic!("x509: malformed issuer");
    }
    //println!("parseCertificate issuer_seq.0 is : {:?}", &issuer_seq.0);
    cert.raw_issuer = issuer_seq.0.clone();
    let issuer_rdns = parse_name(&mut issuer_seq);
    cert.issuer.fill_from_rdn_sequence(&issuer_rdns);

    let mut validity = ASN1String{ 0: Vec::new()};
	if !tbs1.read_asn1(&mut validity, SEQUENCE) {
		panic!("x509: malformed validity");
	}

    (cert.not_before, cert.not_after) = parse_validity(& mut validity).unwrap();

    //println!("parseCertificate cert.not_before is : {:?}", &cert.not_before);
    //println!("parseCertificate cert.not_after is : {:?}", &cert.not_after);
	//cert.not_before, cert.not_after, err = parseValidity(validity);
	//if err != nil {
		//return nil, err
	//}

    let mut subject_seq = ASN1String{ 0: Vec::new()};
	if !tbs1.read_asn1_element(&mut subject_seq, SEQUENCE) {
		panic!("x509: malformed issuer");
	}
	cert.raw_subject = subject_seq.0.clone();
    //println!("parseCertificate cert.raw_subject is : {:?}", &cert.raw_subject);
    let subject_rdns = parse_name(&mut subject_seq);

	cert.subject.fill_from_rdn_sequence(&subject_rdns);

    let mut spki = ASN1String{ 0: Vec::new()};
	if !tbs1.read_asn1_element(&mut spki, SEQUENCE) {
		panic!("x509: malformed spki");
	}
	cert.raw_subject_public_key_info = spki.0.clone();
    let mut spki1 = ASN1String{ 0: Vec::new()};
	if !spki.read_asn1(&mut spki1, SEQUENCE) {
		panic!("x509: malformed spki"); //return nil, errors.New("x509: malformed spki")
	}
	let mut pk_ai_seq = ASN1String{ 0: Vec::new()};
	if !spki1.read_asn1(&mut pk_ai_seq, SEQUENCE) {
		panic!("x509: malformed public key algorithm identifier");
	}

    let pk_ai = parse_ai(&mut pk_ai_seq);//pkAI, err := parseAI(pkAISeq)
    //if err != nil {
		//return nil, err
	//}
	cert.public_key_algorithm = get_public_key_algorithm_from_oid(&pk_ai.algorithm);
	let mut spk = BitString{bytes: Vec::new(), bit_length: 0};//var spk asn1.BitString
	if !spki1.read_asn1_bitstring(&mut spk) {
		panic!("x509: malformed subjectPublicKey");
	}
	if cert.public_key_algorithm != PublicKeyAlgorithm::UnknownPublicKeyAlgorithm {

        let public_key_info = PublicKeyInfo{algorithm: pk_ai, publicKey: spk};
        cert.public_key = parse_public_key(&public_key_info);
		//cert.PublicKey, err = parsePublicKey(&publicKeyInfo{
			//Algorithm: pkAI,
			//PublicKey: spk,
		//})
		//if err != nil {
			//return nil, err
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
            let mut extensions = ASN1String{ 0: Vec::new()};
			let mut present = false;
            if !tbs1.read_optional_asn1(&mut extensions, &mut present, context_specific(constructed(3u8))) {
                panic!("x509: malformed extensions");
            }

            if present {
                /*let mut seen_exts: HashMap<String, bool> = HashMap::new(); // seenExts := make(map[string]bool)
                let mut extensions1 = ASN1String{ 0: Vec::new()};
                if !extensions.read_asn1(&mut extensions1, SEQUENCE) {
                    panic!("x509: malformed extensions");
                }

                while !extensions.0.is_empty() {
                    let mut extension = ASN1String{ 0: Vec::new()};
                    if !extensions.read_asn1(&mut extension, SEQUENCE) {
                        panic!("x509: malformed extension");
                    }
                    let ext = parse_extension(&mut extension); //ext, err := parseExtension(extension)
					//if err != nil {
						//return nil, err
					//}

                    let oid_str = to_oid_string(&ext.id); //oidStr := ext.Id.String()
                    if *seen_exts.get(&oid_str).unwrap() {
                        panic!("x509: certificate contains duplicate extensions");
                    }
					//if seenExts[oidStr] {
						//return nil, errors.New("x509: certificate contains duplicate extensions")
					//}
					seen_exts.insert(oid_str, true); //seenExts[oidStr] = true
                    cert.extensions.push(ext);
                }*/


            }
        }
    }

    /*let mut signature = BitString{ bytes: vec![], bit_length: 0 } ;
	if !input.read_asn1_bitstring(&mut signature) {
		panic!("x509: malformed signature");
	}
    cert.signature = signature.right_align();*/

    cert

}

fn to_oid_string(data: &Vec<i32>) -> String{
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


//struct RDN_sequence {

//};// []RelativeDistinguishedNameSET

//struct RelativeDistinguishedNameSET (
    //Vec<AttributeTypeAndValue>,
//);


// parseName parses a DER encoded Name as defined in RFC 5280. We may
// want to export this function in the future for use in crypto/tls.
pub fn parse_name(raw: &mut ASN1String) -> Vec<Vec<AttributeTypeAndValue>> { // pub fn parse_name(raw: &mut ASN1String) -> RDN_sequence {
    //
    let mut der = ASN1String{ 0: Vec::new()};
    if !raw.read_asn1(&mut der, SEQUENCE) {
        panic!("x509: invalid RDNSequence");
    }

    let mut rdn_seq: Vec<Vec<AttributeTypeAndValue>> = Vec::new(); // let mut rdn_seq: RDNSequence;
    if !raw.0.is_empty(){
        let mut rdn_set: Vec<AttributeTypeAndValue> = Vec::new(); // let mut rdn_set: RelativeDistinguishedNameSET;
        let mut set = ASN1String{ 0: Vec::new()};
        if !raw.read_asn1(&mut set, SET) {
			//return nil, errors.New("x509: invalid RDNSequence")
            panic!("x509: invalid RDNSequence");
		}

        while !set.0.is_empty() {
            let mut atav = ASN1String{ 0: Vec::new()};
            if !set.read_asn1(&mut atav, SEQUENCE) {
                panic!("x509: invalid RDNSequence: invalid attribute");
            }

            let mut attr: AttributeTypeAndValue = AttributeTypeAndValue{atype: Vec::new(), value: String::new()};
            if !atav.read_asn1_object_identifier(&mut attr.atype) {
                panic!("x509: invalid RDNSequence: invalid attribute type");
            }

            let mut raw_value = ASN1String{ 0: Vec::new()};
			let mut value_tag = 0u8;
            if !atav.read_any_asn1(&mut raw_value, &mut value_tag) {
                panic!("x509: invalid RDNSequence: invalid attribute value");
            }

            //(attr.Value, err) = parse_asn1_string(valueTag, rawValue);
            //if err != nil {
				//panic!("x509: invalid RDNSequence: invalid attribute value: %s", err);
			//}
            attr.value = parse_asn1_string(value_tag, &raw_value.0).expect("x509: invalid RDNSequence: invalid attribute value: %s");

            rdn_set.push(attr);

        }
        rdn_seq.push(rdn_set);
    }

    return rdn_seq;
}

// pub fn parse_asn1_string(tag: u8, value: &[u8]) -> Result<String, ASN1Error> {
pub fn parse_asn1_string(tag: u8, value: &[u8]) -> Option<String> {
    match tag {
        T61_STRING => Some(String::from_utf8_lossy(value).to_string()),//Ok(String::from_utf8_lossy(value).to_string()),
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
            if decoded.len() >= 2 && decoded[decoded.len() - 1] == 0 && decoded[decoded.len() - 2] == 0 {
                decoded.pop();
                decoded.pop();
            }

            // Convert u16 vector to String
            match String::from_utf16(&decoded) {
                Ok(string) => return Some(string),
                Err(_) => return None,
            }

            //return Some(String::from_utf16(&decoded)?); // return Ok(String::from_utf16(&decoded).map_err(|_| ASN1Error::InvalidBMPString)?);
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
        _ => None,//Err(ASN1Error::UnsupportedStringType),
    }
}

// Helper functions to validate characters and UTF-8
fn is_printable(b: u8) -> bool {
    (b'b' >= b'a' && b <= b'z') || (b >= b'A' && b <= b'Z') ||
    (b >= b'0' && b <= b'9') || (b >= b'\'' && b <= b')') ||
    (b >= b'+' && b <= b'/') || b == b' ' ||
    b == b':' || b == b'=' || b == b'?' ||
    b == b'*' || b == b'&'
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

//const MAX_ASCII: char = 'u{007F}';

fn is_ia5_string(s: &str) -> bool {
    for r in s.chars() {
        // Per RFC5280 "IA5String is limited to the set of ASCII characters"
        if r > 127 as char { // if r > MAX_ASCII {
            return false;
        }
    }
    return true;
}



// ASN.1 types

#[derive(Debug, Clone)]
pub struct RawValue {
    class: i32,       // ASN.1 class (e.g. Universal, Application, Context-specific, Private)
    tag: i32,         // ASN.1 tag
    is_compound: bool, // Indicates if the RawValue is a compound type
    bytes: Vec<u8>,   // Undecoded bytes of the ASN.1 object
    full_bytes: Vec<u8>, // Complete bytes including the tag and length
}

#[derive(Debug, Clone)]
struct AlgorithmIdentifier {
    algorithm: Vec<i32>,
    parameters: Option<RawValue>
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

    if !der.read_asn1_object_identifier(&mut algorithm){
        panic!("x509: malformed OID");
    }

    if der.0.is_empty(){
        return AlgorithmIdentifier { algorithm, parameters: Some(parameters) };
    }

    let mut params = ASN1String{ 0: Vec::new()};
    let mut tag = 0u8;

    if !der.read_any_asn1_element(&mut params, &mut tag) {
        panic!("x509: malformed parameters");
	}

    parameters.tag = tag as i32;
    parameters.full_bytes = params.0.to_vec();

    return AlgorithmIdentifier { algorithm, parameters: Some(parameters) }
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
    //let params: PssParameters;
    //if unmarshal(ai.parameters.full_bytes, &params){
        //return SignatureAlgorithm::UnknownSignatureAlgorithm;
    //}

    //let mgf1_hash_func: AlgorithmIdentifier;
    //if unmarshal(params.mgf.parameters.full_bytes, &mgf1_hash_func){
        //return SignatureAlgorithm::UnknownSignatureAlgorithm;
    //}

    // PSS is greatly overburdened with options. This code forces them into
	// three buckets by requiring that the MGF1 hash function always match the
	// message hash function (as recommended in RFC 3447, Section 8.1), that the
	// salt length matches the hash length, and that the trailer field has the
	// default value.

    //if (!params.hash.parameters.unwrap().full_bytes.is_empty() && !params.hash.parameters.unwrap().full_bytes.to_vec()==NullBytes.to_vec() ) ||
		//params.mgf.algorithm != oidMGF1 ||
		//mgf1_hash_func.algorithm != params.hash.algorithm ||
        //( mgf1_hash_func.parameters.unwrap().full_bytes.len() != 0 && mgf1_hash_func.parameters.unwrap().full_bytes != NullBytes ) ||
		//params.TrailerField != 1 {
		//return SignatureAlgorithm::UnknownSignatureAlgorithm;
	//}

    //match (params.hash.algorithm, params.salt_length) {
        //(OID_SHA256, 32) => SignatureAlgorithm::SHA256WithRSAPSS,
        //(OID_SHA384, 48) => SignatureAlgorithm::SHA384WithRSAPSS,
        //(OID_SHA512, 64) => SignatureAlgorithm::SHA512WithRSAPSS,
        //_ => SignatureAlgorithm::UnknownSignatureAlgorithm,
    //}

    return SignatureAlgorithm::UnknownSignatureAlgorithm;
}

// RFC 3279, 2.3 Public Key Algorithms
//
//	pkcs-1 OBJECT IDENTIFIER ::== { iso(1) member-body(2) us(840)
//		rsadsi(113549) pkcs(1) 1 }
//
// rsaEncryption OBJECT IDENTIFIER ::== { pkcs1-1 1 }
//
//	id-dsa OBJECT IDENTIFIER ::== { iso(1) member-body(2) us(840)
//		x9-57(10040) x9cm(4) 1 }
const	OID_PUBLIC_KEY_RSA: [i32; 7] = [1, 2, 840, 113549, 1, 1, 1];
const	OID_PUBLIC_KEY_DSA: [i32; 6] = [1, 2, 840, 10040, 4, 1];
// RFC 5480, 2.1.1 Unrestricted Algorithm Identifier and Parameters
//
//	id-ecPublicKey OBJECT IDENTIFIER ::= {
//		iso(1) member-body(2) us(840) ansi-X9-62(10045) keyType(2) 1 }
const	OID_PUBLIC_KEY_ECDSA: [i32; 6] = [1, 2, 840, 10045, 2, 1];
// RFC 8410, Section 3
//
//	id-X25519    OBJECT IDENTIFIER ::= { 1 3 101 110 }
//	id-Ed25519   OBJECT IDENTIFIER ::= { 1 3 101 112 }
const	OID_PUBLIC_KEY_X25519: [i32; 4]  = [1, 3, 101, 110];
const	OID_PUBLIC_KEY_ED25519: [i32; 4] = [1, 3, 101, 112];

fn get_public_key_algorithm_from_oid(oid: &Vec<i32>) -> PublicKeyAlgorithm {
    let oid_slice = oid.as_slice();
    match oid_slice {
        val if val==OID_PUBLIC_KEY_RSA.as_slice() => PublicKeyAlgorithm::RSA,
        val if val==OID_PUBLIC_KEY_DSA.as_slice() => PublicKeyAlgorithm::DSA,
        val if val==OID_PUBLIC_KEY_ECDSA.as_slice() => PublicKeyAlgorithm::ECDSA,
        val if val==OID_PUBLIC_KEY_ED25519.as_slice() => PublicKeyAlgorithm::Ed25519,
        _ => PublicKeyAlgorithm::UnknownPublicKeyAlgorithm,
    }
}
/*
pub fn unmarshal(b: &[u8], val: &mut dyn std::any::Any) -> Option<&[u8]> {
    unmarshal_with_params(b, val, "")
}

pub fn unmarshal_with_params(b: &[u8], val: &mut dyn std::any::Any, params: &str) -> Option<&[u8]> {
    //let v = val.downcast_mut::<&mut dyn std::any::Any>()
        //.ok_or(InvalidUnmarshalError { typ: std::any::TypeId::of::<&mut dyn std::any::Any>() })?;

    let (offset, rest) = parse_field(val, b, 0, parse_field_parameters(params))?;
    Some(&b[offset..])
}

fn parse_field(v: &mut dyn std::any::Any, bytes: &[u8], init_offset: usize, params: FieldParameters) -> Option<(usize, &[u8])> {
    let mut offset = init_offset;

    if offset == bytes.len() {
        if !set_default_value(v, &params) {
            //return Err(SyntaxError { message: "sequence truncated" });
        }
        return Some((offset, bytes));//return Ok((offset, bytes));
    }

    // Handle the ANY type.
    if let Some(iface) = v.downcast_ref::<&dyn std::any::Any>() {
        let (t, new_offset, err) = parse_tag_and_length(bytes, offset)?;
        if err.is_some() {
            return None; // return Err(err.unwrap());
        }

        // Check length and parse according to tag
        if is_invalid_length(new_offset, t.length, bytes.len()) {
            return None; //return Err(SyntaxError { message: "data truncated" });
        }

        let inner_bytes = &bytes[new_offset..new_offset + t.length];
        let result: Box<dyn std::any::Any> = match t.tag {
            Tag::PrintableString => parse_printable_string(inner_bytes)?,
            Tag::NumericString => parse_numeric_string(inner_bytes)?,
            Tag::IA5String => parse_ia5_string(inner_bytes)?,
            Tag::T61String => parse_t61_string(inner_bytes)?,
            Tag::UTF8String => parse_utf8_string(inner_bytes)?,
            Tag::Integer => parse_int64(inner_bytes)?,
            Tag::BitString => parse_bit_string(inner_bytes)?,
            Tag::OID => parse_object_identifier(inner_bytes)?,
            Tag::UTCTime => parse_utc_time(inner_bytes)?,
            Tag::GeneralizedTime => parse_generalized_time(inner_bytes)?,
            Tag::OctetString => inner_bytes.to_vec().into_boxed_slice(),
            Tag::BMPString => parse_bmp_string(inner_bytes)?,
            _ => Box::new(()), // Unknown type handling
        };

        // Update the reference
        mem::swap(v, &mut result);
        offset += t.length;
        return Some((offset, bytes));//Ok((offset, bytes));
    }

    // Normal case; parse according to the ASN.1 rules
    let (t, new_offset, err) = parse_tag_and_length(bytes, offset)?;
    if err.is_some() {
        return None;//return Err(err.unwrap());
    }

    Some((offset, bytes))
}*/

// fn parse_validity(der: &mut ASN1String) -> Option<(u64, u64)> {
fn parse_validity(der: &mut ASN1String) -> Option<(DateTime<Utc>, DateTime<Utc>)> {
	let not_before = parse_time(der);
	if not_before.is_none() {
		return None
	}
	let not_after = parse_time(der);
	if not_after.is_none() {
		return None;
	}

	return Some((not_before.unwrap(), not_after.unwrap()));
}

fn parse_time(der: &mut ASN1String) -> Option<DateTime<Utc> > {
    if der.peek_asn1_tag(TagUTCTime) {
        der.read_asn1_utc_time()
    } else if der.peek_asn1_tag(TagGeneralizedTime) {
        der.read_asn1_generalized_time()
    } else {
        None//Err("Unsupported time format".to_string())
    }
}

fn parse_extension(der: &mut ASN1String) -> Extension { // fn parse_extension(der: &mut ASN1String) -> (pkix.Extension, error) {
	let mut ext: Extension = Extension{ id: vec![], critical: None, value: vec![] };
	if !der.read_asn1_object_identifier(&mut ext.id) {
		panic!("x509: malformed extension OID field");
	}
    let mut ext_critical = false;
	if der.peek_asn1_tag(BOOLEAN) {
		if !der.read_asn1_boolean(&mut ext_critical) {
			panic!("x509: malformed extension critical field");
		}
        ext.critical = Some(ext_critical);
	}
    let mut val = ASN1String{ 0: Vec::new()};
	if !der.read_asn1(&mut val, OCTET_STRING) {
		panic!("x509: malformed extension value field");
	}
	ext.value = val.0;
	return ext;
}

struct PublicKeyInfo {
	//raw:       Vec<u8>,
	algorithm: AlgorithmIdentifier,
	publicKey: BitString,
}

#[derive(Debug, PartialEq)]
enum PublicKey {
    RsaPublicKey(rsa::PublicKey),
    ECDSAPublicKey(ecdsa::PublicKey),
    ED25519PublicKey(ed25519::PublicKey),
    X25519PublicKey,
    DsaPublicKey,
    UnknownPubicKey
}

// RFC 5480, 2.1.1.1. Named Curve
//
//	secp224r1 OBJECT IDENTIFIER ::= {
//	  iso(1) identified-organization(3) certicom(132) curve(0) 33 }
//
//	secp256r1 OBJECT IDENTIFIER ::= {
//	  iso(1) member-body(2) us(840) ansi-X9-62(10045) curves(3)
//	  prime(1) 7 }
//
//	secp384r1 OBJECT IDENTIFIER ::= {
//	  iso(1) identified-organization(3) certicom(132) curve(0) 34 }
//
//	secp521r1 OBJECT IDENTIFIER ::= {
//	  iso(1) identified-organization(3) certicom(132) curve(0) 35 }
//
// NB: secp256r1 is equivalent to prime256v1
const OID_NAMED_CURVE_P224: [i32;5] = [1, 3, 132, 0, 33];
const OID_NAMED_CURVE_P256: [i32;7] = [1, 2, 840, 10045, 3, 1, 7];
const OID_NAMED_CURVE_P384: [i32;5] = [1, 3, 132, 0, 34];
const OID_NAMED_CURVE_P521: [i32;5] = [1, 3, 132, 0, 35];


fn named_curve_from_oid(oid: &Vec<i32>) -> Option<ecdsa::Curve> {

    match oid.as_slice() {
        val if val==OID_NAMED_CURVE_P224.as_slice() => Some(ecdsa::p224()),
        val if val==OID_NAMED_CURVE_P256.as_slice() => Some(ecdsa::p256()),
        val if val==OID_NAMED_CURVE_P384.as_slice() => Some(ecdsa::p384()),
        val if val==OID_NAMED_CURVE_P521.as_slice() => Some(ecdsa::p521()),
        _ => None,
    }
	//switch {
	//case oid.Equal(oidNamedCurveP224):
		//return elliptic.P224()
	//case oid.Equal(oidNamedCurveP256):
		//return elliptic.P256()
	//case oid.Equal(oidNamedCurveP384):
		//return elliptic.P384()
	//case oid.Equal(oidNamedCurveP521):
		//return elliptic.P521()
	//}
	//return nil
}

fn parse_public_key(key_data: &PublicKeyInfo) -> PublicKey {
    let oid = &key_data.algorithm.algorithm;
	let params = key_data.algorithm.parameters.clone().unwrap().clone();
    let mut der = ASN1String{ 0: key_data.publicKey.right_align()}; // der = cryptobyte.String(key_data.publicKey.right_align() );
    match oid.as_slice() {
        val if val==OID_PUBLIC_KEY_RSA.as_slice() => {
            // RSA public keys must have a NULL in the parameters.
            // See RFC 3279, Section 2.3.1.
            if params.full_bytes != NullBytes.to_vec() {
                panic!("x509: RSA key missing NULL parameters");
            }

            let mut p = rsa::PublicKey{n: BigInt::from(0), e: 0i64};
            let mut der1 = ASN1String{ 0: Vec::new()};
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

            return PublicKey::RsaPublicKey((p));
        },
        val if val==OID_PUBLIC_KEY_ECDSA.as_slice() => {
            let mut params_der = ASN1String{ 0: params.full_bytes.clone()};// cryptobyte.String(params.FullBytes)
            let mut named_curve_oid: Vec<i32> = Vec::new();
            if !params_der.read_asn1_object_identifier(&mut named_curve_oid) {
                panic!("x509: invalid ECDSA parameters");
            }
            //let named_curve = named_curve_from_oid(&named_curve_oid);
            match named_curve_from_oid(&named_curve_oid) {
                Some(named_curve) => {
                    //
                    let (x, y) : (BigInt, BigInt) = ecdsa::unmarshal(&named_curve, &der.0);
                    //if x == nil {
                        //panic!("x509: failed to unmarshal elliptic curve point");
                    //}
                    PublicKey::ECDSAPublicKey(ecdsa::PublicKey{curve: named_curve, x, y})
                },
                None => panic!("x509: unsupported elliptic curve"),
            }
        },
        val if val==OID_PUBLIC_KEY_ED25519.as_slice() => {
            // RFC 8410, Section 3
            // > For all of the OIDs, the parameters MUST be absent.
            if !params.full_bytes.is_empty() {
                panic!("x509: Ed25519 key encoded with illegal parameters");
            }
            if der.0.len() != ed25519::PUBLIC_KEY_SIZE {
                panic!("x509: wrong Ed25519 public key size");
            }

            PublicKey::ED25519PublicKey(ed25519::PublicKey(der.0))
        },
        val if val==OID_PUBLIC_KEY_X25519.as_slice() => {
            //
            PublicKey::X25519PublicKey
        },
        val if val==OID_PUBLIC_KEY_DSA.as_slice() => {
            //
            PublicKey::DsaPublicKey
        },
        _ => panic!("x509: unknown public key algorithm"),
    }
}

pub fn check_certs(current_time: i64, check_sum: &[u8], certs_chain: &[u8], signature: &[u8]) -> Option<PublicKey> {
    // extract
    // divide input string into three slices
    //println!("check_certs certs_chain is : {:?}", &certs_chain);

    let len_of_certs_chain = (certs_chain[0] as usize)*65536 + (certs_chain[1] as usize)*256 + (certs_chain[2] as usize);
    //println!("check_certs len_of_certs_chain is : {:?}", len_of_certs_chain);
    //println!("check_certs certs_chain.len() is : {:?}", certs_chain.len());

    if len_of_certs_chain+1 != certs_chain.len() {
        return None;
    }

    let len_of_leaf_cert = (certs_chain[3] as usize)*65536 + (certs_chain[4] as usize)*256 + (certs_chain[5] as usize);
    //println!("check_certs len_of_leaf_cert is : {:?}", len_of_leaf_cert);

    let leaf_cert_slice = &certs_chain[6..len_of_leaf_cert+6];
    //println!("check_certs leaf_cert_slice is : {:?}", leaf_cert_slice);

    let mut leaf_cert = parse_certificate(leaf_cert_slice); // leafCert, err := x509.ParseCertificate(leafCertSlice)
    //if leaf_cert.not_after.Before(time.Now()) || leaf_cert.not_before.After(time.Now()) {
        //false
    //}


    let start_index = len_of_leaf_cert + 8;
    let len_of_internal_cert = (certs_chain[start_index] as usize)*65536 + (certs_chain[start_index+1] as usize)*256 + (certs_chain[start_index+2] as usize);
    //println!("check_certs len_of_internal_cert is : {:?}", len_of_internal_cert);

    let internal_cert_slice = &certs_chain[start_index + 3..start_index + len_of_internal_cert + 3];
    //println!("check_certs internal_cert_slice is : {:?}", internal_cert_slice);

    let mut internal_cert = parse_certificate(internal_cert_slice); // internalCert, err := x509.ParseCertificate(internalCertSlice)

    //println!("check_certs internal_cert.serial_number is : {:?}", internal_cert.serial_number.to_string());


    //if internalCert.NotAfter.Before(time.Now()) || internalCert.NotBefore.After(time.Now()) {
        //return false
    //}
    let start_index = start_index + 3 + len_of_internal_cert + 2;

    let len_of_root_cert = (certs_chain[start_index] as usize)*65536 + (certs_chain[start_index+1] as usize)*256 + (certs_chain[start_index+2] as usize);
    //println!("check_certs len_of_root_cert is : {:?}", len_of_root_cert);
    let root_cert_slice = &certs_chain[start_index + 3..start_index + len_of_root_cert+3];
    //println!("check_certs root_cert_slice is : {:?}", root_cert_slice);

    let root_cert = parse_certificate(root_cert_slice);

    /*if !leaf_cert.check_signature_from(&internal_cert){
        return None;
    }
    //rootCert, err := x509.ParseCertificate(rootCertSlice)
    // if err != nil {
    // fmt.Printf("ParseCertificate (rootCertSlice) err is : %v\n", err.Error())
    // return false
    // }

    if !internal_cert.check_signature_from(&root_cert){
        return None;
    }

    match  leaf_cert.public_key_algorithm.to_string() {
        val if val=="RSA".to_string() => {
            //
            // pubkey, ok := leafCert.PublicKey.(*rsa.PublicKey)
            if let PublicKey::RsaPublicKey(pub_key) = leaf_cert.public_key {
                //
                if !rsa::verify_pkcs1v15(&pub_key,256, check_sum, signature) {
                    return None;
                }
            } else {
                return None; //ErrCertificateTypeMismatch
            }

        },
        val if val == "ECDSA".to_string() => {
            //
            if let PublicKey::ECDSAPublicKey(pub_key) = leaf_cert.public_key {
                //
                let len_of_r = signature[3] as usize;//     lenOfr := signature[3]
                //println!("len_of_r is : {:?}", len_of_r);
                let r_data = &signature[4..4+len_of_r];//     rData := signature[4:4+lenOfr]
                let len_of_s = signature[4 + len_of_r + 1] as usize;
                //println!("len_of_s is : {:?}", len_of_s);
                let s_data = &signature[4 + len_of_r + 2..4 + len_of_r + 2 + len_of_s];
                let r = BigInt::from_bytes_be(Sign::Plus, r_data); //     r := new(big.Int).SetBytes(rData)
                let s = BigInt::from_bytes_be(Sign::Plus, s_data); //     s := new(big.Int).SetBytes(sData)
                if !ecdsa::verify(&pub_key, check_sum, &r, &s) {
                    return None;
                }

            } else {
                return None;  //ErrCertificateTypeMismatch
            }

        },
        _ => panic!("Unknown signature algorithm"),
    }*/


    return Some(root_cert.public_key);
}

pub fn check_certs_with_fixed_root(current_time: i64, check_sum: &[u8], certs_chain: &[u8], signature: &[u8], root_cert_bytes: &[u8]) -> bool {
    //
    let check_certs_result = check_certs(current_time, check_sum, certs_chain, signature);
    if check_certs_result.is_none() {
        return false;
    }

    let proposed_root_cert = parse_certificate(&root_cert_bytes);

    if (proposed_root_cert.public_key==check_certs_result.unwrap()){
        return true;
    }
    return false;

}

pub fn check_certs_with_known_roots(current_time: i64, check_sum: &[u8], certs_chain: &[u8], signature: &[u8]) -> Option<BigInt> {
    //
    let check_certs_result = check_certs(current_time, check_sum, certs_chain, signature);
    if check_certs_result.is_none() {
        return None;
    }
    let root_public_key_from_server = check_certs_result.unwrap();

    for root_cert_bytes in ROOTS_CERTS {
        let root_cert = parse_certificate(&hex::decode(root_cert_bytes).unwrap()); 
        if (root_cert.public_key==root_public_key_from_server){
            return Some(root_cert.serial_number);
        }   
    }

    return None;
}

#[test]
fn test_parsing_leaf_cert(){

    // lenOfRootCert is : 1507
    let cert_bytes = [48, 130, 5, 223, 48, 130, 4, 199, 160, 3, 2, 1, 2, 2, 16, 29, 52, 231, 130, 196, 125, 97, 31, 9, 217, 200, 245, 205,
        198, 186, 21, 48, 13, 6, 9, 42, 134, 72, 134, 247, 13, 1, 1, 11, 5, 0, 48, 59, 49, 11, 48, 9, 6, 3, 85, 4, 6, 19, 2, 85, 83, 49, 30, 48,
        28, 6, 3, 85, 4, 10, 19, 21, 71, 111, 111, 103, 108, 101, 32, 84, 114, 117, 115, 116, 32, 83, 101, 114, 118, 105, 99, 101, 115, 49, 12,
        48, 10, 6, 3, 85, 4, 3, 19, 3, 87, 82, 50, 48, 30, 23, 13, 50, 53, 48, 54, 48, 50, 48, 56, 51, 54, 51, 55, 90, 23, 13, 50, 53, 48, 56, 50,
        53, 48, 56, 51, 54, 51, 54, 90, 48, 34, 49, 32, 48, 30, 6, 3, 85, 4, 3, 19, 23, 117, 112, 108, 111, 97, 100, 46, 118, 105, 100, 101, 111,
        46, 103, 111, 111, 103, 108, 101, 46, 99, 111, 109, 48, 89, 48, 19, 6, 7, 42, 134, 72, 206, 61, 2, 1, 6, 8, 42, 134, 72, 206, 61, 3, 1, 7,
        3, 66, 0, 4, 248, 83, 162, 122, 174, 15, 224, 170, 61, 240, 158, 237, 156, 11, 182, 110, 127, 239, 209, 74, 120, 97, 236, 65, 243, 24, 27,
        36, 129, 74, 199, 81, 187, 0, 174, 91, 146, 116, 246, 216, 103, 159, 198, 205, 143, 254, 100, 152, 123, 224, 77, 174, 122, 94, 243, 213,
        158, 167, 195, 80, 59, 62, 51, 87, 163, 130, 3, 193, 48, 130, 3, 189, 48, 14, 6, 3, 85, 29, 15, 1, 1, 255, 4, 4, 3, 2, 7, 128, 48, 19, 6,
        3, 85, 29, 37, 4, 12, 48, 10, 6, 8, 43, 6, 1, 5, 5, 7, 3, 1, 48, 12, 6, 3, 85, 29, 19, 1, 1, 255, 4, 2, 48, 0, 48, 29, 6, 3, 85, 29, 14,
        4, 22, 4, 20, 116, 64, 113, 233, 144, 151, 116, 8, 12, 37, 102, 162, 200, 138, 133, 220, 67, 196, 64, 182, 48, 31, 6, 3, 85, 29, 35, 4,
        24, 48, 22, 128, 20, 222, 27, 30, 237, 121, 21, 212, 62, 55, 36, 195, 33, 187, 236, 52, 57, 109, 66, 178, 48, 48, 88, 6, 8, 43, 6, 1, 5,
        5, 7, 1, 1, 4, 76, 48, 74, 48, 33, 6, 8, 43, 6, 1, 5, 5, 7, 48, 1, 134, 21, 104, 116, 116, 112, 58, 47, 47, 111, 46, 112, 107, 105, 46,
        103, 111, 111, 103, 47, 119, 114, 50, 48, 37, 6, 8, 43, 6, 1, 5, 5, 7, 48, 2, 134, 25, 104, 116, 116, 112, 58, 47, 47, 105, 46, 112, 107,
        105, 46, 103, 111, 111, 103, 47, 119, 114, 50, 46, 99, 114, 116, 48, 130, 1, 152, 6, 3, 85, 29, 17, 4, 130, 1, 143, 48, 130, 1, 139, 130,
        23, 117, 112, 108, 111, 97, 100, 46, 118, 105, 100, 101, 111, 46, 103, 111, 111, 103, 108, 101, 46, 99, 111, 109, 130, 20, 42, 46, 99, 108,
        105, 101, 110, 116, 115, 46, 103, 111, 111, 103, 108, 101, 46, 99, 111, 109, 130, 17, 42, 46, 100, 111, 99, 115, 46, 103, 111, 111, 103,
        108, 101, 46, 99, 111, 109, 130, 18, 42, 46, 100, 114, 105, 118, 101, 46, 103, 111, 111, 103, 108, 101, 46, 99, 111, 109, 130, 19, 42, 46,
        103, 100, 97, 116, 97, 46, 121, 111, 117, 116, 117, 98, 101, 46, 99, 111, 109, 130, 16, 42, 46, 103, 111, 111, 103, 108, 101, 97, 112, 105,
        115, 46, 99, 111, 109, 130, 19, 42, 46, 112, 104, 111, 116, 111, 115, 46, 103, 111, 111, 103, 108, 101, 46, 99, 111, 109, 130, 23, 42, 46,
        121, 111, 117, 116, 117, 98, 101, 45, 51, 114, 100, 45, 112, 97, 114, 116, 121, 46, 99, 111, 109, 130, 17, 117, 112, 108, 111, 97, 100, 46,
        103, 111, 111, 103, 108, 101, 46, 99, 111, 109, 130, 19, 42, 46, 117, 112, 108, 111, 97, 100, 46, 103, 111, 111, 103, 108, 101, 46, 99, 111,
        109, 130, 18, 117, 112, 108, 111, 97, 100, 46, 121, 111, 117, 116, 117, 98, 101, 46, 99, 111, 109, 130, 20, 42, 46, 117, 112, 108, 111, 97,
        100, 46, 121, 111, 117, 116, 117, 98, 101, 46, 99, 111, 109, 130, 31, 117, 112, 108, 111, 97, 100, 115, 46, 115, 116, 97, 103, 101, 46, 103,
        100, 97, 116, 97, 46, 121, 111, 117, 116, 117, 98, 101, 46, 99, 111, 109, 130, 21, 98, 103, 45, 99, 97, 108, 108, 45, 100, 111, 110, 97, 116,
        105, 111, 110, 46, 103, 111, 111, 103, 130, 27, 98, 103, 45, 99, 97, 108, 108, 45, 100, 111, 110, 97, 116, 105, 111, 110, 45, 97, 108, 112,
        104, 97, 46, 103, 111, 111, 103, 130, 28, 98, 103, 45, 99, 97, 108, 108, 45, 100, 111, 110, 97, 116, 105, 111, 110, 45, 99, 97, 110, 97, 114,
        121, 46, 103, 111, 111, 103, 130, 25, 98, 103, 45, 99, 97, 108, 108, 45, 100, 111, 110, 97, 116, 105, 111, 110, 45, 100, 101, 118, 46, 103,
        111, 111, 103, 48, 19, 6, 3, 85, 29, 32, 4, 12, 48, 10, 48, 8, 6, 6, 103, 129, 12, 1, 2, 1, 48, 54, 6, 3, 85, 29, 31, 4, 47, 48, 45, 48, 43,
        160, 41, 160, 39, 134, 37, 104, 116, 116, 112, 58, 47, 47, 99, 46, 112, 107, 105, 46, 103, 111, 111, 103, 47, 119, 114, 50, 47, 71, 83, 121,
        84, 49, 78, 52, 80, 66, 114, 103, 46, 99, 114, 108, 48, 130, 1, 3, 6, 10, 43, 6, 1, 4, 1, 214, 121, 2, 4, 2, 4, 129, 244, 4, 129, 241, 0, 239,
        0, 118, 0, 221, 220, 202, 52, 149, 215, 225, 22, 5, 231, 149, 50, 250, 199, 159, 248, 61, 28, 80, 223, 219, 0, 58, 20, 18, 118, 10, 44, 172,
        187, 200, 42, 0, 0, 1, 151, 48, 0, 13, 18, 0, 0, 4, 3, 0, 71, 48, 69, 2, 32, 25, 141, 105, 240, 199, 112, 242, 232, 208, 105, 216, 166, 198,
        180, 16, 170, 174, 162, 33, 83, 140, 69, 155, 81, 15, 88, 241, 55, 220, 71, 137, 236, 2, 33, 0, 147, 92, 188, 175, 12, 195, 251, 3, 208, 216,
        50, 217, 185, 244, 35, 53, 6, 13, 224, 137, 152, 132, 209, 23, 99, 2, 209, 204, 104, 39, 35, 72, 0, 117, 0, 125, 89, 30, 18, 225, 120, 42, 123,
        28, 97, 103, 124, 94, 253, 248, 208, 135, 92, 20, 160, 78, 149, 158, 185, 3, 47, 217, 14, 140, 46, 121, 184, 0, 0, 1, 151, 48, 0, 16, 208, 0,
        0, 4, 3, 0, 70, 48, 68, 2, 32, 86, 158, 142, 171, 240, 161, 218, 234, 10, 163, 215, 135, 65, 120, 205, 47, 143, 227, 51, 230, 77, 112, 48, 136,
        100, 237, 136, 188, 205, 109, 90, 253, 2, 32, 64, 92, 213, 9, 82, 102, 85, 218, 69, 87, 96, 98, 122, 235, 105, 165, 218, 55, 81, 91, 232, 94,
        251, 46, 21, 135, 147, 229, 162, 244, 208, 58, 48, 13, 6, 9, 42, 134, 72, 134, 247, 13, 1, 1, 11, 5, 0, 3, 130, 1, 1, 0, 78, 0, 2, 76, 206, 91,
        50, 221, 129, 45, 231, 23, 12, 75, 193, 40, 29, 199, 204, 67, 202, 92, 143, 89, 184, 88, 188, 239, 123, 103, 60, 53, 75, 157, 164, 31, 47, 122,
        150, 158, 92, 128, 110, 61, 15, 220, 150, 132, 113, 219, 92, 116, 154, 234, 58, 38, 210, 108, 74, 177, 255, 177, 152, 61, 36, 192, 169, 80, 82,
        91, 174, 92, 201, 41, 89, 43, 103, 144, 144, 141, 191, 186, 29, 253, 77, 180, 173, 116, 43, 88, 139, 127, 214, 211, 240, 132, 208, 45, 252,
        194, 174, 69, 3, 83, 195, 72, 47, 229, 107, 147, 192, 243, 235, 38, 50, 49, 111, 223, 217, 56, 39, 245, 192, 31, 88, 16, 138, 91, 44, 252, 121,
        222, 6, 179, 251, 251, 214, 111, 49, 175, 194, 228, 200, 23, 9, 38, 95, 236, 34, 197, 88, 45, 64, 131, 118, 113, 226, 28, 131, 166, 230, 217, 43,
        255, 107, 115, 114, 16, 82, 65, 121, 193, 19, 2, 155, 87, 24, 247, 182, 59, 114, 227, 162, 2, 132, 33, 187, 102, 100, 232, 115, 252, 113, 200,
        214, 124, 228, 193, 234, 91, 243, 88, 161, 61, 122, 11, 148, 173, 161, 5, 175, 21, 238, 25, 239, 41, 152, 162, 7, 9, 184, 180, 18, 16, 182, 105,
        24, 130, 170, 97, 140, 247, 142, 68, 65, 138, 182, 235, 17, 241, 151, 219, 137, 204, 70, 183, 131, 65, 70, 186, 107, 234, 22, 172, 179, 255];
    let certificate = parse_certificate(&cert_bytes);

   /*println!("the certificate.version is : {:?}", &certificate.version);
    println!("the certificate.serial_number is : {:?}", &certificate.serial_number.to_string());
    println!("the certificate.issuer.names is : {:?}", &certificate.issuer.names);
    println!("the certificate.key_usage is : {:?}", &certificate.key_usage);
    println!("the certificate.public_key is : {:?}", &certificate.public_key);*/

    let certificate_version: i64 = 3;
    assert_eq!(certificate.version, certificate_version);

    let cert_serial_number = BigInt::from_str("38822306911496578035668995664819698197").unwrap();
    assert_eq!(certificate.serial_number, cert_serial_number);
    
    assert_eq!(certificate.public_key_algorithm, PublicKeyAlgorithm::ECDSA);
    
    /*let etalon_pk = PublicKey::ECDSAPublicKey(ecdsa::PublicKey{
        curve: Curve {
            p: Default::default(),
            n: Default::default(),
            b: Default::default(),
            gx: Default::default(),
            gy: Default::default(),
            bit_size: 0,
            name: "".to_string()
        },
        x: BigInt::from_str("112321356145379214094818301380289946094074558649736816888277127582448981034833"),
        y: BigInt::from_str("84583706057714013775544235912841985893594821113486056562093352390134289281879")
    });*/
}

/*
#[test]
fn test_parsing_internal_cert(){
    // len is 1295
    let cert_bytes = [48, 130, 5, 11, 48, 130, 2, 243, 160, 3, 2, 1, 2, 2, 16, 127, 240, 5, 160, 124, 76, 222, 209, 0, 173, 157, 102, 165,
        16, 123, 152, 48, 13, 6, 9, 42, 134, 72, 134, 247, 13, 1, 1, 11, 5, 0, 48, 71, 49, 11, 48, 9, 6, 3, 85, 4, 6, 19, 2, 85, 83, 49, 34, 48, 32,
        6, 3, 85, 4, 10, 19, 25, 71, 111, 111, 103, 108, 101, 32, 84, 114, 117, 115, 116, 32, 83, 101, 114, 118, 105, 99, 101, 115, 32, 76, 76, 67,
        49, 20, 48, 18, 6, 3, 85, 4, 3, 19, 11, 71, 84, 83, 32, 82, 111, 111, 116, 32, 82, 49, 48, 30, 23, 13, 50, 51, 49, 50, 49, 51, 48, 57, 48, 48, 48, 48, 90, 23, 13, 50, 57, 48, 50, 50, 48, 49, 52, 48, 48, 48, 48, 90, 48, 59, 49, 11, 48, 9, 6, 3, 85, 4, 6, 19, 2, 85, 83, 49, 30, 48, 28, 6, 3, 85, 4, 10, 19, 21, 71, 111, 111, 103, 108, 101, 32, 84, 114, 117, 115, 116, 32, 83, 101, 114, 118, 105, 99, 101, 115, 49, 12, 48, 10, 6, 3, 85, 4, 3, 19, 3, 87, 82, 50, 48, 130, 1, 34, 48, 13, 6, 9, 42, 134, 72, 134, 247, 13, 1, 1, 1, 5, 0, 3, 130, 1, 15, 0, 48, 130, 1, 10, 2, 130, 1, 1, 0, 169, 255, 156, 127, 69, 30, 112, 168, 83, 159, 202, 217, 229, 13, 222, 70, 87, 87, 125, 188, 143, 154, 90, 172, 70, 241, 132, 154, 187, 145, 219, 201, 251, 47, 1, 251, 146, 9, 0, 22, 94, 160, 28, 248, 193, 171, 249, 120, 47, 74, 204, 216, 133, 162, 216, 89, 60, 14, 211, 24, 251, 177, 245, 36, 13, 38, 238, 182, 91, 100, 118, 124, 20, 199, 47, 122, 206, 168, 76, 183, 244, 217, 8, 252, 223, 135, 35, 53, 32, 168, 226, 105, 226, 140, 78, 63, 177, 89, 250, 96, 162, 30, 179, 201, 32, 83, 25, 130, 202, 54, 83, 109, 96, 77, 233, 0, 145, 252, 118, 141, 92, 8, 15, 10, 194, 220, 241, 115, 107, 197, 19, 110, 10, 79, 122, 194, 242, 2, 28, 46, 180, 99, 131, 218, 49, 246, 45, 117, 48, 178, 251, 171, 194, 110, 219, 169, 192, 14, 185, 249, 103, 212, 195, 37, 87, 116, 235, 5, 180, 233, 142, 181, 222, 40, 205, 204, 122, 20, 228, 113, 3, 203, 77, 97, 46, 97, 87, 197, 25, 169, 11, 152, 132, 26, 232, 121, 41, 217, 178, 141, 47, 255, 87, 106, 102, 224, 206, 171, 149, 168, 41, 150, 99, 112, 18, 103, 30, 58, 225, 219, 176, 33, 113, 215, 124, 158, 253, 170, 23, 110, 254, 43, 251, 56, 23, 20, 209, 102, 167, 175, 154, 181, 112, 204, 200, 99, 129, 58, 140, 192, 42, 169, 118, 55, 206, 227, 2, 3, 1, 0, 1, 163, 129, 254, 48, 129, 251, 48, 14, 6, 3, 85, 29, 15, 1, 1, 255, 4, 4, 3, 2, 1, 134, 48, 29, 6, 3, 85, 29, 37, 4, 22, 48, 20, 6, 8, 43, 6, 1, 5, 5, 7, 3, 1, 6, 8, 43, 6, 1, 5, 5, 7, 3, 2, 48, 18, 6, 3, 85, 29, 19, 1, 1, 255, 4, 8, 48, 6, 1, 1, 255, 2, 1, 0, 48, 29, 6, 3, 85, 29, 14, 4, 22, 4, 20, 222, 27, 30, 237, 121, 21, 212, 62, 55, 36, 195, 33, 187, 236, 52, 57, 109, 66, 178, 48, 48, 31, 6, 3, 85, 29, 35, 4, 24, 48, 22, 128, 20, 228, 175, 43, 38, 113, 26, 43, 72, 39, 133, 47, 82, 102, 44, 239, 240, 137, 19, 113, 62, 48, 52, 6, 8, 43, 6, 1, 5, 5, 7, 1, 1, 4, 40, 48, 38, 48, 36, 6, 8, 43, 6, 1, 5, 5, 7, 48, 2, 134, 24, 104, 116, 116, 112, 58, 47, 47, 105, 46, 112, 107, 105, 46, 103, 111, 111, 103, 47, 114, 49, 46, 99, 114, 116, 48, 43, 6, 3, 85, 29, 31, 4, 36, 48, 34, 48, 32, 160, 30, 160, 28, 134, 26, 104, 116, 116, 112, 58, 47, 47, 99, 46, 112, 107, 105, 46, 103, 111, 111, 103, 47, 114, 47, 114, 49, 46, 99, 114, 108, 48, 19, 6, 3, 85, 29, 32, 4, 12, 48, 10, 48, 8, 6, 6, 103, 129, 12, 1, 2, 1, 48, 13, 6, 9, 42, 134, 72, 134, 247, 13, 1, 1, 11, 5, 0, 3, 130, 2, 1, 0, 69, 117, 139, 229, 31, 59, 68, 19, 150, 26, 171, 88, 241, 53, 201, 111, 61, 210, 208, 51, 74, 134, 51, 186, 87, 81, 79, 238, 196, 52, 218, 22, 18, 76, 191, 19, 159, 13, 212, 84, 233, 72, 121, 192, 48, 60, 148, 37, 242, 26, 244, 186, 50, 148, 182, 51, 114, 11, 133, 238, 9, 17, 37, 52, 148, 225, 111, 66, 219, 130, 155, 123, 127, 42, 154, 169, 255, 127, 169, 210, 222, 74, 32, 203, 179, 251, 3, 3, 184, 248, 7, 5, 218, 89, 146, 47, 24, 70, 152, 206, 175, 114, 190, 36, 38, 177, 30, 0, 77, 189, 8, 173, 147, 65, 68, 10, 187, 199, 213, 1, 133, 191, 147, 87, 227, 223, 116, 18, 83, 14, 17, 37, 211, 155, 220, 222, 203, 39, 110, 179, 194, 185, 51, 98, 57, 194, 224, 53, 225, 91, 167, 9, 46, 25, 203, 145, 42, 118, 92, 241, 223, 202, 35, 132, 64, 165, 111, 255, 154, 65, 224, 181, 239, 50, 209, 133, 174, 175, 37, 9, 240, 98, 197, 110, 194, 200, 110, 50, 253, 184, 218, 226, 206, 74, 145, 74, 243, 133, 85, 78, 177, 117, 214, 72, 51, 47, 111, 132, 217, 18, 92, 159, 212, 113, 152, 99, 37, 141, 105, 92, 10, 107, 125, 242, 65, 189, 232, 187, 143, 228, 34, 215, 157, 101, 69, 232, 76, 10, 135, 218, 233, 96, 102, 136, 14, 31, 199, 225, 78, 86, 197, 118, 255, 180, 122, 87, 105, 242, 2, 34, 9, 38, 65, 29, 218, 116, 162, 229, 41, 243, 196, 154, 229, 93, 214, 170, 122, 253, 225, 183, 43, 102, 56, 251, 232, 41, 102, 186, 239, 160, 19, 47, 248, 115, 126, 240, 218, 64, 17, 28, 93, 221, 143, 166, 252, 190, 219, 190, 86, 248, 50, 156, 31, 65, 65, 109, 126, 182, 197, 235, 198, 139, 54, 183, 23, 140, 157, 207, 25, 122, 52, 159, 33, 147, 196, 126, 116, 53, 210, 170, 253, 76, 109, 20, 245, 201, 176, 121, 91, 73, 60, 243, 191, 23, 72, 232, 239, 154, 38, 19, 12, 135, 242, 115, 214, 156, 197, 82, 107, 99, 247, 50, 144, 120, 169, 107, 235, 94, 214, 147, 161, 191, 188, 24, 61, 139, 89, 246, 138, 198, 5, 94, 82, 24, 226, 102, 224, 218, 193, 220, 173, 90, 37, 170, 244, 69, 252, 241, 11, 120, 164, 175, 176, 242, 115, 164, 48, 168, 52, 193, 83, 127, 66, 150, 229, 72, 65, 235, 144, 70, 12, 6, 220, 203, 146, 198, 94, 243, 68, 68, 67, 70, 41, 70, 160, 166, 252, 185, 142, 57, 39, 57, 177, 90, 226, 177, 173, 252, 19, 255, 142, 252, 38, 225, 212, 254, 132, 241, 80, 90, 142, 151, 107, 45, 42, 121, 251, 64, 100, 234, 243, 61, 189, 91, 225, 160, 4, 176, 151, 72, 28, 66, 245, 234, 90, 28, 205, 38, 200, 81, 255, 20, 153, 103, 137, 114, 95, 29, 236, 173, 90, 221];
    let certificate = parse_certificate(&cert_bytes);

    println!("the certificate.version is : {:?}", &certificate.version);
    println!("the certificate.serial_number is : {:?}", &certificate.serial_number);

    // lenOfRootCert is : 1382
    let certificate_version: i64 = 2;
    assert_eq!(certificate.version, certificate_version);
    let cert_serial_number = BigInt(170058220837755766831192027518741805976);
    assert_eq!(certificate.serial_number, cert_serial_number);
}


#[test]
fn test_parsing_root_cert_from_inet(){

    let cert_bytes = [48, 130, 5, 98, 48, 130, 4, 74, 160, 3, 2, 1, 2, 2, 16, 119, 189, 13, 108, 219, 54, 249, 26, 234, 33, 15, 196, 240,
        88, 211, 13, 48, 13, 6, 9, 42, 134, 72, 134, 247, 13, 1, 1, 11, 5, 0, 48, 87, 49, 11, 48, 9, 6, 3, 85, 4, 6, 19, 2, 66, 69, 49, 25, 48, 23,
        6, 3, 85, 4, 10, 19, 16, 71, 108, 111, 98, 97, 108, 83, 105, 103, 110, 32, 110, 118, 45, 115, 97, 49, 16, 48, 14, 6, 3, 85, 4, 11, 19, 7,
        82, 111, 111, 116, 32, 67, 65, 49, 27, 48, 25, 6, 3, 85, 4, 3, 19, 18, 71, 108, 111, 98, 97, 108, 83, 105, 103, 110, 32, 82, 111, 111, 116,
        32, 67, 65, 48, 30, 23, 13, 50, 48, 48, 54, 49, 57, 48, 48, 48, 48, 52, 50, 90, 23, 13, 50, 56, 48, 49, 50, 56, 48, 48, 48, 48, 52, 50, 90,
        48, 71, 49, 11, 48, 9, 6, 3, 85, 4, 6, 19, 2, 85, 83, 49, 34, 48, 32, 6, 3, 85, 4, 10, 19, 25, 71, 111, 111, 103, 108, 101, 32, 84, 114,
        117, 115, 116, 32, 83, 101, 114, 118, 105, 99, 101, 115, 32, 76, 76, 67, 49, 20, 48, 18, 6, 3, 85, 4, 3, 19, 11, 71, 84, 83, 32, 82, 111,
        111, 116, 32, 82, 49, 48, 130, 2, 34, 48, 13, 6, 9, 42, 134, 72, 134, 247, 13, 1, 1, 1, 5, 0, 3, 130, 2, 15, 0, 48, 130, 2, 10, 2, 130, 2,
        1, 0, 182, 17, 2, 139, 30, 227, 161, 119, 155, 59, 220, 191, 148, 62, 183, 149, 167, 64, 60, 161, 253, 130, 249, 125, 50, 6, 130, 113, 246,
        246, 140, 127, 251, 232, 219, 188, 106, 46, 151, 151, 163, 140, 75, 249, 43, 246, 177, 249, 206, 132, 29, 177, 249, 197, 151, 222, 239, 185,
        242, 163, 233, 188, 18, 137, 94, 167, 170, 82, 171, 248, 35, 39, 203, 164, 177, 156, 99, 219, 215, 153, 126, 240, 10, 94, 235, 104, 166, 244,
        198, 90, 71, 13, 77, 16, 51, 227, 78, 177, 19, 163, 200, 24, 108, 75, 236, 252, 9, 144, 223, 157, 100, 41, 37, 35, 7, 161, 180, 210, 61, 46,
        96, 224, 207, 210, 9, 135, 187, 205, 72, 240, 77, 194, 194, 122, 136, 138, 187, 186, 207, 89, 25, 214, 175, 143, 176, 7, 176, 158, 49, 241,
        130, 193, 192, 223, 46, 166, 109, 108, 25, 14, 181, 216, 126, 38, 26, 69, 3, 61, 176, 121, 164, 148, 40, 173, 15, 127, 38, 229, 168, 8, 254,
        150, 232, 60, 104, 148, 83, 238, 131, 58, 136, 43, 21, 150, 9, 178, 224, 122, 140, 46, 117, 214, 156, 235, 167, 86, 100, 143, 150, 79, 104,
        174, 61, 151, 194, 132, 143, 192, 188, 64, 192, 11, 92, 189, 246, 135, 179, 53, 108, 172, 24, 80, 127, 132, 224, 76, 205, 146, 211, 32, 233,
        51, 188, 82, 153, 175, 50, 181, 41, 179, 37, 42, 180, 72, 249, 114, 225, 202, 100, 247, 230, 130, 16, 141, 232, 157, 194, 138, 136, 250, 56,
        102, 138, 252, 99, 249, 1, 249, 120, 253, 123, 92, 119, 250, 118, 135, 250, 236, 223, 177, 14, 121, 149, 87, 180, 189, 38, 239, 214, 1, 209,
        235, 22, 10, 187, 142, 11, 181, 197, 197, 138, 85, 171, 211, 172, 234, 145, 75, 41, 204, 25, 164, 50, 37, 78, 42, 241, 101, 68, 208, 2, 206,
        170, 206, 73, 180, 234, 159, 124, 131, 176, 64, 123, 231, 67, 171, 167, 108, 163, 143, 125, 137, 129, 250, 76, 165, 255, 213, 142, 195, 206,
        75, 224, 181, 216, 179, 142, 69, 207, 118, 192, 237, 64, 43, 253, 83, 15, 176, 167, 213, 59, 13, 177, 138, 162, 3, 222, 49, 173, 204, 119,
        234, 111, 123, 62, 214, 223, 145, 34, 18, 230, 190, 250, 216, 50, 252, 16, 99, 20, 81, 114, 222, 93, 214, 22, 147, 189, 41, 104, 51, 239,
        58, 102, 236, 7, 138, 38, 223, 19, 215, 87, 101, 120, 39, 222, 94, 73, 20, 0, 162, 0, 127, 154, 168, 33, 182, 169, 177, 149, 176, 165, 185,
        13, 22, 17, 218, 199, 108, 72, 60, 64, 224, 126, 13, 90, 205, 86, 60, 209, 151, 5, 185, 203, 75, 237, 57, 75, 156, 196, 63, 210, 85, 19,
        110, 36, 176, 214, 113, 250, 244, 193, 186, 204, 237, 27, 245, 254, 129, 65, 216, 0, 152, 61, 58, 200, 174, 122, 152, 55, 24, 5, 149, 2,
        3, 1, 0, 1, 163, 130, 1, 56, 48, 130, 1, 52, 48, 14, 6, 3, 85, 29, 15, 1, 1, 255, 4, 4, 3, 2, 1, 134, 48, 15, 6, 3, 85, 29, 19, 1, 1, 255,
        4, 5, 48, 3, 1, 1, 255, 48, 29, 6, 3, 85, 29, 14, 4, 22, 4, 20, 228, 175, 43, 38, 113, 26, 43, 72, 39, 133, 47, 82, 102, 44, 239, 240, 137,
        19, 113, 62, 48, 31, 6, 3, 85, 29, 35, 4, 24, 48, 22, 128, 20, 96, 123, 102, 26, 69, 13, 151, 202, 137, 80, 47, 125, 4, 205, 52, 168, 255,
        252, 253, 75, 48, 96, 6, 8, 43, 6, 1, 5, 5, 7, 1, 1, 4, 84, 48, 82, 48, 37, 6, 8, 43, 6, 1, 5, 5, 7, 48, 1, 134, 25, 104, 116, 116, 112,
        58, 47, 47, 111, 99, 115, 112, 46, 112, 107, 105, 46, 103, 111, 111, 103, 47, 103, 115, 114, 49, 48, 41, 6, 8, 43, 6, 1, 5, 5, 7, 48, 2,
        134, 29, 104, 116, 116, 112, 58, 47, 47, 112, 107, 105, 46, 103, 111, 111, 103, 47, 103, 115, 114, 49, 47, 103, 115, 114, 49, 46, 99, 114,
        116, 48, 50, 6, 3, 85, 29, 31, 4, 43, 48, 41, 48, 39, 160, 37, 160, 35, 134, 33, 104, 116, 116, 112, 58, 47, 47, 99, 114, 108, 46, 112,
        107, 105, 46, 103, 111, 111, 103, 47, 103, 115, 114, 49, 47, 103, 115, 114, 49, 46, 99, 114, 108, 48, 59, 6, 3, 85, 29, 32, 4, 52, 48, 50,
        48, 8, 6, 6, 103, 129, 12, 1, 2, 1, 48, 8, 6, 6, 103, 129, 12, 1, 2, 2, 48, 13, 6, 11, 43, 6, 1, 4, 1, 214, 121, 2, 5, 3, 2, 48, 13, 6, 11,
        43, 6, 1, 4, 1, 214, 121, 2, 5, 3, 3, 48, 13, 6, 9, 42, 134, 72, 134, 247, 13, 1, 1, 11, 5, 0, 3, 130, 1, 1, 0, 52, 164, 30, 177, 40, 163,
        208, 180, 118, 23, 166, 49, 122, 33, 233, 209, 82, 62, 200, 219, 116, 22, 65, 136, 184, 61, 53, 29, 237, 228, 255, 147, 225, 92, 95, 171,
        187, 234, 124, 207, 219, 228, 13, 209, 139, 87, 242, 38, 111, 91, 190, 23, 70, 104, 148, 55, 111, 107, 122, 200, 192, 24, 55, 250, 37, 81,
        172, 236, 104, 191, 178, 200, 73, 253, 90, 154, 202, 1, 35, 172, 132, 128, 43, 2, 140, 153, 151, 235, 73, 106, 140, 117, 215, 199, 222,
        178, 201, 151, 159, 88, 72, 87, 14, 53, 161, 228, 26, 214, 253, 111, 131, 129, 111, 239, 140, 207, 151, 175, 192, 133, 42, 240, 245, 78,
        105, 9, 145, 45, 225, 104, 184, 193, 43, 115, 233, 212, 217, 252, 34, 192, 55, 31, 11, 102, 29, 73, 237, 2, 85, 143, 103, 225, 50, 215,
        211, 38, 191, 112, 227, 61, 244, 103, 109, 61, 124, 229, 52, 136, 227, 50, 250, 167, 110, 6, 106, 111, 189, 139, 145, 238, 22, 75, 232,
        59, 169, 179, 55, 231, 195, 68, 164, 126, 216, 108, 215, 199, 70, 245, 146, 155, 231, 213, 33, 190, 102, 146, 25, 148, 85, 108, 212, 41,
        178, 13, 193, 102, 91, 226, 119, 73, 72, 40, 237, 157, 215, 26, 51, 114, 83, 179, 130, 53, 207, 98, 139, 201, 36, 139, 165, 183, 57, 12,
        187, 126, 42, 65, 191, 82, 207, 252, 162, 150, 182, 194, 130, 63];
    let certificate = parse_certificate(&cert_bytes);

    println!("the certificate.version is : {:?}", &certificate.version);
    println!("the certificate.serial_number is : {:?}", &certificate.serial_number.to_string());
    println!("the certificate.issuer.names is : {:?}", &certificate.issuer.names);
    println!("the certificate.public_key is : {:?}", &certificate.public_key);

    // lenOfRootCert is : 1382
    let certificate_version: i64 = 3;
    assert_eq!(certificate.version, certificate_version);
    let cert_serial_number = BigInt::from_str("159159747900478145820483398898491642637").unwrap();
    assert_eq!(certificate.serial_number, cert_serial_number);
    //let cert_issuer = Name{}; // CN=GlobalSign Root CA,OU=Root CA,O=GlobalSign nv-sa,C=BE
    //certificate.issuer = CN=GlobalSign Root CA,OU=Root CA,O=GlobalSign nv-sa,C=BE

    assert_eq!(certificate.public_key_algorithm, PublicKeyAlgorithm::RSA); //rootCert.PublicKeyAlgorithm is : RSA
    // rootCert.PublicKey is : &{742766292573789461138430713106656498577482106105452767343211753017973550878861638590047246174848574634573720584492944669558785810905825702100325794803983120697401526210439826606874730300903862093323398754125584892080731234772626570955922576399434033022944334623029747454371697865218999618129768679013891932765999545116374192173968985738129135224425889467654431372779943313524100225335793262665132039441111162352797240438393795570253671786791600672076401253164614309929080014895216439462173458352253266568535919120175826866378039177020829725517356783703110010084715777806343235841345264684364598708732655710904078855499605447884872767583987312177520332134164321746982952420498393591583416464199126272682424674947720461866762624768163777784559646117979893432692133818266724658906066075396922419161138847526583266030290937955148683298741803605463007526904924936746018546134099068479370078440023459839544052468222048449819089106832452146002755336956394669648596035188293917750838002531358091511944112847917218550963597247358780879029417872466325821996717925086546502702016501643824750668459565101211439428003662613442032518886622942136328590823063627643918273848803884791311375697313014431195473178892344923166262358299334827234064598421 65537}

    let rsa_public_n = BigInt::from_str("742766292573789461138430713106656498577482106105452767343211753017973550878861638590047246174848574634573720584492944669558785810905825702100325794803983120697401526210439826606874730300903862093323398754125584892080731234772626570955922576399434033022944334623029747454371697865218999618129768679013891932765999545116374192173968985738129135224425889467654431372779943313524100225335793262665132039441111162352797240438393795570253671786791600672076401253164614309929080014895216439462173458352253266568535919120175826866378039177020829725517356783703110010084715777806343235841345264684364598708732655710904078855499605447884872767583987312177520332134164321746982952420498393591583416464199126272682424674947720461866762624768163777784559646117979893432692133818266724658906066075396922419161138847526583266030290937955148683298741803605463007526904924936746018546134099068479370078440023459839544052468222048449819089106832452146002755336956394669648596035188293917750838002531358091511944112847917218550963597247358780879029417872466325821996717925086546502702016501643824750668459565101211439428003662613442032518886622942136328590823063627643918273848803884791311375697313014431195473178892344923166262358299334827234064598421").unwrap();
    let etalon_pk = PublicKey::RsaPublicKey(rsa::PublicKey{n: rsa_public_n, e: 65537});
    println!("etalon public_key is : {:?}", &etalon_pk);
    assert_eq!(etalon_pk, certificate.public_key);
}*/

//#[test]
//fn test_parsing_root_cert_from_alina(){
    //parseCertificate cert.Version is : 2
    // parseCertificate cert.SerialNumber is : 146587176229350439916519468929765261721
    // etalonRootCert.Issuer is : CN=GTS Root R4,O=Google Trust Services LLC,C=US
    // etalonRootCert.PublicKeyAlgorithm is : ECDSA
    // etalonRootCert.PublicKey is : &{0x490850 37471137007972414188180584817005857701594611622436499579709175026540926241259029249891351931980308501383755467997302 9183005163897397881300021216631269301828759039006067320487338515525388614843808427732645382476107253937965649436042}
    // etalonRootCert.Issuer is : CN=GTS Root R4,O=Google Trust Services LLC,C=US
//}