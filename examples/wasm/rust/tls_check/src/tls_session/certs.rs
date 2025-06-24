//use core::slice::SlicePattern;
use std::net;
use std::ops::AddAssign;
//use std::time::SystemTime;
use std::ops::Neg;

//use num_bigint::{BigInt, Sign};
use num_bigint::{BigInt, ToBigInt, Sign};
//use num_traits::One;
use std::iter::FromIterator;


pub const ROOT_CERT_FROM_ONLINE: [u8; 1382] = [48, 130, 5, 98, 48, 130, 4, 74, 160, 3, 2, 1, 2, 2, 16, 119, 189, 13, 108, 219, 54, 249, 26, 234, 33, 15, 196, 240,
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

// Методы для Tag
//impl Tag {

    //pub fn constructed(self) -> Tag {
        //Tag(self.0 | CLASS_CONSTRUCTED)
    //}

    // Установка бита контекстного специфического класса
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

// Стандартные комбинации тегов и классов
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
pub const PRINTABLE_STRING: u8 = 19u8;
pub const T61_STRING: u8 = 20u8;
pub const IA5_STRING: u8 = 22u8;
pub const UTC_TIME: u8 = 23u8;
pub const GENERALIZED_TIME: u8 = 24u8;
pub const GENERAL_STRING: u8 = 27u8;

// Предполагается, что String - это структура, которая обрабатывает данные ASN.1
#[derive(Debug, Clone)]
pub struct ASN1String(Vec<u8>); // Используется Vec<u8> для представления строки

impl ASN1String {


    pub fn read_asn1(&mut self, out: &mut ASN1String, tag: u8) -> bool {
        let mut t = 0u8;
        if !self.read_any_asn1(out, &mut t) || t != tag {
            return false;
        }
        true
    }

    // Чтение ASN.1 элемента
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

    // Чтение любого ASN.1
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

    // Проверка тега ASN.1
    pub fn peek_asn1_tag(&self, tag: u8) -> bool {
        self.0.is_empty().then(|| false).unwrap_or(self.0[0] == tag)
    }

    // Пропуск ASN.1
    pub fn skip_asn1(&mut self, tag: u8) -> bool {
        let mut unused = ASN1String(vec![]);
        self.read_asn1(&mut unused, tag)
    }

    // Чтение необязательного ASN.1
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

    // Пропуск необязательного ASN.1
    pub fn skip_optional_asn1(&mut self, tag: u8) -> bool {
        if !self.peek_asn1_tag(tag) {
            return true;
        }
        let mut unused = ASN1String(vec![]);
        self.read_asn1(&mut unused, tag)
    }

    // Чтение необязательного ASN.1 целого числа
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

    pub fn read_asn1_i64(&mut self, out: &mut i64) -> bool {
        let mut bytes = ASN1String{ 0: Vec::new()};
        // if !self.read_asn1(&mut bytes, INTEGER) || !check_asn1_integer(bytes) || !asn1_signed(out, &bytes) {
        if !self.read_asn1(&mut bytes, INTEGER) || !check_asn1_integer(&bytes.0) || !asn1_signed(out, &bytes.0) {
            return false;
        }
        return true;
    }




    // Чтение ASN.1 INTEGER в out
    //pub fn read_asn1_integer(&mut self, out: &mut dyn std::any::Any) -> bool { // pub fn read_asn1_integer(&mut self, out: &mut dyn std::any::Any) -> bool {
        // Пробуем получить указатель на тип числа
        //if let Some(out_int) = out.downcast_mut::<i64>() {
            //let mut i: i64 = 0;
            //if !self.read_asn1_int64(&mut i) {
                //return false;
            //}
            // *out_int = i; // Устанавливаем значение
            //return true;
        //} else if let Some(out_uint) = out.downcast_mut::<u64>() {
            //let mut u: u64 = 0;
            //if !self.read_asn1_uint64(&mut u) {
                //return false;
            //}
            // *out_uint = u; // Устанавливаем значение
            //return true;
        //} else if let Some(out_big) = out.downcast_mut::<BigInt>() {
            //return self.read_asn1_big_int(out_big);
        //} else if let Some(out_bytes) = out.downcast_mut::<Vec<u8>>() {
            //return self.read_asn1_bytes(out_bytes);
        //}

        //panic!("out does not point to an integer type");

    //}


    pub fn read_asn1_int64(&mut self, out: &mut i64) -> bool {
        let mut bytes = ASN1String{ 0: Vec::new()}; // Заполнение bytes должно происходить здесь
        if !self.read_asn1(&mut bytes, INTEGER) || !check_asn1_integer(&bytes.0) || !asn1_signed(out, &bytes.0) {
            return false;
        }
        true
    }

    pub fn read_asn1_uint64(&mut self, out: &mut u64) -> bool {
        let mut bytes = ASN1String{ 0: Vec::new()}; // Заполнение bytes должно происходить здесь
        if !self.read_asn1(&mut bytes, INTEGER) || !check_asn1_integer(&bytes.0) || !asn1_unsigned(out, &bytes.0) {
            return false;
        }
        true
    }

    //pub fn read_asn1_big_int(&mut self, out: &mut BigInt) -> bool {
        //let mut bytes = ASN1String{ 0: Vec::new()}; // Предполагается, что будет обработка, чтобы заполнить bytes
        //if !self.read_asn1(&mut bytes, INTEGER) || !check_asn1_integer(&bytes.0) {
            //return false;
        //}

        //if bytes.0[0] & 0x80 == 0x80 {
            // Отрицательное число.
            //let mut neg = bytes.0.iter().map(|b| !b).collect::<Vec<u8>>();
            //*out = BigInt::from_bytes_be(Sign::Plus,&neg); //out.set_bytes(&neg);
            //out.add_assign(&BigInt::from(1));
            //out.neg(); // out.negate();
        //} else {
            //*out = BigInt::from_bytes_be(Sign::Plus,&bytes.0);//out.set_bytes(&bytes);
        //}
        //true
    //}

    pub fn read_asn1_big_int(&mut self) -> Option<BigInt> {
        let mut bytes = ASN1String{ 0: Vec::new()}; // Предполагается, что будет обработка, чтобы заполнить bytes
        if !self.read_asn1(&mut bytes, INTEGER) || !check_asn1_integer(&bytes.0) {
            return None;
        }

        let big_one = BigInt::from(1);//BigInt::one();

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
        let mut bytes = ASN1String{ 0: Vec::new() }; // Инициализация вектора для хранения байтов
        if !self.read_asn1(&mut bytes, OBJECT_IDENTIFIER) || bytes.0.is_empty() {
            return false;
        }

        // В худшем случае, мы получаем два элемента из первого байта (который кодируется иначе),
        // а затем каждый varint — это один байт.
        let mut components = vec![0; bytes.0.len() + 1];

        // Первый varint - это 40*value1 + value2:
        // value1 может принимать значения 0, 1 и 2.
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
        let mut bytes = ASN1String{ 0: Vec::new()};  // Заполнение bytes должно происходить здесь
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
                return false; // Обработка конца данных
            }
            if i == 5 {
                return false; // Слишком много байтов
            }
            // Избежание переполнения int на 32-битной платформе
            if ret >= 1 << (31 - 7) {
                return false;
            }
            ret <<= 7;
            let b: u8 = self.read(1).unwrap()[0]; // Чтение одного байта

            // ITU-T X.690, секция 8.19.2:
            // Подидентификатор должен быть закодирован в минимально возможном количестве октетов,
            // то есть ведущий октет подидентификатора не должен иметь значение 0x80.
            if i == 0 && b == 0x80 {
                return false;
            }

            ret |= (b & 0x7f) as i32;
            if b & 0x80 == 0 {
                *out = ret;
                return true;
            }
        }
        false // усеченные данные
    }



    pub fn read_asn1_impl(&mut self, out: &mut ASN1String, out_tag: &mut u8, skip_header: bool) -> bool {
        if self.0.len() < 2 {
            return false;
        }

        let tag = self.0[0];
        let len_byte = self.0[1];

        if tag & 0x1f == 0x1f {
            // ITU-T X.690 section 8.1.2
            // Тег с частью 0x1f указывает на идентификатор с высоким номером тега.
            return false;
        }

        //if let Some(out_t) = out_tag {
            // *out_t = Tag(tag);
        //}
        *out_tag = tag;

        // ITU-T X.690 section 8.1.3
        let (length, header_len) = if len_byte & 0x80 == 0 {
            // Короткая длина (section 8.1.3.4), закодированная в битах 1-7.
            (u32::from(len_byte) + 2, 2)
        } else {
            // Длинная длина (section 8.1.3.5).
            let len_len = len_byte & 0x7f;

            if len_len == 0 || len_len > 4 || self.0.len() < (2 + len_len as usize) {
                return false;
            }

            let mut len_bytes = ASN1String(self.0[2..2 + len_len as usize].to_vec());
            let mut len32 = 0u32;
            if !len_bytes.read_unsigned(&mut len32, len_len as usize) {
                return false;
            }

            // ITU-T X.690 section 10.1 (DER length forms) требует кодирования длины
            // с минимальным числом октетов.
            if len32 < 128 {
                return false; // Длина должна была использовать короткое кодирование.
            }
            if (len32 >> ((len_len - 1) * 8)) == 0 {
                return false; // Ведущий октет равен 0.
            }

            let header_len = 2 + len_len as u32;
            if header_len + len32 < len32 {
                return false; // Переполнение.
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

    // Реализация чтения беззнакового целого числа из ASN.1
    // Для упрощения реализации функции, предполагается,
    // что строка достаточной длины и возвращает true на успех.
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

    // Прочитать n байтов, продвигая строку
    fn read(&mut self, n: usize) -> Option<Vec<u8>> {
        if self.0.len() < n || n == 0 {
            return None;
        }

        let v = self.0[..n].to_vec(); // Получаем срез и копируем его
        self.0.drain(..n); // Удаляем прочитанные байты из внутреннего вектора
        Some(v)
    }

    // Реализация чтения байтов из ASN.1
    // По аналогии с другим кодом, должна быть реализована.
    pub fn read_bytes(&mut self, out: &mut ASN1String, length: usize) -> bool {
        if let Some(v) = self.read(length) {
            *out = ASN1String{0:v}; // Копируем прочитанные байты в out
            true
        } else {
            false
        }
    }

    fn skip(&mut self, length: usize) -> bool {
        // Реализация пропуска определенного количества байтов
        if length <= self.0.len() {
            self.0.drain(..length);
            true
        } else {
            false
        }
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
    // Сдвиг для расширения знака результата.
    *out <<= 64 - (length as u8 * 8);
    *out >>= 64 - (length as u8 * 8);
    true
}

pub fn asn1_unsigned(out: &mut u64, n: &[u8]) -> bool {
    let length = n.len();
    if length > 9 || (length == 9 && n[0] != 0) {
        // Слишком велико для uint64.
        return false;
    }
    if n[0] & 0x80 != 0 {
        // Отрицательное число.
        return false;
    }
    for &byte in n {
        *out <<= 8;
        *out |= byte as u64;
    }
    true
}

// Проверка на корректность ASN.1 INTEGER
pub fn check_asn1_integer(bytes: &[u8]) -> bool {
    if bytes.is_empty() {
        // INTEGER кодируется как минимум одним октетом
        return false;
    }
    if bytes.len() == 1 {
        return true;
    }
    if (bytes[0] == 0 && (bytes[1] & 0x80) == 0) || (bytes[0] == 0xff && (bytes[1] & 0x80) == 0x80) {
        // Значение не минимально закодировано
        return false;
    }
    return true;
}

// Представляет набор AttributeTypeAndValue
//#[derive(Debug, Clone)]
//pub struct AttributeTypeAndValueSET {
    //pub rtype: ObjectIdentifier,
    //pub value: Vec<Vec<AttributeTypeAndValue>>, // Вектор векторов
//}

// Представляет расширение
#[derive(Debug, Clone)]
pub struct Extension {
    pub id: Vec<i32>, // //pub id: ObjectIdentifier,
    pub critical: Option<bool>, // Используем Option для обозначения опционального поля
    pub value: Vec<u8>,
}

// Представляет отличительное имя X.509
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
    //pub names: Vec<AttributeTypeAndValue>, // Все разобранные атрибуты
    //pub extra_names: Vec<AttributeTypeAndValue>, // Атрибуты, копируемые в любые сериализованные имена
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
        }
    }
}

