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

use std::io::Read;
use std::io::Seek;

use chrono::prelude::Utc;
use serde_json::Value;
use tvm_abi::PublicKeyData;
use tvm_abi::json_abi::DecodedMessage;
use tvm_block::AccountIdPrefixFull;
use tvm_block::CurrencyCollection;
use tvm_block::Deserializable;
use tvm_block::ExternalInboundMessageHeader;
use tvm_block::GetRepresentationHash;
use tvm_block::InternalMessageHeader;
use tvm_block::Message as TvmMessage;
use tvm_block::MsgAddressExt;
use tvm_block::MsgAddressInt;
use tvm_block::Serializable;
use tvm_block::ShardIdent;
use tvm_block::StateInit;
use tvm_types::AccountId;
use tvm_types::BocReader;
use tvm_types::Ed25519PrivateKey;
use tvm_types::Result;
use tvm_types::SliceData;
use tvm_types::error;
use tvm_types::fail;

use crate::AbiContract;
use crate::MessageId;
use crate::error::SdkError;
use crate::json_helper;

pub struct Contract {}

#[derive(Clone, Debug)]
pub struct FunctionCallSet {
    pub func: String,
    pub header: Option<String>,
    pub input: String,
    pub abi: String,
}

pub struct SdkMessage {
    pub id: MessageId,
    pub serialized_message: Vec<u8>,
    pub message: TvmMessage,
    pub address: MsgAddressInt,
}

// The struct represents contract's image
#[derive(Clone)]
pub struct ContractImage {
    state_init: StateInit,
    id: AccountId,
}

impl ContractImage {
    // Creating contract image from code data and library bags of cells
    pub fn from_code_data_and_library<T>(
        code: &mut T,
        data: Option<&mut T>,
        library: Option<&mut T>,
    ) -> Result<Self>
    where
        T: Read + Seek,
    {
        let mut state_init = StateInit::default();

        state_init.set_code(BocReader::new().read(code)?.withdraw_single_root()?);

        if let Some(data) = data {
            state_init.set_data(BocReader::new().read(data)?.withdraw_single_root()?);
        }

        if let Some(library) = library {
            state_init.set_library(BocReader::new().read(library)?.withdraw_single_root()?);
        }

        let id = AccountId::from(state_init.hash()?);

        Ok(Self { state_init, id })
    }

    pub fn new() -> Result<Self> {
        let state_init = StateInit::default();
        let id = state_init.hash()?.into();

        Ok(Self { state_init, id })
    }

    pub fn from_state_init<T>(state_init_bag: &mut T) -> Result<Self>
    where
        T: Read + Seek,
    {
        let cell = BocReader::new().read(state_init_bag)?.withdraw_single_root()?;
        let state_init: StateInit = StateInit::construct_from_cell(cell)?;
        let id = state_init.hash()?.into();

        Ok(Self { state_init, id })
    }

    pub fn from_state_init_and_key<T>(
        state_init_bag: &mut T,
        pub_key: &PublicKeyData,
    ) -> Result<Self>
    where
        T: Read + Seek,
    {
        let mut result = Self::from_state_init(state_init_bag)?;
        result.set_public_key(pub_key)?;

        Ok(result)
    }

    pub fn from_cell(cell: tvm_types::Cell) -> Result<Self> {
        let id = cell.repr_hash().into();
        let state_init = StateInit::construct_from_cell(cell)?;

        Ok(Self { state_init, id })
    }

    pub fn get_public_key(&self) -> Result<Option<PublicKeyData>> {
        let Some(data) = self.state_init.data.clone() else {
            return Ok(None);
        };
        AbiContract::get_pubkey(&SliceData::load_cell(data)?)
    }

    pub fn set_public_key(&mut self, pub_key: &PublicKeyData) -> Result<()> {
        let state_init = &mut self.state_init;

        let new_data = AbiContract::insert_pubkey(
            SliceData::load_cell(state_init.data.clone().unwrap_or_default())?,
            pub_key,
        )?;
        state_init.set_data(new_data.into_cell());

        self.id = state_init.hash()?.into();

        Ok(())
    }

    pub fn get_serialized_code(&self) -> Result<Vec<u8>> {
        match &self.state_init.code {
            Some(cell) => tvm_types::boc::write_boc(cell),
            None => {
                fail!(SdkError::InvalidData { msg: "State init has no code".to_owned() })
            }
        }
    }

    pub fn get_serialized_data(&self) -> Result<Vec<u8>> {
        match &self.state_init.data {
            Some(cell) => tvm_types::boc::write_boc(cell),
            None => {
                fail!(SdkError::InvalidData { msg: "State init has no data".to_owned() })
            }
        }
    }

    pub fn serialize(&self) -> Result<Vec<u8>> {
        tvm_types::boc::write_boc(&self.state_init.serialize()?)
    }

