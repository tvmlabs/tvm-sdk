//use std::io::{self, Read};
use chrono::{DateTime, FixedOffset, TimeZone};

pub struct Messages {
    pub client_hello: Record,
    pub server_hello: Record,
    pub server_handshake: DecryptedRecord,
    pub encrypted_server_handshake: Record, // not needed
    pub application_request: Record, // not needed
    pub encrypted_ticket: Record, // not needed
    pub http_response: Record,
}


//pub type DecryptedRecord = Vec<u8>;

//impl DecryptedRecord {
    //pub fn type_(&self) -> u8 {
        //&self.last().expect("DecryptedRecord is empty")
    //}

    //pub fn contents(&self) -> &[u8] {
        //&self[..self.len() - 1]
    //}
//}

pub struct DecryptedRecord(pub(crate) Vec<u8>);

impl DecryptedRecord {

    pub fn new() -> DecryptedRecord {
        DecryptedRecord{ 0: vec![]}
    }

    // Возвращает последний байт в записи
    pub fn rtype(&self) -> u8 {
        *self.0.last().expect("DecryptedRecord is empty")
    }

    // Возвращает все байты, кроме последнего
    pub fn contents(&self) -> &[u8] {
        &self.0[..self.0.len() - 1]
    }
}

//pub type Record = Vec<u8>;

//impl Record {
    //pub fn contents(&self) -> &[u8] {
        //&self[5..]
    //}

    //pub fn rtype(&self) -> u8 {
        //*self[0].expect("Record is empty")
    //}
//}

#[derive(Clone)]
pub struct Record(pub Vec<u8>);


impl Record{
    pub fn new() -> Record {
        Record{ 0: vec![]}
    }

    pub fn contents(&self) -> &[u8] {
        &self.0[5..]
    }

    // first five bytes (TLS header) are:
    // 16 or 17 (unencrypted or encrypted)
    // 03 03 (means TLS 1.2)
    // two bytes (length of message + 4)

    pub fn rtype(&self) -> u8 {
        *self.0.first().expect("Record is empty")
    }
}

pub struct ServerHello {
    pub random: [u8;32],
    pub public_key: [u8; 32],
}

pub fn server_name(name: &str) -> Vec<u8> {
    let bytes = name.as_bytes();
    concatenate(&[
        &u16_to_bytes((name.len() + 3) as u16),
        &[0x00],
        &u16_to_bytes(name.len() as u16),
        &bytes]
    )
}

pub fn key_share(public_key: &[u8]) -> Vec<u8> {
    concatenate(&[
        &u16_to_bytes((public_key.len() + 4) as u16),
        &u16_to_bytes(0x1d).as_slice(), // x25519
        &u16_to_bytes(public_key.len() as u16).as_slice(),
        &public_key]
    )
}

