mod aes256gcm;
mod certs;
mod format;
mod hkdf_sha256;
mod sha512;
mod x25519;

use base64url::decode;
use certs::check_certs;
use format::*;
use hex::FromHex;
use hkdf_sha256::*;
use x25519::BASE_POINT;
use x25519::curve25519_donna;

// use std::io::{self, Write, Read};
// use std::ops::Mul;
use crate::tls_session::certs::check_certs_with_fixed_root;

// use crate::{network, format};
// const UnknownSignatureAlgorithm: u16 = 0;
// const MD2WithRSA: u16 = 1;  // Unsupported.
// const MD5WithRSA: u16 = 2;  // Only supported for signing, not verification.
// const SHA1WithRSA: u16 = 3; // Only supported for signing, and verification
// of CRLs, CSRs, and OCSP responses.
const SHA256WITH_RSAE: u16 = 2052; // 08 04 (RSA-PSS-RSAE-SHA256)
const SHA384WITH_RSAE: u16 = 2053; // 08 05 (RSA-PSS-RSAE-SHA384)
const SHA512WITH_RSAE: u16 = 2054; // 08 06 (RSA-PSS-RSAE-SHA512)

const SHA256WITH_RSA: u16 = 1025; // 04 01 (RSA-PKCS1-SHA256)
const SHA384WITH_RSA: u16 = 1281; // 05 01 (RSA-PKCS1-SHA384)
const SHA512WITH_RSA: u16 = 1537; // 06 01 (RSA-PKCS1-SHA512)
// const DSAWithSHA1: u16 = 7;   // Unsupported.
// const DSAWithSHA256: u16 = 8; // Unsupported.
// const ECDSAWithSHA1: u16 = 9; // 03 03 () Only supported for signing, and
// verification of CRLs, CSRs, and OCSP responses.
const ECDSA_WITH_SHA256: u16 = 1027; // 04 03 (ECDSA-SECP256r1-SHA256)
const ECDSA_WITH_SHA384: u16 = 1283; // 05 03 (ECDSA-SECP384r1-SHA384)
const ECDSA_WITH_SHA512: u16 = 1539; // 06 03 (ECDSA-SECP521r1-SHA512)
const SHA256WITH_RSAPSS: u16 = 2057; // 08 09 (RSA-PSS-PSS-SHA256)
const SHA384WITH_RSAPSS: u16 = 2058; // 08 0a (RSA-PSS-PSS-SHA384)
const SHA512WITH_RSAPSS: u16 = 2059; // 08 0b (RSA-PSS-PSS-SHA512)
// const PureEd25519: u16 = 2055; // 08 07 (ED25519)

pub struct Keys {
    pub public: [u8; 32],
    pub private: [u8; 32],
    pub handshake_secret: [u8; 32],
    pub client_handshake_secret: [u8; 32],
    pub client_handshake_key: [u8; 16],
    pub server_handshake_key: [u8; 16],
    pub client_handshake_iv: [u8; 12],
    pub server_handshake_iv: [u8; 12],
    pub client_application_key: [u8; 16],
    pub client_application_iv: [u8; 12],
    pub server_application_key: [u8; 16],
    pub server_application_iv: [u8; 12],
}

// pub fn key_pair() -> Keys {
// let private_key = [231, 226, 189, 128, 175, 192, 46, 233, 160, 243, 227, 168,
// 186, 174, 207, 111, 124, 21, 6, 220, 18, 155, 18, 17, 39, 165, 203, 108, 109,
// 3, 40, 186]; let public_key = curve25519_donna(&private_key, &BASE_POINT);
//
// Keys {
// public: public_key,
// private: private_key,
// server_public: Vec::new(),
// handshake_secret: [0u8;32],
// client_handshake_secret: [0u8;32],
// client_handshake_key: [0u8;16],
// server_handshake_key: [0u8;16],
// client_handshake_iv: [0u8;12],
// server_handshake_iv: [0u8;12],
// client_application_key: [0u8;16],
// client_application_iv: [0u8;12],
// server_application_key: [0u8;16],
// server_application_iv: [0u8;12],
// }
// }