    // Returns future contract's state_init struct
    pub fn state_init(self) -> StateInit {
        self.state_init
    }

    // Returns future contract's identifier
    pub fn account_id(&self) -> AccountId {
        self.id.clone()
    }

    // Returns future contract's address
    pub fn msg_address(&self, workchain_id: i32) -> MsgAddressInt {
        match workchain_id / 128 {
            0 => MsgAddressInt::with_standart(None, workchain_id as i8, self.id.clone()).unwrap(),
            _ => MsgAddressInt::with_variant(None, workchain_id, self.id.clone()).unwrap(),
        }
    }

    /// Allows to change initial values for public contract variables
    pub fn update_data(
        &mut self,
        data_map_supported: bool,
        data_json: &str,
        abi_json: &str,
    ) -> Result<()> {
        let new_data = if data_map_supported {
            tvm_abi::json_abi::update_contract_data(
                abi_json,
                data_json,
                SliceData::load_cell(self.state_init.data.clone().unwrap_or_default())?,
            )?
            .into_cell()
        } else {
            tvm_abi::json_abi::encode_storage_fields(abi_json, Some(data_json))?.into_cell()?
        };

        self.state_init.set_data(new_data);
        self.id = self.state_init.hash()?.into();

        Ok(())
    }
}

pub struct MessageToSign {
    pub message: Vec<u8>,
    pub data_to_sign: Vec<u8>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ShardDescr {
    pub workchain_id: i32,
    #[serde(deserialize_with = "json_helper::deserialize_shard")]
    pub shard: u64,
}

impl Contract {
    /// Decodes output parameters returned by contract function call
    pub fn decode_function_response_json(
        abi: &str,
        function: &str,
        response: SliceData,
        internal: bool,
        allow_partial: bool,
    ) -> Result<String> {
        tvm_abi::json_abi::decode_function_response(
            abi,
            function,
            response,
            internal,
            allow_partial,
        )
    }

    /// Decodes output parameters returned by contract function call from
    /// serialized message body
    pub fn decode_function_response_from_bytes_json(
        abi: &str,
        function: &str,
        response: &[u8],
        internal: bool,
        allow_partial: bool,
    ) -> Result<String> {
        let slice = Self::deserialize_tree_to_slice(response)?;

        Self::decode_function_response_json(abi, function, slice, internal, allow_partial)
    }

    /// Decodes output parameters returned by contract function call
    pub fn decode_unknown_function_response_json(
        abi: &str,
        response: SliceData,
        internal: bool,
        allow_partial: bool,
    ) -> Result<DecodedMessage> {
        tvm_abi::json_abi::decode_unknown_function_response(abi, response, internal, allow_partial)
    }

    /// Decodes output parameters returned by contract function call from
    /// serialized message body
    pub fn decode_unknown_function_response_from_bytes_json(
        abi: &str,
        response: &[u8],
        internal: bool,
        allow_partial: bool,
    ) -> Result<DecodedMessage> {
        let slice = Self::deserialize_tree_to_slice(response)?;

        Self::decode_unknown_function_response_json(abi, slice, internal, allow_partial)
    }

    /// Decodes output parameters returned by contract function call
    pub fn decode_unknown_function_call_json(
        abi: &str,
        response: SliceData,
        internal: bool,
        allow_partial: bool,
    ) -> Result<DecodedMessage> {
        tvm_abi::json_abi::decode_unknown_function_call(abi, response, internal, allow_partial)
    }

    /// Decodes output parameters returned by contract function call from
    /// serialized message body
    pub fn decode_unknown_function_call_from_bytes_json(
        abi: &str,
        response: &[u8],
        internal: bool,
        allow_partial: bool,
    ) -> Result<DecodedMessage> {
        let slice = Self::deserialize_tree_to_slice(response)?;

        Self::decode_unknown_function_call_json(abi, slice, internal, allow_partial)
    }

    // ------- Call constructing functions -------

    // Packs given inputs by abi into an external inbound Message struct.
    // Works with json representation of input and abi.
    // Returns message's bag of cells and identifier.
    pub fn construct_call_ext_in_message_json(
        address: MsgAddressInt,
        src_address: MsgAddressExt,
        params: &FunctionCallSet,
        key_pair: Option<&Ed25519PrivateKey>,
    ) -> Result<SdkMessage> {
        // pack params into bag of cells via ABI
        let msg_body = tvm_abi::encode_function_call(
            &params.abi,
            &params.func,
            params.header.as_deref(),
            &params.input,
            false,
            key_pair,
            Some(&address.to_string()),
        )?;
        let msg = Self::create_ext_in_message(
            address.clone(),
            src_address,
            SliceData::load_cell(msg_body.into_cell()?)?,
        )?;
        let (body, id) = Self::serialize_message(&msg)?;
        Ok(SdkMessage { id, serialized_message: body, message: msg, address })
    }

