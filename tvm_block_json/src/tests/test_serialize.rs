// Copyright (C) 2019-2021 TON Labs. All Rights Reserved.
//
// Licensed under the SOFTWARE EVALUATION License (the "License"); you may not
// use this file except in compliance with the License.  You may obtain a copy
// of the License at:
//
// https://www.ton.dev/licenses
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific TON DEV software governing permissions and
// limitations under the License.

use std::fs::read;
use std::path::Path;

use pretty_assertions::assert_eq;
use tvm_api::ton::ton_node::rempmessagestatus;
use tvm_api::IntoBoxed;
use tvm_block::generate_test_account_by_init_code_hash;
use tvm_block::ShardStateUnsplit;
use tvm_block::Transaction;
use tvm_block::TransactionProcessingStatus;
use tvm_types::base64_decode;
use tvm_types::read_single_root_boc;
use tvm_types::write_boc;
use tvm_types::AccountId;
use tvm_types::IBitstring;

use super::*;

include!("./test_common.rs");

fn assert_json_eq_file(json: &str, name: &str) {
    let expected =
        std::fs::read_to_string(format!("src/tests/data/{}-ethalon.json", name)).unwrap();
    assert_json_eq(json, &expected, name);
}

#[test]
fn test_account_into_json_without_hash_0() {
    let account = generate_test_account_by_init_code_hash(false);
    let boc = account.write_to_bytes().unwrap();
    let sender =
        AccountSerializationSet { account, prev_code_hash: None, boc, boc1: None, proof: None };
    let json = db_serialize_account("id", &sender).unwrap();
    println!("\n\n{:#}", serde_json::json!(json));
    assert_eq!(
        format!("{:#}", serde_json::json!(json)),
        r#"{
  "json_version": 8,
  "id": "983217:0:000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f",
  "workchain_id": 0,
  "boc": "te6ccgEBFgEA9gAEfdxMGQuAAACBAYICgwOEBIUFhgaHB4gIiQmKCosLjAyNDY4Ojw+RZAe+AdbzRWLeAAAAAAAAAACi6Q7dAB73wAkEAwEBQ6APQXKvQr0nmUedLZlpXZ5OtG4xRMeRXZRVYp/Nw8xC5ZgCAA8/////////9AAPP/8f//////QBDz/////////0BQEPP/////////QGAQ8/P///////9AcBDw/////////0CAAPP/8///////QCA87AEQoCASAOCwIBIA0MAAcHMS3JAAUEBLECASAQDwAFBAPpAAUEAyECASAVEgIBIBQTAAUEAlkAAwORAAVQskA=",
  "last_paid": 123456789,
  "bits_dec": "991",
  "bits": "23df",
  "cells_dec": "22",
  "cells": "116",
  "public_cells_dec": "0",
  "public_cells": "00",
  "due_payment_dec": "111",
  "due_payment": "016f",
  "last_trans_lt_dec": "0",
  "last_trans_lt": "00",
  "balance_dec": "100000000000",
  "balance": "09174876e800",
  "balance_other": [
    {
      "currency": 1,
      "value_dec": "100",
      "value": "0164"
    },
    {
      "currency": 2,
      "value_dec": "200",
      "value": "01c8"
    },
    {
      "currency": 3,
      "value_dec": "300",
      "value": "0212c"
    },
    {
      "currency": 4,
      "value_dec": "400",
      "value": "02190"
    },
    {
      "currency": 5,
      "value_dec": "500",
      "value": "021f4"
    },
    {
      "currency": 6,
      "value_dec": "600",
      "value": "02258"
    },
    {
      "currency": 7,
      "value_dec": "10000100",
      "value": "059896e4"
    }
  ],
  "split_depth": 23,
  "tick": false,
  "tock": true,
  "code": "te6ccgEBBQEANgABDz/////////0AQEPP/////////QCAQ8/P///////9AMBDw/////////0BAAPP/8///////Q=",
  "code_hash": "3c28164f21b76a53cfe73510197b99c735d4d97b652e6950f317bcbfe955848a",
  "data": "te6ccgEBAQEACgAADz//H//////0",
  "data_hash": "47cc6bba530c25a982969baf59254598715aecb5b9d14531d96d24d8a623dd93",
  "library": "te6ccgEBAgEALwABQ6APQXKvQr0nmUedLZlpXZ5OtG4xRMeRXZRVYp/Nw8xC5ZgBAA8/////////9A==",
  "library_hash": "4359e3721d98903035218ff07d3df30d0ce59d224abd2d7b0bfe65423fb0f67f",
  "acc_type": 1
}"#
    );
}

#[test]
fn test_account_into_json_with_hash_0() {
    let account = generate_test_account_by_init_code_hash(true);
    let boc = account.write_to_bytes().unwrap();
    let mut builder = BuilderData::new();
    account.write_original_format(&mut builder).unwrap();
    let boc1 = Some(write_boc(&builder.into_cell().unwrap()).unwrap());
    let sender = AccountSerializationSet { account, prev_code_hash: None, boc, boc1, proof: None };
    let json = db_serialize_account("id", &sender).unwrap();
    println!("\n\n{:#}", serde_json::json!(json));
    assert_eq!(
        format!("{:#}", serde_json::json!(json)),
        r#"{
  "json_version": 8,
  "id": "983217:0:000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f",
  "workchain_id": 0,
  "boc": "te6ccgECFgEAARYABL0biYMhcAAAECAwQFBgcICQoLDA0ODxAREhMUFRYXGBkaGxwdHh8iyBOAA63misW8AAAAAAAAAAFF0h26AD3vnhQLJ5DbtSnn85qIDL3M45rqbL2ylzSoeYveX/SqwkVAkEAwEBQ6APQXKvQr0nmUedLZlpXZ5OtG4xRMeRXZRVYp/Nw8xC5ZgCAA8/////////9AAPP/8f//////QBDz/////////0BQEPP/////////QGAQ8/P///////9AcBDw/////////0CAAPP/8///////QCA87AEQoCASAOCwIBIA0MAAcHMS3JAAUEBLECASAQDwAFBAPpAAUEAyECASAVEgIBIBQTAAUEAlkAAwORAAVQskA=",
  "boc1": "te6ccgEBFgEA9gAEfdxMGQuAAACBAYICgwOEBIUFhgaHB4gIiQmKCosLjAyNDY4Ojw+RZAnAAdbzRWLeAAAAAAAAAACi6Q7dAB73wAkEAwEBQ6APQXKvQr0nmUedLZlpXZ5OtG4xRMeRXZRVYp/Nw8xC5ZgCAA8/////////9AAPP/8f//////QBDz/////////0BQEPP/////////QGAQ8/P///////9AcBDw/////////0CAAPP/8///////QCA87AEQoCASAOCwIBIA0MAAcHMS3JAAUEBLECASAQDwAFBAPpAAUEAyECASAVEgIBIBQTAAUEAlkAAwORAAVQskA=",
  "init_code_hash": "3c28164f21b76a53cfe73510197b99c735d4d97b652e6950f317bcbfe955848a",
  "last_paid": 123456789,
  "bits_dec": "1248",
  "bits": "24e0",
  "cells_dec": "22",
  "cells": "116",
  "public_cells_dec": "0",
  "public_cells": "00",
  "due_payment_dec": "111",
  "due_payment": "016f",
  "last_trans_lt_dec": "0",
  "last_trans_lt": "00",
  "balance_dec": "100000000000",
  "balance": "09174876e800",
  "balance_other": [
    {
      "currency": 1,
      "value_dec": "100",
      "value": "0164"
    },
    {
      "currency": 2,
      "value_dec": "200",
      "value": "01c8"
    },
    {
      "currency": 3,
      "value_dec": "300",
      "value": "0212c"
    },
    {
      "currency": 4,
      "value_dec": "400",
      "value": "02190"
    },
    {
      "currency": 5,
      "value_dec": "500",
      "value": "021f4"
    },
    {
      "currency": 6,
      "value_dec": "600",
      "value": "02258"
    },
    {
      "currency": 7,
      "value_dec": "10000100",
      "value": "059896e4"
    }
  ],
  "split_depth": 23,
  "tick": false,
  "tock": true,
  "code": "te6ccgEBBQEANgABDz/////////0AQEPP/////////QCAQ8/P///////9AMBDw/////////0BAAPP/8///////Q=",
  "code_hash": "3c28164f21b76a53cfe73510197b99c735d4d97b652e6950f317bcbfe955848a",
  "data": "te6ccgEBAQEACgAADz//H//////0",
  "data_hash": "47cc6bba530c25a982969baf59254598715aecb5b9d14531d96d24d8a623dd93",
  "library": "te6ccgEBAgEALwABQ6APQXKvQr0nmUedLZlpXZ5OtG4xRMeRXZRVYp/Nw8xC5ZgBAA8/////////9A==",
  "library_hash": "4359e3721d98903035218ff07d3df30d0ce59d224abd2d7b0bfe65423fb0f67f",
  "acc_type": 1
}"#
    );
}