// AEAD helper functions

fn decrypt(key: &[u8; 16], iv: &[u8; 12], wrapper: &[u8]) -> Vec<u8> {
    let block = aes256gcm::new_cipher(key);
    let aes_gcm = aes256gcm::new_gcm(block);

    let additional = &wrapper[0..5];
    let ciphertext = &wrapper[5..];

    let plaintext = aes_gcm.open(&[], iv, ciphertext, additional);
    return plaintext;
}

fn encrypt(key: &[u8; 16], iv: &[u8; 12], plaintext: &[u8], additional: &[u8]) -> Vec<u8> {
    let block = aes256gcm::new_cipher(key);
    let aes_gcm = aes256gcm::new_gcm(block);

    // let nonce = Nonce::from_slice(iv); // 96-bits; retrieve nonce from the IV
    // let ciphertext = aesgcm.encrypt(nonce, additional,
    // plaintext).expect("Encryption failed");
    let ciphertext = aes_gcm.seal(&[], iv, plaintext, additional);

    [additional.to_vec(), ciphertext].concat() // Concatenate additional data with ciphertext
}

pub fn hkdf_expand_label(secret: &[u8; 32], label: &str, context: &[u8], length: u16) -> Vec<u8> {
    // Construct HKDF label
    let mut hkdf_label = vec![];
    hkdf_label.extend_from_slice(&length.to_be_bytes());
    let tls13_prefix = b"tls13 ";
    hkdf_label.push((tls13_prefix.len() + label.as_bytes().len()) as u8);
    hkdf_label.extend_from_slice(tls13_prefix);
    hkdf_label.extend_from_slice(label.as_bytes());

    hkdf_label.push(context.len() as u8);
    hkdf_label.extend_from_slice(context);

    // Expand using HKDF
    let mut reader = hkdf_sha256::expand(secret, &hkdf_label[..]); //let hkdf = Hkdf::<Sha256>::new(Some(secret), &hkdf_label);
    let buf = reader.read(length as usize);

    buf
}

pub fn derive_secret(secret: &[u8; 32], label: &str, transcript_messages: &[u8]) -> [u8; 32] {
    let hash = hkdf_sha256::sum256(transcript_messages);
    let secret = hkdf_expand_label(secret, label, &hash, 32);
    secret.try_into().unwrap()
}