    // Packs given inputs by abi into an internal Message struct.
    // Works with json representation of input and abi.
    // Returns message's bag of cells and identifier.
    pub fn construct_call_int_message_json(
        address: MsgAddressInt,
        src_address: Option<MsgAddressInt>,
        ihr_disabled: bool,
        bounce: bool,
        value: CurrencyCollection,
        params: &FunctionCallSet,
    ) -> Result<SdkMessage> {
        // pack params into bag of cells via ABI
        let msg_body = tvm_abi::encode_function_call(
            &params.abi,
            &params.func,
            None,
            &params.input,
            true,
            None,
            Some(&address.to_string()),
        )?;

        Self::construct_int_message_with_body(
            address,
            src_address,
            ihr_disabled,
            bounce,
            value,
            Some(SliceData::load_cell(msg_body.into_cell()?)?),
        )
    }

    pub fn construct_int_message_with_body(
        dst_address: MsgAddressInt,
        src_address: Option<MsgAddressInt>,
        ihr_disabled: bool,
        bounce: bool,
        value: CurrencyCollection,
        msg_body: Option<SliceData>,
    ) -> Result<SdkMessage> {
        let msg = Self::create_int_message(
            ihr_disabled,
            bounce,
            dst_address.clone(),
            src_address,
            value,
            msg_body,
        )?;
        let (body, id) = Self::serialize_message(&msg)?;
        Ok(SdkMessage { id, serialized_message: body, message: msg, address: dst_address })
    }

    // Packs given inputs by abi into Message struct without sign and returns data
    // to sign. Sign should be then added with `add_sign_to_message` function
    // Works with json representation of input and abi.
    pub fn get_call_message_bytes_for_signing(
        dst_address: MsgAddressInt,
        src_address: MsgAddressExt,
        params: &FunctionCallSet,
    ) -> Result<MessageToSign> {
        // pack params into bag of cells via ABI
        let (msg_body, data_to_sign) = tvm_abi::prepare_function_call_for_sign(
            &params.abi,
            &params.func,
            params.header.as_deref(),
            &params.input,
            Some(&dst_address.to_string()),
        )?;
        let msg = Self::create_ext_in_message(
            dst_address,
            src_address,
            SliceData::load_cell(msg_body.into_cell()?)?,
        )?;

        Self::serialize_message(&msg)
            .map(|(msg_data, _id)| MessageToSign { message: msg_data, data_to_sign })
    }

    // ------- Deploy constructing functions -------

    // Packs given image and input into Message struct.
    // Works with json representation of input and abi.
    // Returns message's bag of cells and identifier.
    pub fn construct_deploy_message_json(
        params: &FunctionCallSet,
        image: ContractImage,
        key_pair: Option<&Ed25519PrivateKey>,
        workchain_id: i32,
        src_address: MsgAddressExt,
    ) -> Result<SdkMessage> {
        let msg_body = tvm_abi::encode_function_call(
            &params.abi,
            &params.func,
            params.header.as_deref(),
            &params.input,
            false,
            key_pair,
            Some(&image.msg_address(workchain_id).to_string()),
        )?;

        let cell = SliceData::load_cell(msg_body.into_cell()?)?;
        let msg = Self::create_ext_deploy_message(Some(cell), image, workchain_id, src_address)?;
        let address = match msg.dst_ref() {
            Some(address) => address.clone(),
            None => fail!(SdkError::InternalError {
                msg: "No address in created deploy message".to_owned()
            }),
        };
        let (body, id) = Self::serialize_message(&msg)?;

        Ok(SdkMessage { id, serialized_message: body, message: msg, address })
    }

    // Packs given image and body into Message struct.
    // Returns message's bag of cells and identifier.
    pub fn construct_deploy_message_with_body(
        image: ContractImage,
        body: Option<&[u8]>,
        workchain_id: i32,
        src_address: MsgAddressExt,
    ) -> Result<TvmMessage> {
        let body_cell = match body {
            None => None,
            Some(data) => Some(Self::deserialize_tree_to_slice(data)?),
        };

        Self::create_ext_deploy_message(body_cell, image, workchain_id, src_address)
    }

    // Packs given image into an external inbound Message struct.
    // Returns message's bag of cells and identifier.
    pub fn construct_deploy_message_no_constructor(
        image: ContractImage,
        workchain_id: i32,
        src_address: MsgAddressExt,
    ) -> Result<TvmMessage> {
        Self::create_ext_deploy_message(None, image, workchain_id, src_address)
    }

    // Packs given image into an internal Message struct.
    // Returns message's bag of cells and identifier.
    pub fn construct_int_deploy_message_no_constructor(
        src: Option<MsgAddressInt>,
        image: ContractImage,
        workchain_id: i32,
        ihr_disabled: bool,
        bounce: bool,
        value: CurrencyCollection,
    ) -> Result<TvmMessage> {
        Self::create_int_deploy_message(src, None, image, workchain_id, ihr_disabled, bounce, value)
    }