#[test]
fn test_account_into_json_q() {
    let account = generate_test_account_by_init_code_hash(false);
    let boc = account.write_to_bytes().unwrap();
    let sender =
        AccountSerializationSet { account, prev_code_hash: None, boc, boc1: None, proof: None };
    let json = db_serialize_account_ex("id", &sender, SerializationMode::QServer).unwrap();
    println!("\n\n{:#}", serde_json::json!(json));
    assert_eq!(
        format!("{:#}", serde_json::json!(json)),
        r#"{
  "json_version": 8,
  "id": "983217:0:000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f",
  "workchain_id": 0,
  "boc": "te6ccgEBFgEA9gAEfdxMGQuAAACBAYICgwOEBIUFhgaHB4gIiQmKCosLjAyNDY4Ojw+RZAe+AdbzRWLeAAAAAAAAAACi6Q7dAB73wAkEAwEBQ6APQXKvQr0nmUedLZlpXZ5OtG4xRMeRXZRVYp/Nw8xC5ZgCAA8/////////9AAPP/8f//////QBDz/////////0BQEPP/////////QGAQ8/P///////9AcBDw/////////0CAAPP/8///////QCA87AEQoCASAOCwIBIA0MAAcHMS3JAAUEBLECASAQDwAFBAPpAAUEAyECASAVEgIBIBQTAAUEAlkAAwORAAVQskA=",
  "last_paid": 123456789,
  "bits": "0x3df",
  "cells": "0x16",
  "public_cells": "0x0",
  "due_payment": "0x6f",
  "last_trans_lt": "0x0",
  "balance": "0x174876e800",
  "balance_other": [
    {
      "currency": 1,
      "value": "0x64"
    },
    {
      "currency": 2,
      "value": "0xc8"
    },
    {
      "currency": 3,
      "value": "0x12c"
    },
    {
      "currency": 4,
      "value": "0x190"
    },
    {
      "currency": 5,
      "value": "0x1f4"
    },
    {
      "currency": 6,
      "value": "0x258"
    },
    {
      "currency": 7,
      "value": "0x9896e4"
    }
  ],
  "split_depth": 23,
  "tick": false,
  "tock": true,
  "code": "te6ccgEBBQEANgABDz/////////0AQEPP/////////QCAQ8/P///////9AMBDw/////////0BAAPP/8///////Q=",
  "code_hash": "3c28164f21b76a53cfe73510197b99c735d4d97b652e6950f317bcbfe955848a",
  "data": "te6ccgEBAQEACgAADz//H//////0",
  "data_hash": "47cc6bba530c25a982969baf59254598715aecb5b9d14531d96d24d8a623dd93",
  "library": "te6ccgEBAgEALwABQ6APQXKvQr0nmUedLZlpXZ5OtG4xRMeRXZRVYp/Nw8xC5ZgBAA8/////////9A==",
  "library_hash": "4359e3721d98903035218ff07d3df30d0ce59d224abd2d7b0bfe65423fb0f67f",
  "acc_type": 1,
  "acc_type_name": "Active"
}"#
    );
}

#[test]
fn test_frozen_account_into_json_0() {
    let mut account = generate_test_account_by_init_code_hash(false);
    let cloned_account = account.clone();
    account.try_freeze().unwrap();
    account.update_storage_stat().unwrap();
    let boc = account.write_to_bytes().unwrap();
    let sender = AccountSerializationSet {
        account,
        prev_code_hash: cloned_account.get_code_hash(),
        boc,
        boc1: None,
        proof: None,
    };
    let json = db_serialize_account("id", &sender).unwrap();
    println!("\n\n{:#}", serde_json::json!(json));
    assert_eq!(
        format!("{:#}", serde_json::json!(json)),
        r#"{
  "json_version": 8,
  "id": "983217:0:000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f",
  "workchain_id": 0,
  "boc": "te6ccgEBDgEAogABudxMGQuAAACBAYICgwOEBIUFhgaHB4gIiQmKCosLjAyNDY4Ojw+Q5AQ6AdbzRWLeAAAAAAAAAACi6Q7dABdObwUyRFy3kg41ItCU5XanJx0dk6e0bxwNLsS2/h0EUgECA87ACQICASAGAwIBIAUEAAcHMS3JAAUEBLECASAIBwAFBAPpAAUEAyECASANCgIBIAwLAAUEAlkAAwORAAVQskA=",
  "last_paid": 123456789,
  "bits_dec": "541",
  "bits": "221d",
  "cells_dec": "14",
  "cells": "0e",
  "public_cells_dec": "0",
  "public_cells": "00",
  "due_payment_dec": "111",
  "due_payment": "016f",
  "last_trans_lt_dec": "0",
  "last_trans_lt": "00",
  "balance_dec": "100000000000",
  "balance": "09174876e800",
  "balance_other": [
    {
      "currency": 1,
      "value_dec": "100",
      "value": "0164"
    },
    {
      "currency": 2,
      "value_dec": "200",
      "value": "01c8"
    },
    {
      "currency": 3,
      "value_dec": "300",
      "value": "0212c"
    },
    {
      "currency": 4,
      "value_dec": "400",
      "value": "02190"
    },
    {
      "currency": 5,
      "value_dec": "500",
      "value": "021f4"
    },
    {
      "currency": 6,
      "value_dec": "600",
      "value": "02258"
    },
    {
      "currency": 7,
      "value_dec": "10000100",
      "value": "059896e4"
    }
  ],
  "state_hash": "d39bc14c91172de4838d48b425395da9c9c74764e9ed1bc7034bb12dbf874114",
  "acc_type": 2,
  "prev_code_hash": "3c28164f21b76a53cfe73510197b99c735d4d97b652e6950f317bcbfe955848a"
}"#
    );
}

#[test]
fn test_frozen_account_into_json_with_hash_0() {
    let mut account = generate_test_account_by_init_code_hash(true);
    let cloned_account = account.clone();
    account.try_freeze().unwrap();
    account.update_storage_stat().unwrap();
    let boc = account.write_to_bytes().unwrap();
    let sender = AccountSerializationSet {
        account,
        prev_code_hash: cloned_account.get_code_hash(),
        boc,
        boc1: None,
        proof: None,
    };
    let json = db_serialize_account("id", &sender).unwrap();
    println!("\n\n{:#}", serde_json::json!(json));
    assert_eq!(
        format!("{:#}", serde_json::json!(json)),
        r#"{
  "json_version": 8,
  "id": "983217:0:000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f",
  "workchain_id": 0,
  "boc": "te6ccgEBDgEAwwAB+xuJgyFwAAAQIDBAUGBwgJCgsMDQ4PEBESExQVFhcYGRobHB0eHyHIDHgDreaKxbwAAAAAAAAAAUXSHboALpzeCmSIuW8kHGpFoSnK7U5OOjsnT2jeOBpdiW38Ogik8KBZPIbdqU8/nNRAZe5nHNdTZe2UuaVDzF7y/6VWEioAECA87ACQICASAGAwIBIAUEAAcHMS3JAAUEBLECASAIBwAFBAPpAAUEAyECASANCgIBIAwLAAUEAlkAAwORAAVQskA=",
  "init_code_hash": "3c28164f21b76a53cfe73510197b99c735d4d97b652e6950f317bcbfe955848a",
  "last_paid": 123456789,
  "bits_dec": "798",
  "bits": "231e",
  "cells_dec": "14",
  "cells": "0e",
  "public_cells_dec": "0",
  "public_cells": "00",
  "due_payment_dec": "111",
  "due_payment": "016f",
  "last_trans_lt_dec": "0",
  "last_trans_lt": "00",
  "balance_dec": "100000000000",
  "balance": "09174876e800",
  "balance_other": [
    {
      "currency": 1,
      "value_dec": "100",
      "value": "0164"
    },
    {
      "currency": 2,
      "value_dec": "200",
      "value": "01c8"
    },
    {
      "currency": 3,
      "value_dec": "300",
      "value": "0212c"
    },
    {
      "currency": 4,
      "value_dec": "400",
      "value": "02190"
    },
    {
      "currency": 5,
      "value_dec": "500",
      "value": "021f4"
    },
    {
      "currency": 6,
      "value_dec": "600",
      "value": "02258"
    },
    {
      "currency": 7,
      "value_dec": "10000100",
      "value": "059896e4"
    }
  ],
  "state_hash": "d39bc14c91172de4838d48b425395da9c9c74764e9ed1bc7034bb12dbf874114",
  "acc_type": 2,
  "prev_code_hash": "3c28164f21b76a53cfe73510197b99c735d4d97b652e6950f317bcbfe955848a"
}"#
    );
}