pub fn parse_server_hello(buf: & [u8]) -> ServerHello {
    let mut hello = ServerHello {
        random: [0u8; 32],
        public_key: [0u8; 32],
    };
    let mut current_pos: usize = 0;

    current_pos = current_pos + 2; // 02 00 ("server hello") // buf.take(2);

    current_pos = current_pos + 2; // buf.take(2);

    current_pos = current_pos + 2; // 03 03 (client protocol version = "TLS 1.2") // buf.take(2);

    if &current_pos + 32 < buf.len() { // if let Some(random_bytes) = buf.take(32) {
        let random_bytes = &buf[current_pos..current_pos+32];
        hello.random = random_bytes.clone().try_into().unwrap(); //hello.random.extend_from_slice(random_bytes);
        current_pos = current_pos + 32;

        let session_id_len: u8 = buf[current_pos];// let session_id_len = buf.read_u8().expect("Can t read len of session ID");
        current_pos = current_pos + 1;
        //let session_id = buf.read_bytes(session_id_len);
        current_pos = current_pos + (session_id_len as usize);

        current_pos = current_pos + 2;//buf.take(2); // cipher suite
        current_pos = current_pos + 1;// buf.take(1); // compression

        let mut dst = [0u8; 2];
        dst.clone_from_slice(&buf[current_pos..current_pos+2]);
        let extensions_len = u16::from_be_bytes(dst);//let extensions_len = buf.read_u16().expect("Can t read len of extensions!");
        //let extensions = buf.read_bytes(extensions_len);
        current_pos = current_pos + 2;

        while &current_pos+2<buf.len() { // !extensions.is_empty()
            //dst.clone_from_slice(&buf[&current_pos..&current_pos+2]);
            let typ =  u16::from_be_bytes(buf[current_pos..current_pos+2].try_into().unwrap() );//let typ = extensions.read_u16().expect("can t read type of extension");
            //current_pos = current_pos + 2;
            //let extension_length = u16::from_be_bytes(buf[&current_pos..&current_pos+2]);// let extension_length = extensions.read_u16().expect("can t read len of extension");
            //current_pos = current_pos + 2;
            //let content = &buf[&current_pos..&current_pos+&extension_length];
            //current_pos = current_pos + extension_length;
            match typ {
                0x0033 => { // key share
                    current_pos = current_pos + 4;
                    // bypass type of key
                    current_pos = current_pos + 2;//let _ = contents.read_u16().expect("can t read type of key"); // 00 1d means x25519
                    let public_key_length = u16::from_be_bytes(buf[current_pos..current_pos+2].try_into().unwrap());//let public_key_length = contents.read_u16().expect("can t read len of public key");
                    current_pos = current_pos + 2;
                    let public_key_bytes = &buf[current_pos..current_pos+(public_key_length as usize)];
                    hello.public_key = public_key_bytes.try_into().unwrap(); // hello.public_key = public_key_bytes.to_vec();
                }
                0x002b => {
                    current_pos = current_pos + 6;
                }
                _ => {
                    let extension_len = u16::from_be_bytes(buf[current_pos..current_pos+2].try_into().unwrap());
                    current_pos = current_pos + 4;
                    current_pos = current_pos + (extension_len as usize);
                    //eprintln!("unknown extension");
                }
            }
        }
    } else {
        //println!("not enough len of random bytes")
    }

    hello
}



pub fn concatenate(bufs: &[&[u8]]) -> Vec<u8> {
    let mut buf = Vec::new();
    for b in bufs {
        buf.extend_from_slice(b);
    }
    buf
}

pub fn u16_to_bytes(n: u16) -> [u8;2] {
    n.to_be_bytes()
}

pub fn extension(id: u16, contents: Vec<u8>) -> Vec<u8> {
    concatenate(&[
        &u16_to_bytes(id),
        &u16_to_bytes(contents.len() as u16),
        &contents]
    )
}

pub fn extract_all_items(item: &str, data: &str) -> Vec<String> {
    //let target = r#""n": ""#; // Подстрока для поиска
    let target = format!("{}{}{}", r#"""#, item, r#"": ""#); // Подстрока для поиска
    let mut results = Vec::new();
    let mut start = 0;

    while let Some(start_index) = data[start..].find(&target) {
        let start_pos = start + start_index + target.len(); // Позиция после подстроки "n": "

        // Ищем конец подстроки
        if let Some(end_index) = data[start_pos..].find('"') {
            let end_pos = start_pos + end_index; // Конечная позиция
            results.push(data[start_pos..end_pos].to_string()); // Добавляем подстроку в результаты
            start = end_pos; // Обновляем стартовую позицию для следующего поиска
        } else {
            break; // Если кавычка не найдена, выходим из цикла
        }
    }

    results // Возвращаем массив найденных подстрок
}

pub fn extract_expires(data: &str) -> i64 {
    let target = r#"Expires: "#; // Подстрока для поиска
    let start= data.find(target).unwrap();
    let start_pos = start + target.len(); // Позиция после подстроки
    let end = data[start_pos..].find('\n').unwrap();
    let end_pos = start_pos + end; // Конечная позиция
    let expires_time_string = data[start_pos..end_pos].to_string();

    let dt: DateTime<FixedOffset> = DateTime::parse_from_rfc2822(&expires_time_string.trim()).unwrap();
    let timestamp = dt.timestamp();
    return timestamp;
}