#[derive(Debug)]
struct Certificate {
    raw: Vec<u8>,                             // Complete ASN.1 DER content
    raw_tbs_certificate: Vec<u8>,             // Certificate part of raw ASN.1 DER content
    raw_subject_public_key_info: Vec<u8>,    // DER encoded SubjectPublicKeyInfo
    raw_subject: Vec<u8>,                     // DER encoded Subject
    raw_issuer: Vec<u8>,                      // DER encoded Issuer

    signature: Vec<u8>,
    //signature_algorithm: SignatureAlgorithm,

    //public_key_algorithm: PublicKeyAlgorithm,
    //public_key: Option<Box<dyn PublicKey>>,    // Using trait object for dynamic dispatch

    version: i64,
    serial_number: BigInt,     // serial_number: Option<BigInt>,          // Type for big integers
    issuer: Name,
    subject: Name,

    /*not_before: SystemTime,                    // Using SystemTime for time representation
    not_after: SystemTime,*/
    //key_usage: KeyUsage,

    extensions: Vec<Extension>,          // Raw X.509 extensions
    extra_extensions: Vec<Extension>,    // Extensions to be copied raw into any marshaled certificates
    //unhandled_critical_extensions: Vec<asn1::ObjectIdentifier>, // List of extension IDs not fully processed