#[test]
fn test_frozen_account_into_json_q() {
    let mut account = generate_test_account_by_init_code_hash(false);
    let cloned_account = account.clone();
    account.try_freeze().unwrap();
    account.update_storage_stat().unwrap();
    let boc = account.write_to_bytes().unwrap();
    let sender = AccountSerializationSet {
        account,
        prev_code_hash: cloned_account.get_code_hash(),
        boc,
        boc1: None,
        proof: None,
    };
    let json = db_serialize_account_ex("id", &sender, SerializationMode::QServer).unwrap();
    println!("\n\n{:#}", serde_json::json!(json));
    assert_eq!(
        format!("{:#}", serde_json::json!(json)),
        r#"{
  "json_version": 8,
  "id": "983217:0:000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f",
  "workchain_id": 0,
  "boc": "te6ccgEBDgEAogABudxMGQuAAACBAYICgwOEBIUFhgaHB4gIiQmKCosLjAyNDY4Ojw+Q5AQ6AdbzRWLeAAAAAAAAAACi6Q7dABdObwUyRFy3kg41ItCU5XanJx0dk6e0bxwNLsS2/h0EUgECA87ACQICASAGAwIBIAUEAAcHMS3JAAUEBLECASAIBwAFBAPpAAUEAyECASANCgIBIAwLAAUEAlkAAwORAAVQskA=",
  "last_paid": 123456789,
  "bits": "0x21d",
  "cells": "0xe",
  "public_cells": "0x0",
  "due_payment": "0x6f",
  "last_trans_lt": "0x0",
  "balance": "0x174876e800",
  "balance_other": [
    {
      "currency": 1,
      "value": "0x64"
    },
    {
      "currency": 2,
      "value": "0xc8"
    },
    {
      "currency": 3,
      "value": "0x12c"
    },
    {
      "currency": 4,
      "value": "0x190"
    },
    {
      "currency": 5,
      "value": "0x1f4"
    },
    {
      "currency": 6,
      "value": "0x258"
    },
    {
      "currency": 7,
      "value": "0x9896e4"
    }
  ],
  "state_hash": "d39bc14c91172de4838d48b425395da9c9c74764e9ed1bc7034bb12dbf874114",
  "acc_type": 2,
  "acc_type_name": "Frozen",
  "prev_code_hash": "3c28164f21b76a53cfe73510197b99c735d4d97b652e6950f317bcbfe955848a"
}"#
    );
}

#[test]
fn test_pruned_account_into_json_0() {
    let account = generate_test_account_by_init_code_hash(false);
    let code = account.get_code().map(|cell| cell.repr_hash());
    let libs = account.libraries().root().map(|cell| cell.repr_hash());

    let cell = account.serialize().unwrap();

    let proof = MerkleProof::create(&cell, |hash| {
        Some(hash) != code.as_ref() && Some(hash) != libs.as_ref()
    })
    .unwrap();
    let account = proof.virtualize().unwrap();
    let boc = proof.write_to_bytes().unwrap();
    let sender =
        AccountSerializationSet { account, prev_code_hash: None, boc, boc1: None, proof: None };
    let json = db_serialize_account("id", &sender).unwrap();
    println!("\n\n{:#}", serde_json::json!(json));
    assert_eq!(
        format!("{:#}", serde_json::json!(json)),
        r#"{
  "json_version": 8,
  "id": "983217:0:000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f",
  "workchain_id": 0,
  "boc": "te6ccgECEgEAAQMACUYDHqSdbCQpXfDXQJv0UmyHqwKlpkLuM4yUD0ZtscULGIIABQEkfdxMGQuAAACBAYICgwOEBIUFhgaHB4gIiQmKCosLjAyNDY4Ojw+RZAe+AdbzRWLeAAAAAAAAAACi6Q7dAB73wAUEAwIoSAEBQ1njch2YkDA1IY/wfT3zDQzlnSJKvS17C/5lQj+w9n8AAQAPP/8f//////QoSAEBPCgWTyG3alPP5zUQGXuZxzXU2XtlLmlQ8xe8v+lVhIoABAIDzsANBgIBIAoHAgEgCQgABwcxLckABQQEsQIBIAwLAAUEA+kABQQDIQIBIBEOAgEgEA8ABQQCWQADA5EABVCyQA==",
  "last_paid": 123456789,
  "bits_dec": "991",
  "bits": "23df",
  "cells_dec": "22",
  "cells": "116",
  "public_cells_dec": "0",
  "public_cells": "00",
  "due_payment_dec": "111",
  "due_payment": "016f",
  "last_trans_lt_dec": "0",
  "last_trans_lt": "00",
  "balance_dec": "100000000000",
  "balance": "09174876e800",
  "balance_other": [
    {
      "currency": 1,
      "value_dec": "100",
      "value": "0164"
    },
    {
      "currency": 2,
      "value_dec": "200",
      "value": "01c8"
    },
    {
      "currency": 3,
      "value_dec": "300",
      "value": "0212c"
    },
    {
      "currency": 4,
      "value_dec": "400",
      "value": "02190"
    },
    {
      "currency": 5,
      "value_dec": "500",
      "value": "021f4"
    },
    {
      "currency": 6,
      "value_dec": "600",
      "value": "02258"
    },
    {
      "currency": 7,
      "value_dec": "10000100",
      "value": "059896e4"
    }
  ],
  "split_depth": 23,
  "tick": false,
  "tock": true,
  "code_hash": "3c28164f21b76a53cfe73510197b99c735d4d97b652e6950f317bcbfe955848a",
  "data": "te6ccgEBAQEACgAADz//H//////0",
  "data_hash": "47cc6bba530c25a982969baf59254598715aecb5b9d14531d96d24d8a623dd93",
  "library_hash": "4359e3721d98903035218ff07d3df30d0ce59d224abd2d7b0bfe65423fb0f67f",
  "acc_type": 1
}"#
    );
}

#[test]
fn test_message_into_json_0() {
    let mut msg = Message::with_ext_in_header(ExternalInboundMessageHeader {
        src: MsgAddressExt::with_extern(SliceData::new(vec![1, 2, 3, 4, 5, 0x80])).unwrap(),
        dst: MsgAddressInt::default(),
        import_fee: 15u64.into(),
    });

    let mut stinit = StateInit::default();
    stinit.set_split_depth(Number5::new(23).unwrap());
    stinit.set_special(TickTock::with_values(false, true));
    let code = SliceData::new(vec![0x3F, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xF4]);
    stinit.set_code(code.into_cell());
    let library = SliceData::new(vec![0x3F, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xF4]);
    stinit.set_library_code(library.into_cell(), false).unwrap();

    msg.set_state_init(stinit);
    msg.set_body(SliceData::new(vec![0x3F, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xF4]));

    let cell = msg.serialize().unwrap();
    let boc = write_boc(&cell).unwrap();
    let id = msg.hash().unwrap();
    let msg = MessageSerializationSet {
        message: msg,
        id,
        block_id: None,
        transaction_id: None,
        transaction_now: Some(123),
        status: MessageProcessingStatus::Processing,
        boc,
        proof: None,
    };
    let json = db_serialize_message("id", &msg).unwrap();
    println!("\n\n{:#}", serde_json::json!(json));
    assert_eq!(
        format!("{:#}", serde_json::json!(json)),
        r#"{
  "json_version": 8,
  "id": "59bf855c9fbee1152e1e151368f5af5850f22f606c819c43adb2fb319e07a4c8",
  "boc": "te6ccgEBAwEAZgACZpFACBAYICwAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAQ+3tH/////////gIBAUOgD0Fyr0K9J5lHnS2ZaV2eTrRuMUTHkV2UVWKfzcPMQuWIAgAPP/////////Q=",
  "status": 2,
  "split_depth": 23,
  "tick": false,
  "tock": true,
  "code": "te6ccgEBAQEACgAADz/////////0",
  "code_hash": "7a0b957a15e93cca3ce96ccb4aecf275a3718a263c8aeca2ab14fe6e1e62172c",
  "library": "te6ccgEBAgEALwABQ6APQXKvQr0nmUedLZlpXZ5OtG4xRMeRXZRVYp/Nw8xC5YgBAA8/////////9A==",
  "library_hash": "c39760fbba54774b6c7fa76bfd46d6fb89d1fe0b19570bef3c4d08decc8b4566",
  "body": "te6ccgEBAQEACgAADz/////////0",
  "body_hash": "7a0b957a15e93cca3ce96ccb4aecf275a3718a263c8aeca2ab14fe6e1e62172c",
  "msg_type": 1,
  "src": ":0102030405",
  "dst": "0:0000000000000000000000000000000000000000000000000000000000000000",
  "dst_workchain_id": 0,
  "import_fee_dec": "15",
  "import_fee": "00f",
  "created_at": 123
}"#
    );
}

