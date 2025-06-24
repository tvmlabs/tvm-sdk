mod aes256gcm;
mod x25519;
mod certs;
mod hkdf_sha256;
mod format;

use x25519::curve25519_donna;
use hkdf_sha256::*;
use format::*;
use certs::check_certs;

use base64url::decode;
use hex::FromHex;

pub fn get_root_cert_from_online() -> [u8;1382] {
    certs::ROOT_CERT_FROM_ONLINE
}

pub struct Keys {
    pub public: [u8; 32],
    pub private: [u8; 32],//Vec<u8>,
    pub handshake_secret: [u8;32],
    pub client_handshake_secret: [u8;32],
    pub client_handshake_key: [u8;16],
    pub server_handshake_key: [u8;16],
    pub client_handshake_iv: [u8;12],
    pub server_handshake_iv: [u8;12],
    pub client_application_key: [u8;16],
    pub client_application_iv: [u8;12],
    pub server_application_key: [u8;16],
    pub server_application_iv: [u8;12],
}

pub fn key_pair() -> Keys {
    //let private_key = random(32);
    //let private_key = random32bytes();
    let private_key = [231, 226, 189, 128, 175, 192, 46, 233, 160, 243, 227, 168, 186, 174, 207, 111, 124, 21, 6, 220, 18, 155, 18, 17, 39, 165, 203, 108, 109, 3, 40, 186];
    let basepoint:[u8;32] = [9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let public_key = curve25519_donna(&private_key, &basepoint);

    Keys {
        public: public_key, // public_key.compress().to_bytes().to_vec(),
        private: private_key,
        //server_public: Vec::new(),
        handshake_secret: [0u8;32],
        client_handshake_secret: [0u8;32],
        client_handshake_key: [0u8;16],
        server_handshake_key: [0u8;16],
        client_handshake_iv: [0u8;12],
        server_handshake_iv: [0u8;12],
        client_application_key: [0u8;16],
        client_application_iv: [0u8;12],
        server_application_key: [0u8;16],
        server_application_iv: [0u8;12],
    }
}

pub fn decrypt(key: &[u8;16], iv: &[u8;12], wrapper: &[u8]) -> Vec<u8> {

    let block = aes256gcm::new_cipher(key);
    let aes_gcm = aes256gcm::new_gcm(block);

    let additional = &wrapper[0..5];
    let ciphertext = &wrapper[5..];

    let plaintext = aes_gcm.open(&[], iv, ciphertext, additional);
    return plaintext;
}

pub fn encrypt(key: &[u8;16], iv: &[u8;12], plaintext: &[u8], additional: &[u8]) -> Vec<u8> {
    let block = aes256gcm::new_cipher(key);
    let aes_gcm = aes256gcm::new_gcm(block);

    //let nonce = Nonce::from_slice(iv); // 96-bits; retrieve nonce from the IV
    //let ciphertext = aesgcm.encrypt(nonce, additional, plaintext).expect("Encryption failed");
    let ciphertext = aes_gcm.seal(&[], iv, plaintext, additional);

    [additional.to_vec(), ciphertext].concat() // Concatenate additional data with ciphertext
}

pub fn hkdf_expand_label(secret: &[u8;32], label: &str, context: &[u8], length: u16) -> Vec<u8> {
    // Construct HKDF label
    let mut hkdf_label = vec![];
    hkdf_label.extend_from_slice(&length.to_be_bytes());
    let tls13_prefix = b"tls13 ";
    hkdf_label.push((tls13_prefix.len()+label.as_bytes().len()) as u8);
    hkdf_label.extend_from_slice(tls13_prefix);
    hkdf_label.extend_from_slice(label.as_bytes());

    hkdf_label.push(context.len() as u8);
    hkdf_label.extend_from_slice(context);

    // Expand using HKDF
    let mut reader = hkdf_sha256::expand(secret, &hkdf_label[..]);//let hkdf = Hkdf::<Sha256>::new(Some(secret), &hkdf_label);
    let buf = reader.read(length as usize);

    buf
}

pub fn derive_secret(secret: &[u8;32], label: &str, transcript_messages: &[u8]) -> [u8; 32] {
    let hash = hkdf_sha256::sum256( transcript_messages);
    let secret = hkdf_expand_label(secret, label, &hash, 32);
    secret.try_into().unwrap()

}

pub fn extract_json_public_key_from_tls(raw: Vec<u8>) -> Vec<u8> {
    let kid = &raw[..20];
    let certificate_len = (256*raw[20] as u16 + raw[21] as u16) as usize;

    let certificate = &raw[22..22+certificate_len];
    let data = &raw[22+certificate_len..];

  // the first output byte indicates the success of the process: if it equals to 1 then success
    // then follows the public keys from json
    // if the first bytes equals to 0 then unsuccess and the error code follows
    let private_key:[u8;32] = data[0..32].try_into().unwrap();
    let records_send: u8 = data[32];
    let records_received: u8 = data[33];
    // check len of data
    if data.len()<6000{ // 6500
        return vec![0u8, 3u8, 33u8]; // "insufficient len" : 0x3, 0x21 = 801
    }
    let client_hello:[u8;166] = data[34..200].try_into().unwrap(); // len is 166 bytes
    if client_hello[0] != 0x16 {
        return vec![0u8, 3u8, 34u8]; // "client hello not found"
    }
    let server_hello:[u8;95] = data[200..295].try_into().unwrap(); // len is 95 bytes
    if server_hello[0] != 0x16 {
        return vec![0u8, 3u8, 35u8]; // "server hello not found"
    }
    let enc_ser_handshake_len = 256*data[298] as u16 + data[299] as u16;
    let handshake_end_index = 295 + 5 + enc_ser_handshake_len as usize;

    // let encrypted_server_handshake:[u8;4350] = data[295..handshake_end_index].try_into().unwrap();
    let encrypted_server_handshake = &data[295..handshake_end_index];

    let application_request:[u8;100] = data[handshake_end_index..handshake_end_index+100].try_into().unwrap();
    let encrypted_ticket:[u8;540] = data[handshake_end_index+100..handshake_end_index+100+540].try_into().unwrap(); // len of ticket is 524
    //let http_response:[u8;1601] = data[handshake_end_index+640..handshake_end_index+640+1601].try_into().unwrap();
    let http_response = &data[handshake_end_index+640..];

    let basepoint:[u8;32] = [9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let public_key = curve25519_donna(&private_key, &basepoint);


   let server_hello_data = parse_server_hello(&server_hello[5..]);

     // ================== begin make handshake keys ===============================================================================================
    let zeros = [0u8; 32];
    let psk = [0u8; 32]; // Предполагается, что psk инициализируется где-то

    let shared_secret = curve25519_donna(&private_key, &server_hello_data.public_key);

    // Хэндшейк с использованием HKDF
    let early_secret = hkdf_sha256::extract(&zeros,&psk);

    let derived_secret = derive_secret(&early_secret, "derived", &[]);

    let handshake_secret = hkdf_sha256::extract(&shared_secret, &derived_secret);

    let handshake_messages = format::concatenate(
            &[&client_hello[5..], &server_hello[5..] ]
    );

    let c_hs_secret = derive_secret(&handshake_secret, "c hs traffic", &handshake_messages);
    let client_handshake_secret = c_hs_secret.clone();
    let client_handshake_key: [u8;16] = hkdf_expand_label(&c_hs_secret, "key", &[], 16).try_into().unwrap();
    let client_handshake_iv: [u8;12] = hkdf_expand_label(&c_hs_secret, "iv", &[], 12).try_into().unwrap();

    let s_hs_secret = derive_secret(&handshake_secret, "s hs traffic", &handshake_messages);
    //let session_keys_server_handshake_key = hkdf_expand_label(&s_hs_secret, "key", &[], 16);

    let server_handshake_key: [u8;16] = hkdf_expand_label(&s_hs_secret, "key", &[], 16).try_into().unwrap();
    let server_handshake_iv: [u8;12] = hkdf_expand_label(&s_hs_secret, "iv", &[], 12).try_into().unwrap();

    // ============== begin parse server handshake =====================
    if encrypted_server_handshake[0] != 0x17 {
        return vec![0u8, 3u8, 36u8];// "not found encrypted server handshake"
    }

    let server_handshake_message = decrypt(&server_handshake_key, &server_handshake_iv, &encrypted_server_handshake[..]);
   let decrypted_server_handshake = DecryptedRecord{ 0: server_handshake_message};

    // ============= begin make application keys ===================================
    let handshake_messages = format::concatenate( &[
        &client_hello[5..],
        &server_hello[5..],
        &decrypted_server_handshake.contents()]
    );

    let derived_secret = derive_secret(&handshake_secret, "derived", &[]);
    let master_secret = hkdf_sha256::extract(&zeros, &derived_secret);//let master_secret = Hkdf::<Sha256>::extract(Some(&zeros), &derived_secret);

    let c_ap_secret = derive_secret(&master_secret, "c ap traffic", &handshake_messages);
    let client_application_key: [u8;16] = hkdf_expand_label(&c_ap_secret, "key", &[], 16).try_into().unwrap();
    let client_application_iv: [u8;12] = hkdf_expand_label(&c_ap_secret, "iv", &[], 12).try_into().unwrap();

    let s_ap_secret = derive_secret(&master_secret, "s ap traffic", &handshake_messages);
    let server_application_key: [u8;16] = hkdf_expand_label(&s_ap_secret, "key", &[], 16).try_into().unwrap();
    let server_application_iv: [u8;12] = hkdf_expand_label(&s_ap_secret, "iv", &[], 12).try_into().unwrap();

    // ========== begin check handshake ================
    let handshake_data = decrypted_server_handshake.contents();//[5..];
    let certs_chain = &handshake_data[7..];

    //next three bytes is the length of certs chain
    let certs_chain_len = (certs_chain[0] as usize)*65536 + (certs_chain[1] as usize)*256 + (certs_chain[2] as usize);

    if !check_certs(&certs_chain[4..certs_chain_len+1]) {
        return vec![0u8, 3u8, 37u8]; // "error in certificates chain !"
    }

    // =================== begin check application request ===================
    let domain = "www.googleapis.com";
    let req = format!("GET /oauth2/v3/certs HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n", domain);
    // req.as_bytes()
    // match with application_request

    // =================== begin decryption ticket and check =========================

    
    let len_of_first_packet = (http_response[3] as usize)*256 + (http_response[4] as usize) + 5;

    let mut iv = server_application_iv.clone();
    let mut records_received: u8= 1;
    iv[11] ^= records_received;

    let mut plaintext = decrypt(&server_application_key, &iv.try_into().unwrap(), &http_response[..len_of_first_packet]);

    // Увеличиваем количество полученных записей
    records_received += 1;

    //let ciphertext2 = &http_response[len_of_first_packet..];
    let mut iv2 = server_application_iv.clone();
    iv2[11] ^= records_received;
    let mut plaintext2 = decrypt(&server_application_key, &iv2.try_into().unwrap(), &http_response[len_of_first_packet..]);

    plaintext.append(&mut plaintext2);

    let plaintext_as_string = String::from_utf8_lossy(&plaintext).to_string();

    let expires_timestamp = format::extract_expires(&plaintext_as_string);

    let strings_n = format::extract_all_items("n",&plaintext_as_string); // = format::extract_all_n(&plaintext_as_string);
    let strings_kid = format::extract_all_items("kid",&plaintext_as_string);

    //let mut hashes_n: Vec<u8> = Vec::new();
    let mut counter = 0;

    for substring in strings_kid {

        //let mut current = substring.as_bytes().to_vec();
        let mut current_decoded_kid = Vec::from_hex(substring).unwrap();

        if current_decoded_kid.eq(&kid.to_vec()){
            let mut current_decoded_n = decode(&strings_n[counter]).unwrap();
            let mut result = vec![1u8];

            append_uint64(&mut result, expires_timestamp as u64);

            result.append(&mut current_decoded_n.to_vec());

            return result;

        }

        //let hash_of_current_n = hkdf_sha256::sum256( &current_decoded_n);
        //hashes_n.append(&mut hash_of_current_n.to_vec());
        counter += 1;
    }

    //let mut result = vec![1u8];
    //result.push(strings_n.len() as u8);
    //result.append(&mut hashes_n);

    //let expires_timestamp = format::extract_expires(&plaintext_as_string);
    //result.append(&mut expires_timestamp.to_be_bytes().to_vec());


    return vec![0u8, 3u8, 43u8]; // "kid not found "
    
    
}