    //ext_key_usage: Vec<ExtKeyUsage>,           // Sequence of extended key usages
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

fn parse_certificate(der: &[u8]) -> Certificate { // fn parse_certificate(der: &[u8]) -> Result<Certificate, Box<dyn Error>> {
    //
    let mut cert = Certificate {
        raw: Vec::new(),
        raw_tbs_certificate: Vec::new(),
        raw_subject_public_key_info: Vec::new(),
        raw_subject: Vec::new(),
        raw_issuer: Vec::new(),
        signature: Vec::new(),

        //signature_algorithm: SignatureAlgorithm::default(), // Default value
        //public_key_algorithm: PublicKeyAlgorithm::default(), // Default value
        //public_key: None,
        version: 0,
        serial_number: BigInt::default(),
        issuer: Name::default(), // Assuming a default implementation
        subject: Name::default(),

       /*not_before: SystemTime::now(),
         not_after: SystemTime::now(),*/

        //key_usage: KeyUsage::default(), // Default value

        extensions: Vec::new(),
        extra_extensions: Vec::new(),

        //unhandled_critical_extensions: Vec::new(),
        //ext_key_usage: Vec::new(),

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

    /*let mut input = ASN1String{ 0: der.to_vec()};
    // we read the SEQUENCE including length and tag bytes so that
	// we can populate Certificate.Raw, before unwrapping the
	// SEQUENCE so it can be operated on

    // Чтение ASN.1 элемента
    let mut input1 = input.clone();

    if !input.read_asn1_element(&mut input1, SEQUENCE) {
        //return Err("x509: malformed certificate".into());
        panic!("x509: malformed certificate");
    }
    cert.raw = input1.0.clone();

    // Чтение основного элемента ASN.1
    if !input1.read_asn1(&mut input, SEQUENCE) {
        //return Err("x509: malformed certificate".into());
        panic!("x509: malformed certificate");
    }

    let mut tbs = ASN1String{ 0: Vec::new()}; // Подходящий тип для tbs

    if !input.read_asn1_element(&mut tbs, SEQUENCE) {
        //return Err("x509: malformed tbs certificate".into());
        panic!("x509: malformed tbs certificate");
    }
    cert.raw_tbs_certificate = tbs.0.clone();

    let mut tbs1 = tbs.clone();
    if !tbs.read_asn1(&mut tbs1, SEQUENCE) {
        //return Err("x509: malformed version".into());
        panic!("x509: malformed tbs certificate");
    }

    // Чтение версии
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
    if cert.version > 3 {
        //return Err("x509: invalid version".into());
        panic!("x509: invalid version");
    }

    // Чтение серийного номера
    //let mut serial = BigInt::default(); // Эквивалент создания нового большого числа
    //if !tbs1.read_asn1_big_int(&mut serial) { // if !tbs1.read_asn1_integer(&mut tbs, &serial) {
        //return Err("x509: malformed serial number".into());
        //panic!("x509: malformed serial number");
    //}
    match tbs1.read_asn1_big_int(){
        Some(serial) => cert.serial_number = serial,
        None => panic!("x509: malformed serial number"),
    }
    //cert.serial_number = serial;*/

    /*
    // Чтение идентификатора алгоритма подписи
    let mut sig_ai_seq = ASN1String{ 0: Vec::new()};
    if !tbs.read_asn1(&mut sig_ai_seq, SEQUENCE) {
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
    if outer_sig_ai_seq.0.cmp(&sig_ai_seq.0).is_eq() { // if outer_sig_ai_seq != sig_ai_seq {
        //return Err("x509: inner and outer signature algorithm identifiers don't match".into());
        panic!("x509: inner and outer signature algorithm identifiers don't match");
    }

    let sig_ai = parse_ai(sig_ai_seq.0)?; // Обработка идентификатора алгоритма
    cert.signature_algorithm = get_signature_algorithm_from_ai(sig_ai);

    // Чтение секвенции издателя
    let issuer_seq = Vec::new();
    if !read_asn1_element(&mut tbs, &mut issuer_seq) {
        return Err("x509: malformed issuer".into());
    }
    cert.raw_issuer = issuer_seq.clone();
    let issuer_rdns = parse_name(issuer_seq)?;
    cert.issuer.fill_from_rdn_sequence(issuer_rdns); // Предполагая, что эта функция существует

    Ok(cert)*/

    cert

}

// Пример реализации функций чтения ASN.1
//fn read_asn1_element(input: &mut Vec<u8>) -> bool {
    // Здесь будет логика обработки ASN.1 элементов
    //true // Обязательно замените на реальную логику
//}

//fn read_asn1(input: &mut Vec<u8>) -> bool {
    // Здесь будет логика обработки ASN.1
    //true // Обязательно замените на реальную логику
//}

//fn read_optional_asn1_integer(input: &mut Vec<u8>, version: &mut i32```rust
//) -> bool {
    // Здесь будет логика чтения необязательного ASN.1 целого числа
    //true // Обязательно замените на реальную логику
//}


#[derive(Debug, Clone)]
pub struct RawValue {
    class: i32,       // ASN.1 class (e.g. Universal, Application, Context-specific, Private)
    tag: i32,         // ASN.1 tag
    is_compound: bool, // Indicates if the RawValue is a compound type
    bytes: Vec<u8>,   // Undecoded bytes of the ASN.1 object
    full_bytes: Vec<u8>, // Complete bytes including the tag and length
}

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

pub fn check_certs(certs_chain: &[u8]) -> bool {
    // extract
    // divide input string into three slices

    let len_of_certs_chain = (certs_chain[0] as usize)*65536 + (certs_chain[1] as usize)*256 + (certs_chain[2] as usize);
   
    if len_of_certs_chain+1 != certs_chain.len() {
        return false;
    }

    let len_of_leaf_cert = (certs_chain[3] as usize)*65536 + (certs_chain[4] as usize)*256 + (certs_chain[5] as usize);

    let leaf_cert_slice = &certs_chain[6..len_of_leaf_cert+6];

    let leaf_cert = parse_certificate(leaf_cert_slice); // leafCert, err := x509.ParseCertificate(leafCertSlice)
    //if leaf_cert.not_after.Before(time.Now()) || leaf_cert.not_before.After(time.Now()) {
        //false
    //}

/*
    let start_index = len_of_leaf_cert + 8;
    let len_of_internal_cert = (certs_chain[start_index] as usize)*65536 + (certs_chain[start_index+1] as usize)*256 + (certs_chain[start_index+2] as usize);

    let internal_cert_slice = &certs_chain[start_index + 3..start_index + len_of_internal_cert + 3];

    let internal_cert = parse_certificate(internal_cert_slice); // internalCert, err := x509.ParseCertificate(internalCertSlice)


    //if internalCert.NotAfter.Before(time.Now()) || internalCert.NotBefore.After(time.Now()) {
        //return false
    //}
    let start_index = start_index + 3 + len_of_internal_cert + 2;

    let len_of_root_cert = (certs_chain[start_index] as usize)*65536 + (certs_chain[start_index+1] as usize)*256 + (certs_chain[start_index+2] as usize);
    let root_cert_slice = &certs_chain[start_index + 3..start_index + len_of_root_cert+3];

    //rootCert, err := x509.ParseCertificate(rootCertSlice)
    // if err != nil {
    // return false
    // }*/

    return true;
}