#[test]
fn test_message_into_json_q() {
    let mut msg = Message::with_ext_in_header(ExternalInboundMessageHeader {
        src: MsgAddressExt::with_extern(SliceData::new(vec![1, 2, 3, 4, 5, 0x80])).unwrap(),
        dst: MsgAddressInt::default(),
        import_fee: 15u64.into(),
    });

    let mut stinit = StateInit::default();
    stinit.set_split_depth(Number5::new(23).unwrap());
    stinit.set_special(TickTock::with_values(false, true));
    let code = SliceData::new(vec![0x3F, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xF4]);
    stinit.set_code(code.into_cell());
    let library = SliceData::new(vec![0x3F, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xF4]);
    stinit.set_library_code(library.into_cell(), false).unwrap();

    msg.set_state_init(stinit);
    msg.set_body(SliceData::new(vec![0x3F, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xF4]));

    let cell = msg.serialize().unwrap();
    let boc = write_boc(&cell).unwrap();
    let id = msg.hash().unwrap();
    let msg = MessageSerializationSet {
        message: msg,
        id,
        block_id: None,
        transaction_id: None,
        transaction_now: Some(123),
        status: MessageProcessingStatus::Processing,
        boc,
        proof: None,
    };
    let json = db_serialize_message_ex("id", &msg, SerializationMode::QServer).unwrap();
    println!("\n\n{:#}", serde_json::json!(json));
    assert_eq!(
        format!("{:#}", serde_json::json!(json)),
        r#"{
  "json_version": 8,
  "id": "59bf855c9fbee1152e1e151368f5af5850f22f606c819c43adb2fb319e07a4c8",
  "boc": "te6ccgEBAwEAZgACZpFACBAYICwAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAQ+3tH/////////gIBAUOgD0Fyr0K9J5lHnS2ZaV2eTrRuMUTHkV2UVWKfzcPMQuWIAgAPP/////////Q=",
  "status": 2,
  "status_name": "processing",
  "split_depth": 23,
  "tick": false,
  "tock": true,
  "code": "te6ccgEBAQEACgAADz/////////0",
  "code_hash": "7a0b957a15e93cca3ce96ccb4aecf275a3718a263c8aeca2ab14fe6e1e62172c",
  "library": "te6ccgEBAgEALwABQ6APQXKvQr0nmUedLZlpXZ5OtG4xRMeRXZRVYp/Nw8xC5YgBAA8/////////9A==",
  "library_hash": "c39760fbba54774b6c7fa76bfd46d6fb89d1fe0b19570bef3c4d08decc8b4566",
  "body": "te6ccgEBAQEACgAADz/////////0",
  "body_hash": "7a0b957a15e93cca3ce96ccb4aecf275a3718a263c8aeca2ab14fe6e1e62172c",
  "msg_type": 1,
  "msg_type_name": "extIn",
  "src": ":0102030405",
  "dst": "0:0000000000000000000000000000000000000000000000000000000000000000",
  "dst_workchain_id": 0,
  "import_fee": "0xf",
  "created_at": 123
}"#
    );
}

#[test]
fn test_transaction_wo_out_msgs_into_json() {
    let mut transaction = generate_tranzaction(AccountId::from([55; 32]));
    transaction.out_msgs = OutMessages::default();
    let cell = transaction.serialize().unwrap();
    let boc = write_boc(&cell).unwrap();
    let id = transaction.hash().unwrap();
    let tr = TransactionSerializationSetEx {
        transaction: &transaction,
        id: &id,
        status: TransactionProcessingStatus::Preliminary,
        block_id: None,
        workchain_id: None,
        boc: &boc,
        proof: None,
    };

    let json = db_serialize_transaction("id", tr).unwrap();
    println!("\n\n{:#}", serde_json::json!(json));
    assert_eq!(
        format!("{:#}", serde_json::json!(json)),
        r#"{
  "json_version": 8,
  "id": "6abd00aa196e92234902649a4b0955167f65f545c9e5a76046af73b89a026dd3",
  "boc": "te6ccgECDgEAAuIAA7Nzc3Nzc3Nzc3Nzc3Nzc3Nzc3Nzc3Nzc3Nzc3Nzc3Nzc3AAAAAAAB4h8AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAHJAUagDAgEAAwACAIJyAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAEBoAQEYwIAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAt78CgkIBQHe////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////BgHe/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+BwDepqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqamAUOgD0Fyr0K9J5lHnS2ZaV2eTrRuMUTHkV2UVWKfzcPMQuWYCQAPP/////////QBDz/////////0CwFFrcm6/FaUEVZY+ivf5GUVGjIDaUr/zQCPNovSzIzIEPtrW1EMAUWtybr8VpQRVlj6K9/kZRUaMgNpSv/NAI82i9LMjMgQ+2tbUQ0ARa3JuvxWlBFXWPor3+RlFRoyA2lK/80AjzaL0syMyBD7a1tR",
  "status": 1,
  "compute": {
    "skipped_reason": 0,
    "compute_type": 0
  },
  "credit_first": false,
  "aborted": false,
  "destroyed": false,
  "tr_type": 0,
  "lt_dec": "123423",
  "lt": "41e21f",
  "prev_trans_hash": "0000000000000000000000000000000000000000000000000000000000000000",
  "prev_trans_lt_dec": "0",
  "prev_trans_lt": "00",
  "now": 0,
  "outmsg_cnt": 3,
  "orig_status": 1,
  "end_status": 2,
  "in_msg": "c47d870b4ce181071c0d69e7ef34dd781562d58a9303a132558bc760b501d9bc",
  "out_msgs": [],
  "account_addr": "0:0000000000000000000000000000000000000000000000000000000000000000",
  "workchain_id": 0,
  "total_fees_dec": "653",
  "total_fees": "0228d",
  "balance_delta_dec": "-653",
  "balance_delta": "-fdd72",
  "old_hash": "0000000000000000000000000000000000000000000000000000000000000000",
  "new_hash": "0000000000000000000000000000000000000000000000000000000000000000"
}"#
    );
}

#[test]
fn test_transaction_into_json_0() {
    let transaction = generate_tranzaction(AccountId::from([55; 32]));

    let message = transaction.get_out_msg(0).unwrap().unwrap();
    let cell = message.serialize().unwrap();
    let boc = write_boc(&cell).unwrap();
    let id = message.hash().unwrap();
    let msg = MessageSerializationSet {
        message,
        id,
        block_id: None,
        transaction_id: transaction.hash().ok(),
        transaction_now: Some(transaction.now()),
        status: MessageProcessingStatus::Processing,
        boc,
        proof: None,
    };
    let json = db_serialize_message("id", &msg).unwrap();
    println!("\n\n{:#}", serde_json::json!(json));
    assert_eq!(
        format!("{:#}", serde_json::json!(json)),
        r#"{
  "json_version": 8,
  "id": "c47d870b4ce181071c0d69e7ef34dd781562d58a9303a132558bc760b501d9bc",
  "transaction_id": "fcbde06ab4179324274309905a9efdaa11a00782da3a62dbe9978d351e453576",
  "boc": "te6ccgECCgEAAjgABGMCAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAALe/AYFBAEB3v///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////wIB3v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/gMA3qampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampgFDoA9Bcq9CvSeZR50tmWldnk60bjFEx5FdlFVin83DzELlmAUADz/////////0AQ8/////////9AcBRa3JuvxWlBFWWPor3+RlFRoyA2lK/80AjzaL0syMyBD7a1tRCAFFrcm6/FaUEVZY+ivf5GUVGjIDaUr/zQCPNovSzIzIEPtrW1EJAEWtybr8VpQRV1j6K9/kZRUaMgNpSv/NAI82i9LMjMgQ+2tbUQ==",
  "status": 2,
  "split_depth": 23,
  "tick": false,
  "tock": true,
  "code": "te6ccgEBBAEAfAABDz/////////0AQFFrcm6/FaUEVZY+ivf5GUVGjIDaUr/zQCPNovSzIzIEPtrW1ECAUWtybr8VpQRVlj6K9/kZRUaMgNpSv/NAI82i9LMjMgQ+2tbUQMARa3JuvxWlBFXWPor3+RlFRoyA2lK/80AjzaL0syMyBD7a1tR",
  "code_hash": "360f4a95e55ffb03c422b80f244e624bce83701769594d1f28fee6675365c649",
  "data": "te6ccgEBAQEACgAADz/////////0",
  "data_hash": "7a0b957a15e93cca3ce96ccb4aecf275a3718a263c8aeca2ab14fe6e1e62172c",
  "library": "te6ccgEBAgEALwABQ6APQXKvQr0nmUedLZlpXZ5OtG4xRMeRXZRVYp/Nw8xC5ZgBAA8/////////9A==",
  "library_hash": "4359e3721d98903035218ff07d3df30d0ce59d224abd2d7b0bfe65423fb0f67f",
  "body": "te6ccgECAwEAAVUAAd7///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////8BAd7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v4CAN6mpqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqY=",
  "body_hash": "7cedc673b096999b859bad6d552c4574bec3c5aafca70fe586794cc6aff7326b",
  "msg_type": 0,
  "src": "",
  "dst": "0:0000000000000000000000000000000000000000000000000000000000000000",
  "dst_workchain_id": 0,
  "ihr_disabled": false,
  "ihr_fee_dec": "0",
  "ihr_fee": "000",
  "fwd_fee_dec": "0",
  "fwd_fee": "000",
  "bounce": false,
  "bounced": false,
  "value_dec": "0",
  "value": "000",
  "created_lt_dec": "0",
  "created_lt": "00",
  "created_at": 0
}"#
    );

    let cell = transaction.serialize().unwrap();
    let boc = write_boc(&cell).unwrap();
    let id = transaction.hash().unwrap();
    let tr = TransactionSerializationSet {
        transaction,
        id,
        status: TransactionProcessingStatus::Preliminary,
        block_id: None,
        workchain_id: -1,
        boc,
        proof: None,
    };

    let json = db_serialize_transaction("id", &tr).unwrap();
    println!("\n\n{:#}", serde_json::json!(json));
    assert_eq!(
        format!("{:#}", serde_json::json!(json)),
        r#"{
  "json_version": 8,
  "id": "fcbde06ab4179324274309905a9efdaa11a00782da3a62dbe9978d351e453576",
  "boc": "te6ccgECFAEAAysAA7Nzc3Nzc3Nzc3Nzc3Nzc3Nzc3Nzc3Nzc3Nzc3Nzc3Nzc3AAAAAAAB4h8AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAHJAUagDAgEAAwACAIJyAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAIB4AoEAgHbBgUBAUgIAgEgCQcBASAIAGACAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABASAKBGMCAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAALe/BAPDgsB3v///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////wwB3v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/g0A3qampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampgFDoA9Bcq9CvSeZR50tmWldnk60bjFEx5FdlFVin83DzELlmA8ADz/////////0AQ8/////////9BEBRa3JuvxWlBFWWPor3+RlFRoyA2lK/80AjzaL0syMyBD7a1tREgFFrcm6/FaUEVZY+ivf5GUVGjIDaUr/zQCPNovSzIzIEPtrW1ETAEWtybr8VpQRV1j6K9/kZRUaMgNpSv/NAI82i9LMjMgQ+2tbUQ==",
  "status": 1,
  "compute": {
    "skipped_reason": 0,
    "compute_type": 0
  },
  "credit_first": false,
  "aborted": false,
  "destroyed": false,
  "tr_type": 0,
  "lt_dec": "123423",
  "lt": "41e21f",
  "prev_trans_hash": "0000000000000000000000000000000000000000000000000000000000000000",
  "prev_trans_lt_dec": "0",
  "prev_trans_lt": "00",
  "now": 0,
  "outmsg_cnt": 3,
  "orig_status": 1,
  "end_status": 2,
  "in_msg": "c47d870b4ce181071c0d69e7ef34dd781562d58a9303a132558bc760b501d9bc",
  "out_msgs": [
    "c47d870b4ce181071c0d69e7ef34dd781562d58a9303a132558bc760b501d9bc",
    "b06fccd0ce364412491c6e10ef2c3c9ff8bec53fe0e3cb022048c7a5d0c07123",
    "b06fccd0ce364412491c6e10ef2c3c9ff8bec53fe0e3cb022048c7a5d0c07123"
  ],
  "account_addr": "-1:3737373737373737373737373737373737373737373737373737373737373737",
  "workchain_id": -1,
  "total_fees_dec": "653",
  "total_fees": "0228d",
  "balance_delta_dec": "-653",
  "balance_delta": "-fdd72",
  "old_hash": "0000000000000000000000000000000000000000000000000000000000000000",
  "new_hash": "0000000000000000000000000000000000000000000000000000000000000000"
}"#
    );
}