    // Packs given image and input into Message struct without signature and returns
    // data to sign. Signature should be then added with `add_sign_to_message`
    // function Works with json representation of input and abi.
    pub fn get_deploy_message_bytes_for_signing(
        params: &FunctionCallSet,
        image: ContractImage,
        workchain_id: i32,
        src_address: MsgAddressExt,
    ) -> Result<MessageToSign> {
        let (msg_body, data_to_sign) = tvm_abi::prepare_function_call_for_sign(
            &params.abi,
            &params.func,
            params.header.as_deref(),
            &params.input,
            Some(&image.msg_address(workchain_id).to_string()),
        )?;

        let cell = SliceData::load_cell(msg_body.into_cell()?)?;
        let msg = Self::create_ext_deploy_message(Some(cell), image, workchain_id, src_address)?;
        Self::serialize_message(&msg)
            .map(|(msg_data, _id)| MessageToSign { message: msg_data, data_to_sign })
    }

    // Packs given image and input into Message struct with internal header and
    // returns data. Works with json representation of input and abi.
    pub fn get_int_deploy_message_bytes(
        src: Option<MsgAddressInt>,
        params: &FunctionCallSet,
        image: ContractImage,
        workchain_id: i32,
        ihr_disabled: bool,
        bounce: bool,
        value: CurrencyCollection,
    ) -> Result<Vec<u8>> {
        let msg_body = tvm_abi::encode_function_call(
            &params.abi,
            &params.func,
            None,
            &params.input,
            true,
            None,
            Some(&image.msg_address(workchain_id).to_string()),
        )?;

        let cell = SliceData::load_cell(msg_body.into_cell()?)?;
        let msg = Self::create_int_deploy_message(
            src,
            Some(cell),
            image,
            workchain_id,
            ihr_disabled,
            bounce,
            value,
        )?;

        Self::serialize_message(&msg).map(|(msg_data, _id)| msg_data)
    }

    // Add sign to message, returned by `get_deploy_message_bytes_for_signing` or
    // `get_run_message_bytes_for_signing` function.
    // Returns serialized message and identifier.
    pub fn add_sign_to_message(
        abi: &str,
        signature: &[u8],
        public_key: Option<&[u8]>,
        message: &[u8],
    ) -> Result<SdkMessage> {
        let mut slice = Self::deserialize_tree_to_slice(message)?;

        let mut message: TvmMessage = TvmMessage::construct_from(&mut slice)?;

        let body = message
            .body()
            .ok_or(error!(SdkError::InvalidData { msg: "No message body".to_owned() }))?;

        let signed_body = tvm_abi::add_sign_to_function_call(
            abi,
            signature.try_into()?,
            public_key.map(|slice| slice.try_into()).transpose()?,
            body,
        )?;
        message.set_body(SliceData::load_cell(signed_body.into_cell()?)?);

        let address = match message.dst_ref() {
            Some(address) => address.clone(),
            None => {
                fail!(SdkError::InternalError { msg: "No address in signed message".to_owned() })
            }
        };
        let (body, id) = Self::serialize_message(&message)?;

        Ok(SdkMessage { id, address, serialized_message: body, message })
    }

    // Add sign to message, returned by `get_deploy_message_bytes_for_signing` or
    // `get_run_message_bytes_for_signing` function.
    // Returns serialized message and identifier.
    pub fn attach_signature(
        abi: &AbiContract,
        signature: &[u8],
        public_key: Option<&[u8]>,
        message: &[u8],
    ) -> Result<SdkMessage> {
        let mut slice = Self::deserialize_tree_to_slice(message)?;

        let mut message: TvmMessage = TvmMessage::construct_from(&mut slice)?;

        let body = message
            .body()
            .ok_or(error!(SdkError::InvalidData { msg: "No message body".to_owned() }))?;

        let signed_body = abi.add_sign_to_encoded_input(
            signature.try_into()?,
            public_key.map(|slice| slice.try_into()).transpose()?,
            body,
        )?;
        message.set_body(SliceData::load_cell(signed_body.into_cell()?)?);

        let address = match message.dst_ref() {
            Some(address) => address.clone(),
            None => {
                fail!(SdkError::InternalError { msg: "No address in signed message".to_owned() })
            }
        };
        let (body, id) = Self::serialize_message(&message)?;

        Ok(SdkMessage { id, address, serialized_message: body, message })
    }

    fn create_ext_in_message(
        address: MsgAddressInt,
        src: MsgAddressExt,
        msg_body: SliceData,
    ) -> Result<TvmMessage> {
        let msg_header = ExternalInboundMessageHeader { src, dst: address, ..Default::default() };
        // let mut msg_header = ExternalInboundMessageHeader::default();
        // msg_header.dst = address;

        let mut msg = TvmMessage::with_ext_in_header(msg_header);
        msg.set_body(msg_body);

        Ok(msg)
    }

