// Copyright 2018-2021 TON Labs LTD.
//
// Licensed under the SOFTWARE EVALUATION License (the "License"); you may not
// use this file except in compliance with the License.
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific TON DEV software governing permissions and
// limitations under the License.

#[allow(unused_imports)]
use std::str::FromStr;

use tvm_block::{Deserializable, Message, MessageOld};
use tvm_block::Serializable;
use tvm_types::UInt256;
use tvm_types::base64_decode;
use tvm_types::base64_encode;

use crate::ClientContext;
use crate::boc::BocCacheType;
use crate::boc::Error;
use crate::error::ClientResult;

pub(crate) fn get_boc_hash(boc: &[u8]) -> ClientResult<String> {
    let cells = tvm_types::boc::read_single_root_boc(boc).map_err(Error::invalid_boc)?;
    let id: Vec<u8> = cells.repr_hash().as_slice()[..].into();
    Ok(hex::encode(id))
}

pub fn deserialize_cell_from_base64(
    b64: &str,
    name: &str,
) -> ClientResult<(Vec<u8>, tvm_types::Cell)> {
    let bytes = base64_decode(b64)
        .map_err(|err| Error::invalid_boc(format!("error decode {} BOC base64: {}", name, err)))?;

    let cell = tvm_types::boc::read_single_root_boc(&bytes).map_err(|err| {
        Error::invalid_boc(format!("{} BOC deserialization error: {}", name, err))
    })?;

    Ok((bytes, cell))
}

pub fn deserialize_message_from_cell(
    cell: tvm_types::Cell,
    name: &str,
) -> ClientResult<Message> {
    let tip = "Please check that you have specified the message's BOC, not body, as a parameter.";
    let tip_full = if !tip.is_empty() { format!(".\nTip: {}", tip) } else { "".to_string() };
    let cell_clone = cell.clone();
    let mut res = Message::construct_from_cell(cell);
    if res.is_err() {
        res = MessageOld::construct_from_cell(cell_clone).map(|v| v.into())
    }
    res.map_err(|err| {
        Error::invalid_boc(format!("cannot deserialize {} from BOC: {}{}", name, err, tip_full))
    })
}

pub fn deserialize_object_from_cell<S: Deserializable>(
    cell: tvm_types::Cell,
    name: &str,
) -> ClientResult<S> {
    let tip = match name {
        "message" => {
            "Please check that you have specified the message's BOC, not body, as a parameter."
        }
        _ => "",
    };
    let tip_full = if !tip.is_empty() { format!(".\nTip: {}", tip) } else { "".to_string() };
    S::construct_from_cell(cell).map_err(|err| {
        Error::invalid_boc(format!("cannot deserialize {} from BOC: {}{}", name, err, tip_full))
    })
}

#[derive(Clone)]
pub enum DeserializedBoc {
    Cell(tvm_types::Cell),
    Bytes(Vec<u8>),
}

impl DeserializedBoc {
    pub fn bytes(self, name: &str) -> ClientResult<Vec<u8>> {
        match self {
            DeserializedBoc::Bytes(vec) => Ok(vec),
            DeserializedBoc::Cell(cell) => serialize_cell_to_bytes(&cell, name),
        }
    }
}

#[derive(Clone)]
pub struct DeserializedObject<S: Deserializable> {
    pub boc: DeserializedBoc,
    pub cell: tvm_types::Cell,
    pub object: S,
}

pub fn deserialize_object_from_base64<S: Deserializable>(
    b64: &str,
    name: &str,
) -> ClientResult<DeserializedObject<S>> {
    let (bytes, cell) = deserialize_cell_from_base64(b64, name)?;
    let object = deserialize_object_from_cell(cell.clone(), name)?;

    Ok(DeserializedObject { boc: DeserializedBoc::Bytes(bytes), cell, object })
}

pub fn serialize_object_to_cell<S: Serializable>(
    object: &S,
    name: &str,
) -> ClientResult<tvm_types::Cell> {
    object.serialize().map_err(|err| Error::serialization_error(err, name))
}

pub fn serialize_cell_to_bytes(cell: &tvm_types::Cell, name: &str) -> ClientResult<Vec<u8>> {
    tvm_types::boc::write_boc(cell).map_err(|err| Error::serialization_error(err, name))
}

pub fn serialize_cell_to_base64(cell: &tvm_types::Cell, name: &str) -> ClientResult<String> {
    Ok(base64_encode(serialize_cell_to_bytes(cell, name)?))
}

pub fn serialize_object_to_base64<S: Serializable>(object: &S, name: &str) -> ClientResult<String> {
    let cell = serialize_object_to_cell(object, name)?;
    serialize_cell_to_base64(&cell, name)
}

pub fn deserialize_cell_from_boc(
    context: &ClientContext,
    boc: &str,
    name: &str,
) -> ClientResult<(DeserializedBoc, tvm_types::Cell)> {
    context.bocs.deserialize_cell(boc, name)
}

pub fn deserialize_message_from_boc(
    context: &ClientContext,
    boc: &str,
    name: &str,
) -> ClientResult<DeserializedObject<Message>> {
    let (boc, cell) = deserialize_cell_from_boc(context, boc, name)?;

    let object = deserialize_message_from_cell(cell.clone(), name)?;

    Ok(DeserializedObject { boc, cell, object })
}


pub fn deserialize_object_from_boc<S: Deserializable>(
    context: &ClientContext,
    boc: &str,
    name: &str,
) -> ClientResult<DeserializedObject<S>> {
    let (boc, cell) = deserialize_cell_from_boc(context, boc, name)?;

    let object = deserialize_object_from_cell(cell.clone(), name)?;

    Ok(DeserializedObject { boc, cell, object })
}

pub fn deserialize_object_from_boc_bin<S: Deserializable>(
    boc: &[u8],
) -> ClientResult<(S, UInt256)> {
    let cell = tvm_types::boc::read_single_root_boc(boc).map_err(Error::invalid_boc)?;
    let root_hash = cell.repr_hash();
    let object = S::construct_from_cell(cell).map_err(Error::invalid_boc)?;

    Ok((object, root_hash))
}

pub fn serialize_cell_to_boc(
    context: &ClientContext,
    cell: tvm_types::Cell,
    name: &str,
    boc_cache: Option<BocCacheType>,
) -> ClientResult<String> {
    if let Some(cache_type) = boc_cache {
        context.bocs.add(cache_type, cell, None).map(|hash| format!("*{:x}", hash))
    } else {
        serialize_cell_to_base64(&cell, name)
    }
}

pub fn serialize_object_to_boc<S: Serializable>(
    context: &ClientContext,
    object: &S,
    name: &str,
    boc_cache: Option<BocCacheType>,
) -> ClientResult<String> {
    let cell = serialize_object_to_cell(object, name)?;
    serialize_cell_to_boc(context, cell, name, boc_cache)
}