#[test]
fn test_transaction_into_json_q() {
    let transaction = generate_tranzaction(AccountId::from([55; 32]));

    let mut message = transaction.get_out_msg(0).unwrap().unwrap();
    if let CommonMsgInfo::IntMsgInfo(header) = message.header_mut() {
        header.set_src(MsgAddressInt::default());
    }
    let cell = message.serialize().unwrap();
    let boc = write_boc(&cell).unwrap();
    let id = message.hash().unwrap();
    let msg = MessageSerializationSet {
        message,
        id,
        block_id: None,
        transaction_id: transaction.hash().ok(),
        transaction_now: Some(transaction.now()),
        status: MessageProcessingStatus::Processing,
        boc,
        proof: None,
    };
    let json = db_serialize_message_ex("id", &msg, SerializationMode::QServer).unwrap();
    println!("\n\n{:#}", serde_json::json!(json));
    assert_eq!(
        format!("{:#}", serde_json::json!(json)),
        r#"{
  "json_version": 8,
  "id": "735df65db81101d019011e7af787d55fb0cf006e68307991bc4a2840a09b893d",
  "transaction_id": "fcbde06ab4179324274309905a9efdaa11a00782da3a62dbe9978d351e453576",
  "boc": "te6ccgECCgEAAlkABKUIAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAFvfgYFBAEB3v///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////wIB3v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/gMA3qampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampgFDoA9Bcq9CvSeZR50tmWldnk60bjFEx5FdlFVin83DzELlmAUADz/////////0AQ8/////////9AcBRa3JuvxWlBFWWPor3+RlFRoyA2lK/80AjzaL0syMyBD7a1tRCAFFrcm6/FaUEVZY+ivf5GUVGjIDaUr/zQCPNovSzIzIEPtrW1EJAEWtybr8VpQRV1j6K9/kZRUaMgNpSv/NAI82i9LMjMgQ+2tbUQ==",
  "status": 2,
  "status_name": "processing",
  "split_depth": 23,
  "tick": false,
  "tock": true,
  "code": "te6ccgEBBAEAfAABDz/////////0AQFFrcm6/FaUEVZY+ivf5GUVGjIDaUr/zQCPNovSzIzIEPtrW1ECAUWtybr8VpQRVlj6K9/kZRUaMgNpSv/NAI82i9LMjMgQ+2tbUQMARa3JuvxWlBFXWPor3+RlFRoyA2lK/80AjzaL0syMyBD7a1tR",
  "code_hash": "360f4a95e55ffb03c422b80f244e624bce83701769594d1f28fee6675365c649",
  "data": "te6ccgEBAQEACgAADz/////////0",
  "data_hash": "7a0b957a15e93cca3ce96ccb4aecf275a3718a263c8aeca2ab14fe6e1e62172c",
  "library": "te6ccgEBAgEALwABQ6APQXKvQr0nmUedLZlpXZ5OtG4xRMeRXZRVYp/Nw8xC5ZgBAA8/////////9A==",
  "library_hash": "4359e3721d98903035218ff07d3df30d0ce59d224abd2d7b0bfe65423fb0f67f",
  "body": "te6ccgECAwEAAVUAAd7///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////8BAd7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v4CAN6mpqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqY=",
  "body_hash": "7cedc673b096999b859bad6d552c4574bec3c5aafca70fe586794cc6aff7326b",
  "msg_type": 0,
  "msg_type_name": "internal",
  "src": "0:0000000000000000000000000000000000000000000000000000000000000000",
  "src_workchain_id": 0,
  "dst": "0:0000000000000000000000000000000000000000000000000000000000000000",
  "dst_workchain_id": 0,
  "ihr_disabled": false,
  "ihr_fee": "0x0",
  "fwd_fee": "0x0",
  "bounce": false,
  "bounced": false,
  "value": "0x0",
  "created_lt": "0x0",
  "created_at": 0
}"#
    );

    let cell = transaction.serialize().unwrap();
    let boc = write_boc(&cell).unwrap();
    let id = transaction.hash().unwrap();
    let tr = TransactionSerializationSet {
        transaction,
        id,
        status: TransactionProcessingStatus::Preliminary,
        block_id: None,
        boc,
        proof: None,
        workchain_id: -1,
    };

    let json = db_serialize_transaction_ex("id", &tr, SerializationMode::QServer).unwrap();
    println!("\n\n{:#}", serde_json::json!(json));
    assert_eq!(
        format!("{:#}", serde_json::json!(json)),
        r#"{
  "json_version": 8,
  "id": "fcbde06ab4179324274309905a9efdaa11a00782da3a62dbe9978d351e453576",
  "boc": "te6ccgECFAEAAysAA7Nzc3Nzc3Nzc3Nzc3Nzc3Nzc3Nzc3Nzc3Nzc3Nzc3Nzc3AAAAAAAB4h8AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAHJAUagDAgEAAwACAIJyAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAIB4AoEAgHbBgUBAUgIAgEgCQcBASAIAGACAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABASAKBGMCAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAALe/BAPDgsB3v///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////wwB3v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/v7+/g0A3qampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampqampgFDoA9Bcq9CvSeZR50tmWldnk60bjFEx5FdlFVin83DzELlmA8ADz/////////0AQ8/////////9BEBRa3JuvxWlBFWWPor3+RlFRoyA2lK/80AjzaL0syMyBD7a1tREgFFrcm6/FaUEVZY+ivf5GUVGjIDaUr/zQCPNovSzIzIEPtrW1ETAEWtybr8VpQRV1j6K9/kZRUaMgNpSv/NAI82i9LMjMgQ+2tbUQ==",
  "status": 1,
  "status_name": "preliminary",
  "compute": {
    "skipped_reason": 0,
    "skipped_reason_name": "noState",
    "compute_type": 0,
    "compute_type_name": "skipped"
  },
  "credit_first": false,
  "aborted": false,
  "destroyed": false,
  "tr_type": 0,
  "tr_type_name": "ordinary",
  "lt": "0x1e21f",
  "prev_trans_hash": "0000000000000000000000000000000000000000000000000000000000000000",
  "prev_trans_lt": "0x0",
  "now": 0,
  "outmsg_cnt": 3,
  "orig_status": 1,
  "orig_status_name": "Active",
  "end_status": 2,
  "end_status_name": "Frozen",
  "in_msg": "c47d870b4ce181071c0d69e7ef34dd781562d58a9303a132558bc760b501d9bc",
  "out_msgs": [
    "c47d870b4ce181071c0d69e7ef34dd781562d58a9303a132558bc760b501d9bc",
    "b06fccd0ce364412491c6e10ef2c3c9ff8bec53fe0e3cb022048c7a5d0c07123",
    "b06fccd0ce364412491c6e10ef2c3c9ff8bec53fe0e3cb022048c7a5d0c07123"
  ],
  "account_addr": "-1:3737373737373737373737373737373737373737373737373737373737373737",
  "workchain_id": -1,
  "total_fees": "0x28d",
  "balance_delta": "-0x28d",
  "old_hash": "0000000000000000000000000000000000000000000000000000000000000000",
  "new_hash": "0000000000000000000000000000000000000000000000000000000000000000"
}"#
    );
}