    fn create_int_message(
        ihr_disabled: bool,
        bounce: bool,
        dst: MsgAddressInt,
        src: Option<MsgAddressInt>,
        value: CurrencyCollection,
        msg_body: Option<SliceData>,
    ) -> Result<TvmMessage> {
        let mut msg_header = InternalMessageHeader::default();
        if let Some(src) = src {
            msg_header.set_src(src);
        }
        msg_header.set_dst(dst);
        msg_header.value = value;
        msg_header.ihr_disabled = ihr_disabled;
        msg_header.bounce = bounce;
        let mut msg = TvmMessage::with_int_header(msg_header);
        if let Some(body) = msg_body {
            msg.set_body(body)
        }

        Ok(msg)
    }

    pub(crate) fn create_ext_deploy_message(
        msg_body: Option<SliceData>,
        image: ContractImage,
        workchain_id: i32,
        src: MsgAddressExt,
    ) -> Result<TvmMessage> {
        let msg_header = ExternalInboundMessageHeader {
            dst: image.msg_address(workchain_id),
            src,
            ..Default::default()
        };
        let mut msg = TvmMessage::with_ext_in_header(msg_header);
        msg.set_state_init(image.state_init());
        if let Some(body) = msg_body {
            msg.set_body(body)
        }

        Ok(msg)
    }

    pub(crate) fn create_int_deploy_message(
        src: Option<MsgAddressInt>,
        msg_body: Option<SliceData>,
        image: ContractImage,
        workchain_id: i32,
        ihr_disabled: bool,
        bounce: bool,
        value: CurrencyCollection,
    ) -> Result<TvmMessage> {
        let dst = image.msg_address(workchain_id);
        let mut msg_header = InternalMessageHeader::default();
        if let Some(src) = src {
            msg_header.set_src(src);
        }
        msg_header.set_dst(dst);
        msg_header.ihr_disabled = ihr_disabled;
        msg_header.bounce = bounce;
        msg_header.value = value;

        let mut msg = TvmMessage::with_int_header(msg_header);
        msg.set_state_init(image.state_init());
        if let Some(body) = msg_body {
            msg.set_body(body)
        }

        Ok(msg)
    }

    pub fn serialize_message(msg: &TvmMessage) -> Result<(Vec<u8>, MessageId)> {
        let cells = msg.write_to_new_cell()?.into_cell()?;
        Ok((tvm_types::boc::write_boc(&cells)?, (&cells.repr_hash().as_slice()[..]).into()))
    }

    /// Deserializes tree of cells from byte array into `SliceData`
    pub fn deserialize_tree_to_slice(data: &[u8]) -> Result<SliceData> {
        SliceData::load_cell(tvm_types::boc::read_single_root_boc(data)?)
    }

    pub fn get_dst_from_msg(msg: &[u8]) -> Result<MsgAddressInt> {
        match Contract::deserialize_message(msg)?.dst_ref() {
            Some(address) => Ok(address.clone()),
            None => fail!(SdkError::InvalidData { msg: "Wrong message type (extOut)".to_owned() }),
        }
    }

    /// Deserializes TvmMessage from byte array
    pub fn deserialize_message(message: &[u8]) -> Result<TvmMessage> {
        TvmMessage::construct_from_bytes(message)
    }

    pub fn now() -> u32 {
        Utc::now().timestamp() as u32
    }

    pub fn check_shard_match(shard_descr: Value, address: &MsgAddressInt) -> Result<bool> {
        let descr: ShardDescr = serde_json::from_value(shard_descr)?;
        let ident = ShardIdent::with_tagged_prefix(descr.workchain_id, descr.shard)?;
        Ok(ident.contains_full_prefix(&AccountIdPrefixFull::prefix(address)?))
    }

    pub fn find_matching_shard(shards: &Vec<Value>, address: &MsgAddressInt) -> Result<Value> {
        for shard in shards {
            if Self::check_shard_match(shard.clone(), address)? {
                return Ok(shard.clone());
            }
        }
        Ok(Value::Null)
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use serde_json::json;
    use tvm_abi::PublicKeyData;
    use tvm_abi::token::Token;
    use tvm_abi::token::TokenValue;
    use tvm_block::CurrencyCollection;
    use tvm_block::ExtOutMessageHeader;
    use tvm_block::MsgAddressExt;
    use tvm_block::MsgAddressInt;
    use tvm_block::MsgAddressIntOrNone;
    use tvm_block::Serializable;
    use tvm_block::StateInit;
    use tvm_types::BuilderData;
    use tvm_types::Cell;
    use tvm_types::SliceData;
    use tvm_types::ed25519_generate_private_key;

    use super::*;

    const TEST_ABI: &str = r#"{
        "ABI version": 2,
        "header": ["time", "expire", "pubkey"],
        "functions": [
            {
                "name": "touch",
                "inputs": [{"name": "value", "type": "uint32"}],
                "outputs": [{"name": "ok", "type": "bool"}]
            },
            {
                "name": "constructor",
                "inputs": [],
                "outputs": []
            }
        ],
        "events": [],
        "data": []
    }"#;