pub fn extract_json_public_key_from_tls(raw: Vec<u8>) -> Vec<u8> {
    if raw.len() < 4000 {
        return vec![0u8, 3u8, 33u8]; // "insufficient len" : 0x3, 0x21 = 801
    }

    let timestamp_bytes = &raw[..4];
    let len_of_provider = raw[4] as usize;
    if len_of_provider < 4 || len_of_provider > 100 {
        return vec![0u8, 3u8, 34u8]; // "corrupted provider name (not lv format) / incorrect issuer len" : 0x3, 0x22 = 802
    }
    let provider = &raw[5..5 + len_of_provider];
    let start_kid_pos = 5 + len_of_provider;

    let len_of_kid = raw[start_kid_pos] as usize;
    if len_of_kid < 1 || len_of_kid > 30 {
        return vec![0u8, 3u8, 35u8]; // "corrupted kid (not lv format) / incorrect kid len" : 0x3, 0x22 = 802
    }

    let kid = &raw[start_kid_pos + 1..start_kid_pos + 1 + len_of_kid];// let kid = &raw[5..5 + len_of_kid]; // let kid = &raw[4..24];
    let start_cert = start_kid_pos + 1 + len_of_kid; // let start_cert = 5 + len_of_kid;
    let certificate_len = (256 * raw[start_cert] as u16 + raw[start_cert + 1] as u16) as usize; // let certificate_len = (256*raw[24] as u16 + raw[25] as u16) as usize;

    if certificate_len < 500 { // for example 525 is valid ECDSA cert
        return vec![0u8, 3u8, 36u8]; // "insufficient len of external certificate" : 0x3, 0x21 = 801
    }

    let external_root_cert = &raw[start_cert + 2..start_cert + 2 + certificate_len]; // let external_root_cert = &raw[26..26+certificate_len];
    let data = &raw[start_cert + 2 + certificate_len..]; // let data = &raw[26+certificate_len..];

    // the first output byte indicates the success of the process: if it equals to 1
    // then success then follows the public keys from json
    // if the first bytes equals to 0 then unsuccess and the error code follows
    let timestamp_shortened = aes256gcm::uint32(&timestamp_bytes);
    let timestamp = timestamp_shortened as i64;
    let private_key: [u8; 32] = data[0..32].try_into().unwrap();

    let records_send: u8 = data[32];
    let records_received_declared: u8 = data[33];
    // check len of data

    let client_hello_len = data[38] as usize;

    if client_hello_len < 100 || len_of_kid > 240 {
        return vec![0u8, 3u8, 38u8]; // "incorrect client hello len"
    }

    let client_hello: &[u8] = &data[34..39 + client_hello_len]; //let client_hello:[u8;166] = data[34..200].try_into().unwrap(); // len is 166 bytes

    if client_hello[0] != 0x16 {
        return vec![0u8, 3u8, 39u8]; // "client hello not found"
    }
    let server_hello_start = 39 + client_hello_len;
    let server_hello: [u8; 95] =
        data[server_hello_start..server_hello_start + 95].try_into().unwrap(); //let server_hello:[u8;95] = data[200..295].try_into().unwrap(); // len is 95 bytes
    if server_hello[0] != 0x16 {
        return vec![0u8, 3u8, 40u8]; // "server hello not found"
    }
    let enc_ser_handshake_len =
        256 * data[server_hello_start + 98] as u16 + data[server_hello_start + 99] as u16; // let enc_ser_handshake_len = 256*data[298] as u16 + data[299] as u16;
    if enc_ser_handshake_len<2500 {
        return vec![0u8, 3u8, 41u8]; // "server handshake len not sufficient"
    }
    let handshake_end_index = server_hello_start + 95 + 5 + enc_ser_handshake_len as usize; // let handshake_end_index = 295 + 5 + enc_ser_handshake_len as usize;

    // let encrypted_server_handshake:[u8;4350] =
    // data[295..handshake_end_index].try_into().unwrap();
    let encrypted_server_handshake = &data[server_hello_start + 95..handshake_end_index]; // let encrypted_server_handshake = &data[295..handshake_end_index];

    let app_request_len =
        256 * data[handshake_end_index + 3] as usize + data[handshake_end_index + 4] as usize + 5;
    let application_request = &data[handshake_end_index..handshake_end_index + app_request_len]; // let application_request:[u8;100] = data[handshake_end_index..handshake_end_index+100].try_into().unwrap();

    let mut records_received: u8 = 1;
    let mut encr_ticket_len = 256 * data[handshake_end_index + app_request_len + 3] as usize
        + data[handshake_end_index + app_request_len + 4] as usize
        + 5;
    //if encr_ticket_len == 241 {
        // if encr_ticket_len < 300 {
        //encr_ticket_len = encr_ticket_len * 2;
        //records_received = 2;
    //}
    let next_packet_len = 256 * data[handshake_end_index + app_request_len + encr_ticket_len + 3] as usize 
        + data[handshake_end_index + app_request_len + encr_ticket_len + 4] as usize
        + 5;
    if next_packet_len==encr_ticket_len {
        encr_ticket_len += next_packet_len; // double encrypted session ticket
        records_received = 2;
    }

    let encrypted_ticket: &[u8] = &data[handshake_end_index + app_request_len
        ..handshake_end_index + app_request_len + encr_ticket_len]; // let encrypted_ticket: &[u8] = &data[handshake_end_index + app_request_len..handshake_end_index + app_request_len +540];// let encrypted_ticket:[u8;540] = data[handshake_end_index+100..handshake_end_index+100+540].try_into().unwrap(); // len of ticket is 524

    //if ... {
        //return vec![0u8, 3u8, 42u8]; // some trouble with encrypted session ticket
    //}

    // let http_response:[u8;1601] =
    // data[handshake_end_index+640..handshake_end_index+640+1601].try_into().
    // unwrap();
    let http_response = &data[handshake_end_index + app_request_len + encr_ticket_len..]; // let http_response = &data[handshake_end_index + app_request_len + 540..]; // let http_response = &data[handshake_end_index+640..];

    if http_response.len() < 1000 {
        return vec![0u8, 3u8, 43u8]; // "insufficient http response len"
    }

    let public_key = curve25519_donna(&private_key, &BASE_POINT);

    let server_hello_data = parse_server_hello(&server_hello[5..]);

    // ================== begin make handshake keys
    // ===============================================================================================
    let zeros = [0u8; 32];
    let psk = [0u8; 32];

    let shared_secret = curve25519_donna(&private_key, &server_hello_data.public_key);

    // Handshake using HKDF
    let early_secret = hkdf_sha256::extract(&zeros, &psk);
    let derived_secret = derive_secret(&early_secret, "derived", &[]);

    let handshake_secret = hkdf_sha256::extract(&shared_secret, &derived_secret);

    let handshake_messages = format::concatenate(&[&client_hello[5..], &server_hello[5..]]);

    let c_hs_secret = derive_secret(&handshake_secret, "c hs traffic", &handshake_messages);
    let client_handshake_secret = c_hs_secret.clone();
    let client_handshake_key: [u8; 16] =
        hkdf_expand_label(&c_hs_secret, "key", &[], 16).try_into().unwrap();
    let client_handshake_iv: [u8; 12] =
        hkdf_expand_label(&c_hs_secret, "iv", &[], 12).try_into().unwrap();

    let s_hs_secret = derive_secret(&handshake_secret, "s hs traffic", &handshake_messages);
    // let session_keys_server_handshake_key = hkdf_expand_label(&s_hs_secret,
    // "key", &[], 16);

    let server_handshake_key: [u8; 16] =
        hkdf_expand_label(&s_hs_secret, "key", &[], 16).try_into().unwrap();
    let server_handshake_iv: [u8; 12] =
        hkdf_expand_label(&s_hs_secret, "iv", &[], 12).try_into().unwrap();

    // ============== begin parse server handshake =====================
    if encrypted_server_handshake[0] != 0x17 {
        return vec![0u8, 3u8, 50u8]; // "not found encrypted server handshake"
    }

    let server_handshake_message =
        decrypt(&server_handshake_key, &server_handshake_iv, &encrypted_server_handshake[..]);
    let decrypted_server_handshake = DecryptedRecord { 0: server_handshake_message };

    // ============= begin make application keys ===================================
    let handshake_messages = format::concatenate(&[
        &client_hello[5..],
        &server_hello[5..],
        &decrypted_server_handshake.contents(),
    ]);

    let derived_secret = derive_secret(&handshake_secret, "derived", &[]);
    let master_secret = hkdf_sha256::extract(&zeros, &derived_secret); //let master_secret = Hkdf::<Sha256>::extract(Some(&zeros), &derived_secret);

    // let c_ap_secret = derive_secret(&master_secret, "c ap traffic",
    // &handshake_messages); let client_application_key: [u8;16] =
    // hkdf_expand_label(&c_ap_secret, "key", &[], 16).try_into().unwrap();
    // let client_application_iv: [u8;12] = hkdf_expand_label(&c_ap_secret, "iv",
    // &[], 12).try_into().unwrap();

    let s_ap_secret = derive_secret(&master_secret, "s ap traffic", &handshake_messages);
    let server_application_key: [u8; 16] =
        hkdf_expand_label(&s_ap_secret, "key", &[], 16).try_into().unwrap();
    let server_application_iv: [u8; 12] =
        hkdf_expand_label(&s_ap_secret, "iv", &[], 12).try_into().unwrap();

    // ========== begin check handshake ================
    let handshake_data = decrypted_server_handshake.contents(); //[5..];
    // let certs_chain = &handshake_data[7..];
    let len_of_padding = handshake_data[3] as usize;
    let certs_chain = &handshake_data[4 + len_of_padding + 1..];

    // next three bytes is the length of certs chain
    let certs_chain_len = (certs_chain[0] as usize) * 65536
        + (certs_chain[1] as usize) * 256
        + (certs_chain[2] as usize);

    if certs_chain_len < 1000 { // minimal chain is two ecdsa certs each of them approx 500 bytes len
        return vec![0u8, 3u8, 70u8]; // "certs_chain_len is not sufficiet"
    }

    if certs_chain[certs_chain_len + 3] != 0xf {
        return vec![0u8, 3u8, 71u8]; // "signature not found"
    }

    let sign_type =
        (certs_chain[certs_chain_len + 7] as u16) * 256 + (certs_chain[certs_chain_len + 8] as u16);

    let signature_len = (certs_chain[certs_chain_len + 9] as usize) * 256
        + (certs_chain[certs_chain_len + 10] as usize);
    if signature_len < 64 {
        return vec![0u8, 3u8, 72u8]; // "insufficient signature length"
    }
    let signature = &certs_chain[certs_chain_len + 11..certs_chain_len + 11 + signature_len];
    // let signature_with_type = concatenate(&[ &certs_chain[certs_chain_len +
    // 7..certs_chain_len + 8], &signature]);

    let client_server_hello = format::concatenate(&[
        &client_hello[5..],
        &server_hello[5..],
        &handshake_data[..4 + len_of_padding + 1 + certs_chain_len + 3],
    ]);

    // if sign_type!=SHA256WITH_RSAE && sign_type!=SHA256WITH_RSA &&
    // sign_type!=ECDSA_WITH_SHA256 && sign_type!=SHA256WITH_RSAPSS {
    // return vec![0u8, 3u8, 38u8];// "not supported (not sha256) type of signature"
    //}
    let check_sum = hkdf_sha256::sum256(&client_server_hello).to_vec();

    let context: [u8; 98] = [
        32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32,
        32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32,
        32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 84, 76, 83, 32, 49,
        46, 51, 44, 32, 115, 101, 114, 118, 101, 114, 32, 67, 101, 114, 116, 105, 102, 105, 99, 97,
        116, 101, 86, 101, 114, 105, 102, 121, 0,
    ];

    let check_sum_extend = format::concatenate(&[&context, &check_sum]);

    let check_prepared = match sign_type {
        SHA256WITH_RSAE => hkdf_sha256::sum256(&check_sum_extend).to_vec(),
        SHA256WITH_RSA => hkdf_sha256::sum256(&check_sum_extend).to_vec(),
        ECDSA_WITH_SHA256 => hkdf_sha256::sum256(&check_sum_extend).to_vec(),
        SHA256WITH_RSAPSS => hkdf_sha256::sum256(&check_sum_extend).to_vec(),
        ECDSA_WITH_SHA384 => sha512::sum384(&check_sum_extend).to_vec(),
        SHA384WITH_RSAPSS => sha512::sum384(&check_sum_extend).to_vec(),
        SHA384WITH_RSA => sha512::sum384(&check_sum_extend).to_vec(),
        SHA384WITH_RSAE => sha512::sum384(&check_sum_extend).to_vec(),
        ECDSA_WITH_SHA512 => sha512::sum512(&check_sum_extend).to_vec(),
        SHA512WITH_RSA => sha512::sum512(&check_sum_extend).to_vec(),
        SHA512WITH_RSAE => sha512::sum512(&check_sum_extend).to_vec(),
        SHA512WITH_RSAPSS => sha512::sum512(&check_sum_extend).to_vec(),
        _ => return vec![0u8, 3u8, 73u8], // "not supported (not sha256, sha384 or sha512) type of signature" 
    };

    if let Err(e) = check_certs_with_fixed_root( // !check_certs_with_fixed_root(
        timestamp,
        &provider,
        &check_prepared,
        &certs_chain[4..certs_chain_len + 1],
        &signature,
        &external_root_cert,
    ) {
        return e; // return vec![0u8, 3u8, 74u8]; // "error in certificates chain !"
    }

    // =================== begin check application request ===================
    // let domain = "www.googleapis.com";
    // let etalon_req = format!("GET /oauth2/v3/certs HTTP/1.1\r\nHost:
    // {}\r\nConnection: close\r\n\r\n", domain); let etalon_req_bytes =
    // etalon_req.as_bytes(); encrypt etalon application request
    // let mut data_vec = etalon_req_bytes.to_vec();
    // data_vec.push(0x17);
    // let additional_length = (data_vec.len() + 16) as u16;
    // let additional = format::concatenate(&[
    // &[0x17, 0x03, 0x03], &format::u16_to_bytes(additional_length)
    // ]);
    // let etalon_encrypted = encrypt(&client_application_key,
    // &client_application_iv, &data_vec[..], &additional[..]); match with
    // application_request if application_request.to_vec() != etalon_encrypted {
    // return vec![0u8, 3u8, 40u8]; // "incorrect application request !"
    // }

    // =================== begin decryption ticket and check
    // =========================

    // =================== begin decryption application response
    // =====================

    let mut len_of_first_packet =
        (http_response[3] as usize) * 256 + (http_response[4] as usize) + 5;

    let mut iv = server_application_iv.clone();
    // let mut records_received: u8 = 1;
    iv[11] ^= records_received;

    let mut plaintext = decrypt(
        &server_application_key,
        &iv.try_into().unwrap(),
        &http_response[..len_of_first_packet],
    );
    plaintext = format::trunc_end_with_trailer(&plaintext, 23u8); // trunc end zeroes with 23

    while records_received < records_received_declared - 1 {
        records_received += 1;
        let start_index = len_of_first_packet;
        let len_of_packet = (http_response[start_index + 3] as usize) * 256
            + (http_response[start_index + 4] as usize)
            + 5;

        let ciphertext2 = &http_response[len_of_first_packet..len_of_first_packet + len_of_packet];
        let mut iv2 = server_application_iv.clone();
        iv2[11] ^= records_received;
        let mut plaintext2 =
            decrypt(&server_application_key, &iv2.try_into().unwrap(), &ciphertext2);
        // plaintext2.pop();
        plaintext2 = format::trunc_end_with_trailer(&plaintext2, 23u8); // trunc end zeroes with 23

        plaintext.append(&mut plaintext2);
        len_of_first_packet = len_of_first_packet + len_of_packet;
    }

    let plaintext_as_string = String::from_utf8_lossy(&plaintext).to_string();

    let expires_timestamp = format::extract_expires(&plaintext_as_string);

    let strings_n = format::extract_all_items("n", &plaintext_as_string); // = format::extract_all_n(&plaintext_as_string);
    let strings_kid = format::extract_all_items("kid", &plaintext_as_string);

    // let mut hashes_n: Vec<u8> = Vec::new();
    let mut counter = 0;

    for substring in strings_kid {
        // let mut current = substring.as_bytes().to_vec();
        let current_decoded_kid = Vec::from_hex(substring).unwrap();

        if current_decoded_kid.eq(&kid.to_vec()) {
            let mut current_decoded_n = decode(&strings_n[counter]).unwrap();
            let mut result = vec![1u8];

            append_uint64(&mut result, expires_timestamp as u64);
            result.append(&mut current_decoded_n.to_vec());

            return result;
        }

        // let hash_of_current_n = hkdf_sha256::sum256( &current_decoded_n);
        // hashes_n.append(&mut hash_of_current_n.to_vec());
        counter += 1;
    }

    // let mut result = vec![1u8];
    // result.push(strings_n.len() as u8);
    // result.append(&mut hashes_n);

    // let expires_timestamp = format::extract_expires(&plaintext_as_string);
    // result.append(&mut expires_timestamp.to_be_bytes().to_vec());

    return vec![0u8, 3u8, 200u8]; // "kid not found "
}