fn test_json_block(blockhash: &str, mode: SerializationMode) {
    let filename = format!("{}.boc", blockhash);
    let in_path = Path::new("src/tests/data").join(&filename);
    let boc = read(in_path.clone()).expect(&format!("Error reading file {:?}", in_path));
    let cell = read_single_root_boc(&boc).expect("Error deserializing single root BOC");

    let block = Block::construct_from_cell(cell).unwrap();
    let id = block.hash().unwrap();
    let block = BlockSerializationSet { block, id, status: BlockProcessingStatus::Proposed, boc };

    let json =
        format!("{:#}", serde_json::json!(db_serialize_block_ex("id", &block, mode).unwrap()));
    let filename =
        format!("{}{}", blockhash, if let SerializationMode::QServer = mode { "-Q" } else { "" });
    assert_json_eq_file(&json, &filename);
}

#[test]
fn test_get_config() {
    let filename =
        "src/tests/data/9C9906A80D020952E0192DC60C0B2BF1F55FE9A9E065606E8FE25C08BD1AA6B2.boc";
    let in_path = Path::new(filename);
    let boc = read(in_path).expect(&format!("Error reading file {:?}", filename));
    let cell = read_single_root_boc(&boc).expect("Error deserializing single root BOC");

    let block = Block::construct_from_cell(cell).unwrap();

    let extra = block.read_extra().unwrap();
    let master = extra.read_custom().unwrap().unwrap();
    let config = master.config().unwrap();
    let json = serialize_config_param(&config, 12).unwrap();
    let etalon = r#"{
  "p12": [
    {
      "workchain_id": 0,
      "enabled_since": 1573821854,
      "actual_min_split": 0,
      "min_split": 2,
      "max_split": 32,
      "active": true,
      "accept_msgs": true,
      "flags": 0,
      "zerostate_root_hash": "55b13f6d0e1d0c34c9c2160f6f918e92d82bf9ddcf8de2e4c94a3fdf39d15446",
      "zerostate_file_hash": "ee0bedfe4b32761fb35e9e1d8818ea720cad1a0e7b4d2ed673c488e72e910342",
      "version": 0,
      "basic": true,
      "vm_version": -1,
      "vm_mode": 0
    }
  ]
}"#;

    assert_eq!(etalon, json);
    // if json != etalon {
    // std::fs::write("real_tvm_data/p12-config-param.json", &json).unwrap();
    // panic!("json != etalon")
    // }
}

#[test]
fn test_block_into_json_1() {
    test_json_block(
        "89ED400A43E76664437EFC9C79B84AC387493A9EE5E789338FF71C25F54218BE",
        SerializationMode::Standart,
    )
}

#[test]
fn test_block_into_json_2() {
    test_json_block(
        "18AFCDD25BE0989CE516504263EB351818A0FF8F6AB3689501C8E3B767EF413C",
        SerializationMode::Standart,
    )
}

#[test]
fn test_block_into_json_3() {
    test_json_block(
        "046784ea72574ace66375629229700afa4c7e032a360fc94df4c20231fddea45",
        SerializationMode::Standart,
    )
}

#[test]
fn test_block_into_json_q() {
    test_json_block(
        "89ED400A43E76664437EFC9C79B84AC387493A9EE5E789338FF71C25F54218BE",
        SerializationMode::QServer,
    )
}

#[test]
fn test_key_block_into_json() {
    test_json_block(
        "9C9906A80D020952E0192DC60C0B2BF1F55FE9A9E065606E8FE25C08BD1AA6B2",
        SerializationMode::Standart,
    )
}

#[test]
fn test_block_with_copyleft_into_json() {
    test_json_block(
        "ea67954c1c58997c66b5d91b4a3369cfa795b96662c7f7ea7daad677266fb7a3",
        SerializationMode::Standart,
    )
}

#[test]
fn test_masterblock_with_copyleft_into_json() {
    test_json_block(
        "f7fdf140aa14f8dd8426e6c6aa339ae65e3bf492ce26dd7ce08916066d6d64c6",
        SerializationMode::Standart,
    )
}

fn get_validator_set() -> ValidatorSet {
    let keydat = base64_decode("7w3fX5jiuo8PyQoFaEL+K9pE/XvbKjH63i0JcraLlBM=").unwrap();

    let key = SigPubKey::from_bytes(&keydat).unwrap();
    let vd1 = ValidatorDescr::with_params(key, 1, None, None);

    let key = SigPubKey::from_bytes(&keydat).unwrap();
    let vd2 = ValidatorDescr::with_params(key, 2, None, None);

    ValidatorSet::new(1234567, 39237233, 1, vec![vd1, vd2]).unwrap()
}

fn get_config_param11() -> ConfigParam11 {
    let normal_params = ConfigProposalSetup {
        min_tot_rounds: 1,
        max_tot_rounds: 2,
        min_wins: 3,
        max_losses: 4,
        min_store_sec: 5,
        max_store_sec: 6,
        bit_price: 7,
        cell_price: 8,
    };
    let critical_params = ConfigProposalSetup {
        min_tot_rounds: 10,
        max_tot_rounds: 20,
        min_wins: 30,
        max_losses: 40,
        min_store_sec: 50000,
        max_store_sec: 60000,
        bit_price: 70000,
        cell_price: 80000,
    };
    ConfigVotingSetup::new(&normal_params, &critical_params).unwrap()
}

#[test]
fn test_crafted_key_block_into_json() {
    let filename =
        "src/tests/data/48377CD82FF8091D6A45908727C8D4E5FC521603E5633AF3AC8C9E45F9579D5B.boc";
    let in_path = Path::new(filename);
    let boc = read(in_path).expect(&format!("Error reading file {:?}", filename));
    let cell = read_single_root_boc(&boc).expect("Error deserializing single root BOC");
    // println!("slice = {}", root_cell);
    let key = base64_decode("7w3fX5jiuo8PyQoFaEL+K9pE/XvbKjH63i0JcraLlBM=").unwrap();
    // ef0ddf5f98e2ba8f0fc90a056842fe2bda44fd7bdb2a31fade2d0972b68b9413

    let mut block = Block::construct_from_cell(cell).unwrap();

    // Need to add next config params: 3 4 6 9 33 35 36 37 39

    let cp3 = ConfigParamEnum::ConfigParam3(ConfigParam3 {
        fee_collector_addr: UInt256::from([133; 32]),
    });
    let cp4 =
        ConfigParamEnum::ConfigParam4(ConfigParam4 { dns_root_addr: UInt256::from([144; 32]) });
    let cp6 = ConfigParamEnum::ConfigParam6(ConfigParam6 {
        mint_new_price: 123u64.into(),
        mint_add_price: 1458347523u64.into(),
    });
    let cp9 = ConfigParamEnum::ConfigParam9({
        let mut mp = MandatoryParams::default();
        for i in 1..10u32 {
            mp.add_key(&i).unwrap();
        }
        ConfigParam9 { mandatory_params: mp }
    });
    let cp11 = ConfigParamEnum::ConfigParam11(get_config_param11());

    let mut cp33 = ConfigParam33::new();
    cp33.prev_temp_validators = get_validator_set();
    let cp33 = ConfigParamEnum::ConfigParam33(cp33);

    let mut cp35 = ConfigParam35::new();
    cp35.cur_temp_validators = get_validator_set();
    let cp35 = ConfigParamEnum::ConfigParam35(cp35);

    let mut cp36 = ConfigParam36::new();
    cp36.next_validators = get_validator_set();
    let cp36 = ConfigParamEnum::ConfigParam36(cp36);

    let mut cp37 = ConfigParam37::new();
    cp37.next_temp_validators = get_validator_set();
    let cp37 = ConfigParamEnum::ConfigParam37(cp37);

    let cp39 = ConfigParamEnum::ConfigParam39({
        let mut cp = ConfigParam39::new();

        let spk = SigPubKey::from_bytes(&key).unwrap();
        let cs = CryptoSignature::from_r_s(&[1; 32], &[2; 32]).unwrap();
        let vtk = ValidatorTempKey::with_params(UInt256::from([3; 32]), spk, 100500, 1562663724);
        let vstk = ValidatorSignedTempKey::with_key_and_signature(vtk, cs);
        cp.insert(&UInt256::from([1; 32]), &vstk).unwrap();

        let spk = SigPubKey::from_bytes(&key).unwrap();
        let cs = CryptoSignature::from_r_s(&[6; 32], &[7; 32]).unwrap();
        let vtk = ValidatorTempKey::with_params(UInt256::from([8; 32]), spk, 500100, 1562664724);
        let vstk = ValidatorSignedTempKey::with_key_and_signature(vtk, cs);
        cp.insert(&UInt256::from([2; 32]), &vstk).unwrap();

        cp
    });

    let mut suspended = SuspendedAddresses::new();
    suspended.add_suspended_address(0, UInt256::max()).unwrap();
    suspended.add_suspended_address(-1, UInt256::default()).unwrap();
    let cp44 = ConfigParamEnum::ConfigParam44(suspended);

    let mut extra = block.read_extra().unwrap();
    let mut custom = extra.read_custom().unwrap().unwrap();

    // Need to add prev_block_signatures
    let cs = CryptoSignature::from_r_s(&[1; 32], &[2; 32]).unwrap();
    let csp = CryptoSignaturePair::with_params(UInt256::from([12; 32]), cs.clone());
    custom.prev_blk_signatures_mut().set(&123_u16, &csp).unwrap();
    custom.prev_blk_signatures_mut().set(&345_u16, &csp).unwrap();

    // Need to add shard with FutureSplitMerge
    let sd = ShardDescr::with_params(
        42,
        17,
        25,
        UInt256::from_le_bytes(&[70]),
        FutureSplitMerge::Split { split_utime: 0x12345678, interval: 0x87654321 },
    );
    let mut wc0 = custom.hashes().get(&0_u32).unwrap().unwrap();
    let mut key = tvm_types::BuilderData::new();
    key.append_bit_one().unwrap();
    key.append_bit_one().unwrap();
    let key = SliceData::load_builder(key).unwrap();
    wc0.0.split(key, |old| Ok((old, sd))).unwrap();
    custom.hashes_mut().set(&0_u32, &wc0).unwrap();

    assert_eq!(custom.prev_blk_signatures().len().unwrap(), 2);

    let config_params = custom.config_mut().as_mut().unwrap();

    config_params.set_config(cp3).unwrap();
    config_params.set_config(cp4).unwrap();
    config_params.set_config(cp6).unwrap();
    config_params.set_config(cp9).unwrap();
    config_params.set_config(cp11).unwrap();
    config_params.set_config(cp33).unwrap();
    config_params.set_config(cp35).unwrap();
    config_params.set_config(cp36).unwrap();
    config_params.set_config(cp37).unwrap();
    config_params.set_config(cp39).unwrap();
    config_params.set_config(cp44).unwrap();

    extra.write_custom(Some(&custom)).unwrap();
    block.write_extra(&extra).unwrap();

    let id = block.hash().unwrap();
    let block = BlockSerializationSet { block, id, status: BlockProcessingStatus::Proposed, boc };

    let json = db_serialize_block("id", &block).unwrap();
    let json = format!("{:#}", serde_json::json!(json));

    assert_json_eq_file(&json, "crafted-key-block");
}