    fn test_cell(byte: u8) -> Cell {
        BuilderData::with_raw(vec![byte], 8).unwrap().into_cell().unwrap()
    }

    fn boc(cell: &Cell) -> Vec<u8> {
        tvm_types::boc::write_boc(cell).unwrap()
    }

    fn test_std_address(byte: u8, workchain_id: i8) -> MsgAddressInt {
        MsgAddressInt::with_standart(None, workchain_id, [byte; 32].into()).unwrap()
    }

    fn touch_call_set() -> FunctionCallSet {
        FunctionCallSet {
            func: "touch".to_owned(),
            header: Some(r#"{"time":1,"expire":123}"#.to_owned()),
            input: r#"{"value":"7"}"#.to_owned(),
            abi: TEST_ABI.to_owned(),
        }
    }

    fn constructor_call_set() -> FunctionCallSet {
        FunctionCallSet {
            func: "constructor".to_owned(),
            header: Some("{}".to_owned()),
            input: "{}".to_owned(),
            abi: TEST_ABI.to_owned(),
        }
    }

    fn empty_state_init_boc() -> Vec<u8> {
        tvm_types::boc::write_boc(&StateInit::default().serialize().unwrap()).unwrap()
    }

    #[test]
    fn contract_image_reports_missing_code_and_data_and_can_set_pubkey() {
        let mut image = ContractImage::new().unwrap();

        assert_eq!(image.get_public_key().unwrap(), None);
        assert_eq!(
            image.get_serialized_code().unwrap_err().to_string(),
            "Invalid data: State init has no code"
        );
        assert_eq!(
            image.get_serialized_data().unwrap_err().to_string(),
            "Invalid data: State init has no data"
        );

        let key: PublicKeyData = [7; 32];
        image.set_public_key(&key).unwrap();

        assert_eq!(image.get_public_key().unwrap(), Some(key));
        assert!(!image.get_serialized_data().unwrap().is_empty());
    }

    #[test]
    fn contract_image_roundtrips_code_data_library_and_state_init_sources() {
        let code = test_cell(0xaa);
        let data = test_cell(0xbb);
        let library = test_cell(0xcc);

        let code_boc = boc(&code);
        let data_boc = boc(&data);
        let library_boc = boc(&library);

        let mut code_cursor = Cursor::new(code_boc.clone());
        let mut data_cursor = Cursor::new(data_boc.clone());
        let mut library_cursor = Cursor::new(library_boc);
        let image = ContractImage::from_code_data_and_library(
            &mut code_cursor,
            Some(&mut data_cursor),
            Some(&mut library_cursor),
        )
        .unwrap();

        assert_eq!(image.get_serialized_code().unwrap(), code_boc);
        assert_eq!(image.get_serialized_data().unwrap(), data_boc);

        let serialized = image.serialize().unwrap();
        let roundtrip =
            ContractImage::from_state_init(&mut Cursor::new(serialized.clone())).unwrap();
        assert_eq!(roundtrip.serialize().unwrap(), serialized);

        let from_cell =
            ContractImage::from_cell(tvm_types::boc::read_single_root_boc(&serialized).unwrap())
                .unwrap();
        assert_eq!(from_cell.serialize().unwrap(), serialized);
    }

    #[test]
    fn contract_image_from_state_init_and_key_and_msg_address_cover_branches() {
        let key: PublicKeyData = [3; 32];
        let mut state_init_cursor = Cursor::new(empty_state_init_boc());
        let image = ContractImage::from_state_init_and_key(&mut state_init_cursor, &key).unwrap();

        assert_eq!(image.get_public_key().unwrap(), Some(key));
        assert!(matches!(image.msg_address(0), MsgAddressInt::AddrStd(_)));
        assert!(matches!(image.msg_address(256), MsgAddressInt::AddrVar(_)));
    }

    #[test]
    fn message_helpers_roundtrip_and_cover_destination_errors() {
        let dst = test_std_address(0x11, 0);
        let src_ext = MsgAddressExt::with_extern(SliceData::new(vec![0x55])).unwrap();
        let msg = Contract::create_ext_in_message(
            dst.clone(),
            src_ext,
            SliceData::load_cell(test_cell(0xaa)).unwrap(),
        )
        .unwrap();

        let (serialized, id) = Contract::serialize_message(&msg).unwrap();
        let roundtrip = Contract::deserialize_message(&serialized).unwrap();
        assert_eq!(roundtrip, msg);
        assert_eq!(roundtrip.dst_ref(), Some(&dst));
        assert_eq!(Contract::get_dst_from_msg(&serialized).unwrap(), dst);
        assert!(!id.to_string().is_empty());

        let slice = Contract::deserialize_tree_to_slice(&serialized).unwrap();
        assert_eq!(
            slice.into_cell().repr_hash(),
            msg.write_to_new_cell().unwrap().into_cell().unwrap().repr_hash()
        );

        let ext_out = TvmMessage::with_ext_out_header(ExtOutMessageHeader::with_addresses(
            test_std_address(0x22, 0),
            MsgAddressExt::AddrNone,
        ));
        let ext_out_bytes = Contract::serialize_message(&ext_out).unwrap().0;
        assert_eq!(
            Contract::get_dst_from_msg(&ext_out_bytes).unwrap_err().to_string(),
            "Invalid data: Wrong message type (extOut)"
        );
        assert!(Contract::deserialize_tree_to_slice(b"not a boc").is_err());
        assert!(Contract::deserialize_message(b"not a boc").is_err());
    }

    #[test]
    fn decode_helpers_cover_call_and_response_paths() {
        let sign_key = ed25519_generate_private_key().unwrap();
        let address = test_std_address(0x33, 0);
        let message = Contract::construct_call_ext_in_message_json(
            address.clone(),
            MsgAddressExt::AddrNone,
            &touch_call_set(),
            Some(&sign_key),
        )
        .unwrap();

        let body = message.message.body().unwrap();
        let decoded =
            Contract::decode_unknown_function_call_json(TEST_ABI, body.clone(), false, false)
                .unwrap();
        assert_eq!(decoded.function_name, "touch");
        assert_eq!(decoded.params, r#"{"value":"7"}"#);

        let body_boc = boc(&body.clone().into_cell());
        let decoded_from_bytes = Contract::decode_unknown_function_call_from_bytes_json(
            TEST_ABI, &body_boc, false, false,
        )
        .unwrap();
        assert_eq!(decoded_from_bytes.params, decoded.params);

        let abi = AbiContract::load(TEST_ABI.as_bytes()).unwrap();
        let function = abi.function("touch").unwrap();
        let output = function
            .encode_internal_output(
                function.get_output_id(),
                &[Token::new("ok", TokenValue::Bool(true))],
            )
            .unwrap();
        let output_slice = SliceData::load_builder(output.clone()).unwrap();
        assert_eq!(
            Contract::decode_function_response_json(
                TEST_ABI,
                "touch",
                output_slice.clone(),
                false,
                false
            )
            .unwrap(),
            r#"{"ok":true}"#
        );

        let unknown =
            Contract::decode_unknown_function_response_json(TEST_ABI, output_slice, false, false)
                .unwrap();
        assert_eq!(unknown.function_name, "touch");
        assert_eq!(unknown.params, r#"{"ok":true}"#);

        let output_boc = boc(&output.into_cell().unwrap());
        assert_eq!(
            Contract::decode_function_response_from_bytes_json(
                TEST_ABI,
                "touch",
                &output_boc,
                false,
                false
            )
            .unwrap(),
            r#"{"ok":true}"#
        );
        assert_eq!(
            Contract::decode_unknown_function_response_from_bytes_json(
                TEST_ABI,
                &output_boc,
                false,
                false,
            )
            .unwrap()
            .params,
            r#"{"ok":true}"#
        );
    }

    #[test]
    fn call_and_deploy_builders_cover_signing_and_internal_paths() {
        let sign_key = ed25519_generate_private_key().unwrap();
        let public_key = sign_key.verifying_key();
        let dst = test_std_address(0x44, 0);
        let src = test_std_address(0x55, 0);
        let value = CurrencyCollection::with_grams(77);

        let ext = Contract::construct_call_ext_in_message_json(
            dst.clone(),
            MsgAddressExt::AddrNone,
            &touch_call_set(),
            Some(&sign_key),
        )
        .unwrap();
        assert_eq!(ext.address, dst);
        assert!(ext.message.ext_in_header().is_some());
        assert!(ext.message.body().is_some());

        let internal = Contract::construct_call_int_message_json(
            dst.clone(),
            Some(src.clone()),
            true,
            false,
            value.clone(),
            &touch_call_set(),
        )
        .unwrap();
        let header = internal.message.int_header().unwrap();
        assert_eq!(header.dst, dst);
        assert_eq!(header.src, MsgAddressIntOrNone::Some(src.clone()));
        assert_eq!(header.value, value);
        assert!(header.ihr_disabled);
        assert!(!header.bounce);
        assert_eq!(
            Contract::decode_unknown_function_call_json(
                TEST_ABI,
                internal.message.body().unwrap(),
                true,
                false,
            )
            .unwrap()
            .params,
            r#"{"value":"7"}"#
        );

        let unsigned = Contract::get_call_message_bytes_for_signing(
            dst.clone(),
            MsgAddressExt::AddrNone,
            &touch_call_set(),
        )
        .unwrap();
        let signature = sign_key.sign(&unsigned.data_to_sign);
        let signed = Contract::add_sign_to_message(
            TEST_ABI,
            &signature,
            Some(&public_key),
            &unsigned.message,
        )
        .unwrap();
        let attached = Contract::attach_signature(
            &AbiContract::load(TEST_ABI.as_bytes()).unwrap(),
            &signature,
            Some(&public_key),
            &unsigned.message,
        )
        .unwrap();
        assert_eq!(signed.serialized_message, attached.serialized_message);
        assert_eq!(signed.address, dst);

        let image = ContractImage::new().unwrap();
        let deploy = Contract::construct_deploy_message_json(
            &constructor_call_set(),
            image.clone(),
            Some(&sign_key),
            0,
            MsgAddressExt::AddrNone,
        )
        .unwrap();
        assert_eq!(deploy.address, image.msg_address(0));
        assert!(deploy.message.state_init().is_some());

        let unsigned_deploy = Contract::get_deploy_message_bytes_for_signing(
            &constructor_call_set(),
            image.clone(),
            0,
            MsgAddressExt::AddrNone,
        )
        .unwrap();
        let deploy_signature = sign_key.sign(&unsigned_deploy.data_to_sign);
        let signed_deploy = Contract::add_sign_to_message(
            TEST_ABI,
            &deploy_signature,
            Some(&public_key),
            &unsigned_deploy.message,
        )
        .unwrap();
        assert_eq!(signed_deploy.address, image.msg_address(0));

        let body_boc = boc(&test_cell(0xfe));
        let deploy_with_body = Contract::construct_deploy_message_with_body(
            image.clone(),
            Some(&body_boc),
            0,
            MsgAddressExt::AddrNone,
        )
        .unwrap();
        assert!(deploy_with_body.state_init().is_some());
        assert!(deploy_with_body.body().is_some());

        let deploy_no_ctor = Contract::construct_deploy_message_no_constructor(
            image.clone(),
            0,
            MsgAddressExt::AddrNone,
        )
        .unwrap();
        assert!(deploy_no_ctor.state_init().is_some());
        assert!(deploy_no_ctor.body().is_none());

        let int_deploy = Contract::construct_int_deploy_message_no_constructor(
            Some(src.clone()),
            image.clone(),
            0,
            false,
            true,
            CurrencyCollection::with_grams(10),
        )
        .unwrap();
        let int_deploy_header = int_deploy.int_header().unwrap();
        assert_eq!(int_deploy_header.src, MsgAddressIntOrNone::Some(src));
        assert!(int_deploy_header.bounce);

        let int_deploy_bytes = Contract::get_int_deploy_message_bytes(
            None,
            &constructor_call_set(),
            image,
            0,
            true,
            false,
            CurrencyCollection::with_grams(1),
        )
        .unwrap();
        let int_deploy_message = Contract::deserialize_message(&int_deploy_bytes).unwrap();
        assert!(int_deploy_message.int_header().is_some());
        assert!(int_deploy_message.state_init().is_some());
    }

    #[test]
    fn signing_helpers_report_missing_body() {
        let empty = TvmMessage::with_ext_in_header(tvm_block::ExternalInboundMessageHeader {
            src: MsgAddressExt::AddrNone,
            dst: test_std_address(0x66, 0),
            ..Default::default()
        });
        let serialized = Contract::serialize_message(&empty).unwrap().0;
        let abi = AbiContract::load(TEST_ABI.as_bytes()).unwrap();

        let add_err =
            Contract::add_sign_to_message(TEST_ABI, &[0u8; 64], None, &serialized).err().unwrap();
        assert_eq!(add_err.to_string(), "Invalid data: No message body");

        let attach_err =
            Contract::attach_signature(&abi, &[0u8; 64], None, &serialized).err().unwrap();
        assert_eq!(attach_err.to_string(), "Invalid data: No message body");
    }

    #[test]
    fn shard_matching_helpers_cover_found_missing_and_invalid_cases() {
        let address = test_std_address(0x77, 0);
        let root = json!({ "workchain_id": 0, "shard": "8000000000000000" });
        let other = json!({ "workchain_id": -1, "shard": "8000000000000000" });

        assert!(Contract::check_shard_match(root.clone(), &address).unwrap());
        assert!(!Contract::check_shard_match(other.clone(), &address).unwrap());

        let shards = vec![other.clone(), root.clone()];
        assert_eq!(Contract::find_matching_shard(&shards, &address).unwrap(), root);
        assert_eq!(
            Contract::find_matching_shard(&vec![other], &address).unwrap(),
            serde_json::Value::Null
        );

        assert!(
            Contract::check_shard_match(json!({ "workchain_id": 0, "shard": "oops" }), &address)
                .is_err()
        );
    }
}
