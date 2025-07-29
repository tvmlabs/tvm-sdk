// Copyright (C) 2019-2021 TON Labs. All Rights Reserved.
//
// Licensed under the SOFTWARE EVALUATION License (the "License"); you may not
// use this file except in compliance with the License.
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific TON DEV software governing permissions and
// limitations under the License.
use ark_std::rand::rngs::StdRng;
use std::collections::HashSet;
use std::time::Duration;
use std::time::Instant;

use rand::RngCore;
use tvm_abi::TokenValue;
use tvm_abi::contract::ABI_VERSION_2_4;
use tvm_types::BuilderData;
use tvm_types::Cell;
use tvm_types::ExceptionCode;
use tvm_types::IBitstring;
use tvm_types::SliceData;

use crate::error::TvmError;
use crate::executor::engine::Engine;
use crate::executor::gas::gas_state::Gas;
use crate::executor::math::execute_xor;
use crate::executor::math::DivMode;
use crate::executor::serialize_currency_collection;
use crate::executor::token::execute_run_wasm;
use crate::executor::token::execute_run_wasm_concat_multiarg;
use crate::executor::types::Instruction;
use crate::executor::types::InstructionOptions;
use crate::executor::zk::execute_poseidon_zk_login;
use crate::executor::zk::execute_vergrth16;
use crate::stack::Stack;
use crate::stack::StackItem;
use crate::stack::integer::IntegerData;
use crate::stack::integer::behavior::OperationBehavior;
use crate::stack::integer::behavior::Quiet;
use crate::stack::integer::behavior::Signaling;
use crate::stack::savelist::SaveList;
use crate::types::Status;
use crate::utils::pack_data_to_cell;
use crate::utils::unpack_data_from_cell;

use crate::utils::pack_string_to_cell;
use crate::utils::unpack_string_from_cell;

use crate::executor::zk_stuff::utils::gen_address_seed;
use crate::executor::zk_stuff::utils::get_zk_login_address;

use ed25519_dalek::Signer;
use fastcrypto::ed25519::Ed25519KeyPair;
use fastcrypto::traits::KeyPair;
use fastcrypto::traits::ToFromBytes;

use crate::executor::zk_stuff::zk_login::CanonicalSerialize;
use crate::executor::zk_stuff::zk_login::JWK;
use crate::executor::zk_stuff::zk_login::JwkId;
use crate::executor::zk_stuff::zk_login::OIDCProvider;
use crate::executor::zk_stuff::zk_login::ZkLoginInputs;
use crate::executor::zk_stuff::curve_utils::Bn254FrElement;
use crate::executor::zk_stuff::error::ZkCryptoError;

use serde::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

use std::collections::HashMap;

use base64::decode;
use base64ct::Encoding as bEncoding;

use rand::Rng;
use rand::SeedableRng;
use rand::thread_rng;

use crate::executor::deserialization::execute_schkrefs;
use crate::executor::math::execute_divmod;

use num_bigint::BigInt;
use num_bigint::BigUint;
use std::str::FromStr;
use num_traits::FromPrimitive;

static DEFAULT_CAPABILITIES: u64 = 0x572e;

fn read_boc(filename: &str) -> Vec<u8> {
    let mut bytes = Vec::new();
    let mut file = std::fs::File::open(filename).unwrap();
    std::io::Read::read_to_end(&mut file, &mut bytes).unwrap();
    bytes
}

fn load_boc(filename: &str) -> tvm_types::Cell {
    let bytes = read_boc(filename);
    tvm_types::read_single_root_boc(bytes).unwrap()
}

pub fn secret_key_from_integer_map(key_data: HashMap<String, u8>) -> Vec<u8> {
    let mut vec: Vec<u8> = Vec::new();
    for i in 0..=31 {
        if let Some(value) = key_data.get(&i.to_string()) {
            vec.push(value.clone());
        }
    }
    return vec;
}

#[derive(Debug, Deserialize)]
pub struct EphemeralKeyPair {
    pub keypair: Keypair,
}