#[test]
fn test_db_serialize_block_signatures() {
    let doc = serde_json::to_string_pretty(&serde_json::json!(
        db_serialize_block_signatures(
            "_id",
            &UInt256::from([1; 32]),
            &[
                CryptoSignaturePair::with_params(
                    UInt256::from([2; 32]),
                    CryptoSignature::from_r_s(&[3; 32], &[4; 32]).unwrap()
                ),
                CryptoSignaturePair::with_params(
                    UInt256::from([5; 32]),
                    CryptoSignature::from_r_s(&[6; 32], &[7; 32]).unwrap()
                )
            ]
        )
        .unwrap()
    ))
    .unwrap();

    println!("{}", doc);

    assert_eq!(
        doc,
        r#"{
  "json_version": 8,
  "_id": "0101010101010101010101010101010101010101010101010101010101010101",
  "signatures": [
    {
      "node_id": "0202020202020202020202020202020202020202020202020202020202020202",
      "r": "0303030303030303030303030303030303030303030303030303030303030303",
      "s": "0404040404040404040404040404040404040404040404040404040404040404"
    },
    {
      "node_id": "0505050505050505050505050505050505050505050505050505050505050505",
      "r": "0606060606060606060606060606060606060606060606060606060606060606",
      "s": "0707070707070707070707070707070707070707070707070707070707070707"
    }
  ]
}"#
    )
}

#[test]
fn test_serialize_shard_descr() {
    let sd = ShardDescr::default();
    let doc = serialize_shard_descr(&sd, SerializationMode::Standart).unwrap();
    print!("{}", serde_json::to_string_pretty(&doc).unwrap());
    assert_eq!(
        doc,
        serde_json::from_str::<serde_json::Value>(
            r#"
    {
      "seq_no": 0,
      "reg_mc_seqno": 0,
      "start_lt_dec": "0",
      "start_lt": "00",
      "end_lt_dec": "0",
      "end_lt": "00",
      "root_hash": "0000000000000000000000000000000000000000000000000000000000000000",
      "file_hash": "0000000000000000000000000000000000000000000000000000000000000000",
      "before_split": false,
      "before_merge": false,
      "want_split": false,
      "want_merge": false,
      "nx_cc_updated": false,
      "gen_utime": 0,
      "next_catchain_seqno": 0,
      "next_validator_shard": "0000000000000000",
      "min_ref_mc_seqno": 0,
      "fees_collected_dec": "0",
      "fees_collected": "000",
      "funds_created_dec": "0",
      "funds_created": "000",
      "flags": 0,
      "copyleft_rewards": []
    }
    "#
        )
        .unwrap()
    );
}

#[test]
fn test_db_serialize_block_proof() {
    let boc = read("src/tests/data/block_proof").expect(&format!("Error reading proof file"));
    let cell = read_single_root_boc(&boc).expect("Error deserializing single root BOC");

    let proof = BlockProof::construct_from_cell(cell).unwrap();

    let json = serde_json::to_string_pretty(&serde_json::json!(
        db_serialize_block_proof("_id", &proof).unwrap()
    ))
    .unwrap();

    assert_json_eq_file(&json, "proof");
}

fn prepare_shard_state_json(name: &str, workchain_id: i32, mode: SerializationMode) -> String {
    let boc = read(format!("src/tests/data/states/{}", name))
        .expect(&format!("Error reading file {:?}", name));
    let cell = read_single_root_boc(&boc).expect("Error deserializing single root BOC");
    let id = format!("state:{:x}", cell.repr_hash());

    let state = ShardStateUnsplit::construct_from_cell(cell).unwrap();

    let set = ShardStateSerializationSet { state, boc, id, block_id: None, workchain_id };

    format!("{:#}", serde_json::json!(db_serialize_shard_state_ex("id", &set, mode).unwrap()))
}

fn check_shard_state(name: &str, workchain_id: i32, mode: SerializationMode) {
    let json = prepare_shard_state_json(name, workchain_id, mode);

    let postfix = match mode {
        SerializationMode::QServer => "-Q",
        SerializationMode::Standart => "",
        _ => panic!(),
    };
    // std::fs::write(file_name.clone() + postfix + "-ethalon.json",
    // &json).unwrap();
    let name = format!("states/{}{}", name, postfix);
    assert_json_eq_file(&json, &name);
}

#[test]
fn test_serialize_mc_zerostate_s() {
    check_shard_state(
        "main_ton_dev_zerostate_-1_D270B87B2952B5BA7DAA70AAF0A8C361BEFCF4D8D2DB92F9640D5443070838E4",
        -1,
        SerializationMode::Standart,
    );
}

#[test]
fn test_serialize_mc_zerostate_q() {
    check_shard_state(
        "main_ton_dev_zerostate_-1_D270B87B2952B5BA7DAA70AAF0A8C361BEFCF4D8D2DB92F9640D5443070838E4",
        -1,
        SerializationMode::QServer,
    );
}

#[test]
fn test_serialize_wc_zerostate_s() {
    check_shard_state(
        "main_ton_dev_zerostate_0_97AF4602A57FC884F68BB4659BAB8875DC1F5E45A9FD4FBAFD0C9BC10AA5067C",
        0,
        SerializationMode::Standart,
    );
}

#[test]
fn test_serialize_wc_zerostate_q() {
    check_shard_state(
        "main_ton_dev_zerostate_0_97AF4602A57FC884F68BB4659BAB8875DC1F5E45A9FD4FBAFD0C9BC10AA5067C",
        0,
        SerializationMode::QServer,
    );
}

#[test]
fn test_serialize_wc_state_s() {
    check_shard_state(
        "state_4723_0_c800000000000000_81832210A895E93967B7D2A0638159FC5FD88C1DB402545AAAABA509BE93017F",
        0,
        SerializationMode::Standart,
    );
}

#[test]
fn test_serialize_wc_state_q() {
    check_shard_state(
        "state_4723_0_c800000000000000_81832210A895E93967B7D2A0638159FC5FD88C1DB402545AAAABA509BE93017F",
        0,
        SerializationMode::QServer,
    );
}

fn check_transaction_field(
    file: &str,
    field_name: &str,
    std_value: impl Into<Value>,
    q_value: impl Into<Value>,
) {
    let boc = std::fs::read(Path::new("src/tests/data/transactions").join(file)).unwrap();
    let cell = read_single_root_boc(&boc).expect("Error deserializing single root BOC");
    let id = cell.repr_hash();
    let tr = Transaction::construct_from_cell(cell).unwrap();
    let set = TransactionSerializationSet {
        block_id: None,
        boc,
        id,
        proof: None,
        status: TransactionProcessingStatus::Finalized,
        workchain_id: 0,
        transaction: tr,
    };
    let serialized = db_serialize_transaction_ex("id", &set, SerializationMode::Standart).unwrap();
    assert_eq!(serde_json::json!(serialized)[field_name], std_value.into());
    let serialized = db_serialize_transaction_ex("id", &set, SerializationMode::QServer).unwrap();
    assert_eq!(serde_json::json!(serialized)[field_name], q_value.into());
}

#[test]
fn test_balance_delta() {
    check_transaction_field("aborted_bounced.boc", "balance_delta", "000", "0x0");
    check_transaction_field("ext_in&int_out.boc", "balance_delta", "-f8e11fafa9", "-0x1ee05056");
    check_transaction_field(
        "ext_in&int_out_special.boc",
        "balance_delta",
        "-f2d92301e7d945ff",
        "-0x26dcfe1826ba00",
    );
    check_transaction_field("int_in.boc", "balance_delta", "0c71b149203e800", "0x71b149203e800");
}

#[test]
fn test_ext_in_msg_fee() {
    check_transaction_field("aborted_bounced.boc", "ext_in_msg_fee", Value::Null, Value::Null);
    check_transaction_field("ext_in&int_out.boc", "ext_in_msg_fee", "051c80e0", "0x1c80e0");
    check_transaction_field("ext_in&int_out_special.boc", "ext_in_msg_fee", "000", "0x0");
    check_transaction_field("int_in.boc", "ext_in_msg_fee", Value::Null, Value::Null);
}

#[test]
fn test_serialize_deleted_account_s() {
    let account = generate_test_account_by_init_code_hash(true);
    let set = DeletedAccountSerializationSet {
        account_id: MsgAddressInt::default().get_address(),
        workchain_id: MsgAddressInt::default().get_workchain_id(),
        prev_code_hash: account.get_code_hash(),
    };
    let doc = db_serialize_deleted_account("id", &set).unwrap();

    assert_eq!(
        format!("{:#}", serde_json::json!(doc)),
        r#"{
  "json_version": 8,
  "id": "0:0000000000000000000000000000000000000000000000000000000000000000",
  "workchain_id": 0,
  "acc_type": 3,
  "prev_code_hash": "3c28164f21b76a53cfe73510197b99c735d4d97b652e6950f317bcbfe955848a"
}"#
    )
}

#[test]
fn test_serialize_deleted_account_q() {
    let account = generate_test_account_by_init_code_hash(true);
    let set = DeletedAccountSerializationSet {
        account_id: MsgAddressInt::default().get_address(),
        workchain_id: MsgAddressInt::default().get_workchain_id(),
        prev_code_hash: account.get_code_hash(),
    };
    let doc = db_serialize_deleted_account_ex("id", &set, SerializationMode::QServer).unwrap();

    assert_eq!(
        format!("{:#}", serde_json::json!(doc)),
        r#"{
  "json_version": 8,
  "id": "0:0000000000000000000000000000000000000000000000000000000000000000",
  "workchain_id": 0,
  "acc_type": 3,
  "acc_type_name": "NonExist",
  "prev_code_hash": "3c28164f21b76a53cfe73510197b99c735d4d97b652e6950f317bcbfe955848a"
}"#
    )
}

#[test]
fn test_block_order() {
    let block = std::fs::read(
        "src/tests/data/89ED400A43E76664437EFC9C79B84AC387493A9EE5E789338FF71C25F54218BE.boc",
    )
    .unwrap();
    let block = Block::construct_from_bytes(&block).unwrap();
    assert_eq!("4c6dd7m", block_order(&block, 814551).unwrap());
    let block = std::fs::read(
        "src/tests/data/18AFCDD25BE0989CE516504263EB351818A0FF8F6AB3689501C8E3B767EF413C.boc",
    )
    .unwrap();
    let block = Block::construct_from_bytes(&block).unwrap();
    assert_eq!("17b00540960604", block_order(&block, 123).unwrap());
}

fn se_deserialise_remp_status(status: RempMessageStatus) {
    let rr = tvm_api::ton::ton_node::rempreceipt::RempReceipt {
        message_id: "18AFCDD25BE0989CE516504263EB351818A0FF8F6AB3689501C8E3B767EF413C"
            .parse()
            .unwrap(),
        status,
        timestamp: 1640011209924,
        source_id: "18AFCDD25BE0989CE516504263EB351818A0FF8F6AB368950888E3B767EF413C"
            .parse()
            .unwrap(),
    }
    .into_boxed();
    let signature = vec![1, 2, 3, 4];

    let json = format!(
        "{}",
        serde_json::json!(db_serialize_remp_status(&rr, &signature).unwrap()).to_string()
    );
    println!("{}", json);

    let map = serde_json::from_str::<Map<String, Value>>(&json).unwrap();
    let (rr1, signature1) = crate::deserialize::parse_remp_status(&map).unwrap();

    assert_eq!(rr, rr1);
    assert_eq!(signature, signature1);
}

#[test]
fn test_se_deserialise_remp_accepted() {
    se_deserialise_remp_status(RempMessageStatus::TonNode_RempAccepted(
        rempmessagestatus::RempAccepted {
            level: RempMessageLevel::TonNode_RempMasterchain,
            block_id: BlockIdExt::with_params(
                tvm_block::ShardIdent::with_tagged_prefix(0, 0x3800_0000_0000_0000).unwrap(),
                1830539,
                "18AFCDD25BE0989CE516504263EB356618A0FF8F6AB3689501C8E3B767EF413C".parse().unwrap(),
                "18AFCDD25BE0989CE516554263EB351818A0FF8F6AB3689501C8E3B767EF413C".parse().unwrap(),
            )
            .into(),
            master_id: BlockIdExt::with_params(
                tvm_block::ShardIdent::with_tagged_prefix(-1, 0x8000_0000_0000_0000).unwrap(),
                1830539,
                "18AFCD115BE0989CE516504263EB356618A0FF8F6AB3689501C8E3B767EF413C".parse().unwrap(),
                "18AFC2225BE0989CE516554263EB351818A0FF8F6AB3689501C8E3B767EF413C".parse().unwrap(),
            )
            .into(),
        },
    ));
}

#[test]
fn test_se_deserialise_remp_duplicate() {
    se_deserialise_remp_status(RempMessageStatus::TonNode_RempDuplicate(
        rempmessagestatus::RempDuplicate {
            block_id: BlockIdExt::with_params(
                tvm_block::ShardIdent::with_tagged_prefix(0, 0x3800_0000_0000_0000).unwrap(),
                1830539,
                "18AFCDD25BE0989CE516504263EB356618A0FF8F6AB3689501C8E3B767EF413C".parse().unwrap(),
                "18AFCDD25BE0989CE516554263EB351818A0FF8F6AB3689501C8E3B767EF413C".parse().unwrap(),
            )
            .into(),
        },
    ));
}

#[test]
fn test_se_deserialise_remp_ignored() {
    se_deserialise_remp_status(RempMessageStatus::TonNode_RempIgnored(
        rempmessagestatus::RempIgnored {
            level: RempMessageLevel::TonNode_RempMasterchain,
            block_id: BlockIdExt::with_params(
                tvm_block::ShardIdent::with_tagged_prefix(0, 0x3800_0000_0000_0000).unwrap(),
                1830539,
                "18AFCDD25BE0989CE516504263EB356618A0FF8F6AB3689501C8E3B767EF413C".parse().unwrap(),
                "18AFCDD25BE0989CE516554263EB351818A0FF8F6AB3689501C8E3B767EF413C".parse().unwrap(),
            )
            .into(),
        },
    ));
}

#[test]
fn test_se_deserialise_remp_new() {
    se_deserialise_remp_status(RempMessageStatus::TonNode_RempNew);
}

#[test]
fn test_se_deserialise_remp_rejected() {
    se_deserialise_remp_status(RempMessageStatus::TonNode_RempRejected(
        rempmessagestatus::RempRejected {
            level: RempMessageLevel::TonNode_RempMasterchain,
            block_id: BlockIdExt::with_params(
                tvm_block::ShardIdent::with_tagged_prefix(0, 0x3800_0000_0000_0000).unwrap(),
                1830539,
                "18AFCDD25BE0989CE516504263EB356618A0FF8F6AB3689501C8E3B767EF413C".parse().unwrap(),
                "18AFCDD25BE0989CE516554263EB351818A0FF8F6AB3689501C8E3B767EF413C".parse().unwrap(),
            )
            .into(),
            error: "eror 1 2 3".to_string(),
        },
    ));
}

#[test]
fn test_se_deserialise_remp_sent() {
    se_deserialise_remp_status(RempMessageStatus::TonNode_RempSentToValidators(
        rempmessagestatus::RempSentToValidators { sent_to: 10, total_validators: 11 },
    ));
}

#[test]
fn test_se_deserialise_remp_timeout() {
    se_deserialise_remp_status(RempMessageStatus::TonNode_RempTimeout);
}