#[derive(Debug, Deserialize)]
pub struct Keypair {
    pub public_key: HashMap<String, u8>,
    pub secret_key: HashMap<String, u8>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ZkProofs {
    pub proof_points: ProofPoints,
    pub iss_base64_details: IssBase64Details,
    pub header_base64: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProofPoints {
    pub a: Vec<String>,
    pub b: Vec<Vec<String>>,
    pub c: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IssBase64Details {
    pub value: String,
    pub index_mod4: i32,
}

#[derive(Debug, Deserialize)]
pub struct JwtData {
    pub jwt: String,
    pub user_pass_to_int_format: String,
    pub ephemeral_key_pair: EphemeralKeyPair,
    pub zk_addr: String,
    pub zk_proofs: ZkProofs,
    pub extended_ephemeral_public_key: String,
    pub modulus: String,
    pub kid: String,
    pub max_epoch: u64,
    pub verification_key_id: u32
}

 #[derive(Debug, Deserialize)]
pub struct JwtDataDecodedPart1 {
    pub alg: String,
    pub kid: String,
     pub typ: String,
}

#[derive(Debug, Deserialize)]
pub struct JwtDataDecodedPart2Google {
    pub iss: String,
    pub azp: String,
    pub aud: String,
    pub sub: String,
    pub nonce: String,
    pub nbf: u32,
    pub iat: u32,
    pub exp: u32,
    pub jti: String,
}

pub const TEST_AUTH_DATA_1: &str = "{\"jwt\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjMyM2IyMTRhZTY5NzVhMGYwMzRlYTc3MzU0ZGMwYzI1ZDAzNjQyZGMiLCJ0eXAiOiJKV1QifQ.eyJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLCJhenAiOiIyMzI2MjQwODUxOTEtdjF0cTIwZmcxa2RoaGd2YXQ2c2FqN2pmMGhkODIzM3IuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJhdWQiOiIyMzI2MjQwODUxOTEtdjF0cTIwZmcxa2RoaGd2YXQ2c2FqN2pmMGhkODIzM3IuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJzdWIiOiIxMTI4OTc0Njg2MjY3MTY2MjYxMDMiLCJub25jZSI6ImJ4bW5KVzMxcnV6S01HaXIwMVlQR1lMMHhEWSIsIm5iZiI6MTcxNTY4NzAzNiwiaWF0IjoxNzE1Njg3MzM2LCJleHAiOjE3MTU2OTA5MzYsImp0aSI6IjliNjAxZDI1ZjAwMzY0MGMyODg5YTJhMDQ3Nzg5MzgyY2IxY2ZlODcifQ.rTa9KA9HoYm04Agj71D0kDkvsCZ35SeeihBGbABYckBRxaUlCy6LQ-sEaVOTgvnL_DgVn7hx8g3sSmnhJ9kHzj5e6gtUoxoWAe8PuGyK2bmqhmPrQMeEps9f6m2EToQCIA_Id4fGCjSCktjJBi47QHT_Dhe6isHdKk1pgSshOyvCF1VjIvyyeGY5iWQ4cIRBMQNlNBT11o6T01SY6B9DtiiFN_0-ok5taIjQgtMNG6Cwr3tCnqXftuGGQrHlx15y8VgCPODYi-wOtvUbzI2yfx53PmRD_L8O50cMNCrCRE3yYR5MNOu1LlQ_EACy5UFsCJR35xRz84nv-6Iyrufx1g\",\"user_pass_to_int_format\":\"981021191041055255531141165751\",\"ephemeral_key_pair\":{\"keypair\":{\"public_key\":{\"0\":155,\"1\":147,\"2\":37,\"3\":82,\"4\":183,\"5\":109,\"6\":227,\"7\":144,\"8\":85,\"9\":248,\"10\":20,\"11\":45,\"12\":92,\"13\":103,\"14\":160,\"15\":221,\"16\":101,\"17\":44,\"18\":30,\"19\":86,\"20\":96,\"21\":85,\"22\":24,\"23\":224,\"24\":106,\"25\":63,\"26\":13,\"27\":130,\"28\":8,\"29\":119,\"30\":247,\"31\":67},\"secret_key\":{\"0\":192,\"1\":16,\"2\":35,\"3\":54,\"4\":100,\"5\":14,\"6\":88,\"7\":217,\"8\":164,\"9\":21,\"10\":154,\"11\":233,\"12\":248,\"13\":208,\"14\":188,\"15\":4,\"16\":52,\"17\":244,\"18\":125,\"19\":103,\"20\":99,\"21\":26,\"22\":225,\"23\":60,\"24\":140,\"25\":75,\"26\":228,\"27\":157,\"28\":137,\"29\":220,\"30\":1,\"31\":65,\"32\":155,\"33\":147,\"34\":37,\"35\":82,\"36\":183,\"37\":109,\"38\":227,\"39\":144,\"40\":85,\"41\":248,\"42\":20,\"43\":45,\"44\":92,\"45\":103,\"46\":160,\"47\":221,\"48\":101,\"49\":44,\"50\":30,\"51\":86,\"52\":96,\"53\":85,\"54\":24,\"55\":224,\"56\":106,\"57\":63,\"58\":13,\"59\":130,\"60\":8,\"61\":119,\"62\":247,\"63\":67}}},\"zk_addr\":\"0x290623ea2fe67e77502c931e015e910720b59cf99994bfe872da851245a6adb8\",\"zk_proofs\":{\"proofPoints\":{\"a\":[\"4240296169193969312736577528388333411353554120022978085193148043577551744781\",\"5805161066003598301896048908428560240907086333477483881772048922050706263054\",\"1\"],\"b\":[[\"12834391737669124973917765536412427456985620342194191639017091262766903638891\",\"17565396762846717347409742387259908749145765976354144805005547481529916658455\"],[\"10704310067924910937030159163683742097178285875135929496314190235513445131794\",\"5158907077493606386023392148737817037260820737072162547798816810512684527243\"],[\"1\",\"0\"]],\"c\":[\"1422540522119231707130773229384414857146368773886805969586218853559909475064\",\"8843079196273712399340537238369227864378150337693574970239878271571912585171\",\"1\"]},\"issBase64Details\":{\"value\":\"yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC\",\"indexMod4\":1},\"headerBase64\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjMyM2IyMTRhZTY5NzVhMGYwMzRlYTc3MzU0ZGMwYzI1ZDAzNjQyZGMiLCJ0eXAiOiJKV1QifQ\"},\"extended_ephemeral_public_key\":\"AJuTJVK3beOQVfgULVxnoN1lLB5WYFUY4Go/DYIId/dD\", \"verification_key_id\": 0, \"modulus\": \"xjWd1j8GmmWzuz732haG9HECXsSZBvxOBLph3FQhk_tplhWloI1ywx-RdopUZt1lndbOM9n99lZJkpQyNJ1sdy7JFgYLjqj-wtHdEaQlBGEQtmkW8zUjr_N3bmpsxGbPzOzlKe3qddtoxXvn9rI_RvHfJD1YY-6kayQeyPOBz_4ML1lvI_JHV-Bb1MSmSk3WaAh5PzeqleusmUT87Gqfu02cOPrY8cwugqo65D6-wzAEeVvceV8-c36TMoLU5csU05GBVplgd6Ouuw35ZsETG4si4QQJztC3KsZ4jhYM-aJ3jeFPt0r3cQooiXdZBp3JkXSpE-UUaOVPsXo7WiVmww\", \"kid\": \"323b214ae6975a0f034ea77354dc0c25d03642dc\",\"max_epoch\":142}";
pub const TEST_AUTH_DATA_2: &str = "{\"jwt\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjMyM2IyMTRhZTY5NzVhMGYwMzRlYTc3MzU0ZGMwYzI1ZDAzNjQyZGMiLCJ0eXAiOiJKV1QifQ.eyJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLCJhenAiOiIyMzI2MjQwODUxOTEtdjF0cTIwZmcxa2RoaGd2YXQ2c2FqN2pmMGhkODIzM3IuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJhdWQiOiIyMzI2MjQwODUxOTEtdjF0cTIwZmcxa2RoaGd2YXQ2c2FqN2pmMGhkODIzM3IuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJzdWIiOiIxMTI4OTc0Njg2MjY3MTY2MjYxMDMiLCJub25jZSI6IjJKd0VMbjJfUV9Rd0VsTC1rWTFPRnFqdXZCMCIsIm5iZiI6MTcxNTY4NzAyOSwiaWF0IjoxNzE1Njg3MzI5LCJleHAiOjE3MTU2OTA5MjksImp0aSI6ImU2YjM1ZjJmNmFkNjIzOWEwMDAxMTJiMWI5YWI2MWQ0MjRkMGM1OTIifQ.QcrEDE9qmPZKX83nU3Tx2BN8fsinb_mmXkO1Qf7Uv1QTd0NjirSeu7C4Vn9WDNWDaIR-BgCfhOlkwMQPljcahqC4AN43N_66tvbEsXjtEdFejslXrGG4D_BEKvtmD7_WkW388LyU2PxKgtdDfpYFgmuT6wTM2TO5dTbrGrDyn88q3pkPfefC5a8Wi1V6zECfFdSV-pKQlxtPaImi7s3CKAUMDu1n-jcT-Ho2aTgrWKAzhXE56tgEWOpXQO06eJsWCSOqoZSLYtatTrZr4d38U7QRQiNlH-ydHv4zXt1tixLLJ0wvPx-dQaCnCl1kW1orYkJGFfHgjx6A9z5Ol4afuw\",\"user_pass_to_int_format\":\"101119106102103\",\"ephemeral_key_pair\":{\"keypair\":{\"public_key\":{\"0\":194,\"1\":38,\"2\":203,\"3\":255,\"4\":219,\"5\":127,\"6\":105,\"7\":129,\"8\":234,\"9\":222,\"10\":71,\"11\":169,\"12\":108,\"13\":94,\"14\":28,\"15\":48,\"16\":111,\"17\":221,\"18\":113,\"19\":110,\"20\":5,\"21\":226,\"22\":19,\"23\":230,\"24\":232,\"25\":67,\"26\":255,\"27\":179,\"28\":6,\"29\":10,\"30\":209,\"31\":63},\"secret_key\":{\"0\":44,\"1\":32,\"2\":251,\"3\":184,\"4\":109,\"5\":252,\"6\":105,\"7\":67,\"8\":208,\"9\":111,\"10\":86,\"11\":214,\"12\":192,\"13\":135,\"14\":169,\"15\":48,\"16\":162,\"17\":36,\"18\":216,\"19\":145,\"20\":232,\"21\":64,\"22\":17,\"23\":14,\"24\":29,\"25\":56,\"26\":39,\"27\":118,\"28\":143,\"29\":250,\"30\":31,\"31\":66,\"32\":194,\"33\":38,\"34\":203,\"35\":255,\"36\":219,\"37\":127,\"38\":105,\"39\":129,\"40\":234,\"41\":222,\"42\":71,\"43\":169,\"44\":108,\"45\":94,\"46\":28,\"47\":48,\"48\":111,\"49\":221,\"50\":113,\"51\":110,\"52\":5,\"53\":226,\"54\":19,\"55\":230,\"56\":232,\"57\":67,\"58\":255,\"59\":179,\"60\":6,\"61\":10,\"62\":209,\"63\":63}}},\"zk_addr\":\"0x9d28c04a423b33d6901065b2e23440d80c963e2d8cf60619aed131cf302a3345\",\"zk_proofs\":{\"proofPoints\":{\"a\":[\"10113442204684515220664612836724727112601024759319365467272456423129044788607\",\"1622056145268645528934658046911045406324940175278473377024147189407527440953\",\"1\"],\"b\":[[\"16638441944380099215425740101953753038808466958852552979180365845498468757656\",\"15160836857346434734063515954042830497610079883703780011464867547889770445695\"],[\"18562910453341688699790780964434211467815845944672185772065803860963710445937\",\"8200691834141582017549140597895023392490964486044036655696113278873832146838\"],[\"1\",\"0\"]],\"c\":[\"4229037146526046139176767312447148765936834700862335953317784850097077554287\",\"14155516063621997063825085002662503289554536312724791903045026922766401869119\",\"1\"]},\"issBase64Details\":{\"value\":\"yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC\",\"indexMod4\":1},\"headerBase64\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjMyM2IyMTRhZTY5NzVhMGYwMzRlYTc3MzU0ZGMwYzI1ZDAzNjQyZGMiLCJ0eXAiOiJKV1QifQ\"},\"extended_ephemeral_public_key\":\"AMImy//bf2mB6t5HqWxeHDBv3XFuBeIT5uhD/7MGCtE/\", \"verification_key_id\": 0, \"modulus\": \"xjWd1j8GmmWzuz732haG9HECXsSZBvxOBLph3FQhk_tplhWloI1ywx-RdopUZt1lndbOM9n99lZJkpQyNJ1sdy7JFgYLjqj-wtHdEaQlBGEQtmkW8zUjr_N3bmpsxGbPzOzlKe3qddtoxXvn9rI_RvHfJD1YY-6kayQeyPOBz_4ML1lvI_JHV-Bb1MSmSk3WaAh5PzeqleusmUT87Gqfu02cOPrY8cwugqo65D6-wzAEeVvceV8-c36TMoLU5csU05GBVplgd6Ouuw35ZsETG4si4QQJztC3KsZ4jhYM-aJ3jeFPt0r3cQooiXdZBp3JkXSpE-UUaOVPsXo7WiVmww\", \"kid\": \"323b214ae6975a0f034ea77354dc0c25d03642dc\",\"max_epoch\":142}";
pub const TEST_AUTH_DATA_3: &str = "{\"jwt\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjMyM2IyMTRhZTY5NzVhMGYwMzRlYTc3MzU0ZGMwYzI1ZDAzNjQyZGMiLCJ0eXAiOiJKV1QifQ.eyJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLCJhenAiOiIyMzI2MjQwODUxOTEtdjF0cTIwZmcxa2RoaGd2YXQ2c2FqN2pmMGhkODIzM3IuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJhdWQiOiIyMzI2MjQwODUxOTEtdjF0cTIwZmcxa2RoaGd2YXQ2c2FqN2pmMGhkODIzM3IuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJzdWIiOiIxMTI4OTc0Njg2MjY3MTY2MjYxMDMiLCJub25jZSI6IkFadlVHUkI5MU5VZmdnYVV5bUdCQU9kSmM2ayIsIm5iZiI6MTcxNTY4NzAyMiwiaWF0IjoxNzE1Njg3MzIyLCJleHAiOjE3MTU2OTA5MjIsImp0aSI6ImZiNGNhMzdjOGE5MjEzOTFjZTE2ZDQwNmE2NmVmYjA1MTQxNTg5YjYifQ.C7zNP2sxRMF62irwNjO2y_JVMjYLqGk6sAWy0rKoXswa7SA6KhPrWocMAB2GKaQW-CeqUzMJdypgJz1RcMzmOWg30cv4diEgqBSM1I1ocOI5ivRE2Atj8g-Oj2uAm_DBvuJBLzTA6wfb34QTasOTZqLsMyoaQavxUprzPi-1z-MUE-darDjZ-IkWu7SctdEzNhSuUfQPJo_sbN5_38dQm300plXK-9iJgDxMWmT4NPO91hSQaGKbBm_euMI-fBAfYwARMnlaTETvSiCNSAyzphNrBi9kU49BFi5X04GoIkSW4zFwb74OeFbL49_14AZZ9Z2Mw7EPQ9sAAjzanxPUfA\",\"user_pass_to_int_format\":\"98118101102104106100\",\"ephemeral_key_pair\":{\"keypair\":{\"public_key\":{\"0\":219,\"1\":225,\"2\":68,\"3\":197,\"4\":249,\"5\":59,\"6\":249,\"7\":200,\"8\":218,\"9\":242,\"10\":184,\"11\":214,\"12\":247,\"13\":159,\"14\":9,\"15\":162,\"16\":60,\"17\":174,\"18\":162,\"19\":13,\"20\":111,\"21\":5,\"22\":61,\"23\":179,\"24\":155,\"25\":167,\"26\":207,\"27\":6,\"28\":174,\"29\":163,\"30\":23,\"31\":23},\"secret_key\":{\"0\":28,\"1\":117,\"2\":37,\"3\":14,\"4\":166,\"5\":188,\"6\":125,\"7\":36,\"8\":70,\"9\":193,\"10\":162,\"11\":142,\"12\":79,\"13\":218,\"14\":210,\"15\":131,\"16\":217,\"17\":32,\"18\":88,\"19\":246,\"20\":195,\"21\":214,\"22\":135,\"23\":80,\"24\":27,\"25\":198,\"26\":131,\"27\":31,\"28\":3,\"29\":240,\"30\":199,\"31\":129,\"32\":219,\"33\":225,\"34\":68,\"35\":197,\"36\":249,\"37\":59,\"38\":249,\"39\":200,\"40\":218,\"41\":242,\"42\":184,\"43\":214,\"44\":247,\"45\":159,\"46\":9,\"47\":162,\"48\":60,\"49\":174,\"50\":162,\"51\":13,\"52\":111,\"53\":5,\"54\":61,\"55\":179,\"56\":155,\"57\":167,\"58\":207,\"59\":6,\"60\":174,\"61\":163,\"62\":23,\"63\":23}}},\"zk_addr\":\"0xeccbb76b41c1fd5e19950f0c005e5d2a2596b9cc510e98b6f69bb3cf590b3cf8\",\"zk_proofs\":{\"proofPoints\":{\"a\":[\"11651445672013969095011012560101085321682180365624939394143647198080899422642\",\"1099774502834574451399947043208869188329872135932897351612210181871486714260\",\"1\"],\"b\":[[\"4095258550782358133185755302461547336434190495389756275789648565352453295275\",\"11290282088300413285686821769617771231670721476484846359206004074570380534935\"],[\"10130196410049440247754977520268298700433580296307256932070052957562923587210\",\"18578315450133100598244014262861961858129311260491371986249505812898194068790\"],[\"1\",\"0\"]],\"c\":[\"3621803486710965065098877836422521469652420656514094958857631583114966034063\",\"10775419351495516109888010278620848514990288696189982169937651175162131341248\",\"1\"]},\"issBase64Details\":{\"value\":\"yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC\",\"indexMod4\":1},\"headerBase64\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjMyM2IyMTRhZTY5NzVhMGYwMzRlYTc3MzU0ZGMwYzI1ZDAzNjQyZGMiLCJ0eXAiOiJKV1QifQ\"},\"extended_ephemeral_public_key\":\"ANvhRMX5O/nI2vK41vefCaI8rqINbwU9s5unzwauoxcX\", \"verification_key_id\": 0, \"modulus\": \"xjWd1j8GmmWzuz732haG9HECXsSZBvxOBLph3FQhk_tplhWloI1ywx-RdopUZt1lndbOM9n99lZJkpQyNJ1sdy7JFgYLjqj-wtHdEaQlBGEQtmkW8zUjr_N3bmpsxGbPzOzlKe3qddtoxXvn9rI_RvHfJD1YY-6kayQeyPOBz_4ML1lvI_JHV-Bb1MSmSk3WaAh5PzeqleusmUT87Gqfu02cOPrY8cwugqo65D6-wzAEeVvceV8-c36TMoLU5csU05GBVplgd6Ouuw35ZsETG4si4QQJztC3KsZ4jhYM-aJ3jeFPt0r3cQooiXdZBp3JkXSpE-UUaOVPsXo7WiVmww\", \"kid\": \"323b214ae6975a0f034ea77354dc0c25d03642dc\",\"max_epoch\":142}";
pub const TEST_AUTH_DATA_4: &str = "{\"jwt\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjMyM2IyMTRhZTY5NzVhMGYwMzRlYTc3MzU0ZGMwYzI1ZDAzNjQyZGMiLCJ0eXAiOiJKV1QifQ.eyJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLCJhenAiOiIyMzI2MjQwODUxOTEtdjF0cTIwZmcxa2RoaGd2YXQ2c2FqN2pmMGhkODIzM3IuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJhdWQiOiIyMzI2MjQwODUxOTEtdjF0cTIwZmcxa2RoaGd2YXQ2c2FqN2pmMGhkODIzM3IuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJzdWIiOiIxMTI4OTc0Njg2MjY3MTY2MjYxMDMiLCJub25jZSI6Inh4VjZnc3RReGY2WW5Kb0QyMXMtZkNWdm9ESSIsIm5iZiI6MTcxNTY4NzAxNSwiaWF0IjoxNzE1Njg3MzE1LCJleHAiOjE3MTU2OTA5MTUsImp0aSI6ImQzOWJjMTUyOWVhODMwNTRiNGU0MDRlNDRhMmE4ZWRjOTJlZWFiYWYifQ.u23WQFEtc4TldMtNqrU7DiYdL33X2QySNxueCW79LQHc00P7g-Pu7xPX0XK_TLxP6ReZEdpdCmjfG6g--XBYXh313FKcqVhcrtKdBE06jf5acAf4fQ3TVzG5CFWqjISRhLL0eGjX20DZm8drrSFYTgfWPl9ANo6TV2IFF6BR9TOO_flxzmXPRVvER9ZA4QO52JCqagVYBw4bFZcUebiN_KXYuXOYWzUAiHM7lKUdVKoCte9JDKnTfRNg3r-i5tt5Oiovswwh9jubQd5c8nCQckQ8Fj9T5nPmlfPtF282kfd76xlHckvL94mM3HUKNuFrxeiFX07f_5Ff7NxvQ3QPgw\",\"user_pass_to_int_format\":\"118102101104106\",\"ephemeral_key_pair\":{\"keypair\":{\"public_key\":{\"0\":25,\"1\":68,\"2\":102,\"3\":77,\"4\":86,\"5\":118,\"6\":203,\"7\":106,\"8\":41,\"9\":192,\"10\":205,\"11\":144,\"12\":20,\"13\":158,\"14\":42,\"15\":167,\"16\":18,\"17\":30,\"18\":27,\"19\":103,\"20\":51,\"21\":222,\"22\":226,\"23\":224,\"24\":168,\"25\":111,\"26\":16,\"27\":214,\"28\":128,\"29\":165,\"30\":10,\"31\":183},\"secret_key\":{\"0\":204,\"1\":128,\"2\":233,\"3\":135,\"4\":233,\"5\":64,\"6\":127,\"7\":97,\"8\":231,\"9\":135,\"10\":123,\"11\":149,\"12\":126,\"13\":145,\"14\":173,\"15\":252,\"16\":33,\"17\":141,\"18\":251,\"19\":181,\"20\":223,\"21\":9,\"22\":77,\"23\":32,\"24\":19,\"25\":187,\"26\":3,\"27\":180,\"28\":110,\"29\":49,\"30\":114,\"31\":167,\"32\":25,\"33\":68,\"34\":102,\"35\":77,\"36\":86,\"37\":118,\"38\":203,\"39\":106,\"40\":41,\"41\":192,\"42\":205,\"43\":144,\"44\":20,\"45\":158,\"46\":42,\"47\":167,\"48\":18,\"49\":30,\"50\":27,\"51\":103,\"52\":51,\"53\":222,\"54\":226,\"55\":224,\"56\":168,\"57\":111,\"58\":16,\"59\":214,\"60\":128,\"61\":165,\"62\":10,\"63\":183}}},\"zk_addr\":\"0x9440174050c8a69f3736aade438d256444387d7f99afaf9b5a9f29c6f0fba0c3\",\"zk_proofs\":{\"proofPoints\":{\"a\":[\"12337032776119704699956096862904448418119911526311506121881119564201699892276\",\"21261432927871679671381948020842646421823600661053961908567605147368225372658\",\"1\"],\"b\":[[\"361501104451650926380087094710685809078127996371826342961671838349546013669\",\"6224896865231367783073876006741593926823975323893517814398563485217838362592\"],[\"17991862631010087641911530148948529285385885925990265147692471125933697566220\",\"3919918348467391624469564417209140189505145619892305626999747602773689849635\"],[\"1\",\"0\"]],\"c\":[\"2974798412198231516644318932878285282801453498857240613838304706754188993145\",\"18411763423260631630440151338922210964792206590205572668118109635459867927504\",\"1\"]},\"issBase64Details\":{\"value\":\"yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC\",\"indexMod4\":1},\"headerBase64\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjMyM2IyMTRhZTY5NzVhMGYwMzRlYTc3MzU0ZGMwYzI1ZDAzNjQyZGMiLCJ0eXAiOiJKV1QifQ\"},\"extended_ephemeral_public_key\":\"ABlEZk1WdstqKcDNkBSeKqcSHhtnM97i4KhvENaApQq3\", \"verification_key_id\": 0, \"modulus\": \"xjWd1j8GmmWzuz732haG9HECXsSZBvxOBLph3FQhk_tplhWloI1ywx-RdopUZt1lndbOM9n99lZJkpQyNJ1sdy7JFgYLjqj-wtHdEaQlBGEQtmkW8zUjr_N3bmpsxGbPzOzlKe3qddtoxXvn9rI_RvHfJD1YY-6kayQeyPOBz_4ML1lvI_JHV-Bb1MSmSk3WaAh5PzeqleusmUT87Gqfu02cOPrY8cwugqo65D6-wzAEeVvceV8-c36TMoLU5csU05GBVplgd6Ouuw35ZsETG4si4QQJztC3KsZ4jhYM-aJ3jeFPt0r3cQooiXdZBp3JkXSpE-UUaOVPsXo7WiVmww\", \"kid\": \"323b214ae6975a0f034ea77354dc0c25d03642dc\",\"max_epoch\":142}";
pub const TEST_AUTH_DATA_5: &str = "{\"jwt\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjMyM2IyMTRhZTY5NzVhMGYwMzRlYTc3MzU0ZGMwYzI1ZDAzNjQyZGMiLCJ0eXAiOiJKV1QifQ.eyJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLCJhenAiOiIyMzI2MjQwODUxOTEtdjF0cTIwZmcxa2RoaGd2YXQ2c2FqN2pmMGhkODIzM3IuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJhdWQiOiIyMzI2MjQwODUxOTEtdjF0cTIwZmcxa2RoaGd2YXQ2c2FqN2pmMGhkODIzM3IuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJzdWIiOiIxMTI4OTc0Njg2MjY3MTY2MjYxMDMiLCJub25jZSI6IlZXM0VrSmp5ZXM0M2pvNENXU0FyU1pORHk5ayIsIm5iZiI6MTcxNTY4NzAwNywiaWF0IjoxNzE1Njg3MzA3LCJleHAiOjE3MTU2OTA5MDcsImp0aSI6IjMwNDczMjk0MmI3MDQzOTUzN2M3OTE4YTIyZDMxODA1YTVjYzkzZTYifQ.sa6ee8fcMhF3JgGQcl03IY0alries0KC7SRH-HVUnA5cqTVYomJ6fr0NTDJmYXNKOeIcaT85LLN0ALsKtEQdZjhu1g4m16kbS-5MybFIXT85JIPhBOz7zYldrbiy-Me8XRNWPkR3X_lV9pwqvYJTnZ0ley5dDITRvIXE1w2ZmjGNDlDxG3aM2XOQDICQ1ztsCZkn20ShuvG7tZHq7cp7K0hd6JdX0fFRY85eSIeapW7NnnWdvJi2xvuiCcwqm8sshldcJI5uU9xikhoN2WA7c8fJ5rtshqp5-RtTOfbzLn2a6m0WeDE0JqUd8jbh6_T8mGtYYeYMAWfWb-jVPa8aNg\",\"user_pass_to_int_format\":\"118981041155255\",\"ephemeral_key_pair\":{\"keypair\":{\"public_key\":{\"0\":76,\"1\":252,\"2\":245,\"3\":48,\"4\":8,\"5\":126,\"6\":211,\"7\":60,\"8\":249,\"9\":241,\"10\":111,\"11\":107,\"12\":148,\"13\":35,\"14\":224,\"15\":237,\"16\":179,\"17\":74,\"18\":84,\"19\":7,\"20\":130,\"21\":96,\"22\":198,\"23\":40,\"24\":23,\"25\":4,\"26\":50,\"27\":62,\"28\":191,\"29\":222,\"30\":119,\"31\":195},\"secret_key\":{\"0\":217,\"1\":194,\"2\":91,\"3\":84,\"4\":244,\"5\":214,\"6\":113,\"7\":57,\"8\":79,\"9\":43,\"10\":104,\"11\":85,\"12\":61,\"13\":225,\"14\":26,\"15\":139,\"16\":139,\"17\":206,\"18\":110,\"19\":48,\"20\":118,\"21\":99,\"22\":130,\"23\":122,\"24\":59,\"25\":6,\"26\":224,\"27\":144,\"28\":146,\"29\":25,\"30\":147,\"31\":225,\"32\":76,\"33\":252,\"34\":245,\"35\":48,\"36\":8,\"37\":126,\"38\":211,\"39\":60,\"40\":249,\"41\":241,\"42\":111,\"43\":107,\"44\":148,\"45\":35,\"46\":224,\"47\":237,\"48\":179,\"49\":74,\"50\":84,\"51\":7,\"52\":130,\"53\":96,\"54\":198,\"55\":40,\"56\":23,\"57\":4,\"58\":50,\"59\":62,\"60\":191,\"61\":222,\"62\":119,\"63\":195}}},\"zk_addr\":\"0x71444450505074fe9d9205f02747fb34f49dda22eb33eaf7929bb8561ffd45f2\",\"zk_proofs\":{\"proofPoints\":{\"a\":[\"4549343359411304649846201661164616647369749072820883051997393354186530425088\",\"3937997833930688873121017483900547354352969510911658484353113904856895725039\",\"1\"],\"b\":[[\"737118397015523176881783675037843258491735390512712007670938320351154476838\",\"18093386738096496776241258608856280732173952478987786488484944779094702670649\"],[\"17783469782238073070748856104623185946400565050372789961482242728023613389739\",\"15824649467012100671772283318060553156148444804907193757065241285355958322525\"],[\"1\",\"0\"]],\"c\":[\"15112690010634489290938122084488710379345235713605729023472643459768097669053\",\"21568492795931010980780236148561695295582527237009199544419907898465140630575\",\"1\"]},\"issBase64Details\":{\"value\":\"yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC\",\"indexMod4\":1},\"headerBase64\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjMyM2IyMTRhZTY5NzVhMGYwMzRlYTc3MzU0ZGMwYzI1ZDAzNjQyZGMiLCJ0eXAiOiJKV1QifQ\"},\"extended_ephemeral_public_key\":\"AEz89TAIftM8+fFva5Qj4O2zSlQHgmDGKBcEMj6/3nfD\", \"verification_key_id\": 0, \"modulus\": \"xjWd1j8GmmWzuz732haG9HECXsSZBvxOBLph3FQhk_tplhWloI1ywx-RdopUZt1lndbOM9n99lZJkpQyNJ1sdy7JFgYLjqj-wtHdEaQlBGEQtmkW8zUjr_N3bmpsxGbPzOzlKe3qddtoxXvn9rI_RvHfJD1YY-6kayQeyPOBz_4ML1lvI_JHV-Bb1MSmSk3WaAh5PzeqleusmUT87Gqfu02cOPrY8cwugqo65D6-wzAEeVvceV8-c36TMoLU5csU05GBVplgd6Ouuw35ZsETG4si4QQJztC3KsZ4jhYM-aJ3jeFPt0r3cQooiXdZBp3JkXSpE-UUaOVPsXo7WiVmww\", \"kid\": \"323b214ae6975a0f034ea77354dc0c25d03642dc\",\"max_epoch\":142}";
pub const TEST_AUTH_DATA_6: &str = "{\"jwt\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjMyM2IyMTRhZTY5NzVhMGYwMzRlYTc3MzU0ZGMwYzI1ZDAzNjQyZGMiLCJ0eXAiOiJKV1QifQ.eyJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLCJhenAiOiIyMzI2MjQwODUxOTEtdjF0cTIwZmcxa2RoaGd2YXQ2c2FqN2pmMGhkODIzM3IuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJhdWQiOiIyMzI2MjQwODUxOTEtdjF0cTIwZmcxa2RoaGd2YXQ2c2FqN2pmMGhkODIzM3IuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJzdWIiOiIxMTI4OTc0Njg2MjY3MTY2MjYxMDMiLCJub25jZSI6IjMySWx6VWJuSFY4MV9QTTBJSDBSZmFveVN1TSIsIm5iZiI6MTcxNTY4NzAwMCwiaWF0IjoxNzE1Njg3MzAwLCJleHAiOjE3MTU2OTA5MDAsImp0aSI6ImRhOTU5NmIwMTljOTQ2NWE4MzA0MWIxMDA3OTI2OGU3NDgwY2ZjMDkifQ.HFkMZFhHu6BGBbWhC1NwCvJ9_bKOL8jOdOHuRG21mKh-CaJPffnGtaVNcwEJjf4jOVVPPZNfcJPWOd7KoT_R2Giw7An2dUcJFvVJHUv4h55u4DinU50R7h7ACyEl5GwbKCI-cgxORbcoUdQRukDt1zJHe1eeWm1S8URlE2f4U0w2tPPaE_NmChIRyvU_CjB0dLwxzIWU74pvnbkLSSD2pTWhGbLT1yNhfMTh6yukLyEt2kWvNdZOgGbDfIU6xFjxJLtnPrm5WGiOiWyBmMuDput47-ns4821l3KogdIbWr6TLWW0PMwyJuHnif5pV7wJI9JL5XdFv8KZ0IReAYOIEg\",\"user_pass_to_int_format\":\"1021035256\",\"ephemeral_key_pair\":{\"keypair\":{\"public_key\":{\"0\":169,\"1\":238,\"2\":219,\"3\":251,\"4\":231,\"5\":87,\"6\":175,\"7\":233,\"8\":185,\"9\":44,\"10\":161,\"11\":207,\"12\":48,\"13\":166,\"14\":79,\"15\":104,\"16\":225,\"17\":53,\"18\":68,\"19\":236,\"20\":49,\"21\":204,\"22\":99,\"23\":208,\"24\":2,\"25\":134,\"26\":101,\"27\":212,\"28\":221,\"29\":142,\"30\":69,\"31\":196},\"secret_key\":{\"0\":94,\"1\":128,\"2\":26,\"3\":130,\"4\":137,\"5\":40,\"6\":61,\"7\":27,\"8\":79,\"9\":58,\"10\":100,\"11\":117,\"12\":200,\"13\":118,\"14\":156,\"15\":202,\"16\":165,\"17\":34,\"18\":238,\"19\":237,\"20\":90,\"21\":63,\"22\":84,\"23\":119,\"24\":86,\"25\":2,\"26\":221,\"27\":177,\"28\":224,\"29\":4,\"30\":233,\"31\":99,\"32\":169,\"33\":238,\"34\":219,\"35\":251,\"36\":231,\"37\":87,\"38\":175,\"39\":233,\"40\":185,\"41\":44,\"42\":161,\"43\":207,\"44\":48,\"45\":166,\"46\":79,\"47\":104,\"48\":225,\"49\":53,\"50\":68,\"51\":236,\"52\":49,\"53\":204,\"54\":99,\"55\":208,\"56\":2,\"57\":134,\"58\":101,\"59\":212,\"60\":221,\"61\":142,\"62\":69,\"63\":196}}},\"zk_addr\":\"0xb1dfac568641e785f1fbd385f43f9ab5751f30e942ffd0618ea3cacf2feb884f\",\"zk_proofs\":{\"proofPoints\":{\"a\":[\"12140334820013650239749561964826061158522594132954339836339110630367427672527\",\"21543355833919541708094668850466443067263177907229807762067953508321817783804\",\"1\"],\"b\":[[\"11929519532343982399968491980281874410531815035766070083344475081372092452425\",\"13741260533480647813301201467326069876472210148610447598292633272004546481630\"],[\"14605296808789442404291984821803068302067977919075239981788942874792752578522\",\"20230214791286972912596895174545361255719543417377972941442631629070781210055\"],[\"1\",\"0\"]],\"c\":[\"6046227686259383004231849145260526357580306829730644608118177932582255490991\",\"1343314209137088066016224766407952045954639818725548553059063245802388749310\",\"1\"]},\"issBase64Details\":{\"value\":\"yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC\",\"indexMod4\":1},\"headerBase64\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjMyM2IyMTRhZTY5NzVhMGYwMzRlYTc3MzU0ZGMwYzI1ZDAzNjQyZGMiLCJ0eXAiOiJKV1QifQ\"},\"extended_ephemeral_public_key\":\"AKnu2/vnV6/puSyhzzCmT2jhNUTsMcxj0AKGZdTdjkXE\", \"verification_key_id\": 0, \"modulus\": \"xjWd1j8GmmWzuz732haG9HECXsSZBvxOBLph3FQhk_tplhWloI1ywx-RdopUZt1lndbOM9n99lZJkpQyNJ1sdy7JFgYLjqj-wtHdEaQlBGEQtmkW8zUjr_N3bmpsxGbPzOzlKe3qddtoxXvn9rI_RvHfJD1YY-6kayQeyPOBz_4ML1lvI_JHV-Bb1MSmSk3WaAh5PzeqleusmUT87Gqfu02cOPrY8cwugqo65D6-wzAEeVvceV8-c36TMoLU5csU05GBVplgd6Ouuw35ZsETG4si4QQJztC3KsZ4jhYM-aJ3jeFPt0r3cQooiXdZBp3JkXSpE-UUaOVPsXo7WiVmww\", \"kid\": \"323b214ae6975a0f034ea77354dc0c25d03642dc\",\"max_epoch\":142}";
pub const TEST_AUTH_DATA_7: &str = "{\"jwt\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjMyM2IyMTRhZTY5NzVhMGYwMzRlYTc3MzU0ZGMwYzI1ZDAzNjQyZGMiLCJ0eXAiOiJKV1QifQ.eyJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLCJhenAiOiIyMzI2MjQwODUxOTEtdjF0cTIwZmcxa2RoaGd2YXQ2c2FqN2pmMGhkODIzM3IuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJhdWQiOiIyMzI2MjQwODUxOTEtdjF0cTIwZmcxa2RoaGd2YXQ2c2FqN2pmMGhkODIzM3IuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJzdWIiOiIxMTI4OTc0Njg2MjY3MTY2MjYxMDMiLCJub25jZSI6IjhJNGF5NDF5THpHWnk4czNCUGR3QzItR2Q5ayIsIm5iZiI6MTcxNTY4Njk5MywiaWF0IjoxNzE1Njg3MjkzLCJleHAiOjE3MTU2OTA4OTMsImp0aSI6ImFjYmJiNmQ3NGY0ZmU3MjMxYzc2ZDQxNzE0ZDM4NDJiNzZlNDU4YzIifQ.nsmqj7tDDv7wJSn47YfaFBXabPYVBjZosGzH_bPHZZToPfvdQyXZrO5CXbaJmojxTPRmzZ2bPI39K9GMX7Y8gaOqk_LYHR7eemVaEj0wNpPPtmUFmHmyrL8nPkTN0a-87L2eu6t7yBZtEiT5e2Jz46RBu9rQL138seOvK3vm0YwhtnLGxhZQnoAKu076qZ_ItlsRn9PqM-sd83bqQoG_SPQVCZL6spWoFunXtj1FeKE-3gRRD8BopORDhFp4xytWDamd1XgIdCNp0a8u7mvElPZCjc3ZUAtFYBWwvfI9r2wN5X4gbNe_pbfpBmgg-2zxwt6c32IhNXlrDQkLxJYqkg\",\"user_pass_to_int_format\":\"9899115100106104\",\"ephemeral_key_pair\":{\"keypair\":{\"public_key\":{\"0\":35,\"1\":64,\"2\":69,\"3\":29,\"4\":242,\"5\":9,\"6\":183,\"7\":224,\"8\":98,\"9\":254,\"10\":210,\"11\":82,\"12\":213,\"13\":2,\"14\":137,\"15\":66,\"16\":71,\"17\":61,\"18\":80,\"19\":154,\"20\":135,\"21\":100,\"22\":176,\"23\":189,\"24\":187,\"25\":96,\"26\":245,\"27\":194,\"28\":163,\"29\":250,\"30\":15,\"31\":37},\"secret_key\":{\"0\":117,\"1\":94,\"2\":35,\"3\":85,\"4\":116,\"5\":80,\"6\":126,\"7\":55,\"8\":166,\"9\":193,\"10\":94,\"11\":109,\"12\":238,\"13\":86,\"14\":132,\"15\":192,\"16\":225,\"17\":240,\"18\":26,\"19\":65,\"20\":211,\"21\":18,\"22\":195,\"23\":36,\"24\":225,\"25\":158,\"26\":143,\"27\":141,\"28\":21,\"29\":174,\"30\":139,\"31\":13,\"32\":35,\"33\":64,\"34\":69,\"35\":29,\"36\":242,\"37\":9,\"38\":183,\"39\":224,\"40\":98,\"41\":254,\"42\":210,\"43\":82,\"44\":213,\"45\":2,\"46\":137,\"47\":66,\"48\":71,\"49\":61,\"50\":80,\"51\":154,\"52\":135,\"53\":100,\"54\":176,\"55\":189,\"56\":187,\"57\":96,\"58\":245,\"59\":194,\"60\":163,\"61\":250,\"62\":15,\"63\":37}}},\"zk_addr\":\"0x2130548addf21464dba0598e4306193fc658433793260241bd224fa5a186eea1\",\"zk_proofs\":{\"proofPoints\":{\"a\":[\"11563763779979887221682129962776185026792805331576343366100386476995832665737\",\"11230623338801856741023013148077980370341441565413488652841279984753971030674\",\"1\"],\"b\":[[\"459996434316864652818633810305056376561329097756558823429320916262609240883\",\"9149790799426074072032368390512074348954812141386022619414187192076850710684\"],[\"21136831034524197906636931934376551157061262869485003235799208746070603082410\",\"7423352680736750974836973800304252036668418183885087029886854244313632685127\"],[\"1\",\"0\"]],\"c\":[\"13616579662900237409901679872544397096722160915603059752960265571802149963290\",\"17724386432768174493966206493099783171212386514205046762827409640509581679264\",\"1\"]},\"issBase64Details\":{\"value\":\"yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC\",\"indexMod4\":1},\"headerBase64\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjMyM2IyMTRhZTY5NzVhMGYwMzRlYTc3MzU0ZGMwYzI1ZDAzNjQyZGMiLCJ0eXAiOiJKV1QifQ\"},\"extended_ephemeral_public_key\":\"ACNARR3yCbfgYv7SUtUCiUJHPVCah2Swvbtg9cKj+g8l\", \"verification_key_id\": 0, \"modulus\": \"xjWd1j8GmmWzuz732haG9HECXsSZBvxOBLph3FQhk_tplhWloI1ywx-RdopUZt1lndbOM9n99lZJkpQyNJ1sdy7JFgYLjqj-wtHdEaQlBGEQtmkW8zUjr_N3bmpsxGbPzOzlKe3qddtoxXvn9rI_RvHfJD1YY-6kayQeyPOBz_4ML1lvI_JHV-Bb1MSmSk3WaAh5PzeqleusmUT87Gqfu02cOPrY8cwugqo65D6-wzAEeVvceV8-c36TMoLU5csU05GBVplgd6Ouuw35ZsETG4si4QQJztC3KsZ4jhYM-aJ3jeFPt0r3cQooiXdZBp3JkXSpE-UUaOVPsXo7WiVmww\", \"kid\": \"323b214ae6975a0f034ea77354dc0c25d03642dc\",\"max_epoch\":142}";
pub const TEST_AUTH_DATA_8: &str = "{\"jwt\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjMyM2IyMTRhZTY5NzVhMGYwMzRlYTc3MzU0ZGMwYzI1ZDAzNjQyZGMiLCJ0eXAiOiJKV1QifQ.eyJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLCJhenAiOiIyMzI2MjQwODUxOTEtdjF0cTIwZmcxa2RoaGd2YXQ2c2FqN2pmMGhkODIzM3IuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJhdWQiOiIyMzI2MjQwODUxOTEtdjF0cTIwZmcxa2RoaGd2YXQ2c2FqN2pmMGhkODIzM3IuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJzdWIiOiIxMTI4OTc0Njg2MjY3MTY2MjYxMDMiLCJub25jZSI6IjlzWGhqc0xVZlNmdE12bl9iYmQtWkEwZnY5OCIsIm5iZiI6MTcxNTY4Njk4NiwiaWF0IjoxNzE1Njg3Mjg2LCJleHAiOjE3MTU2OTA4ODYsImp0aSI6IjhiNDUzYjUzNTY4MGU3OWZkZWUyOGE3NDVmMzgzMDBkMmNmYjNmODQifQ.a2fpzvW0PxyOcvE8P6WEtIs_mdfTQ9kJb4MIUC5T5uRYJ9ySqSa2qT-MICspGYBuNzCtWIvI6KHY9cIWE2XF3yv7d7gTk_IkhXJud0s5hMhsIxWuNXla_-HducNufaXxXxWYJ2g8dy2xsIMnPr5OC-r4dAX3DM3AchB8qA-RYJdtgwlLytyANp6I35BRT7ewXyDDdlqMLnz5dv4xh1y1wrXFL7VDyzV2XVTK3ap12Cev9IZtHnSGsDgl-vEXj1OYIyiaDgtDhA7rfLXWRTQEeVnRpF-v3AIwZmRu1qaXFoqUbMaSQpFotwb6m8fMQ1q9efOK1Xrv8dL3jBDcUA3w3w\",\"user_pass_to_int_format\":\"98118115106\",\"ephemeral_key_pair\":{\"keypair\":{\"public_key\":{\"0\":52,\"1\":224,\"2\":199,\"3\":19,\"4\":180,\"5\":128,\"6\":181,\"7\":171,\"8\":55,\"9\":210,\"10\":168,\"11\":100,\"12\":198,\"13\":241,\"14\":150,\"15\":156,\"16\":226,\"17\":233,\"18\":32,\"19\":175,\"20\":153,\"21\":53,\"22\":23,\"23\":58,\"24\":196,\"25\":29,\"26\":16,\"27\":170,\"28\":245,\"29\":46,\"30\":71,\"31\":177},\"secret_key\":{\"0\":47,\"1\":184,\"2\":41,\"3\":167,\"4\":98,\"5\":225,\"6\":50,\"7\":146,\"8\":173,\"9\":129,\"10\":201,\"11\":41,\"12\":181,\"13\":239,\"14\":8,\"15\":249,\"16\":159,\"17\":200,\"18\":159,\"19\":80,\"20\":194,\"21\":79,\"22\":41,\"23\":26,\"24\":200,\"25\":82,\"26\":74,\"27\":200,\"28\":38,\"29\":172,\"30\":84,\"31\":187,\"32\":52,\"33\":224,\"34\":199,\"35\":19,\"36\":180,\"37\":128,\"38\":181,\"39\":171,\"40\":55,\"41\":210,\"42\":168,\"43\":100,\"44\":198,\"45\":241,\"46\":150,\"47\":156,\"48\":226,\"49\":233,\"50\":32,\"51\":175,\"52\":153,\"53\":53,\"54\":23,\"55\":58,\"56\":196,\"57\":29,\"58\":16,\"59\":170,\"60\":245,\"61\":46,\"62\":71,\"63\":177}}},\"zk_addr\":\"0xd704fd1fa5b1d8603b91081d104c08a025e9a952cd6b5b44324fcca2ed432737\",\"zk_proofs\":{\"proofPoints\":{\"a\":[\"12104554633236481277286668189930576438264898269322388260846346074721767290773\",\"12396613925509861793005815245783240999567113519994130032649036118285018908597\",\"1\"],\"b\":[[\"1950992742588131071369658940220202257834946772534232957497529743913085624908\",\"13592611568444679350754388983552527571019415309901710535712414143531288069409\"],[\"16680699225604481493782973126773355417557338915104879244979908308676269902149\",\"7242446539394843603528008588061352122030003516933411896066602483137632866329\"],[\"1\",\"0\"]],\"c\":[\"17095909781059243761149234557016161052123209525874162987135833613569429453315\",\"8531296608559822287633863219696197152375138627859243631029781182381653695377\",\"1\"]},\"issBase64Details\":{\"value\":\"yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC\",\"indexMod4\":1},\"headerBase64\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjMyM2IyMTRhZTY5NzVhMGYwMzRlYTc3MzU0ZGMwYzI1ZDAzNjQyZGMiLCJ0eXAiOiJKV1QifQ\"},\"extended_ephemeral_public_key\":\"ADTgxxO0gLWrN9KoZMbxlpzi6SCvmTUXOsQdEKr1Lkex\", \"verification_key_id\": 0, \"modulus\": \"xjWd1j8GmmWzuz732haG9HECXsSZBvxOBLph3FQhk_tplhWloI1ywx-RdopUZt1lndbOM9n99lZJkpQyNJ1sdy7JFgYLjqj-wtHdEaQlBGEQtmkW8zUjr_N3bmpsxGbPzOzlKe3qddtoxXvn9rI_RvHfJD1YY-6kayQeyPOBz_4ML1lvI_JHV-Bb1MSmSk3WaAh5PzeqleusmUT87Gqfu02cOPrY8cwugqo65D6-wzAEeVvceV8-c36TMoLU5csU05GBVplgd6Ouuw35ZsETG4si4QQJztC3KsZ4jhYM-aJ3jeFPt0r3cQooiXdZBp3JkXSpE-UUaOVPsXo7WiVmww\", \"kid\": \"323b214ae6975a0f034ea77354dc0c25d03642dc\",\"max_epoch\":142}";
pub const TEST_AUTH_DATA_9: &str = "{\"jwt\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjMyM2IyMTRhZTY5NzVhMGYwMzRlYTc3MzU0ZGMwYzI1ZDAzNjQyZGMiLCJ0eXAiOiJKV1QifQ.eyJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLCJhenAiOiIyMzI2MjQwODUxOTEtdjF0cTIwZmcxa2RoaGd2YXQ2c2FqN2pmMGhkODIzM3IuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJhdWQiOiIyMzI2MjQwODUxOTEtdjF0cTIwZmcxa2RoaGd2YXQ2c2FqN2pmMGhkODIzM3IuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJzdWIiOiIxMTI4OTc0Njg2MjY3MTY2MjYxMDMiLCJub25jZSI6ImRLMlRMdjktRXFnaGR3SWQzMzlPNXZmN1JENCIsIm5iZiI6MTcxNTY4Njk3OSwiaWF0IjoxNzE1Njg3Mjc5LCJleHAiOjE3MTU2OTA4NzksImp0aSI6IjQxYTgwZTI3N2U5NzIxYjcwZDkyMjRkMWY5MzRlNjJhYTYwNzcyZGEifQ.NLM6YIR61HOzlEVS1ianwnFoG6OfSeLyuGpjH-Wt7eiWt27fHbDhOWTo-2ysx7cXuAl3gV8ZzMta24QSpjIiiaooGdurX92cWuDcARyewX5_4UuwBWBTXe66irHuqjwIOB2WwyN6PuOwvM6Y_IcL9vPwg76iJoupbeCHXBswiRVzVyBQus1k9SGigU8_ZuwGYoTLPd68MX7Z68NrK7mCF04Xaijs__zwJigIhVOK3TXN2Xy84Ha76mrXJRJZuWErrSNWagVO-dxb2oMT8vm5ND9aJ4q4NaIeGa8PIN2X1cfg9A6LZVBsGIc9JV2FG39yK4T2XAH6tn_HtoMzy_Vuvg\",\"user_pass_to_int_format\":\"10011898115106104\",\"ephemeral_key_pair\":{\"keypair\":{\"public_key\":{\"0\":78,\"1\":247,\"2\":200,\"3\":7,\"4\":84,\"5\":131,\"6\":33,\"7\":223,\"8\":6,\"9\":241,\"10\":100,\"11\":90,\"12\":91,\"13\":2,\"14\":31,\"15\":23,\"16\":138,\"17\":130,\"18\":115,\"19\":150,\"20\":202,\"21\":79,\"22\":12,\"23\":132,\"24\":168,\"25\":153,\"26\":155,\"27\":131,\"28\":31,\"29\":69,\"30\":170,\"31\":112},\"secret_key\":{\"0\":98,\"1\":144,\"2\":57,\"3\":245,\"4\":40,\"5\":191,\"6\":248,\"7\":149,\"8\":147,\"9\":12,\"10\":229,\"11\":76,\"12\":157,\"13\":3,\"14\":241,\"15\":94,\"16\":134,\"17\":124,\"18\":226,\"19\":177,\"20\":31,\"21\":140,\"22\":224,\"23\":58,\"24\":57,\"25\":95,\"26\":235,\"27\":246,\"28\":120,\"29\":89,\"30\":33,\"31\":149,\"32\":78,\"33\":247,\"34\":200,\"35\":7,\"36\":84,\"37\":131,\"38\":33,\"39\":223,\"40\":6,\"41\":241,\"42\":100,\"43\":90,\"44\":91,\"45\":2,\"46\":31,\"47\":23,\"48\":138,\"49\":130,\"50\":115,\"51\":150,\"52\":202,\"53\":79,\"54\":12,\"55\":132,\"56\":168,\"57\":153,\"58\":155,\"59\":131,\"60\":31,\"61\":69,\"62\":170,\"63\":112}}},\"zk_addr\":\"0x4493e2aab6fcd5d7259e066291ed6f42f6e0b732ecbd38bbaf8a98546a7d0cba\",\"zk_proofs\":{\"proofPoints\":{\"a\":[\"6187760498712900389422022394560825973187662358740291343829568808375698843239\",\"3663904360488418820404220406786944885702547623862334490191838865255632801941\",\"1\"],\"b\":[[\"17208058907245387104889127891010282196539728213379257608213444054211064433036\",\"9822512703540345824827246410723992174766686970531763618190197664729418117984\"],[\"9555481236549941306688205540885297760448987185399187813240300069134845655152\",\"17967781633941820778916846359708064205041390458485667635199415296702341964940\"],[\"1\",\"0\"]],\"c\":[\"12374452924342055287727719327288397498526425907741014437332085255604038084453\",\"7084903967634108603521121616612807600817728267672878238097194166039392876060\",\"1\"]},\"issBase64Details\":{\"value\":\"yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC\",\"indexMod4\":1},\"headerBase64\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjMyM2IyMTRhZTY5NzVhMGYwMzRlYTc3MzU0ZGMwYzI1ZDAzNjQyZGMiLCJ0eXAiOiJKV1QifQ\"},\"extended_ephemeral_public_key\":\"AE73yAdUgyHfBvFkWlsCHxeKgnOWyk8MhKiZm4MfRapw\", \"verification_key_id\": 0, \"modulus\": \"xjWd1j8GmmWzuz732haG9HECXsSZBvxOBLph3FQhk_tplhWloI1ywx-RdopUZt1lndbOM9n99lZJkpQyNJ1sdy7JFgYLjqj-wtHdEaQlBGEQtmkW8zUjr_N3bmpsxGbPzOzlKe3qddtoxXvn9rI_RvHfJD1YY-6kayQeyPOBz_4ML1lvI_JHV-Bb1MSmSk3WaAh5PzeqleusmUT87Gqfu02cOPrY8cwugqo65D6-wzAEeVvceV8-c36TMoLU5csU05GBVplgd6Ouuw35ZsETG4si4QQJztC3KsZ4jhYM-aJ3jeFPt0r3cQooiXdZBp3JkXSpE-UUaOVPsXo7WiVmww\", \"kid\": \"323b214ae6975a0f034ea77354dc0c25d03642dc\",\"max_epoch\":142}";
pub const TEST_AUTH_DATA_10: &str = "{\"jwt\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjMyM2IyMTRhZTY5NzVhMGYwMzRlYTc3MzU0ZGMwYzI1ZDAzNjQyZGMiLCJ0eXAiOiJKV1QifQ.eyJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLCJhenAiOiIyMzI2MjQwODUxOTEtdjF0cTIwZmcxa2RoaGd2YXQ2c2FqN2pmMGhkODIzM3IuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJhdWQiOiIyMzI2MjQwODUxOTEtdjF0cTIwZmcxa2RoaGd2YXQ2c2FqN2pmMGhkODIzM3IuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJzdWIiOiIxMTI4OTc0Njg2MjY3MTY2MjYxMDMiLCJub25jZSI6InZjdk1UaEZFdjFzVTBGVGR1M3Z5TmxlVUd0MCIsIm5iZiI6MTcxNTY4Njk3MiwiaWF0IjoxNzE1Njg3MjcyLCJleHAiOjE3MTU2OTA4NzIsImp0aSI6ImFiMDMwNTMzZjQyOWJmOGFhMDNmYzZhYjAxZjg2MGQ3MTg5ZDBlNjkifQ.f11q7mTu1uScsGvj4-KgVHHEhfAqk53JbAIC0PT8-CU40D4fSWbBoyXrUUQw6zly4KsyqazAFJ_1JqjiFvYFOhCAsoGWpgiA-hnL4QK-uxqUV4ule7Wt9xs8QVPivYxTrK2jmDgPGosvTUmlrGeyZk2XwilO3mbTe5wN-zMkUF0zUTdlIBTPrKbXMS1PklWTjUgDa1bXb-hOaFILkfZ4UgQI3PYHjZul3Rm_UUHHHVRkLgt0M449CGjuKSsIFvVkslfL319_71DLo7W0sYkJkWGOa482vTvyHgR9SjalUPV4TzPhpe_6DZZlKna7MXgq4FWOS9710PC6_HAXF2n-ag\",\"user_pass_to_int_format\":\"11898100104102106115\",\"ephemeral_key_pair\":{\"keypair\":{\"public_key\":{\"0\":146,\"1\":239,\"2\":26,\"3\":188,\"4\":228,\"5\":23,\"6\":17,\"7\":118,\"8\":183,\"9\":248,\"10\":93,\"11\":219,\"12\":0,\"13\":213,\"14\":164,\"15\":161,\"16\":140,\"17\":200,\"18\":97,\"19\":183,\"20\":135,\"21\":18,\"22\":103,\"23\":137,\"24\":234,\"25\":122,\"26\":246,\"27\":20,\"28\":155,\"29\":72,\"30\":212,\"31\":15},\"secret_key\":{\"0\":107,\"1\":202,\"2\":67,\"3\":226,\"4\":108,\"5\":41,\"6\":149,\"7\":181,\"8\":238,\"9\":3,\"10\":97,\"11\":189,\"12\":216,\"13\":94,\"14\":143,\"15\":210,\"16\":192,\"17\":213,\"18\":224,\"19\":200,\"20\":253,\"21\":67,\"22\":168,\"23\":88,\"24\":140,\"25\":106,\"26\":235,\"27\":247,\"28\":54,\"29\":146,\"30\":251,\"31\":123,\"32\":146,\"33\":239,\"34\":26,\"35\":188,\"36\":228,\"37\":23,\"38\":17,\"39\":118,\"40\":183,\"41\":248,\"42\":93,\"43\":219,\"44\":0,\"45\":213,\"46\":164,\"47\":161,\"48\":140,\"49\":200,\"50\":97,\"51\":183,\"52\":135,\"53\":18,\"54\":103,\"55\":137,\"56\":234,\"57\":122,\"58\":246,\"59\":20,\"60\":155,\"61\":72,\"62\":212,\"63\":15}}},\"zk_addr\":\"0xb86a18deea59af2850ab3800e2d46f63cfbea3bae309359089945d55949aef84\",\"zk_proofs\":{\"proofPoints\":{\"a\":[\"2575938484642353074459611431508941853614856803645537593538048270397701877180\",\"18525747234426619072147704335372433454079655655225636793928970068265541595508\",\"1\"],\"b\":[[\"5146896444986257903458872614168031344366471557324420746422302593221564486610\",\"19134791144810013840937258347062701987554745426617919650818846823708095832550\"],[\"3133101512761334334340993079649721452024653991833325456466256722050883608250\",\"21877263483512108853787895465249721341909931993800128255134630466114688578666\"],[\"1\",\"0\"]],\"c\":[\"3069457366306376197755607218741517434199413283376424243014529567457206056402\",\"4929625283757609606431630951067242799347282963225969540629139985267066740824\",\"1\"]},\"issBase64Details\":{\"value\":\"yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC\",\"indexMod4\":1},\"headerBase64\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjMyM2IyMTRhZTY5NzVhMGYwMzRlYTc3MzU0ZGMwYzI1ZDAzNjQyZGMiLCJ0eXAiOiJKV1QifQ\"},\"extended_ephemeral_public_key\":\"AJLvGrzkFxF2t/hd2wDVpKGMyGG3hxJniep69hSbSNQP\", \"verification_key_id\": 0, \"modulus\": \"xjWd1j8GmmWzuz732haG9HECXsSZBvxOBLph3FQhk_tplhWloI1ywx-RdopUZt1lndbOM9n99lZJkpQyNJ1sdy7JFgYLjqj-wtHdEaQlBGEQtmkW8zUjr_N3bmpsxGbPzOzlKe3qddtoxXvn9rI_RvHfJD1YY-6kayQeyPOBz_4ML1lvI_JHV-Bb1MSmSk3WaAh5PzeqleusmUT87Gqfu02cOPrY8cwugqo65D6-wzAEeVvceV8-c36TMoLU5csU05GBVplgd6Ouuw35ZsETG4si4QQJztC3KsZ4jhYM-aJ3jeFPt0r3cQooiXdZBp3JkXSpE-UUaOVPsXo7WiVmww\", \"kid\": \"323b214ae6975a0f034ea77354dc0c25d03642dc\",\"max_epoch\":142}";
pub const TEST_AUTH_DATA_11: &str = "{\"jwt\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjMyM2IyMTRhZTY5NzVhMGYwMzRlYTc3MzU0ZGMwYzI1ZDAzNjQyZGMiLCJ0eXAiOiJKV1QifQ.eyJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLCJhenAiOiIyMzI2MjQwODUxOTEtdjF0cTIwZmcxa2RoaGd2YXQ2c2FqN2pmMGhkODIzM3IuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJhdWQiOiIyMzI2MjQwODUxOTEtdjF0cTIwZmcxa2RoaGd2YXQ2c2FqN2pmMGhkODIzM3IuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJzdWIiOiIxMTI4OTc0Njg2MjY3MTY2MjYxMDMiLCJub25jZSI6IkJ4RzlwNFhoV1B4WEt3NVRRdG9FbDljZzI5NCIsIm5iZiI6MTcxNTY4Njk0MSwiaWF0IjoxNzE1Njg3MjQxLCJleHAiOjE3MTU2OTA4NDEsImp0aSI6IjMyYmMxN2VjNWY3ZTQ5ZWRkOTM2MzVkZjE5MDk2N2E3NTg5Y2ZmNTgifQ.w3cT9MVhKTvnmAlmKClFFG6hjB2zrwHonYuN6l5S2unwyR6P_tGE42KhaFSNCY-imysy8k42awfmAafXwftKClLvqzk1T6bi5Li6caVd6-la8wj_FxNWkE5Cy-N4grOiEYJtV5SZezFzifmL6LOstv-Nc4X2b9Z6utuGOWYq3W9LNPveD0v5GnBCR6JRtHJkI6e5yZnMwDDE5o1P-LZbGuFXP75P6jseGem956the_WbrwIsnnTdFgjgjbXn_1gkh4SYGQ1ig0NVKcs75hUhKuQi7V6VqycuyXTgACOCsIfh2guoKha-APZUeul3z33zNbsqUcgkWwl6CkvDSdGWiQ\",\"user_pass_to_int_format\":\"1145652515748\",\"ephemeral_key_pair\":{\"keypair\":{\"public_key\":{\"0\":153,\"1\":152,\"2\":146,\"3\":133,\"4\":135,\"5\":137,\"6\":8,\"7\":27,\"8\":197,\"9\":109,\"10\":12,\"11\":221,\"12\":49,\"13\":15,\"14\":10,\"15\":1,\"16\":64,\"17\":236,\"18\":222,\"19\":97,\"20\":181,\"21\":214,\"22\":200,\"23\":214,\"24\":130,\"25\":247,\"26\":204,\"27\":212,\"28\":49,\"29\":33,\"30\":169,\"31\":172},\"secret_key\":{\"0\":159,\"1\":96,\"2\":35,\"3\":206,\"4\":32,\"5\":121,\"6\":5,\"7\":32,\"8\":37,\"9\":203,\"10\":15,\"11\":252,\"12\":99,\"13\":107,\"14\":57,\"15\":211,\"16\":139,\"17\":123,\"18\":6,\"19\":233,\"20\":56,\"21\":15,\"22\":35,\"23\":224,\"24\":243,\"25\":148,\"26\":44,\"27\":114,\"28\":112,\"29\":161,\"30\":226,\"31\":255,\"32\":153,\"33\":152,\"34\":146,\"35\":133,\"36\":135,\"37\":137,\"38\":8,\"39\":27,\"40\":197,\"41\":109,\"42\":12,\"43\":221,\"44\":49,\"45\":15,\"46\":10,\"47\":1,\"48\":64,\"49\":236,\"50\":222,\"51\":97,\"52\":181,\"53\":214,\"54\":200,\"55\":214,\"56\":130,\"57\":247,\"58\":204,\"59\":212,\"60\":49,\"61\":33,\"62\":169,\"63\":172}}},\"zk_addr\":\"0x41c25944949f0e3bf80fea41d9ab27acfa26e0b25ecd7e468235b2284e5b0c09\",\"zk_proofs\":{\"proofPoints\":{\"a\":[\"10607121052143170357142710430122120898934487918266599021712929788471219763472\",\"13359690919698524885136984693561112109891470903700041135375248695741012306373\",\"1\"],\"b\":[[\"3247989990207989646120856507929936403874972366284220250880918537588838028173\",\"20347831818628957019286012207626379731554938194907710010892594024137236752987\"],[\"18217798786390957788883983024823206348636485136705276787854998111125834676541\",\"11824109578691812603938426242725149605448845948255194504928078330266973720614\"],[\"1\",\"0\"]],\"c\":[\"16499583001208064509247079494271177710897656329498349773613236383353749984739\",\"1944718879141050229961827816471755841829876012643055740792265283564642185697\",\"1\"]},\"issBase64Details\":{\"value\":\"yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC\",\"indexMod4\":1},\"headerBase64\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjMyM2IyMTRhZTY5NzVhMGYwMzRlYTc3MzU0ZGMwYzI1ZDAzNjQyZGMiLCJ0eXAiOiJKV1QifQ\"},\"extended_ephemeral_public_key\":\"AJmYkoWHiQgbxW0M3TEPCgFA7N5htdbI1oL3zNQxIams\", \"verification_key_id\": 0, \"modulus\": \"xjWd1j8GmmWzuz732haG9HECXsSZBvxOBLph3FQhk_tplhWloI1ywx-RdopUZt1lndbOM9n99lZJkpQyNJ1sdy7JFgYLjqj-wtHdEaQlBGEQtmkW8zUjr_N3bmpsxGbPzOzlKe3qddtoxXvn9rI_RvHfJD1YY-6kayQeyPOBz_4ML1lvI_JHV-Bb1MSmSk3WaAh5PzeqleusmUT87Gqfu02cOPrY8cwugqo65D6-wzAEeVvceV8-c36TMoLU5csU05GBVplgd6Ouuw35ZsETG4si4QQJztC3KsZ4jhYM-aJ3jeFPt0r3cQooiXdZBp3JkXSpE-UUaOVPsXo7WiVmww\", \"kid\": \"323b214ae6975a0f034ea77354dc0c25d03642dc\",\"max_epoch\":142}";
pub const TEST_AUTH_DATA_12: &str = "{\"jwt\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjMyM2IyMTRhZTY5NzVhMGYwMzRlYTc3MzU0ZGMwYzI1ZDAzNjQyZGMiLCJ0eXAiOiJKV1QifQ.eyJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLCJhenAiOiIyMzI2MjQwODUxOTEtdjF0cTIwZmcxa2RoaGd2YXQ2c2FqN2pmMGhkODIzM3IuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJhdWQiOiIyMzI2MjQwODUxOTEtdjF0cTIwZmcxa2RoaGd2YXQ2c2FqN2pmMGhkODIzM3IuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJzdWIiOiIxMTI4OTc0Njg2MjY3MTY2MjYxMDMiLCJub25jZSI6IndNcWhEa3BxQllkSlowQVBwXzUtbVZDLUU2OCIsIm5iZiI6MTcxNTY4NjkzMywiaWF0IjoxNzE1Njg3MjMzLCJleHAiOjE3MTU2OTA4MzMsImp0aSI6IjEwNGRhZTE1ZWMwODRjODU3MzBjYTRiZGI1MThiZTkwZDdkMTQ3NmEifQ.KBzhI2UOTstRFpgkZiFFlCmhy-E0PwoWdfWhXem6Kr0HjOgCfr-a5TGVRyMf0b7-Tnf712tMPf4N7-uPSoyaBsmtiYmAudj8whha2obUVhzWjghiURrbYkiCBWys5Z4v3SnVKDqXPsUFmNucBSA3l6DIWbhLT4WqTszGY-Qc_cKhR-7y5i3t90lhGNmwrvCR72jAXaF-xbBvsaiMXxhfCS5fnMFNRibIE3tRx1r3mkx59etA8E3xQAu8LPzFyC0ecEKL0K6a5ZWNWBFPbGSzAhSK9D3ak1gzON6rhccCPpLRErk2MIhUQq4HBnOywg5Lf1w0onxhkJtU6docO2VVAA\",\"user_pass_to_int_format\":\"11099104102117\",\"ephemeral_key_pair\":{\"keypair\":{\"public_key\":{\"0\":173,\"1\":37,\"2\":48,\"3\":14,\"4\":54,\"5\":38,\"6\":225,\"7\":52,\"8\":254,\"9\":178,\"10\":32,\"11\":56,\"12\":162,\"13\":128,\"14\":135,\"15\":55,\"16\":10,\"17\":222,\"18\":131,\"19\":175,\"20\":166,\"21\":161,\"22\":145,\"23\":219,\"24\":44,\"25\":231,\"26\":183,\"27\":245,\"28\":141,\"29\":178,\"30\":237,\"31\":92},\"secret_key\":{\"0\":108,\"1\":38,\"2\":149,\"3\":222,\"4\":132,\"5\":184,\"6\":128,\"7\":164,\"8\":27,\"9\":101,\"10\":217,\"11\":92,\"12\":24,\"13\":245,\"14\":209,\"15\":31,\"16\":88,\"17\":174,\"18\":237,\"19\":144,\"20\":78,\"21\":127,\"22\":73,\"23\":195,\"24\":194,\"25\":229,\"26\":208,\"27\":176,\"28\":220,\"29\":60,\"30\":229,\"31\":253,\"32\":173,\"33\":37,\"34\":48,\"35\":14,\"36\":54,\"37\":38,\"38\":225,\"39\":52,\"40\":254,\"41\":178,\"42\":32,\"43\":56,\"44\":162,\"45\":128,\"46\":135,\"47\":55,\"48\":10,\"49\":222,\"50\":131,\"51\":175,\"52\":166,\"53\":161,\"54\":145,\"55\":219,\"56\":44,\"57\":231,\"58\":183,\"59\":245,\"60\":141,\"61\":178,\"62\":237,\"63\":92}}},\"zk_addr\":\"0xe5433ade6e56883e0cc13044783fc6e0d835db866e8ef69d305622f4dbfd7730\",\"zk_proofs\":{\"proofPoints\":{\"a\":[\"20315021530892971959830664693110327999639349964485536174303351139810441711270\",\"9363226245552972448215999928614529638129956136095863617353608229521342156596\",\"1\"],\"b\":[[\"13215029653817105228530429395766730210769586389024965762310641194113200165202\",\"7799676398333409903573594921069872917500921399080042730183754684502821618481\"],[\"13048821293399627652827197503115267831066766008561767009809325017447715880491\",\"331361016081752781071859245948286166830568341165278760117629920699739892753\"],[\"1\",\"0\"]],\"c\":[\"7347702391542317289078324477957712035210582056186479239076715504548941012834\",\"795883936884678581860170407596096541519605830081875833581950897247827301651\",\"1\"]},\"issBase64Details\":{\"value\":\"yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC\",\"indexMod4\":1},\"headerBase64\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjMyM2IyMTRhZTY5NzVhMGYwMzRlYTc3MzU0ZGMwYzI1ZDAzNjQyZGMiLCJ0eXAiOiJKV1QifQ\"},\"extended_ephemeral_public_key\":\"AK0lMA42JuE0/rIgOKKAhzcK3oOvpqGR2yznt/WNsu1c\", \"verification_key_id\": 0, \"modulus\": \"xjWd1j8GmmWzuz732haG9HECXsSZBvxOBLph3FQhk_tplhWloI1ywx-RdopUZt1lndbOM9n99lZJkpQyNJ1sdy7JFgYLjqj-wtHdEaQlBGEQtmkW8zUjr_N3bmpsxGbPzOzlKe3qddtoxXvn9rI_RvHfJD1YY-6kayQeyPOBz_4ML1lvI_JHV-Bb1MSmSk3WaAh5PzeqleusmUT87Gqfu02cOPrY8cwugqo65D6-wzAEeVvceV8-c36TMoLU5csU05GBVplgd6Ouuw35ZsETG4si4QQJztC3KsZ4jhYM-aJ3jeFPt0r3cQooiXdZBp3JkXSpE-UUaOVPsXo7WiVmww\", \"kid\": \"323b214ae6975a0f034ea77354dc0c25d03642dc\",\"max_epoch\":142}";
pub const TEST_AUTH_DATA_13: &str = "{\"jwt\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjMyM2IyMTRhZTY5NzVhMGYwMzRlYTc3MzU0ZGMwYzI1ZDAzNjQyZGMiLCJ0eXAiOiJKV1QifQ.eyJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLCJhenAiOiIyMzI2MjQwODUxOTEtdjF0cTIwZmcxa2RoaGd2YXQ2c2FqN2pmMGhkODIzM3IuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJhdWQiOiIyMzI2MjQwODUxOTEtdjF0cTIwZmcxa2RoaGd2YXQ2c2FqN2pmMGhkODIzM3IuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJzdWIiOiIxMTI4OTc0Njg2MjY3MTY2MjYxMDMiLCJub25jZSI6IjVqNVBySXFpYW9xaUNFWkR0eklNazY2MUotRSIsIm5iZiI6MTcxNTY4NjkyNCwiaWF0IjoxNzE1Njg3MjI0LCJleHAiOjE3MTU2OTA4MjQsImp0aSI6IjJmYzk0MmM1MDBiMmJmNGE5YzZiZjUwN2Y0MjU4NTg3MGM4YmQ5N2QifQ.GU70HImKkqZyGmWAC_onzc-ccUhALeT7ebQ0LrE0QGqjCZyCnonjOeDhatB4Q1GQCVQ-KPWKCdg4NNPCPvKwLYAjwNF0sorwS5h6jKKVvRgT_t12dbDzrPKJE7xW0_0kfmfj7lKGZp_W4HNVxd_hlPiwJb56X0ZVkt3pwpkwBe8MU-Nzb3QyrJtDRJDDb4v_bVdOJSyUNEtssFvAgFB4diGI_GFQzZpbQnBeciST-lS7rGHpItnlwe0mRNf3e34S7A7wUOo_YTvy-TKTViSekMdkMKt9hgGkti9c4dYwI8NMExe4wtnLFVOh6XZ0FtrdnVGrYZFMWTJjNGizUmFMZQ\",\"user_pass_to_int_format\":\"525451555057114102\",\"ephemeral_key_pair\":{\"keypair\":{\"public_key\":{\"0\":221,\"1\":11,\"2\":223,\"3\":171,\"4\":2,\"5\":140,\"6\":112,\"7\":100,\"8\":233,\"9\":182,\"10\":68,\"11\":219,\"12\":126,\"13\":215,\"14\":96,\"15\":164,\"16\":201,\"17\":227,\"18\":132,\"19\":169,\"20\":157,\"21\":120,\"22\":187,\"23\":16,\"24\":40,\"25\":208,\"26\":174,\"27\":209,\"28\":89,\"29\":163,\"30\":255,\"31\":62},\"secret_key\":{\"0\":5,\"1\":6,\"2\":91,\"3\":164,\"4\":51,\"5\":203,\"6\":161,\"7\":246,\"8\":61,\"9\":156,\"10\":92,\"11\":96,\"12\":69,\"13\":141,\"14\":93,\"15\":73,\"16\":208,\"17\":85,\"18\":37,\"19\":52,\"20\":167,\"21\":121,\"22\":63,\"23\":221,\"24\":215,\"25\":165,\"26\":48,\"27\":232,\"28\":136,\"29\":10,\"30\":71,\"31\":92,\"32\":221,\"33\":11,\"34\":223,\"35\":171,\"36\":2,\"37\":140,\"38\":112,\"39\":100,\"40\":233,\"41\":182,\"42\":68,\"43\":219,\"44\":126,\"45\":215,\"46\":96,\"47\":164,\"48\":201,\"49\":227,\"50\":132,\"51\":169,\"52\":157,\"53\":120,\"54\":187,\"55\":16,\"56\":40,\"57\":208,\"58\":174,\"59\":209,\"60\":89,\"61\":163,\"62\":255,\"63\":62}}},\"zk_addr\":\"0x0934ba96e39b32a66b83afdd089d9534b91336d0c72324acb72b718e2d8adcd8\",\"zk_proofs\":{\"proofPoints\":{\"a\":[\"3292113297742390468701372446942400025026948502434627571571387058022780524172\",\"4608365882159831859997420943605862565647863626478617897572911626264555729258\",\"1\"],\"b\":[[\"5662407938030293510048180382159430467791189346676904212329490391470516566946\",\"14655907382794614210872210515582570998106075620115645016125280695488094003217\"],[\"3337061425406207163991320131711738442766654603337106758166291266688030689117\",\"4469383376673348053098454774700074508703514397281065469277327859575940584146\"],[\"1\",\"0\"]],\"c\":[\"6592007510647447256322156763481821378802835999285873915184749854236303252416\",\"16208563039085392733361585085996378606127672981771155339865393880548209917912\",\"1\"]},\"issBase64Details\":{\"value\":\"yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC\",\"indexMod4\":1},\"headerBase64\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjMyM2IyMTRhZTY5NzVhMGYwMzRlYTc3MzU0ZGMwYzI1ZDAzNjQyZGMiLCJ0eXAiOiJKV1QifQ\"},\"extended_ephemeral_public_key\":\"AN0L36sCjHBk6bZE237XYKTJ44SpnXi7ECjQrtFZo/8+\", \"verification_key_id\": 0, \"modulus\": \"xjWd1j8GmmWzuz732haG9HECXsSZBvxOBLph3FQhk_tplhWloI1ywx-RdopUZt1lndbOM9n99lZJkpQyNJ1sdy7JFgYLjqj-wtHdEaQlBGEQtmkW8zUjr_N3bmpsxGbPzOzlKe3qddtoxXvn9rI_RvHfJD1YY-6kayQeyPOBz_4ML1lvI_JHV-Bb1MSmSk3WaAh5PzeqleusmUT87Gqfu02cOPrY8cwugqo65D6-wzAEeVvceV8-c36TMoLU5csU05GBVplgd6Ouuw35ZsETG4si4QQJztC3KsZ4jhYM-aJ3jeFPt0r3cQooiXdZBp3JkXSpE-UUaOVPsXo7WiVmww\", \"kid\": \"323b214ae6975a0f034ea77354dc0c25d03642dc\",\"max_epoch\":142}";
pub const TEST_AUTH_DATA_14: &str = "{\"jwt\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjMyM2IyMTRhZTY5NzVhMGYwMzRlYTc3MzU0ZGMwYzI1ZDAzNjQyZGMiLCJ0eXAiOiJKV1QifQ.eyJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLCJhenAiOiIyMzI2MjQwODUxOTEtdjF0cTIwZmcxa2RoaGd2YXQ2c2FqN2pmMGhkODIzM3IuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJhdWQiOiIyMzI2MjQwODUxOTEtdjF0cTIwZmcxa2RoaGd2YXQ2c2FqN2pmMGhkODIzM3IuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJzdWIiOiIxMTI4OTc0Njg2MjY3MTY2MjYxMDMiLCJub25jZSI6IjN6eHc2SUFERzVXcjJDR244SGczS1Q3Nm1qOCIsIm5iZiI6MTcxNTY4NjkwNSwiaWF0IjoxNzE1Njg3MjA1LCJleHAiOjE3MTU2OTA4MDUsImp0aSI6IjNkOTM4NmFlODMxZDVhYjdiZTI1NjcxMDhmZjhkMTM1N2YzNDZjOTUifQ.R3s_OfTiDlMMSFsEfp4xM6rLoJ99GALalEE1TVG8aneruEWuI1qxz241YmX9r9-49t1ja5BfO0eh3Fu_p6lg1O32sNSLR626Mvrv1Ph60syPQN01Tam4RCV_YBK3b2Pj-rWeJq3WSCGQg2rab2QyHy3Al9VPdXlkbaaH69QzRSXFyNojixgo92cPhABxbAxI1a5pYmzwwfkDDO0FY5uRUt3w4wuBhx9gQ6g_kboF03pIzQ5kvGUYBPGax66faTzulAGdTADmU9xgG6denQoZWn3Lh6dfdQX8KXkn9jVY8gMIY_rbobc8nkIMmslsjjio7BXb90-YD_WJT5so5Cre3A\",\"user_pass_to_int_format\":\"1031021041155552\",\"ephemeral_key_pair\":{\"keypair\":{\"public_key\":{\"0\":97,\"1\":100,\"2\":35,\"3\":169,\"4\":212,\"5\":9,\"6\":238,\"7\":108,\"8\":186,\"9\":80,\"10\":106,\"11\":26,\"12\":209,\"13\":87,\"14\":84,\"15\":117,\"16\":235,\"17\":25,\"18\":81,\"19\":248,\"20\":137,\"21\":197,\"22\":146,\"23\":139,\"24\":214,\"25\":127,\"26\":143,\"27\":179,\"28\":137,\"29\":79,\"30\":181,\"31\":216},\"secret_key\":{\"0\":70,\"1\":76,\"2\":130,\"3\":97,\"4\":75,\"5\":0,\"6\":7,\"7\":122,\"8\":166,\"9\":56,\"10\":85,\"11\":179,\"12\":143,\"13\":55,\"14\":136,\"15\":47,\"16\":75,\"17\":211,\"18\":125,\"19\":145,\"20\":130,\"21\":206,\"22\":118,\"23\":212,\"24\":87,\"25\":200,\"26\":130,\"27\":38,\"28\":65,\"29\":93,\"30\":37,\"31\":44,\"32\":97,\"33\":100,\"34\":35,\"35\":169,\"36\":212,\"37\":9,\"38\":238,\"39\":108,\"40\":186,\"41\":80,\"42\":106,\"43\":26,\"44\":209,\"45\":87,\"46\":84,\"47\":117,\"48\":235,\"49\":25,\"50\":81,\"51\":248,\"52\":137,\"53\":197,\"54\":146,\"55\":139,\"56\":214,\"57\":127,\"58\":143,\"59\":179,\"60\":137,\"61\":79,\"62\":181,\"63\":216}}},\"zk_addr\":\"0x87b9236aadcbc8de1a2bce17bb104cbae2f8c955f89808ee2d258cf2bc1cce1f\",\"zk_proofs\":{\"proofPoints\":{\"a\":[\"18415333747085688267133796133445868671450647215619171648016630248725573444572\",\"13021999644739913954648136527237689315935942107782566659768353668730521796833\",\"1\"],\"b\":[[\"10379715945772584677584721710592153467187645980157575584584703890180885281296\",\"21114541349211062821701871386552875726196087055162878583823021987759476907947\"],[\"21741245524391086016724288544952241247835975701957615054057894483829435111137\",\"19675246006347690391662817422022652459552259504790883596539945355325572896761\"],[\"1\",\"0\"]],\"c\":[\"6388980351498388564470364481867721519510272532387680761911853865824806443040\",\"2927953057998420964296253396822428516251336255094433794401337892358172944522\",\"1\"]},\"issBase64Details\":{\"value\":\"yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC\",\"indexMod4\":1},\"headerBase64\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjMyM2IyMTRhZTY5NzVhMGYwMzRlYTc3MzU0ZGMwYzI1ZDAzNjQyZGMiLCJ0eXAiOiJKV1QifQ\"},\"extended_ephemeral_public_key\":\"AGFkI6nUCe5sulBqGtFXVHXrGVH4icWSi9Z/j7OJT7XY\", \"verification_key_id\": 0, \"modulus\": \"xjWd1j8GmmWzuz732haG9HECXsSZBvxOBLph3FQhk_tplhWloI1ywx-RdopUZt1lndbOM9n99lZJkpQyNJ1sdy7JFgYLjqj-wtHdEaQlBGEQtmkW8zUjr_N3bmpsxGbPzOzlKe3qddtoxXvn9rI_RvHfJD1YY-6kayQeyPOBz_4ML1lvI_JHV-Bb1MSmSk3WaAh5PzeqleusmUT87Gqfu02cOPrY8cwugqo65D6-wzAEeVvceV8-c36TMoLU5csU05GBVplgd6Ouuw35ZsETG4si4QQJztC3KsZ4jhYM-aJ3jeFPt0r3cQooiXdZBp3JkXSpE-UUaOVPsXo7WiVmww\", \"kid\": \"323b214ae6975a0f034ea77354dc0c25d03642dc\",\"max_epoch\":142}";
pub const TEST_AUTH_DATA_15: &str = "{\"jwt\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjMyM2IyMTRhZTY5NzVhMGYwMzRlYTc3MzU0ZGMwYzI1ZDAzNjQyZGMiLCJ0eXAiOiJKV1QifQ.eyJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLCJhenAiOiIyMzI2MjQwODUxOTEtdjF0cTIwZmcxa2RoaGd2YXQ2c2FqN2pmMGhkODIzM3IuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJhdWQiOiIyMzI2MjQwODUxOTEtdjF0cTIwZmcxa2RoaGd2YXQ2c2FqN2pmMGhkODIzM3IuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJzdWIiOiIxMTI4OTc0Njg2MjY3MTY2MjYxMDMiLCJub25jZSI6IjRtbk1TajFjTmdlSUU4Unk3Z2dBOWNRVWRLSSIsIm5iZiI6MTcxNTY4Njg3MywiaWF0IjoxNzE1Njg3MTczLCJleHAiOjE3MTU2OTA3NzMsImp0aSI6ImVmYzU3YjVmMGEzZmEwZGI2ZTQzNWFhZjI4OTEwNjY2YjM0NWViNmEifQ.YRFPl_szPP8iBid__ACAj4Etr4YDZEmeawFTas_MFw7rR_sD_tQ268F2g9O4VOU3VWSWT-LCG1gp_NdRVvb5SFBzuMIYp4YrUEvzJdaO_ab1a2Xp_EVVEmjMwNHVpnFZjS9El0e0oOmaw_PQgC2soauJkfLvRhayx-_Vps7htHm94PW1aHBOxwr2HpR58mjzT4JyutyiioCLgLqhnvGW4N6CBlx6iLNfITk0wwsAOHRcdjW_hk0hHarjMy3U2VdbcPkmq1OIg8ZDQo2jbUGEWevUC6zrGeNWYjp38f3Wo1NUqf7_ne0YeJEBtyK5r9BuDxdr6YRyUXKnpxJpr9cZ-Q\",\"user_pass_to_int_format\":\"5256515057\",\"ephemeral_key_pair\":{\"keypair\":{\"public_key\":{\"0\":226,\"1\":91,\"2\":63,\"3\":50,\"4\":152,\"5\":120,\"6\":233,\"7\":249,\"8\":177,\"9\":205,\"10\":1,\"11\":233,\"12\":153,\"13\":199,\"14\":101,\"15\":124,\"16\":112,\"17\":15,\"18\":160,\"19\":228,\"20\":124,\"21\":169,\"22\":57,\"23\":196,\"24\":118,\"25\":117,\"26\":94,\"27\":132,\"28\":228,\"29\":108,\"30\":145,\"31\":117},\"secret_key\":{\"0\":168,\"1\":100,\"2\":5,\"3\":144,\"4\":15,\"5\":220,\"6\":219,\"7\":42,\"8\":52,\"9\":1,\"10\":7,\"11\":203,\"12\":43,\"13\":71,\"14\":99,\"15\":90,\"16\":8,\"17\":66,\"18\":137,\"19\":155,\"20\":200,\"21\":27,\"22\":69,\"23\":112,\"24\":209,\"25\":173,\"26\":109,\"27\":93,\"28\":152,\"29\":210,\"30\":96,\"31\":194,\"32\":226,\"33\":91,\"34\":63,\"35\":50,\"36\":152,\"37\":120,\"38\":233,\"39\":249,\"40\":177,\"41\":205,\"42\":1,\"43\":233,\"44\":153,\"45\":199,\"46\":101,\"47\":124,\"48\":112,\"49\":15,\"50\":160,\"51\":228,\"52\":124,\"53\":169,\"54\":57,\"55\":196,\"56\":118,\"57\":117,\"58\":94,\"59\":132,\"60\":228,\"61\":108,\"62\":145,\"63\":117}}},\"zk_addr\":\"0xc2e01f23756fd4fc3e8ee98f96751729c911acc1b8abc4e5d8f732a0b6a69602\",\"zk_proofs\":{\"proofPoints\":{\"a\":[\"6734924940747627006546678824977458478287976951246203795880487352562116664933\",\"14763323532227801517600705873776227782564830701170466315373208681644431001874\",\"1\"],\"b\":[[\"19846719805329609703868726781640931109590522837532762309497922458996335263239\",\"15420764526732603133646176483042915906318651960194518136943858294265541434918\"],[\"3657954841783806502381750774780312041530173171470043250309926815017975476219\",\"3502207265482905042029962996793932548717468210237619905023157797841132512624\"],[\"1\",\"0\"]],\"c\":[\"1288521393482492105362792882426011805774869298603270001189992299082351112997\",\"3336108234609612516660580995781529303851605528785003185796473743343393403477\",\"1\"]},\"issBase64Details\":{\"value\":\"yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC\",\"indexMod4\":1},\"headerBase64\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjMyM2IyMTRhZTY5NzVhMGYwMzRlYTc3MzU0ZGMwYzI1ZDAzNjQyZGMiLCJ0eXAiOiJKV1QifQ\"},\"extended_ephemeral_public_key\":\"AOJbPzKYeOn5sc0B6ZnHZXxwD6DkfKk5xHZ1XoTkbJF1\", \"verification_key_id\": 0, \"modulus\": \"xjWd1j8GmmWzuz732haG9HECXsSZBvxOBLph3FQhk_tplhWloI1ywx-RdopUZt1lndbOM9n99lZJkpQyNJ1sdy7JFgYLjqj-wtHdEaQlBGEQtmkW8zUjr_N3bmpsxGbPzOzlKe3qddtoxXvn9rI_RvHfJD1YY-6kayQeyPOBz_4ML1lvI_JHV-Bb1MSmSk3WaAh5PzeqleusmUT87Gqfu02cOPrY8cwugqo65D6-wzAEeVvceV8-c36TMoLU5csU05GBVplgd6Ouuw35ZsETG4si4QQJztC3KsZ4jhYM-aJ3jeFPt0r3cQooiXdZBp3JkXSpE-UUaOVPsXo7WiVmww\", \"kid\": \"323b214ae6975a0f034ea77354dc0c25d03642dc\",\"max_epoch\":142}";
pub const TEST_AUTH_DATA_16: &str = "{\"jwt\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjMyM2IyMTRhZTY5NzVhMGYwMzRlYTc3MzU0ZGMwYzI1ZDAzNjQyZGMiLCJ0eXAiOiJKV1QifQ.eyJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLCJhenAiOiIyMzI2MjQwODUxOTEtdjF0cTIwZmcxa2RoaGd2YXQ2c2FqN2pmMGhkODIzM3IuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJhdWQiOiIyMzI2MjQwODUxOTEtdjF0cTIwZmcxa2RoaGd2YXQ2c2FqN2pmMGhkODIzM3IuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJzdWIiOiIxMTI4OTc0Njg2MjY3MTY2MjYxMDMiLCJub25jZSI6ImpKS0poeXJtbHE3T1UxU0NwZnlfX2F3MEZxVSIsIm5iZiI6MTcxNTY4Njg2NSwiaWF0IjoxNzE1Njg3MTY1LCJleHAiOjE3MTU2OTA3NjUsImp0aSI6Ijg4ZDVmNTg1OGMzNDIwMmY1OTAyOWE5ODM4YzhhOWMzYWVlMDZjNDMifQ.cjuamUo90ycOmkGffs4qe6Ozb0q-UhG6oG4pLf3a5zMgRUXr_PcKNj9GcHujqYzWFbVsiuYdoVwMmPsHeKmLnkuIDS4mwT0z-LhWvYrXdx2FksXyv0ECIBJNGHWNtf6JyhA_3XGYSqzn4sQncKxHK82aFAJZYaPfXCgKJJK0c9PFjONxY2nQoDV-IM89vm6x9vpNPjYxMxxE60p_5qceLLU9pgy4jgP2Eyco0sGfCFTry7zVqgYsMSinh_UIWk4naihDtgrZxAdNkoAA-4PQkWxrlTO8b68YQp8K4ncerUwmOJDs-0NUxDm8mTjwq47Qgf_UAcTxVN9_YpIqEoxdwQ\",\"user_pass_to_int_format\":\"100102104119101105101121102\",\"ephemeral_key_pair\":{\"keypair\":{\"public_key\":{\"0\":109,\"1\":20,\"2\":190,\"3\":101,\"4\":236,\"5\":16,\"6\":171,\"7\":49,\"8\":222,\"9\":170,\"10\":22,\"11\":241,\"12\":224,\"13\":116,\"14\":18,\"15\":124,\"16\":48,\"17\":1,\"18\":20,\"19\":126,\"20\":94,\"21\":16,\"22\":164,\"23\":173,\"24\":180,\"25\":226,\"26\":71,\"27\":184,\"28\":218,\"29\":162,\"30\":145,\"31\":87},\"secret_key\":{\"0\":131,\"1\":117,\"2\":15,\"3\":104,\"4\":243,\"5\":100,\"6\":1,\"7\":157,\"8\":31,\"9\":54,\"10\":163,\"11\":215,\"12\":45,\"13\":202,\"14\":70,\"15\":51,\"16\":77,\"17\":200,\"18\":206,\"19\":59,\"20\":210,\"21\":59,\"22\":129,\"23\":250,\"24\":53,\"25\":166,\"26\":201,\"27\":57,\"28\":9,\"29\":13,\"30\":255,\"31\":18,\"32\":109,\"33\":20,\"34\":190,\"35\":101,\"36\":236,\"37\":16,\"38\":171,\"39\":49,\"40\":222,\"41\":170,\"42\":22,\"43\":241,\"44\":224,\"45\":116,\"46\":18,\"47\":124,\"48\":48,\"49\":1,\"50\":20,\"51\":126,\"52\":94,\"53\":16,\"54\":164,\"55\":173,\"56\":180,\"57\":226,\"58\":71,\"59\":184,\"60\":218,\"61\":162,\"62\":145,\"63\":87}}},\"zk_addr\":\"0x3a26feb6fa552d6e2796e37cfcfaa19ff8d09b9b3e30060557f313ec82e7809a\",\"zk_proofs\":{\"proofPoints\":{\"a\":[\"17553248964460513660899064860794334313854327380740726458707730696116622715951\",\"19780935404993030841853448182973442738138094386982905919654095638585438825727\",\"1\"],\"b\":[[\"21560192083940754229490187081097411180154947135453319375957763951829010741758\",\"19864576266509862087012277908356289924851686435133221538927641836878678315039\"],[\"5332198541444016097635381835036279771892300735490162251066050727152100828695\",\"4562785582599067136108384927870755899035073041220030123445496806313655366742\"],[\"1\",\"0\"]],\"c\":[\"17180793399699270264610473764500109290307106335241771936808740744446379111802\",\"19531923144281240440166451089649574065952850605237982747209921274042428958350\",\"1\"]},\"issBase64Details\":{\"value\":\"yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC\",\"indexMod4\":1},\"headerBase64\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjMyM2IyMTRhZTY5NzVhMGYwMzRlYTc3MzU0ZGMwYzI1ZDAzNjQyZGMiLCJ0eXAiOiJKV1QifQ\"},\"extended_ephemeral_public_key\":\"AG0UvmXsEKsx3qoW8eB0EnwwARR+XhCkrbTiR7jaopFX\", \"verification_key_id\": 0, \"modulus\": \"xjWd1j8GmmWzuz732haG9HECXsSZBvxOBLph3FQhk_tplhWloI1ywx-RdopUZt1lndbOM9n99lZJkpQyNJ1sdy7JFgYLjqj-wtHdEaQlBGEQtmkW8zUjr_N3bmpsxGbPzOzlKe3qddtoxXvn9rI_RvHfJD1YY-6kayQeyPOBz_4ML1lvI_JHV-Bb1MSmSk3WaAh5PzeqleusmUT87Gqfu02cOPrY8cwugqo65D6-wzAEeVvceV8-c36TMoLU5csU05GBVplgd6Ouuw35ZsETG4si4QQJztC3KsZ4jhYM-aJ3jeFPt0r3cQooiXdZBp3JkXSpE-UUaOVPsXo7WiVmww\", \"kid\": \"323b214ae6975a0f034ea77354dc0c25d03642dc\",\"max_epoch\":142}";
pub const TEST_AUTH_DATA_17: &str = "{\"jwt\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjMyM2IyMTRhZTY5NzVhMGYwMzRlYTc3MzU0ZGMwYzI1ZDAzNjQyZGMiLCJ0eXAiOiJKV1QifQ.eyJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLCJhenAiOiIyMzI2MjQwODUxOTEtdjF0cTIwZmcxa2RoaGd2YXQ2c2FqN2pmMGhkODIzM3IuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJhdWQiOiIyMzI2MjQwODUxOTEtdjF0cTIwZmcxa2RoaGd2YXQ2c2FqN2pmMGhkODIzM3IuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJzdWIiOiIxMTI4OTc0Njg2MjY3MTY2MjYxMDMiLCJub25jZSI6IkxPYUk4MkJHS1lSWXVETVRMVXU4Z0ktV3ZsayIsIm5iZiI6MTcxNTY4Njg1NywiaWF0IjoxNzE1Njg3MTU3LCJleHAiOjE3MTU2OTA3NTcsImp0aSI6ImMyOGE2MTEwYzMwYmQ0ZDkzMmEwOTNlOWVmZjllNzEyZjUzYWI2MjQifQ.fHYn7sAFbOtPneSX_YBA52ASidwosnl42uWF7RmroUU132sPO3Jmzf7tqZrqQFu04Y1G2LeGvTeHklUowVdWdKQkomV9bCeputcMRkDPD9-5-UJdpDY8eIAzfHzWN9nWyu5St0Iz0S0FIth6cesMmPUkCrq6pCUyHLWgrxUoICuYIbbtEO5ZVnF8lIeMjUTLXT4_9svFBRhugkD0nvHnQnWS8H0ijS53lCs8z7xVy0cm_MawsCMpApMQvWm-4CeIq69p3m2HXclXNmwSxg7oeGDKn-yqhPaXX3Pn4PfHKPj-XHXOR2rr9uG2lYi73yOyDve84wCXzV9kmiUnc0YGUQ\",\"user_pass_to_int_format\":\"515253575654\",\"ephemeral_key_pair\":{\"keypair\":{\"public_key\":{\"0\":100,\"1\":216,\"2\":222,\"3\":171,\"4\":27,\"5\":27,\"6\":171,\"7\":132,\"8\":172,\"9\":13,\"10\":174,\"11\":188,\"12\":196,\"13\":208,\"14\":35,\"15\":125,\"16\":10,\"17\":214,\"18\":5,\"19\":29,\"20\":118,\"21\":41,\"22\":114,\"23\":70,\"24\":166,\"25\":37,\"26\":189,\"27\":136,\"28\":37,\"29\":106,\"30\":245,\"31\":15},\"secret_key\":{\"0\":75,\"1\":60,\"2\":159,\"3\":196,\"4\":243,\"5\":180,\"6\":224,\"7\":198,\"8\":228,\"9\":147,\"10\":22,\"11\":104,\"12\":69,\"13\":182,\"14\":80,\"15\":232,\"16\":127,\"17\":195,\"18\":43,\"19\":2,\"20\":99,\"21\":206,\"22\":161,\"23\":47,\"24\":106,\"25\":44,\"26\":131,\"27\":5,\"28\":133,\"29\":110,\"30\":82,\"31\":140,\"32\":100,\"33\":216,\"34\":222,\"35\":171,\"36\":27,\"37\":27,\"38\":171,\"39\":132,\"40\":172,\"41\":13,\"42\":174,\"43\":188,\"44\":196,\"45\":208,\"46\":35,\"47\":125,\"48\":10,\"49\":214,\"50\":5,\"51\":29,\"52\":118,\"53\":41,\"54\":114,\"55\":70,\"56\":166,\"57\":37,\"58\":189,\"59\":136,\"60\":37,\"61\":106,\"62\":245,\"63\":15}}},\"zk_addr\":\"0x89045412e3f5c808e7bf0ea6d47008a6c75f14b48a7fea54f420b03d3298ef4e\",\"zk_proofs\":{\"proofPoints\":{\"a\":[\"257137064185145448465242836924275827726618300508610920187643303682623341809\",\"13136104385342155450873138185971548464179081219858145555616259274682267182602\",\"1\"],\"b\":[[\"9623367967771069752036280248035047299371597257306258748768218269896381701321\",\"12210765432002064938981141260402135327544184192147240766501813387730760651726\"],[\"15251118264052002493837427778759923199895437430037469672801786148252966111936\",\"12243121821747384937988506024826071890328897029202152518609157933400978560340\"],[\"1\",\"0\"]],\"c\":[\"4201350126073080124441494110984461007902792066210632786985402248838880518314\",\"10425614983366289743736253875955608779721351186796918402238008669517994775682\",\"1\"]},\"issBase64Details\":{\"value\":\"yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC\",\"indexMod4\":1},\"headerBase64\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjMyM2IyMTRhZTY5NzVhMGYwMzRlYTc3MzU0ZGMwYzI1ZDAzNjQyZGMiLCJ0eXAiOiJKV1QifQ\"},\"extended_ephemeral_public_key\":\"AGTY3qsbG6uErA2uvMTQI30K1gUddilyRqYlvYglavUP\", \"verification_key_id\": 0, \"modulus\": \"xjWd1j8GmmWzuz732haG9HECXsSZBvxOBLph3FQhk_tplhWloI1ywx-RdopUZt1lndbOM9n99lZJkpQyNJ1sdy7JFgYLjqj-wtHdEaQlBGEQtmkW8zUjr_N3bmpsxGbPzOzlKe3qddtoxXvn9rI_RvHfJD1YY-6kayQeyPOBz_4ML1lvI_JHV-Bb1MSmSk3WaAh5PzeqleusmUT87Gqfu02cOPrY8cwugqo65D6-wzAEeVvceV8-c36TMoLU5csU05GBVplgd6Ouuw35ZsETG4si4QQJztC3KsZ4jhYM-aJ3jeFPt0r3cQooiXdZBp3JkXSpE-UUaOVPsXo7WiVmww\", \"kid\": \"323b214ae6975a0f034ea77354dc0c25d03642dc\",\"max_epoch\":142}";
pub const TEST_AUTH_DATA_18: &str = "{\"jwt\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjMyM2IyMTRhZTY5NzVhMGYwMzRlYTc3MzU0ZGMwYzI1ZDAzNjQyZGMiLCJ0eXAiOiJKV1QifQ.eyJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLCJhenAiOiIyMzI2MjQwODUxOTEtdjF0cTIwZmcxa2RoaGd2YXQ2c2FqN2pmMGhkODIzM3IuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJhdWQiOiIyMzI2MjQwODUxOTEtdjF0cTIwZmcxa2RoaGd2YXQ2c2FqN2pmMGhkODIzM3IuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJzdWIiOiIxMTI4OTc0Njg2MjY3MTY2MjYxMDMiLCJub25jZSI6IlhhakktZ0NqbFVVY3ZBTFdmcDlyTURCelBzVSIsIm5iZiI6MTcxNTY4Njg0OSwiaWF0IjoxNzE1Njg3MTQ5LCJleHAiOjE3MTU2OTA3NDksImp0aSI6IjZmYmI3NmEzN2NjOTAwYTQ5NThlYmNlZmNmYmVhNjMzMzkyMzM2OTUifQ.RqLBHMZMuXbsZGW5YGDNbfTSGG5Ezv_XtJRvMbBIXytAqGoT70RrfZSwU3e8yXaq-o4RBoeypQIygj_Sjxq0JJXVRuypVqkbismASkWKWH77avFgRUe0Etvc8EFXupmwj1biRpURUukroVUyjktOI17m3DvFIIan7_rq3SQBNxLyjFZav517zaJaUVXdYMDAYIEVs1Es04G2kWTxBYQ6iu0jyHtuNcg9_kosGQEZjnp2HsnvegrRwloyjuFByMRv90bRuV6cc3f-3GPO23tcrhFzeoOQXUfcSdlqE3C92gb6E_3uBld414mNj2LelnagKtpvPjTCgX3tic2c7fB_CQ\",\"user_pass_to_int_format\":\"989911510011710554\",\"ephemeral_key_pair\":{\"keypair\":{\"public_key\":{\"0\":81,\"1\":243,\"2\":172,\"3\":238,\"4\":183,\"5\":132,\"6\":17,\"7\":7,\"8\":200,\"9\":125,\"10\":73,\"11\":248,\"12\":71,\"13\":220,\"14\":159,\"15\":138,\"16\":16,\"17\":207,\"18\":25,\"19\":103,\"20\":70,\"21\":23,\"22\":193,\"23\":72,\"24\":27,\"25\":94,\"26\":241,\"27\":155,\"28\":98,\"29\":155,\"30\":212,\"31\":118},\"secret_key\":{\"0\":89,\"1\":13,\"2\":132,\"3\":115,\"4\":95,\"5\":59,\"6\":196,\"7\":68,\"8\":136,\"9\":46,\"10\":22,\"11\":70,\"12\":5,\"13\":188,\"14\":76,\"15\":116,\"16\":156,\"17\":15,\"18\":226,\"19\":232,\"20\":167,\"21\":204,\"22\":143,\"23\":148,\"24\":230,\"25\":69,\"26\":18,\"27\":166,\"28\":234,\"29\":47,\"30\":178,\"31\":31,\"32\":81,\"33\":243,\"34\":172,\"35\":238,\"36\":183,\"37\":132,\"38\":17,\"39\":7,\"40\":200,\"41\":125,\"42\":73,\"43\":248,\"44\":71,\"45\":220,\"46\":159,\"47\":138,\"48\":16,\"49\":207,\"50\":25,\"51\":103,\"52\":70,\"53\":23,\"54\":193,\"55\":72,\"56\":27,\"57\":94,\"58\":241,\"59\":155,\"60\":98,\"61\":155,\"62\":212,\"63\":118}}},\"zk_addr\":\"0xa41d812b2137a9e701512dd1e77643b94c3eff566c5be50365d174d1db60a415\",\"zk_proofs\":{\"proofPoints\":{\"a\":[\"17614076016833085587577424708926810169376585820815427961767207039192652769013\",\"14135032500684844024372302628625185135058981148802759791768119531914068811069\",\"1\"],\"b\":[[\"12223738407851653989769057205672802523120105196291312843502064346721414495287\",\"633499823246797838323329834422571844397737716575556571535890211670105511423\"],[\"6003190178099558462113377195569012506764289704920013648574803015959961275195\",\"2773541228770509456407096233964565540804779880988894871588471857181885931620\"],[\"1\",\"0\"]],\"c\":[\"1069242590881057236271046634996302431045048055413724998035360474439616694142\",\"4170832142623397447640837445675045579300900045561776497865454715900704844006\",\"1\"]},\"issBase64Details\":{\"value\":\"yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC\",\"indexMod4\":1},\"headerBase64\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjMyM2IyMTRhZTY5NzVhMGYwMzRlYTc3MzU0ZGMwYzI1ZDAzNjQyZGMiLCJ0eXAiOiJKV1QifQ\"},\"extended_ephemeral_public_key\":\"AFHzrO63hBEHyH1J+Efcn4oQzxlnRhfBSBte8Ztim9R2\", \"verification_key_id\": 0, \"modulus\": \"xjWd1j8GmmWzuz732haG9HECXsSZBvxOBLph3FQhk_tplhWloI1ywx-RdopUZt1lndbOM9n99lZJkpQyNJ1sdy7JFgYLjqj-wtHdEaQlBGEQtmkW8zUjr_N3bmpsxGbPzOzlKe3qddtoxXvn9rI_RvHfJD1YY-6kayQeyPOBz_4ML1lvI_JHV-Bb1MSmSk3WaAh5PzeqleusmUT87Gqfu02cOPrY8cwugqo65D6-wzAEeVvceV8-c36TMoLU5csU05GBVplgd6Ouuw35ZsETG4si4QQJztC3KsZ4jhYM-aJ3jeFPt0r3cQooiXdZBp3JkXSpE-UUaOVPsXo7WiVmww\", \"kid\": \"323b214ae6975a0f034ea77354dc0c25d03642dc\",\"max_epoch\":142}";
pub const TEST_AUTH_DATA_19: &str = "{\"jwt\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjMyM2IyMTRhZTY5NzVhMGYwMzRlYTc3MzU0ZGMwYzI1ZDAzNjQyZGMiLCJ0eXAiOiJKV1QifQ.eyJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLCJhenAiOiIyMzI2MjQwODUxOTEtdjF0cTIwZmcxa2RoaGd2YXQ2c2FqN2pmMGhkODIzM3IuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJhdWQiOiIyMzI2MjQwODUxOTEtdjF0cTIwZmcxa2RoaGd2YXQ2c2FqN2pmMGhkODIzM3IuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJzdWIiOiIxMTI4OTc0Njg2MjY3MTY2MjYxMDMiLCJub25jZSI6IkxyN2JvekNHNUw4UHlLVVNPRzNwMDZYYVM1USIsIm5iZiI6MTcxNTY4Njg0MSwiaWF0IjoxNzE1Njg3MTQxLCJleHAiOjE3MTU2OTA3NDEsImp0aSI6Ijc2MzdiMGMyM2ZiMWIwZTFjMGEyYWE3NWVkOGMxNjA1YzE1YWFiZjAifQ.uNMFOgl9xdG5wljwZrIDzWm3SS_F9OLhR9avDGRhHSxYNSzexcOHtGT7HY9zsWloN9LWFZxu2t3yG-jWduo5qYgyM-OXpdAXLzfXZwQSNxgtXl2yisxeBU18_7lPpmjMzTMUPCXtJxrB75VYoZAybkyGnFmC_tPD13MIShT04iUGkNLFPpaof4BGxnmCE4hNob-tVijFTH_EIdNXg0fr-rQ-qxd3vw7NVDIF0yDNxCeSYMz0GKuGPlvXk3SPtUzfUfZaJFau3QpfcrXhkNrUS0fW3HcXRLMhiVqNIJ5Y5wYJdq5IvEe_lElrv4NS4apswDNVI1s7B_iMDvcjFASD9Q\",\"user_pass_to_int_format\":\"5255515057565453\",\"ephemeral_key_pair\":{\"keypair\":{\"public_key\":{\"0\":223,\"1\":83,\"2\":48,\"3\":161,\"4\":204,\"5\":195,\"6\":149,\"7\":141,\"8\":132,\"9\":65,\"10\":55,\"11\":201,\"12\":245,\"13\":60,\"14\":139,\"15\":236,\"16\":39,\"17\":130,\"18\":130,\"19\":162,\"20\":215,\"21\":104,\"22\":235,\"23\":117,\"24\":152,\"25\":71,\"26\":252,\"27\":46,\"28\":73,\"29\":54,\"30\":170,\"31\":251},\"secret_key\":{\"0\":90,\"1\":130,\"2\":14,\"3\":79,\"4\":237,\"5\":213,\"6\":128,\"7\":240,\"8\":11,\"9\":61,\"10\":50,\"11\":225,\"12\":67,\"13\":212,\"14\":26,\"15\":215,\"16\":84,\"17\":207,\"18\":4,\"19\":3,\"20\":95,\"21\":124,\"22\":35,\"23\":123,\"24\":72,\"25\":189,\"26\":115,\"27\":153,\"28\":16,\"29\":105,\"30\":73,\"31\":216,\"32\":223,\"33\":83,\"34\":48,\"35\":161,\"36\":204,\"37\":195,\"38\":149,\"39\":141,\"40\":132,\"41\":65,\"42\":55,\"43\":201,\"44\":245,\"45\":60,\"46\":139,\"47\":236,\"48\":39,\"49\":130,\"50\":130,\"51\":162,\"52\":215,\"53\":104,\"54\":235,\"55\":117,\"56\":152,\"57\":71,\"58\":252,\"59\":46,\"60\":73,\"61\":54,\"62\":170,\"63\":251}}},\"zk_addr\":\"0x37c4424a1b9970b94dd7276aecae5b9d1c035c7e3c88f4f1155aa0e5127ef6e4\",\"zk_proofs\":{\"proofPoints\":{\"a\":[\"7945493796354284921600453177054285773255312631482061079984629835363646515586\",\"2166517138136751277833084326208436279446476764263036612847849710284540628729\",\"1\"],\"b\":[[\"14768147580014515274920059533450969526917243519129235569375547705391356814034\",\"10926704359346438742364088104571886636979515204481541507299552373423645137538\"],[\"18345707220306299341155061798987886250677895640406984732019863169577306401665\",\"13781450607771983148196301814354815025344242496715512941320154501577226245887\"],[\"1\",\"0\"]],\"c\":[\"16100487697721354409255314346417284275475569122937970611421991273969908317416\",\"20037727069966515075925458192010761910249599063237527691300280470015098486501\",\"1\"]},\"issBase64Details\":{\"value\":\"yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC\",\"indexMod4\":1},\"headerBase64\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjMyM2IyMTRhZTY5NzVhMGYwMzRlYTc3MzU0ZGMwYzI1ZDAzNjQyZGMiLCJ0eXAiOiJKV1QifQ\"},\"extended_ephemeral_public_key\":\"AN9TMKHMw5WNhEE3yfU8i+wngoKi12jrdZhH/C5JNqr7\", \"verification_key_id\": 0, \"modulus\": \"xjWd1j8GmmWzuz732haG9HECXsSZBvxOBLph3FQhk_tplhWloI1ywx-RdopUZt1lndbOM9n99lZJkpQyNJ1sdy7JFgYLjqj-wtHdEaQlBGEQtmkW8zUjr_N3bmpsxGbPzOzlKe3qddtoxXvn9rI_RvHfJD1YY-6kayQeyPOBz_4ML1lvI_JHV-Bb1MSmSk3WaAh5PzeqleusmUT87Gqfu02cOPrY8cwugqo65D6-wzAEeVvceV8-c36TMoLU5csU05GBVplgd6Ouuw35ZsETG4si4QQJztC3KsZ4jhYM-aJ3jeFPt0r3cQooiXdZBp3JkXSpE-UUaOVPsXo7WiVmww\", \"kid\": \"323b214ae6975a0f034ea77354dc0c25d03642dc\",\"max_epoch\":142}";
pub const TEST_AUTH_DATA_20: &str = "{\"jwt\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjMyM2IyMTRhZTY5NzVhMGYwMzRlYTc3MzU0ZGMwYzI1ZDAzNjQyZGMiLCJ0eXAiOiJKV1QifQ.eyJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLCJhenAiOiIyMzI2MjQwODUxOTEtdjF0cTIwZmcxa2RoaGd2YXQ2c2FqN2pmMGhkODIzM3IuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJhdWQiOiIyMzI2MjQwODUxOTEtdjF0cTIwZmcxa2RoaGd2YXQ2c2FqN2pmMGhkODIzM3IuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJzdWIiOiIxMTI4OTc0Njg2MjY3MTY2MjYxMDMiLCJub25jZSI6ImJlMTgtZlBDdV9ZRndybE43TDhKV1BHX3hFbyIsIm5iZiI6MTcxNTY4NjgzMSwiaWF0IjoxNzE1Njg3MTMxLCJleHAiOjE3MTU2OTA3MzEsImp0aSI6IjdmZTFkZTM5NDVkMjliYTBhOWQ4MGFlODZiZGRmNjkyMDE1N2RlMDcifQ.RMM8wIiEzZ97DVdngkDhKapTMZq-R7woI2yjclLqTgnYZKTZ5N9y67zFJLDfcg017VyyRK18OS1OLsnUgnphi3ULotImnJ2292VDBd7kxhyq9QAqfHVDK2-MYNlJXy53UIr2xS9td1aoHUDZkvBy690IhV4nPrxLOUhI8c4gAvpkfHFmAvxYuQoUu69c_hSzREhrVOa979t5nZuJjNWwUcwgD40To1DM6Dxwy186basvY4AyWPHcI4ARFoyPEMRFUOtO05fUrwUH8O63Ay6K1DwxaLXDzx4T7O9X9nlrCj2uROdahsv-Dj24hruudSYxi4GH2uO6u0a1RlTIvWJ-1A\",\"user_pass_to_int_format\":\"525451525655\",\"ephemeral_key_pair\":{\"keypair\":{\"public_key\":{\"0\":66,\"1\":155,\"2\":237,\"3\":117,\"4\":45,\"5\":166,\"6\":245,\"7\":92,\"8\":78,\"9\":225,\"10\":218,\"11\":156,\"12\":7,\"13\":132,\"14\":164,\"15\":47,\"16\":114,\"17\":174,\"18\":4,\"19\":86,\"20\":18,\"21\":212,\"22\":182,\"23\":62,\"24\":50,\"25\":219,\"26\":104,\"27\":185,\"28\":183,\"29\":108,\"30\":38,\"31\":252},\"secret_key\":{\"0\":13,\"1\":127,\"2\":13,\"3\":29,\"4\":128,\"5\":121,\"6\":142,\"7\":51,\"8\":210,\"9\":28,\"10\":131,\"11\":160,\"12\":209,\"13\":42,\"14\":214,\"15\":198,\"16\":137,\"17\":147,\"18\":155,\"19\":40,\"20\":86,\"21\":167,\"22\":168,\"23\":10,\"24\":249,\"25\":180,\"26\":188,\"27\":132,\"28\":41,\"29\":146,\"30\":192,\"31\":28,\"32\":66,\"33\":155,\"34\":237,\"35\":117,\"36\":45,\"37\":166,\"38\":245,\"39\":92,\"40\":78,\"41\":225,\"42\":218,\"43\":156,\"44\":7,\"45\":132,\"46\":164,\"47\":47,\"48\":114,\"49\":174,\"50\":4,\"51\":86,\"52\":18,\"53\":212,\"54\":182,\"55\":62,\"56\":50,\"57\":219,\"58\":104,\"59\":185,\"60\":183,\"61\":108,\"62\":38,\"63\":252}}},\"zk_addr\":\"0x86ab13e3c90b7f5b52a0e7d045425f1a5ce4f2938d82fe32013b5c5dffc8aa40\",\"zk_proofs\":{\"proofPoints\":{\"a\":[\"13280060882937967421268531103181473070897547065707830941266439167277535861998\",\"9934433138500558951890280258062370504217382435917636186873727481367280202864\",\"1\"],\"b\":[[\"3838124130316726849360987686807592227651253623250263168834039482151640975443\",\"10050190797101422174255450354163308725608018844614813729840170282126147936409\"],[\"18360080471111693027482741715722945557865825591442098780536696036281663618095\",\"1378964582828950987975075563637558653759765511530268169302574447782691787466\"],[\"1\",\"0\"]],\"c\":[\"1373142722414479432483215105546507593017308819682036641663292686387425172376\",\"3353210342014729825799146687716012229927760750084040417279416030868174996451\",\"1\"]},\"issBase64Details\":{\"value\":\"yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC\",\"indexMod4\":1},\"headerBase64\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjMyM2IyMTRhZTY5NzVhMGYwMzRlYTc3MzU0ZGMwYzI1ZDAzNjQyZGMiLCJ0eXAiOiJKV1QifQ\"},\"extended_ephemeral_public_key\":\"AEKb7XUtpvVcTuHanAeEpC9yrgRWEtS2PjLbaLm3bCb8\", \"verification_key_id\": 0, \"modulus\": \"xjWd1j8GmmWzuz732haG9HECXsSZBvxOBLph3FQhk_tplhWloI1ywx-RdopUZt1lndbOM9n99lZJkpQyNJ1sdy7JFgYLjqj-wtHdEaQlBGEQtmkW8zUjr_N3bmpsxGbPzOzlKe3qddtoxXvn9rI_RvHfJD1YY-6kayQeyPOBz_4ML1lvI_JHV-Bb1MSmSk3WaAh5PzeqleusmUT87Gqfu02cOPrY8cwugqo65D6-wzAEeVvceV8-c36TMoLU5csU05GBVplgd6Ouuw35ZsETG4si4QQJztC3KsZ4jhYM-aJ3jeFPt0r3cQooiXdZBp3JkXSpE-UUaOVPsXo7WiVmww\", \"kid\": \"323b214ae6975a0f034ea77354dc0c25d03642dc\",\"max_epoch\":142}";
pub const TEST_AUTH_DATA_21: &str = "{\"jwt\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjMyM2IyMTRhZTY5NzVhMGYwMzRlYTc3MzU0ZGMwYzI1ZDAzNjQyZGMiLCJ0eXAiOiJKV1QifQ.eyJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLCJhenAiOiIyMzI2MjQwODUxOTEtdjF0cTIwZmcxa2RoaGd2YXQ2c2FqN2pmMGhkODIzM3IuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJhdWQiOiIyMzI2MjQwODUxOTEtdjF0cTIwZmcxa2RoaGd2YXQ2c2FqN2pmMGhkODIzM3IuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJzdWIiOiIxMTI4OTc0Njg2MjY3MTY2MjYxMDMiLCJub25jZSI6Im12NUNKY1dsMXE1ek03Y0lyR1ZUdHF6SnFTNCIsIm5iZiI6MTcxNTY4NjgxOSwiaWF0IjoxNzE1Njg3MTE5LCJleHAiOjE3MTU2OTA3MTksImp0aSI6IjYxMDM2YmQwZWE3YjI5MDY4MjgwODYxMzMyODZhODZlNmY0ZmMwNGEifQ.VE2a8s2ZuyTVklFwSvh05y_mGrDMJXww-5Pu3-UUIQi3sBQnMzpnvWo3MIb32rXxwU6Obtx9izsR-Csk-U0QH4WuseGHnhHA90lACdeXNXHUWNktsY62_z2lkseTlJQV_ccNVctNgqornxmtV6gRvihLKkYCJt08umhAcRe8-Fh9iNmlCf5sMngaA-k0bvIbdnxkoP0KI9em7sgpTDB0FJFCgVAVYkzQTuJJlfuKjeF0lgpLnkjTOtgMyCpuZrrxf9GH6wY2VSme3Zk6xVJfl5cC6YugQFs-t56CEhPDrm-LIlLTD9JuNAKctlRRaTmkTembZAzweu6Wqh322MDx1g\",\"user_pass_to_int_format\":\"100102100102100115106107\",\"ephemeral_key_pair\":{\"keypair\":{\"public_key\":{\"0\":227,\"1\":142,\"2\":234,\"3\":83,\"4\":36,\"5\":125,\"6\":219,\"7\":233,\"8\":159,\"9\":30,\"10\":60,\"11\":195,\"12\":110,\"13\":130,\"14\":105,\"15\":107,\"16\":44,\"17\":46,\"18\":151,\"19\":154,\"20\":116,\"21\":131,\"22\":237,\"23\":231,\"24\":159,\"25\":119,\"26\":35,\"27\":130,\"28\":56,\"29\":90,\"30\":121,\"31\":26},\"secret_key\":{\"0\":34,\"1\":107,\"2\":197,\"3\":227,\"4\":209,\"5\":156,\"6\":36,\"7\":233,\"8\":231,\"9\":171,\"10\":100,\"11\":210,\"12\":113,\"13\":247,\"14\":59,\"15\":222,\"16\":214,\"17\":129,\"18\":238,\"19\":254,\"20\":13,\"21\":13,\"22\":3,\"23\":151,\"24\":9,\"25\":173,\"26\":77,\"27\":113,\"28\":126,\"29\":7,\"30\":203,\"31\":52,\"32\":227,\"33\":142,\"34\":234,\"35\":83,\"36\":36,\"37\":125,\"38\":219,\"39\":233,\"40\":159,\"41\":30,\"42\":60,\"43\":195,\"44\":110,\"45\":130,\"46\":105,\"47\":107,\"48\":44,\"49\":46,\"50\":151,\"51\":154,\"52\":116,\"53\":131,\"54\":237,\"55\":231,\"56\":159,\"57\":119,\"58\":35,\"59\":130,\"60\":56,\"61\":90,\"62\":121,\"63\":26}}},\"zk_addr\":\"0xb092062dc38ee15b239fedd8955547cae553068e350add6a186a900308ca1704\",\"zk_proofs\":{\"proofPoints\":{\"a\":[\"19083893384522082364200015848081882762611466511855277173108395395690822433582\",\"15765871630522826744343212165387977339454134778029147662517994756792436892191\",\"1\"],\"b\":[[\"3347275249816013439391836622904014049361323482158752310150932588414843416768\",\"9261935324040115069949005166058871304117632278716385673922763868694265924905\"],[\"10774327302040930015542399179222458502634829694095804484749135988841930351850\",\"409015645239595129631982791901837203813000443262394810220799589635024410401\"],[\"1\",\"0\"]],\"c\":[\"805618312212200473153836203801534856685701602304097276485035246177305246575\",\"12984127923817330198936709848850019846193356630613954592225359007088568774616\",\"1\"]},\"issBase64Details\":{\"value\":\"yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC\",\"indexMod4\":1},\"headerBase64\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjMyM2IyMTRhZTY5NzVhMGYwMzRlYTc3MzU0ZGMwYzI1ZDAzNjQyZGMiLCJ0eXAiOiJKV1QifQ\"},\"extended_ephemeral_public_key\":\"AOOO6lMkfdvpnx48w26CaWssLpeadIPt5593I4I4Wnka\", 
 \"verification_key_id\": 0, \"modulus\": \"xjWd1j8GmmWzuz732haG9HECXsSZBvxOBLph3FQhk_tplhWloI1ywx-RdopUZt1lndbOM9n99lZJkpQyNJ1sdy7JFgYLjqj-wtHdEaQlBGEQtmkW8zUjr_N3bmpsxGbPzOzlKe3qddtoxXvn9rI_RvHfJD1YY-6kayQeyPOBz_4ML1lvI_JHV-Bb1MSmSk3WaAh5PzeqleusmUT87Gqfu02cOPrY8cwugqo65D6-wzAEeVvceV8-c36TMoLU5csU05GBVplgd6Ouuw35ZsETG4si4QQJztC3KsZ4jhYM-aJ3jeFPt0r3cQooiXdZBp3JkXSpE-UUaOVPsXo7WiVmww\", \"kid\": \"323b214ae6975a0f034ea77354dc0c25d03642dc\",\"max_epoch\":142}";

pub const TEST_AUTH_DATA_FRESH: &str = "{\"jwt\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6ImYxMDMzODYwNzE2ZTNhMmFhYjM4MGYwMGRiZTM5YTcxMTQ4NDZiYTEiLCJ0eXAiOiJKV1QifQ.eyJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLCJhenAiOiIyMzI2MjQwODUxOTEtdjF0cTIwZmcxa2RoaGd2YXQ2c2FqN2pmMGhkODIzM3IuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJhdWQiOiIyMzI2MjQwODUxOTEtdjF0cTIwZmcxa2RoaGd2YXQ2c2FqN2pmMGhkODIzM3IuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJzdWIiOiIxMTI4OTc0Njg2MjY3MTY2MjYxMDMiLCJub25jZSI6Iml6RW1FOEpLZjFOaWVHa1BVNndVQWR0cWJtbyIsIm5iZiI6MTc1MjYxMzE2MSwiaWF0IjoxNzUyNjEzNDYxLCJleHAiOjE3NTI2MTcwNjEsImp0aSI6IjFhZTQyMGMyOGExZjY4YTNhYjBjNDYyZDNjNTNlOTg3NGE5MzVkNDMifQ.mzmcsnVqkzjEydnxcdFab-lRA9B7yknqTUfaV2Vlmbrp2xpbs6PmMtaGkBGpZdr0Y0N8PawUpLYSiJch-KmVh4VbPiGVDnUKx2lZAERAtbDio4wWimlQnWUDiXVz5DhKM9Pfihydh6q1wIBbamiUmDVdTvXkCtv3zO_ArfIjtBLnYC_6KGa5IByDkUdlib9FbDQELDbcxLf8YcUSyMBazWOYPTjpA6EiO6Tcbe9SFHnoReKcA1Ek6K6UCndt9Zr1ADcL9drdI7ixK1c2SHGXXOUhbqOjsYyWmUropm2C8Rn1MWbZz9WWLYXxjOTlTC3gCfp6itnTOTEp-oVW2VAaJQ\", \"verification_key_id\": 0, \"modulus\": \"sdyhQG7zNcpPq1-s5ZmUP45pVcwpjjyhQVoXWe0g19iGLJOLTtwiwJUI_qJzEyIwCZkgjwcWoAIR6Anfox2c-vb2vP59B7530pOX28eIY-J_ppbeDIHogvAqhX9ahYhVQCzbbul1_Wxv9cLk_06w846kx0LrL7dXuCzBtyTGYXBzPS2PXoQOcjJi4JfqZI6lvIDSRVoN7hgfq9BUwcVp-UXW1Jf5EKyQmyAx0Ivixu5IaSiMsWEvDo7_uDEv9CDmwlKZsF9qut3wTII3yzCg69qu36bEEBRdx7M5g0dtsuRVTnAIJ82H7n4G9LOF5PP4bgXed1rf3oPt31fc_XXzLQ\", \"kid\": \"f1033860716e3a2aab380f00dbe39a7114846ba1\", \"user_pass_to_int_format\":\"535455565748\",\"zk_addr\":\"0xa780f54b9c44308332c81d9910470e067380af96c2f494205e927c5f93386005\",\"ephemeral_key_pair\":{\"keypair\":{\"public_key\":{\"0\":29,\"1\":23,\"2\":196,\"3\":190,\"4\":224,\"5\":168,\"6\":9,\"7\":252,\"8\":242,\"9\":112,\"10\":68,\"11\":4,\"12\":183,\"13\":44,\"14\":7,\"15\":22,\"16\":124,\"17\":230,\"18\":7,\"19\":43,\"20\":46,\"21\":177,\"22\":246,\"23\":62,\"24\":128,\"25\":11,\"26\":126,\"27\":8,\"28\":171,\"29\":226,\"30\":225,\"31\":236},\"secret_key\":{\"0\":58,\"1\":213,\"2\":168,\"3\":116,\"4\":197,\"5\":50,\"6\":206,\"7\":228,\"8\":106,\"9\":209,\"10\":14,\"11\":222,\"12\":254,\"13\":77,\"14\":191,\"15\":124,\"16\":142,\"17\":218,\"18\":217,\"19\":138,\"20\":67,\"21\":99,\"22\":26,\"23\":83,\"24\":45,\"25\":119,\"26\":3,\"27\":141,\"28\":235,\"29\":78,\"30\":99,\"31\":229,\"32\":29,\"33\":23,\"34\":196,\"35\":190,\"36\":224,\"37\":168,\"38\":9,\"39\":252,\"40\":242,\"41\":112,\"42\":68,\"43\":4,\"44\":183,\"45\":44,\"46\":7,\"47\":22,\"48\":124,\"49\":230,\"50\":7,\"51\":43,\"52\":46,\"53\":177,\"54\":246,\"55\":62,\"56\":128,\"57\":11,\"58\":126,\"59\":8,\"60\":171,\"61\":226,\"62\":225,\"63\":236}}},\"max_epoch\":1837114018,\"extended_ephemeral_public_key\":\"AB0XxL7gqAn88nBEBLcsBxZ85gcrLrH2PoALfgir4uHs\",\"zk_proofs\":{\"proofPoints\":{\"a\":[\"8098501600751384607181957645677626866532764788942422666412343156192339990006\",\"10528383926009508904901560728659084063525809434312214815294299598511272252222\",\"1\"],\"b\":[[\"12273362542038333747074136237347628667941686059782692695276273693714075094673\",\"2749069376382143209930861303528393317873864398313230406407907981917555114272\"],[\"17109196682169777907568373999801323000639248111802898565368344964148009079205\",\"20352890122634867334039055397632120557786427784229577313094916831738514221399\"],[\"1\",\"0\"]],\"c\":[\"881201683053613832821423547970858663302376847569702386628102966219039283015\",\"2358963888410267814511637656505351097303584267645553588604891102365780883725\",\"1\"]},  \"issBase64Details\":{\"value\":\"yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC\",\"indexMod4\":1},\"headerBase64\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6ImYxMDMzODYwNzE2ZTNhMmFhYjM4MGYwMGRiZTM5YTcxMTQ4NDZiYTEiLCJ0eXAiOiJKV1QifQ\"}}";

#[test]
fn test_poseidon_and_vergrth16_for_multiple_data() {
    let elector_code = load_boc("benches/elector-code.boc");
    let elector_data = load_boc("benches/elector-data.boc");
    let config_data = load_boc("benches/config-data.boc");
    let mut ctrls = SaveList::default();
    ctrls.put(4, &mut StackItem::Cell(elector_data)).unwrap();
    let params = vec![
        StackItem::int(0x76ef1ea),
        StackItem::int(0),
        StackItem::int(0),
        StackItem::int(1633458077),
        StackItem::int(0),
        StackItem::int(0),
        StackItem::int(0),
        StackItem::tuple(vec![StackItem::int(1000000000), StackItem::None]),
        StackItem::slice(
            SliceData::from_string(
                "9fe0000000000000000000000000000000000000000000000000000000000000001_",
            )
            .unwrap(),
        ),
        StackItem::cell(config_data.reference(0).unwrap()),
        StackItem::None,
        StackItem::int(0),
    ];
    ctrls.put(7, &mut StackItem::tuple(vec![StackItem::tuple(params)])).unwrap();
    let stack = Stack::new();
    let mut engine = Engine::with_capabilities(DEFAULT_CAPABILITIES).setup_with_libraries(
        SliceData::load_cell_ref(&elector_code).unwrap(),
        Some(ctrls.clone()),
        Some(stack.clone()),
        None,
        vec![],
    );

    let data: Vec<&str> = vec![
            TEST_AUTH_DATA_1,
            TEST_AUTH_DATA_2,
            TEST_AUTH_DATA_3,
            TEST_AUTH_DATA_4,
            TEST_AUTH_DATA_5,
            TEST_AUTH_DATA_6,
            TEST_AUTH_DATA_7,
            TEST_AUTH_DATA_8,
            TEST_AUTH_DATA_9,
            TEST_AUTH_DATA_10,
            TEST_AUTH_DATA_11,
            TEST_AUTH_DATA_12,
            TEST_AUTH_DATA_13,
            TEST_AUTH_DATA_14,
            TEST_AUTH_DATA_15,
            TEST_AUTH_DATA_16,
            TEST_AUTH_DATA_17,
            TEST_AUTH_DATA_18,
            TEST_AUTH_DATA_19,
            TEST_AUTH_DATA_20,
            TEST_AUTH_DATA_21,
            TEST_AUTH_DATA_FRESH         
    ];


    let mut average_poseidon: u128 = 0;
    let mut average_vergrth16: u128 = 0;

    for i in 0..data.len() {
        println!("");
        println!("====================== Iter@ is {i} =========================");
        println!("jwt_data: {:?}", data[i]);
        let jwt_data: JwtData = serde_json::from_str(&data[i]).unwrap();
        println!("jwt_data: {:?}", jwt_data);

        let verification_key_id: u32 = jwt_data.verification_key_id;

        let content: JWK = JWK {
            kty: "RSA".to_string(),
            e: "AQAB".to_string(),
            n: jwt_data.modulus,
            alg: "RS256".to_string(),
        };

        let mut all_jwk = HashMap::new();
        all_jwk.insert(
            JwkId::new(
                OIDCProvider::Google.get_config().iss,
                jwt_data.kid,
            ),
            content,
        );

        let user_pass_salt = jwt_data.user_pass_to_int_format.as_str();
        println!("user_pass_salt is {user_pass_salt}");

        println!("{:?}", jwt_data.ephemeral_key_pair.keypair.public_key);
        let eph_secret_key = secret_key_from_integer_map(jwt_data.ephemeral_key_pair.keypair.secret_key);

        let ephemeral_kp = Ed25519KeyPair::from_bytes(&eph_secret_key).unwrap();
        let mut eph_pubkey: Vec<u8> = Vec::new(); // vec![0x00];
        eph_pubkey.extend(ephemeral_kp.public().as_ref());

        println!("ephemeral secret_key is {:?}", eph_secret_key);
        println!("ephemeral public_key is {:?}", eph_pubkey);

        let eph_pubkey_len = eph_pubkey.clone().len();
        println!("len eph_pubkey: {:?}", eph_pubkey_len);

        let jwt_data_vector: Vec<&str> = jwt_data.jwt.split(".").collect();
        let jwt_data_1 = decode(jwt_data_vector[0]).expect("Base64 decoding failed");

        let jwt_string_1 = String::from_utf8(jwt_data_1).expect("UTF-8 conversion failed");
        println!("jwt_string_1 is {:?}", jwt_string_1); // jwt_string_1 is
        // "{\"alg\":\"RS256\",\"kid\":\"323b214ae6975a0f034ea77354dc0c25d03642dc\",\"
        // typ\":\"JWT\"}"
        // JwtDataDecodedPart1
        let jwt_data_decoded1: JwtDataDecodedPart1 =
            serde_json::from_str(&jwt_string_1).unwrap();
        println!("kid: {:?}", jwt_data_decoded1.kid);

        let jwt_data_2 = decode(jwt_data_vector[1]).expect("Base64 decoding failed");
        let jwt_string_2 = String::from_utf8(jwt_data_2).expect("UTF-8 conversion failed");
        println!("jwt_string_2 is {:?}", jwt_string_2); 
        
        let jwt_data_decoded2: JwtDataDecodedPart2Google =
            serde_json::from_str(&jwt_string_2).unwrap();
        println!("aud: {:?}", jwt_data_decoded2.aud);
        println!("sub: {:?}", jwt_data_decoded2.sub);

        let zk_seed = gen_address_seed(
            user_pass_salt,
            "sub",
            jwt_data_decoded2.sub.as_str(), 
            jwt_data_decoded2.aud.as_str(),
            )
        .unwrap();

        println!("jwt_data.zk_proofs = {:?}", jwt_data.zk_proofs);
        let proof_and_jwt = serde_json::to_string(&jwt_data.zk_proofs).unwrap();

        let zk_login_inputs =
            ZkLoginInputs::from_json(&*proof_and_jwt.to_string(), &*zk_seed.to_string())
        .unwrap();

        let (iss, kid) =
            (zk_login_inputs.get_iss().to_string(), zk_login_inputs.get_kid().to_string());
        let jwk = all_jwk.get(&JwkId::new(iss.clone(), kid.clone())).ok_or_else(|| {
        ZkCryptoError::GeneralError(format!("JWK not found ({} - {})", iss, kid))
        }).unwrap();

        let max_epoch = jwt_data.max_epoch;

        let modulus = base64ct::Base64UrlUnpadded::decode_vec(&jwk.n)
        .map_err(|_| {
            ZkCryptoError::GeneralError("Invalid Base64 encoded jwk modulus".to_string())
        })
        .unwrap();

        let public_inputs = &[zk_login_inputs.calculate_all_inputs_hash(&eph_pubkey, &modulus, max_epoch).unwrap()];

        println!("modulus_ hex = {:?}", hex::encode(modulus.clone()));

        let mut public_inputs_as_bytes = vec![];
        public_inputs.serialize_compressed(&mut public_inputs_as_bytes).unwrap();
        println!("HERE public_inputs_as_bytes : {:?}", public_inputs_as_bytes);
        println!("HERE public_inputs_as_bytes len : {:?}", public_inputs_as_bytes.len());
        println!("====== Start Poseidon ========");
    
        let index_mod_4 = jwt_data.zk_proofs.iss_base64_details.index_mod4;
        engine.cc.stack.push(StackItem::int(index_mod_4));
        engine.cc.stack.push(StackItem::int(max_epoch));
        engine.cc.stack.push(StackItem::integer(IntegerData::from_unsigned_bytes_be(&eph_pubkey.clone())));

        let modulus_cell = pack_data_to_cell(&modulus.clone(), &mut 0).unwrap();
        println!("modulus_cell = {:?}", modulus_cell);
        engine.cc.stack.push(StackItem::cell(modulus_cell.clone()));

        let iss_base_64= jwt_data.zk_proofs.iss_base64_details.value;
    
        let iss_base_64_cell = pack_string_to_cell(&iss_base_64, &mut 0).unwrap();
        println!("iss_base_64_cell = {:?}", iss_base_64_cell);
        engine.cc.stack.push(StackItem::cell(iss_base_64_cell.clone()));

        let header_base_64 = jwt_data.zk_proofs.header_base64;

        let header_base_64_cell = pack_string_to_cell(&header_base_64, &mut 0).unwrap();
        println!("header_base_64_cell = {:?}", header_base_64_cell);
        engine.cc.stack.push(StackItem::cell(header_base_64_cell.clone()));

        let zk_seed_cell = pack_string_to_cell(&zk_seed.clone(), &mut 0).unwrap();
        println!("zk_seed_cell = {:?}", zk_seed_cell);
        engine.cc.stack.push(StackItem::cell(zk_seed_cell.clone()));

        let start: Instant = Instant::now();
        let status = execute_poseidon_zk_login(&mut engine).unwrap();
        let poseidon_elapsed = start.elapsed().as_micros();

        
        let poseidon_res = engine.cc.stack.get(0).as_cell().unwrap();
        let slice = SliceData::load_cell(poseidon_res.clone()).unwrap();
        let poseidon_res = unpack_data_from_cell(slice, &mut engine).unwrap();
        println!("poseidon_res from stack: {:?}", hex::encode(poseidon_res.clone()));

        println!("public_inputs hex (computed in test): {:?}", hex::encode(public_inputs_as_bytes.clone()));
        assert!(poseidon_res == public_inputs_as_bytes);

        println!("poseidon_elapsed in microsecond: {:?}", poseidon_elapsed);  

        average_poseidon = average_poseidon + poseidon_elapsed;


        println!("====== Start VERGRTH16 ========");
        let proof = &zk_login_inputs.get_proof().as_arkworks().unwrap();
        let mut proof_as_bytes = vec![];
        proof.serialize_compressed(&mut proof_as_bytes).unwrap();
        println!("proof_as_bytes : {:?}", proof_as_bytes);
        println!("proof_as_bytes : {:?}", hex::encode(proof_as_bytes.clone()));
        println!("proof_as_bytes len: {:?}", proof_as_bytes.len());

        let proof_cell = pack_data_to_cell(&proof_as_bytes, &mut 0).unwrap();
        engine.cc.stack.push(StackItem::cell(proof_cell.clone()));

        let public_inputs_cell = pack_data_to_cell(&public_inputs_as_bytes.clone(), &mut 0).unwrap();
        engine.cc.stack.push(StackItem::cell(public_inputs_cell.clone()));

        
        engine.cc.stack.push(StackItem::int(verification_key_id));

        let start: Instant = Instant::now();
        let status = execute_vergrth16(&mut engine).unwrap();
        let vergrth16_elapsed = start.elapsed().as_micros();

        println!("vergrth16_elapsed in microsecond: {:?}", vergrth16_elapsed); 

        let res = engine.cc.stack.get(0).as_integer().unwrap();
        println!("res: {:?}", res);
        assert!(*res == IntegerData::minus_one());

        average_vergrth16 = average_vergrth16 + vergrth16_elapsed;
    }

    println!("===================================");
    println!("===================================");
    println!("===================================");
    println!("===================================");

    let average_poseidon_=  average_poseidon / (data.len() as u128);
    println!("average_poseidon_ in microsecond: {:?}", average_poseidon_);  
    let average_vergrth16_=  average_vergrth16 / (data.len() as u128);
    println!("average_vergrth16_ in microsecond: {:?}", average_vergrth16_); 

}

///////////////////////////////////

#[test]
fn test_poseidon_and_vergrth16() {
    let elector_code = load_boc("benches/elector-code.boc");
    let elector_data = load_boc("benches/elector-data.boc");
    let config_data = load_boc("benches/config-data.boc");
    let mut ctrls = SaveList::default();
    ctrls.put(4, &mut StackItem::Cell(elector_data)).unwrap();
    let params = vec![
        StackItem::int(0x76ef1ea),
        StackItem::int(0),
        StackItem::int(0),
        StackItem::int(1633458077),
        StackItem::int(0),
        StackItem::int(0),
        StackItem::int(0),
        StackItem::tuple(vec![StackItem::int(1000000000), StackItem::None]),
        StackItem::slice(
            SliceData::from_string(
                "9fe0000000000000000000000000000000000000000000000000000000000000001_",
            )
            .unwrap(),
        ),
        StackItem::cell(config_data.reference(0).unwrap()),
        StackItem::None,
        StackItem::int(0),
    ];
    ctrls.put(7, &mut StackItem::tuple(vec![StackItem::tuple(params)])).unwrap();
    let stack = Stack::new();
    let mut engine = Engine::with_capabilities(DEFAULT_CAPABILITIES).setup_with_libraries(
        SliceData::load_cell_ref(&elector_code).unwrap(),
        Some(ctrls.clone()),
        Some(stack.clone()),
        None,
        vec![],
    );

 
     // password was 567890 in ascii 535455565748
    let user_pass_salt = "535455565748";
    let secret_key = [222, 248, 61, 101, 214, 199, 113, 189, 223, 94, 151, 140, 235, 182, 203, 46, 143, 162, 166, 87, 162, 250, 176, 4, 29, 19, 42, 221, 116, 33, 178, 14,
    ];
    let ephemeral_kp = Ed25519KeyPair::from_bytes(&secret_key).unwrap(); // 
    let mut eph_pubkey = Vec::new();
    eph_pubkey.extend(ephemeral_kp.public().as_ref());
    println!("eph_pubkey: {:?}", eph_pubkey);
    println!("len eph_pubkey: {:?}", eph_pubkey.len());
    let eph_pubkey_hex_number = "0x".to_owned() + &hex::encode(eph_pubkey.clone());
    println!("eph_pubkey_hex_number: {:?}", eph_pubkey_hex_number);
    let zk_seed = gen_address_seed(
        user_pass_salt,
        "sub",
        "112897468626716626103",
        "232624085191-v1tq20fg1kdhhgvat6saj7jf0hd8233r.apps.googleusercontent.com",
    )
    .unwrap();
    println!("zk_seed = {:?}", zk_seed);
    let proof_and_jwt = "{\"proofPoints\":{\"a\":[\"2352077003566407045854435506409565889408960755152253285189640818725808263237\",\
    \"9548308350778027075240385782578683112366097953461273569343148999989145049123\",\"1\"],\
    \"b\":[[\"2172697685172701179756462481453772004245591587568555358926512547679273443868\",\
    \"11300889616992175665271080883374830731684409375838395487979439153562369168807\"],\
    [\"18769153619672444537277685186545610305405730219274884099876386487766026068190\",\
    \"12892936063156115176399929981646174277274895601746717550262309650970826515227\"],[\"1\",\"0\"]],\
    \"c\":[\"21276833037675249246843718004583052134371270695679878402069223253610209272159\",\
    \"8637596258221986824049981569842218428861929142818091935707054543971817804456\",\"1\"]},\
    \"issBase64Details\":{\"value\":\"yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC\",\"indexMod4\":1},\
    \"headerBase64\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6ImEzYjc2MmY4NzFjZGIzYmFlMDA0NGM2NDk2MjJmYzEzOTZlZGEzZTMiLCJ0eXAiOiJKV1QifQ\"}";
    let len = proof_and_jwt.bytes().len();
    println!(" proof_and_jwt_bytes len (in bytes) = {:?}", len);

    println!("proof_and_jwt: {}", proof_and_jwt);

    let iss_and_header_base64details = "{\"issBase64Details\":{\"value\":\"yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC\",\"indexMod4\":1},\"headerBase64\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6ImEzYjc2MmY4NzFjZGIzYmFlMDA0NGM2NDk2MjJmYzEzOTZlZGEzZTMiLCJ0eXAiOiJKV1QifQ\"}";

    println!("iss_and_header_base64details: {}", iss_and_header_base64details);

    let header_base_64 = "eyJhbGciOiJSUzI1NiIsImtpZCI6ImEzYjc2MmY4NzFjZGIzYmFlMDA0NGM2NDk2MjJmYzEzOTZlZGEzZTMiLCJ0eXAiOiJKV1QifQ";
    let iss_base_64 = "yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC";
    

    let zk_login_inputs = ZkLoginInputs::from_json(&*proof_and_jwt, &*zk_seed.to_string()).unwrap();
    let content: JWK = JWK {
        kty: "RSA".to_string(),
        e: "AQAB".to_string(),
        n: "uBHF-esPKiNlFaAvpdpejD4vpONW9FL0rgLDg1z8Q-x_CiHCvJCpiSehD41zmDOhzXP_fbMMSGpGL7R3duiz01nK5r_YmRw3RXeB0kcS7Z9H8MN6IJcde9MWbqkMabCDduFgdr6gvH0QbTipLB1qJK_oI_IBfRgjk6G0bGrKz3PniQw5TZ92r0u1LM-1XdBIb3aTYTGDW9KlOsrTTuKq0nj-anW5TXhecuxqSveFM4Hwlw7pw34ydBunFjFWDx4VVJqGNSqWCfcERxOulizIFruZIHJGkgunZnB4DF7mCZOttx2dwT9j7s3GfLJf0xoGumqpOMvecuipfTPeIdAzcQ".to_string(), // Alina's data
        alg: "RS256".to_string(),
    };

    let mut all_jwk = HashMap::new();
     all_jwk.insert(
        JwkId::new(
            OIDCProvider::Google.get_config().iss,
            "a3b762f871cdb3bae0044c649622fc1396eda3e3".to_string(), 
        ),
        content,
    );

    let (iss, kid) =
            (zk_login_inputs.get_iss().to_string(), zk_login_inputs.get_kid().to_string());
    let jwk = all_jwk.get(&JwkId::new(iss.clone(), kid.clone())).ok_or_else(|| {
        ZkCryptoError::GeneralError(format!("JWK not found ({} - {})", iss, kid))
    }).unwrap();

    let max_epoch = 142; 

    let modulus = base64ct::Base64UrlUnpadded::decode_vec(&jwk.n)
        .map_err(|_| {
            ZkCryptoError::GeneralError("Invalid Base64 encoded jwk modulus".to_string())
        })
    .unwrap();

    let public_inputs = &[zk_login_inputs.calculate_all_inputs_hash(&eph_pubkey, &modulus, max_epoch).unwrap()];

    let mut public_inputs_as_bytes = vec![];
    public_inputs.serialize_compressed(&mut public_inputs_as_bytes).unwrap();
    println!("HERE public_inputs_as_bytes : {:?}", public_inputs_as_bytes);
    println!("HERE public_inputs_as_bytes len : {:?}", public_inputs_as_bytes.len());
    println!("====== Start Poseidon ========");
    
    let index_mod_4 = 1;
    engine.cc.stack.push(StackItem::int(index_mod_4));
    engine.cc.stack.push(StackItem::int(max_epoch));
    engine.cc.stack.push(StackItem::integer(IntegerData::from_unsigned_bytes_be(&eph_pubkey.clone())));

    let modulus_cell = pack_data_to_cell(&modulus.clone(), &mut 0).unwrap();
    println!("modulus_cell = {:?}", modulus_cell);
    engine.cc.stack.push(StackItem::cell(modulus_cell.clone()));
    
    let iss_base_64_cell = pack_string_to_cell(&iss_base_64, &mut 0).unwrap();
    println!("iss_base_64_cell = {:?}", iss_base_64_cell);
    engine.cc.stack.push(StackItem::cell(iss_base_64_cell.clone()));

    let header_base_64_cell = pack_string_to_cell(&header_base_64, &mut 0).unwrap();
    println!("header_base_64_cell = {:?}", header_base_64_cell);
    engine.cc.stack.push(StackItem::cell(header_base_64_cell.clone()));

    let zk_seed_cell = pack_string_to_cell(&zk_seed.clone(), &mut 0).unwrap();
    println!("zk_seed_cell = {:?}", zk_seed_cell);
    engine.cc.stack.push(StackItem::cell(zk_seed_cell.clone()));

    let start: Instant = Instant::now();
    let status = execute_poseidon_zk_login(&mut engine).unwrap();
    let poseidon_elapsed = start.elapsed().as_micros();

    println!("poseidon_elapsed in microsecond: {:?}", poseidon_elapsed);  

    let poseidon_res = engine.cc.stack.get(0).as_cell().unwrap();
    let slice = SliceData::load_cell(poseidon_res.clone()).unwrap();
    let poseidon_res = unpack_data_from_cell(slice, &mut engine).unwrap();
    println!("poseidon_res from stack: {:?}", hex::encode(poseidon_res.clone()));

    println!("public_inputs hex (computed in test): {:?}", hex::encode(public_inputs_as_bytes.clone()));
    assert!(poseidon_res == public_inputs_as_bytes);

    println!("====== Start VERGRTH16 ========");
    let proof = &zk_login_inputs.get_proof().as_arkworks().unwrap();
    let mut proof_as_bytes = vec![];
    proof.serialize_compressed(&mut proof_as_bytes).unwrap();
    println!("proof_as_bytes : {:?}", proof_as_bytes);
    println!("proof_as_bytes len: {:?}", proof_as_bytes.len());

    let proof_cell = pack_data_to_cell(&proof_as_bytes, &mut 0).unwrap();
    engine.cc.stack.push(StackItem::cell(proof_cell.clone()));

    let public_inputs_cell = pack_data_to_cell(&public_inputs_as_bytes.clone(), &mut 0).unwrap();
    engine.cc.stack.push(StackItem::cell(public_inputs_cell.clone()));

    let verification_key_id: u32 = 0; // valid key id
    //let verification_key_id: u32 = 1; //invalid key id
    engine.cc.stack.push(StackItem::int(verification_key_id));

    let start: Instant = Instant::now();
    let status = execute_vergrth16(&mut engine).unwrap();
    let vergrth16_elapsed = start.elapsed().as_micros();

    println!("vergrth16_elapsed in microsecond: {:?}", vergrth16_elapsed); 

    let res = engine.cc.stack.get(0).as_integer().unwrap();
    println!("res: {:?}", res);
    assert!(*res == IntegerData::minus_one());

    
}




 #[derive(Debug, Deserialize)]
pub struct JwtDataDecodedPart2 {
        pub iss: String,
        pub aud: String,
        pub sub: String,
        pub nonce: String,
        pub iat: u32,
        pub exp: u32,
        pub jti: String,
        pub given_name: String,
        pub family_name: String,
        pub name: String,
        pub picture: String,
    }








pub const FACEBOOK_DATA: &str = "{\"jwt\":\"eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCIsImtpZCI6ImQ4N2QyNDc0ODk2ZjIxM2VlNTJlNjA2OWFlMGRkMTU1MzM0MGEwOGMifQ.eyJpc3MiOiJodHRwczpcL1wvd3d3LmZhY2Vib29rLmNvbSIsImF1ZCI6IjQ2NTcyNDU5NjEyMjAyOSIsInN1YiI6IjM5NDQwNDYyNjU4MzY4MzciLCJpYXQiOjE3NTI2MTUzODQsImV4cCI6MTc1MjYxODk4NCwianRpIjoiUERHcy5iY2JhNzI2ZWRlMGNhYzUzNTJjOWQ5ZmJlYTczZDEwNmViYmEzOWUzMDJmYjZkZTMzZWQ1OTlhMTE5MWU2M2U1Iiwibm9uY2UiOiJDNEY1UGdGVEg4amRLQ3dZUkxDWDJQc1BiZlUiLCJnaXZlbl9uYW1lIjoiXHUwNDEwXHUwNDNiXHUwNDM4XHUwNDNkXHUwNDMwIiwiZmFtaWx5X25hbWUiOiJcdTA0MTBcdTA0M2JcdTA0MzhcdTA0M2RcdTA0M2VcdTA0MzJcdTA0M2RcdTA0MzAiLCJuYW1lIjoiXHUwNDEwXHUwNDNiXHUwNDM4XHUwNDNkXHUwNDMwIFx1MDQxMFx1MDQzYlx1MDQzOFx1MDQzZFx1MDQzZVx1MDQzMlx1MDQzZFx1MDQzMCIsInBpY3R1cmUiOiJodHRwczpcL1wvcGxhdGZvcm0tbG9va2FzaWRlLmZic2J4LmNvbVwvcGxhdGZvcm1cL3Byb2ZpbGVwaWNcLz9hc2lkPTM5NDQwNDYyNjU4MzY4MzcmaGVpZ2h0PTEwMCZ3aWR0aD0xMDAmZXh0PTE3NTUyMDczODUmaGFzaD1BVDhjYXFRLVdleWlTWFBfR2NDcWRIYmsifQ.itdC4hXpVQaq9N_9blDiiKYwecGfPXEm3tXYSbwRQ2AiMrkoolziQ5qd4K0a3l2-hsnniD2qRdskWqZsCYjKP6KUE0UitWwHZYyypaScnxPZZHyCJmllCuFYKwTcYgNvK_QdXh_1lr6l2g6D5UXxyh5JFDuu5zcYQHmHHT3z4hIElTjRvAf3Abs2IQfycMwIVvcEdeJe1eSKABcwVv7qrphr9Bf6EazjpBLtlFwodpEyjRgHniuoBHPu6IM9VZuRCIaGrWLJSLeHXRIVqkP3Of2Tskj2EN9xibgPpZITdfuIRvthA2154Zbk2853Xl1SQg7SF1w4rEjPhl1mFRyYig\",\"user_pass_to_int_format\":\"535455565748\",\"zk_addr\":\"0x8fe042d5f16bc3e9e7fe2e282d56c42f018e65fea745c35a85d485c9d5e29969\",\"ephemeral_key_pair\":{\"keypair\":{\"public_key\":{\"0\":192,\"1\":32,\"2\":198,\"3\":100,\"4\":183,\"5\":233,\"6\":4,\"7\":41,\"8\":196,\"9\":119,\"10\":87,\"11\":131,\"12\":27,\"13\":78,\"14\":207,\"15\":175,\"16\":3,\"17\":17,\"18\":223,\"19\":203,\"20\":33,\"21\":0,\"22\":25,\"23\":92,\"24\":55,\"25\":90,\"26\":41,\"27\":110,\"28\":183,\"29\":209,\"30\":125,\"31\":203},\"secret_key\":{\"0\":138,\"1\":164,\"2\":19,\"3\":76,\"4\":65,\"5\":242,\"6\":197,\"7\":243,\"8\":232,\"9\":69,\"10\":52,\"11\":25,\"12\":92,\"13\":42,\"14\":78,\"15\":176,\"16\":149,\"17\":41,\"18\":23,\"19\":237,\"20\":203,\"21\":74,\"22\":29,\"23\":135,\"24\":103,\"25\":236,\"26\":105,\"27\":73,\"28\":213,\"29\":237,\"30\":233,\"31\":173,\"32\":192,\"33\":32,\"34\":198,\"35\":100,\"36\":183,\"37\":233,\"38\":4,\"39\":41,\"40\":196,\"41\":119,\"42\":87,\"43\":131,\"44\":27,\"45\":78,\"46\":207,\"47\":175,\"48\":3,\"49\":17,\"50\":223,\"51\":203,\"52\":33,\"53\":0,\"54\":25,\"55\":92,\"56\":55,\"57\":90,\"58\":41,\"59\":110,\"60\":183,\"61\":209,\"62\":125,\"63\":203}}},\"maxEpoch\":1837114018,\"extended_ephemeral_public_key\":\"AMAgxmS36QQpxHdXgxtOz68DEd/LIQAZXDdaKW630X3L\",\"zk_proofs\":{\"proofPoints\":{\"a\":[\"2462569216429571312007124289082630220401530723684912514166683366599699515539\",\"7883821542718525444091301469123358819780258365694589397889572410033752768278\",\"1\"],\"b\":[[\"2351917491525896894596397339414693508942272445003022943327429885171724593513\",\"15954576869770155190826948931450206948983828571722169741523592666702018090523\"],[\"2375679854915147062110128367485439044173775008177572852094708591857305802863\",\"3858921924155309032950196640309813006096991762314983759261692814930796775298\"],[\"1\",\"0\"]],\"c\":[\"20427193921456089297737019108797399317941127534506254906097503006873270177037\",\"5082645918023219178920606941558600931735655771354722424867057533628742920487\",\"1\"]},\"issBase64Details\":{\"value\":\"yJpc3MiOiJodHRwczpcL1wvd3d3LmZhY2Vib29rLmNvbSIs\",\"indexMod4\":1},\"headerBase64\":\"eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCIsImtpZCI6ImQ4N2QyNDc0ODk2ZjIxM2VlNTJlNjA2OWFlMGRkMTU1MzM0MGEwOGMifQ\"}}";
#[test]
fn test_poseidon_and_vergrth16_for_fb_data() {
    let elector_code = load_boc("benches/elector-code.boc");
    let elector_data = load_boc("benches/elector-data.boc");
    let config_data = load_boc("benches/config-data.boc");
    let mut ctrls = SaveList::default();
    ctrls.put(4, &mut StackItem::Cell(elector_data)).unwrap();
    let params = vec![
        StackItem::int(0x76ef1ea),
        StackItem::int(0),
        StackItem::int(0),
        StackItem::int(1633458077),
        StackItem::int(0),
        StackItem::int(0),
        StackItem::int(0),
        StackItem::tuple(vec![StackItem::int(1000000000), StackItem::None]),
        StackItem::slice(
            SliceData::from_string(
                "9fe0000000000000000000000000000000000000000000000000000000000000001_",
            )
            .unwrap(),
        ),
        StackItem::cell(config_data.reference(0).unwrap()),
        StackItem::None,
        StackItem::int(0),
    ];
    ctrls.put(7, &mut StackItem::tuple(vec![StackItem::tuple(params)])).unwrap();
    let stack = Stack::new();
    let mut engine = Engine::with_capabilities(DEFAULT_CAPABILITIES).setup_with_libraries(
        SliceData::load_cell_ref(&elector_code).unwrap(),
        Some(ctrls.clone()),
        Some(stack.clone()),
        None,
        vec![],
    );

    //////



    let content: JWK = JWK {
            kty: "RSA".to_string(),
            e: "AQAB".to_string(),
            n: "zg_Y5w8jd7O7P7nH7jPNIl4trbDDzwlpapY6OpOlPy0g9u2siZ5VH3fc33fwOg-2SMYtoOf_acb2Ic5SgwtrvCB5YDi9X_DZuM_4zT9RIA64KneIZU0Ld8IVND9qrBUq1Z9JOFUe8pVuFrtZjQye5fPr-VHX6EYbUPSoFeNClB6hDTCS1DeKP0itJpC7t7e2TeFt21laraTlsXgtzjwNNvtwI3J6smQUJud6N6QvyLnY01ys0Y-DdvfckBKXNK9d8rP1aP5hRuXkoUW2eNrHm61mI2mmyyeZhyTaPP7GdfvvuFW0uBEqe33HTf2gW4yaUdfY7kQVE4cn2V-Fl9GEUQ".to_string(),
            alg: "RS256".to_string(),
        };

        let mut all_jwk = HashMap::new();
        all_jwk.insert(
            JwkId::new(
                OIDCProvider::Facebook.get_config().iss,
                "d87d2474896f213ee52e6069ae0dd1553340a08c".to_string(),
            ),
            content,
        );
    let header_base_64 = "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCIsImtpZCI6ImQ4N2QyNDc0ODk2ZjIxM2VlNTJlNjA2OWFlMGRkMTU1MzM0MGEwOGMifQ";//"eyJhbGciOiJSUzI1NiIsImtpZCI6IjMyM2IyMTRhZTY5NzVhMGYwMzRlYTc3MzU0ZGMwYzI1ZDAzNjQyZGMiLCJ0eXAiOiJKV1QifQ";
    let iss_base_64 = "yJpc3MiOiJodHRwczpcL1wvd3d3LmZhY2Vib29rLmNvbSIs";//"yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC";

    

    let data = [
            FACEBOOK_DATA
            
    ];

    let mut average_poseidon: u128 = 0;
    let mut average_vergrth16: u128 = 0;

    for i in 0..data.len() {
        println!("");
        println!("====================== Iter@ is {i} =========================");
        let jwt_data: JwtData = serde_json::from_str(&data[i]).unwrap();
        println!("jwt_data: {:?}", jwt_data);

        let user_pass_salt = jwt_data.user_pass_to_int_format.as_str();
        println!("user_pass_salt is {user_pass_salt}");

        println!("{:?}", jwt_data.ephemeral_key_pair.keypair.public_key);
        let eph_secret_key = secret_key_from_integer_map(jwt_data.ephemeral_key_pair.keypair.secret_key);

        let ephemeral_kp = Ed25519KeyPair::from_bytes(&eph_secret_key).unwrap();
        let mut eph_pubkey: Vec<u8> = Vec::new(); // vec![0x00];
        eph_pubkey.extend(ephemeral_kp.public().as_ref());

        println!("ephemeral secret_key is {:?}", hex::encode(eph_secret_key.clone()));
        println!("ephemeral public_key is {:?}", hex::encode(eph_pubkey.clone()));

        let eph_pubkey_len = eph_pubkey.clone().len();
        println!("len eph_pubkey: {:?}", eph_pubkey_len);

        let jwt_data_vector: Vec<&str> = jwt_data.jwt.split(".").collect();
        let jwt_data_1 = decode(jwt_data_vector[0]).expect("Base64 decoding failed");

        let jwt_string_1 = String::from_utf8(jwt_data_1).expect("UTF-8 conversion failed");
        println!("jwt_string_1 is {:?}", jwt_string_1); // jwt_string_1 is
        // "{\"alg\":\"RS256\",\"kid\":\"323b214ae6975a0f034ea77354dc0c25d03642dc\",\"
        // typ\":\"JWT\"}"
        // JwtDataDecodedPart1
        let jwt_data_decoded1: JwtDataDecodedPart1 =
            serde_json::from_str(&jwt_string_1).unwrap();
        println!("kid: {:?}", jwt_data_decoded1.kid);

        let jwt_data_2 = decode(jwt_data_vector[1]).expect("Base64 decoding failed");
        let jwt_string_2 = String::from_utf8(jwt_data_2).expect("UTF-8 conversion failed");
        println!("jwt_string_2 is {:?}", jwt_string_2); // "{\"iss\":\"https://accounts.google.com\",\"azp\":\"232624085191-v1tq20fg1kdhhgvat6saj7jf0hd8233r.apps.googleusercontent.com\",\"aud\":\"232624085191-v1tq20fg1kdhhgvat6saj7jf0hd8233r.apps.googleusercontent.com\",\"sub\":\"112897468626716626103\",\"nonce\":\"bxmnJW31ruzKMGir01YPGYL0xDY\",\"nbf\":1715687036,\"iat\":1715687336,\"exp\":1715690936,\"jti\":\"9b601d25f003640c2889a2a047789382cb1cfe87\"}"

        // JwtDataDecodedPart2
        let jwt_data_decoded2: JwtDataDecodedPart2 =
            serde_json::from_str(&jwt_string_2).unwrap();
        println!("aud: {:?}", jwt_data_decoded2.aud);
        println!("sub: {:?}", jwt_data_decoded2.sub);

        let zk_seed = gen_address_seed(
            user_pass_salt,
            "sub",
            jwt_data_decoded2.sub.as_str(), 
            jwt_data_decoded2.aud.as_str(),
            )
        .unwrap();

        println!("zk_seed = {:?}", zk_seed);

        println!("jwt_data.zk_proofs = {:?}", jwt_data.zk_proofs);
        let proof_and_jwt = serde_json::to_string(&jwt_data.zk_proofs).unwrap();

        let zk_login_inputs =
            ZkLoginInputs::from_json(&*proof_and_jwt.to_string(), &*zk_seed.to_string())
        .unwrap();

        // let verification_key_id: u32 = 2;
        let verification_key_id: u32 = 0;

        let (iss, kid) =
            (zk_login_inputs.get_iss().to_string(), zk_login_inputs.get_kid().to_string());
        let jwk = all_jwk.get(&JwkId::new(iss.clone(), kid.clone())).ok_or_else(|| {
        ZkCryptoError::GeneralError(format!("JWK not found ({} - {})", iss, kid))
        }).unwrap();

        let max_epoch = 1837114018; //142; 

        let modulus = base64ct::Base64UrlUnpadded::decode_vec(&jwk.n)
        .map_err(|_| {
            ZkCryptoError::GeneralError("Invalid Base64 encoded jwk modulus".to_string())
        })
        .unwrap();

        let public_inputs = &[zk_login_inputs.calculate_all_inputs_hash(&eph_pubkey, &modulus, max_epoch).unwrap()];

        println!("modulus_ hex = {:?}", hex::encode(modulus.clone()));

        let mut public_inputs_as_bytes = vec![];
        public_inputs.serialize_compressed(&mut public_inputs_as_bytes).unwrap();
        println!("HERE public_inputs_as_bytes : {:?}", public_inputs_as_bytes);
        println!("HERE public_inputs_as_bytes len : {:?}", public_inputs_as_bytes.len());
        println!("====== Start Poseidon ========");
    
        let index_mod_4 = 1;
        engine.cc.stack.push(StackItem::int(index_mod_4));
        engine.cc.stack.push(StackItem::int(max_epoch));
        engine.cc.stack.push(StackItem::integer(IntegerData::from_unsigned_bytes_be(&eph_pubkey.clone())));

        let modulus_cell = pack_data_to_cell(&modulus.clone(), &mut 0).unwrap();
        println!("modulus_cell = {:?}", modulus_cell);
        engine.cc.stack.push(StackItem::cell(modulus_cell.clone()));
    
        let iss_base_64_cell = pack_string_to_cell(&iss_base_64, &mut 0).unwrap();
        println!("iss_base_64_cell = {:?}", iss_base_64_cell);
        engine.cc.stack.push(StackItem::cell(iss_base_64_cell.clone()));

        let header_base_64_cell = pack_string_to_cell(&header_base_64, &mut 0).unwrap();
        println!("header_base_64_cell = {:?}", header_base_64_cell);
        engine.cc.stack.push(StackItem::cell(header_base_64_cell.clone()));

        let zk_seed_cell = pack_string_to_cell(&zk_seed.clone(), &mut 0).unwrap();
        println!("zk_seed_cell = {:?}", zk_seed_cell);
        engine.cc.stack.push(StackItem::cell(zk_seed_cell.clone()));

        let start: Instant = Instant::now();
        let status = execute_poseidon_zk_login(&mut engine).unwrap();
        let poseidon_elapsed = start.elapsed().as_micros();

        

        let poseidon_res = engine.cc.stack.get(0).as_cell().unwrap();
        let slice = SliceData::load_cell(poseidon_res.clone()).unwrap();
        let poseidon_res = unpack_data_from_cell(slice, &mut engine).unwrap();
        println!("poseidon_res from stack: {:?}", hex::encode(poseidon_res.clone()));

        println!("public_inputs hex (computed in test): {:?}", hex::encode(public_inputs_as_bytes.clone()));
        assert!(poseidon_res == public_inputs_as_bytes);

        println!("poseidon_elapsed in microsecond: {:?}", poseidon_elapsed);  

        average_poseidon = average_poseidon + poseidon_elapsed;


        println!("====== Start VERGRTH16 ========");
        let proof = &zk_login_inputs.get_proof().as_arkworks().unwrap();
        let mut proof_as_bytes = vec![];
        proof.serialize_compressed(&mut proof_as_bytes).unwrap();
        println!("proof_as_bytes : {:?}", proof_as_bytes);
        println!("proof_as_bytes : {:?}", hex::encode(proof_as_bytes.clone()));
        println!("proof_as_bytes len: {:?}", proof_as_bytes.len());

        let proof_cell = pack_data_to_cell(&proof_as_bytes, &mut 0).unwrap();
        engine.cc.stack.push(StackItem::cell(proof_cell.clone()));

        let public_inputs_cell = pack_data_to_cell(&public_inputs_as_bytes.clone(), &mut 0).unwrap();
        engine.cc.stack.push(StackItem::cell(public_inputs_cell.clone()));

        let verification_key_id: u32 = 0; // valid key id
        //let verification_key_id: u32 = 1; //invalid key id
        engine.cc.stack.push(StackItem::int(verification_key_id));

        let start: Instant = Instant::now();
        let status = execute_vergrth16(&mut engine).unwrap();
        let vergrth16_elapsed = start.elapsed().as_micros();

        println!("vergrth16_elapsed in microsecond: {:?}", vergrth16_elapsed); 

        let res = engine.cc.stack.get(0).as_integer().unwrap();
        println!("res: {:?}", res);
        assert!(*res == IntegerData::minus_one());

        average_vergrth16 = average_vergrth16 + vergrth16_elapsed;
    }

    println!("===================================");
    println!("===================================");
    println!("===================================");
    println!("===================================");

    let average_poseidon_=  average_poseidon / (data.len() as u128);
    println!("average_poseidon_ in microsecond: {:?}", average_poseidon_);  
    let average_vergrth16_=  average_vergrth16 / (data.len() as u128);
    println!("average_vergrth16_ in microsecond: {:?}", average_vergrth16_); 

}


#[test]
fn test_poseidon_1() {
    let mut stack = Stack::new();

 
    // password was 567890 in ascii 535455565748
    let user_pass_salt = "535455565748";
    let secret_key = [222, 248, 61, 101, 214, 199, 113, 189, 223, 94, 151, 140, 235, 182, 203, 46, 143, 162, 166, 87, 162, 250, 176, 4, 29, 19, 42, 221, 116, 33, 178, 14,
    ];
    let ephemeral_kp = Ed25519KeyPair::from_bytes(&secret_key).unwrap(); // 
    let mut eph_pubkey = Vec::new();
    eph_pubkey.extend(ephemeral_kp.public().as_ref());
    println!("eph_pubkey: {:?}", eph_pubkey);
    println!("len eph_pubkey: {:?}", eph_pubkey.len());
    let eph_pubkey_hex_number = "0x".to_owned() + &hex::encode(eph_pubkey.clone());
    println!("eph_pubkey_hex_number: {:?}", eph_pubkey_hex_number);
    let zk_seed = gen_address_seed(
        user_pass_salt,
        "sub",
        "112897468626716626103",
        "232624085191-v1tq20fg1kdhhgvat6saj7jf0hd8233r.apps.googleusercontent.com",
    )
    .unwrap();
    println!("zk_seed = {:?}", zk_seed);
    let proof_and_jwt = "{\"proofPoints\":{\"a\":[\"2352077003566407045854435506409565889408960755152253285189640818725808263237\",\
    \"9548308350778027075240385782578683112366097953461273569343148999989145049123\",\"1\"],\
    \"b\":[[\"2172697685172701179756462481453772004245591587568555358926512547679273443868\",\
    \"11300889616992175665271080883374830731684409375838395487979439153562369168807\"],\
    [\"18769153619672444537277685186545610305405730219274884099876386487766026068190\",\
    \"12892936063156115176399929981646174277274895601746717550262309650970826515227\"],[\"1\",\"0\"]],\
    \"c\":[\"21276833037675249246843718004583052134371270695679878402069223253610209272159\",\
    \"8637596258221986824049981569842218428861929142818091935707054543971817804456\",\"1\"]},\
    \"issBase64Details\":{\"value\":\"yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC\",\"indexMod4\":1},\
    \"headerBase64\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6ImEzYjc2MmY4NzFjZGIzYmFlMDA0NGM2NDk2MjJmYzEzOTZlZGEzZTMiLCJ0eXAiOiJKV1QifQ\"}";
    let len = proof_and_jwt.bytes().len();
    println!(" proof_and_jwt_bytes len (in bytes) = {:?}", len);

    println!("proof_and_jwt: {}", proof_and_jwt);

    let iss_and_header_base64details = "{\"issBase64Details\":{\"value\":\"yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC\",\"indexMod4\":1},\"headerBase64\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6ImEzYjc2MmY4NzFjZGIzYmFlMDA0NGM2NDk2MjJmYzEzOTZlZGEzZTMiLCJ0eXAiOiJKV1QifQ\"}";

    println!("iss_and_header_base64details: {}", iss_and_header_base64details);

    let header_base_64 = "eyJhbGciOiJSUzI1NiIsImtpZCI6ImEzYjc2MmY4NzFjZGIzYmFlMDA0NGM2NDk2MjJmYzEzOTZlZGEzZTMiLCJ0eXAiOiJKV1QifQ";
    let iss_base_64 = "yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC";
    

    let zk_login_inputs = ZkLoginInputs::from_json(&*proof_and_jwt, &*zk_seed.to_string()).unwrap();
    let content: JWK = JWK {
        kty: "RSA".to_string(),
        e: "AQAB".to_string(),
        n: "uBHF-esPKiNlFaAvpdpejD4vpONW9FL0rgLDg1z8Q-x_CiHCvJCpiSehD41zmDOhzXP_fbMMSGpGL7R3duiz01nK5r_YmRw3RXeB0kcS7Z9H8MN6IJcde9MWbqkMabCDduFgdr6gvH0QbTipLB1qJK_oI_IBfRgjk6G0bGrKz3PniQw5TZ92r0u1LM-1XdBIb3aTYTGDW9KlOsrTTuKq0nj-anW5TXhecuxqSveFM4Hwlw7pw34ydBunFjFWDx4VVJqGNSqWCfcERxOulizIFruZIHJGkgunZnB4DF7mCZOttx2dwT9j7s3GfLJf0xoGumqpOMvecuipfTPeIdAzcQ".to_string(), // Alina's data
        alg: "RS256".to_string(),
    };

    let mut all_jwk = HashMap::new();
     all_jwk.insert(
        JwkId::new(
            OIDCProvider::Google.get_config().iss,
            "a3b762f871cdb3bae0044c649622fc1396eda3e3".to_string(), 
        ),
        content,
    );

    let (iss, kid) =
            (zk_login_inputs.get_iss().to_string(), zk_login_inputs.get_kid().to_string());
    let jwk = all_jwk.get(&JwkId::new(iss.clone(), kid.clone())).ok_or_else(|| {
        ZkCryptoError::GeneralError(format!("JWK not found ({} - {})", iss, kid))
    }).unwrap();

    let max_epoch = 142; 

    let modulus = base64ct::Base64UrlUnpadded::decode_vec(&jwk.n)
        .map_err(|_| {
            ZkCryptoError::GeneralError("Invalid Base64 encoded jwk modulus".to_string())
        })
    .unwrap();

    let public_inputs = &[zk_login_inputs.calculate_all_inputs_hash(&eph_pubkey, &modulus, max_epoch).unwrap()];

    let mut public_inputs_as_bytes = vec![];
    public_inputs.serialize_compressed(&mut public_inputs_as_bytes).unwrap();
    println!("HERE public_inputs_as_bytes : {:?}", public_inputs_as_bytes);
    println!("HERE public_inputs_as_bytes len : {:?}", public_inputs_as_bytes.len());
    println!("====== Start Poseidon ========");
    
    let index_mod_4 = 1;
    stack.push(StackItem::int(index_mod_4));
    stack.push(StackItem::int(max_epoch));
    stack.push(StackItem::integer(IntegerData::from_unsigned_bytes_be(&eph_pubkey.clone())));

    let modulus_cell = pack_data_to_cell(&modulus.clone(), &mut 0).unwrap();
    println!("modulus_cell = {:?}", modulus_cell);
    stack.push(StackItem::cell(modulus_cell.clone()));
    
    let iss_base_64_cell = pack_string_to_cell(&iss_base_64, &mut 0).unwrap();
    println!("iss_base_64_cell = {:?}", iss_base_64_cell);
    stack.push(StackItem::cell(iss_base_64_cell.clone()));

    let header_base_64_cell = pack_string_to_cell(&header_base_64, &mut 0).unwrap();
    println!("header_base_64_cell = {:?}", header_base_64_cell);
    stack.push(StackItem::cell(header_base_64_cell.clone()));

    let zk_seed_cell = pack_string_to_cell(&zk_seed.clone(), &mut 0).unwrap();
    println!("zk_seed_cell = {:?}", zk_seed_cell);
    stack.push(StackItem::cell(zk_seed_cell.clone()));

    let start: Instant = Instant::now();

    let mut res = Vec::<u8>::with_capacity(2);
    res.push(0xC7);
    res.push(0x32);
    res.push(0x80);
    

    let code = SliceData::new(res);

    let mut engine = Engine::with_capabilities(0).setup_with_libraries(code, None, Some(stack), None, vec![]);
    engine.execute();
    let poseidon_elapsed = start.elapsed().as_micros();

    println!("poseidon_elapsed in microsecond: {:?}", poseidon_elapsed);  

    let poseidon_res = engine.cc.stack.get(0).as_cell().unwrap();
    let slice = SliceData::load_cell(poseidon_res.clone()).unwrap();
    let poseidon_res = unpack_data_from_cell(slice, &mut engine).unwrap();
    println!("poseidon_res from stack: {:?}", poseidon_res.clone());

    println!("public_inputs hex (computed in test): {:?}",public_inputs_as_bytes.clone());
    assert!(poseidon_res == public_inputs_as_bytes);
  
}

#[test]
fn test_vergrth16_1() {
    
    let mut stack = Stack::new();
    
    // password was 567890 in ascii 535455565748
    let user_pass_salt = "535455565748";
    let secret_key = [222, 248, 61, 101, 214, 199, 113, 189, 223, 94, 151, 140, 235, 182, 203, 46, 143, 162, 166, 87, 162, 250, 176, 4, 29, 19, 42, 221, 116, 33, 178, 14,
    ];
    let ephemeral_kp = Ed25519KeyPair::from_bytes(&secret_key).unwrap(); // 
    let mut eph_pubkey = Vec::new();
    eph_pubkey.extend(ephemeral_kp.public().as_ref());
    println!("eph_pubkey: {:?}", eph_pubkey);
    println!("len eph_pubkey: {:?}", eph_pubkey.len());
    let eph_pubkey_hex_number = "0x".to_owned() + &hex::encode(eph_pubkey.clone());
    println!("eph_pubkey_hex_number: {:?}", eph_pubkey_hex_number);
    let zk_seed = gen_address_seed(
        user_pass_salt,
        "sub",
        "112897468626716626103",
        "232624085191-v1tq20fg1kdhhgvat6saj7jf0hd8233r.apps.googleusercontent.com",
    )
    .unwrap();
    println!("zk_seed = {:?}", zk_seed);
    let proof_and_jwt = "{\"proofPoints\":{\"a\":[\"2352077003566407045854435506409565889408960755152253285189640818725808263237\",\
    \"9548308350778027075240385782578683112366097953461273569343148999989145049123\",\"1\"],\
    \"b\":[[\"2172697685172701179756462481453772004245591587568555358926512547679273443868\",\
    \"11300889616992175665271080883374830731684409375838395487979439153562369168807\"],\
    [\"18769153619672444537277685186545610305405730219274884099876386487766026068190\",\
    \"12892936063156115176399929981646174277274895601746717550262309650970826515227\"],[\"1\",\"0\"]],\
    \"c\":[\"21276833037675249246843718004583052134371270695679878402069223253610209272159\",\
    \"8637596258221986824049981569842218428861929142818091935707054543971817804456\",\"1\"]},\
    \"issBase64Details\":{\"value\":\"yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC\",\"indexMod4\":1},\
    \"headerBase64\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6ImEzYjc2MmY4NzFjZGIzYmFlMDA0NGM2NDk2MjJmYzEzOTZlZGEzZTMiLCJ0eXAiOiJKV1QifQ\"}";
    let len = proof_and_jwt.bytes().len();
    println!(" proof_and_jwt_bytes len (in bytes) = {:?}", len);

    println!("proof_and_jwt: {}", proof_and_jwt);

    let iss_and_header_base64details = "{\"issBase64Details\":{\"value\":\"yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC\",\"indexMod4\":1},\"headerBase64\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6ImEzYjc2MmY4NzFjZGIzYmFlMDA0NGM2NDk2MjJmYzEzOTZlZGEzZTMiLCJ0eXAiOiJKV1QifQ\"}";

    println!("iss_and_header_base64details: {}", iss_and_header_base64details);

    let header_base_64 = "eyJhbGciOiJSUzI1NiIsImtpZCI6ImEzYjc2MmY4NzFjZGIzYmFlMDA0NGM2NDk2MjJmYzEzOTZlZGEzZTMiLCJ0eXAiOiJKV1QifQ";
    let iss_base_64 = "yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC";
    

    let zk_login_inputs = ZkLoginInputs::from_json(&*proof_and_jwt, &*zk_seed.to_string()).unwrap();
    let content: JWK = JWK {
        kty: "RSA".to_string(),
        e: "AQAB".to_string(),
        n: "uBHF-esPKiNlFaAvpdpejD4vpONW9FL0rgLDg1z8Q-x_CiHCvJCpiSehD41zmDOhzXP_fbMMSGpGL7R3duiz01nK5r_YmRw3RXeB0kcS7Z9H8MN6IJcde9MWbqkMabCDduFgdr6gvH0QbTipLB1qJK_oI_IBfRgjk6G0bGrKz3PniQw5TZ92r0u1LM-1XdBIb3aTYTGDW9KlOsrTTuKq0nj-anW5TXhecuxqSveFM4Hwlw7pw34ydBunFjFWDx4VVJqGNSqWCfcERxOulizIFruZIHJGkgunZnB4DF7mCZOttx2dwT9j7s3GfLJf0xoGumqpOMvecuipfTPeIdAzcQ".to_string(), // Alina's data
        alg: "RS256".to_string(),
    };

    let mut all_jwk = HashMap::new();
     all_jwk.insert(
        JwkId::new(
            OIDCProvider::Google.get_config().iss,
            "a3b762f871cdb3bae0044c649622fc1396eda3e3".to_string(), 
        ),
        content,
    );

    let (iss, kid) =
            (zk_login_inputs.get_iss().to_string(), zk_login_inputs.get_kid().to_string());
    let jwk = all_jwk.get(&JwkId::new(iss.clone(), kid.clone())).ok_or_else(|| {
        ZkCryptoError::GeneralError(format!("JWK not found ({} - {})", iss, kid))
    }).unwrap();

    let max_epoch = 142; 

    let modulus = base64ct::Base64UrlUnpadded::decode_vec(&jwk.n)
        .map_err(|_| {
            ZkCryptoError::GeneralError("Invalid Base64 encoded jwk modulus".to_string())
        })
    .unwrap();

    let public_inputs = &[zk_login_inputs.calculate_all_inputs_hash(&eph_pubkey, &modulus, max_epoch).unwrap()];

    let mut public_inputs_as_bytes = vec![];
    public_inputs.serialize_compressed(&mut public_inputs_as_bytes).unwrap();
    println!("HERE public_inputs_as_bytes : {:?}", public_inputs_as_bytes);
    println!("HERE public_inputs_as_bytes len : {:?}", public_inputs_as_bytes.len());
    
    println!("====== Start VERGRTH16 ========");
    let proof = &zk_login_inputs.get_proof().as_arkworks().unwrap();
    let mut proof_as_bytes = vec![];
    proof.serialize_compressed(&mut proof_as_bytes).unwrap();
    println!("proof_as_bytes : {:?}", proof_as_bytes);
    println!("proof_as_bytes len: {:?}", proof_as_bytes.len());

    let proof_cell = pack_data_to_cell(&proof_as_bytes, &mut 0).unwrap();
    stack.push(StackItem::cell(proof_cell.clone()));

    let public_inputs_cell = pack_data_to_cell(&public_inputs_as_bytes.clone(), &mut 0).unwrap();
    stack.push(StackItem::cell(public_inputs_cell.clone()));

    let verification_key_id: u32 = 0; // valid key id
    //let verification_key_id: u32 = 1; //invalid key id
    stack.push(StackItem::int(verification_key_id));

    let start: Instant = Instant::now();

    let mut res = Vec::<u8>::with_capacity(2);
    res.push(0xC7);
    res.push(0x31);
    res.push(0x80);
    

    let code = SliceData::new(res);

    let mut engine = Engine::with_capabilities(0).setup_with_libraries(code, None, Some(stack), None, vec![]);
    engine.execute();
    //let status = execute_vergrth16(&mut engine).unwrap();
    let vergrth16_elapsed = start.elapsed().as_micros();

    println!("vergrth16_elapsed in microsecond: {:?}", vergrth16_elapsed); 

    let res = engine.cc.stack.get(0).as_integer().unwrap();
    println!("res: {:?}", res);
    assert!(*res == IntegerData::minus_one());

    
}

