use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonEngine.networkProtocol.confirmValidation`\n\n```text\ntonEngine.networkProtocol.confirmValidation \n    id:long \n    peer:int \n    result:long\n    block_seq_no:int\n    block_start_lt:long\n    block_end_lt:long\n    block_gen_utime:int\n= tonEngine.NetworkProtocol;\n```\n"]
pub struct ConfirmValidation {
    pub id: crate::ton::long,
    pub peer: crate::ton::int,
    pub result: crate::ton::long,
    pub block_seq_no: crate::ton::int,
    pub block_start_lt: crate::ton::long,
    pub block_end_lt: crate::ton::long,
    pub block_gen_utime: crate::ton::int,
}
impl Eq for ConfirmValidation {}
impl crate::BareSerialize for ConfirmValidation {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x249e0c6f)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let ConfirmValidation {
            id,
            peer,
            result,
            block_seq_no,
            block_start_lt,
            block_end_lt,
            block_gen_utime,
        } = self;
        _ser.write_bare::<crate::ton::long>(id)?;
        _ser.write_bare::<crate::ton::int>(peer)?;
        _ser.write_bare::<crate::ton::long>(result)?;
        _ser.write_bare::<crate::ton::int>(block_seq_no)?;
        _ser.write_bare::<crate::ton::long>(block_start_lt)?;
        _ser.write_bare::<crate::ton::long>(block_end_lt)?;
        _ser.write_bare::<crate::ton::int>(block_gen_utime)?;
        Ok(())
    }
}
impl crate::BareDeserialize for ConfirmValidation {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_bare::<crate::ton::long>()?;
            let peer = _de.read_bare::<crate::ton::int>()?;
            let result = _de.read_bare::<crate::ton::long>()?;
            let block_seq_no = _de.read_bare::<crate::ton::int>()?;
            let block_start_lt = _de.read_bare::<crate::ton::long>()?;
            let block_end_lt = _de.read_bare::<crate::ton::long>()?;
            let block_gen_utime = _de.read_bare::<crate::ton::int>()?;
            Ok(Self {
                id,
                peer,
                result,
                block_seq_no,
                block_start_lt,
                block_end_lt,
                block_gen_utime,
            })
        }
    }
}
impl crate::IntoBoxed for ConfirmValidation {
    type Boxed = crate::ton::ton_engine::NetworkProtocol;

    fn into_boxed(self) -> crate::ton::ton_engine::NetworkProtocol {
        crate::ton::ton_engine::NetworkProtocol::TonEngine_NetworkProtocol_ConfirmValidation(
            Box::new(self),
        )
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonEngine.networkProtocol.emptyStepRequest`\n\n```text\ntonEngine.networkProtocol.emptyStepRequest \n    id:long\n    empty_step:bytes\n= tonEngine.NetworkProtocol;\n```\n"]
pub struct EmptyStepRequest {
    pub id: crate::ton::long,
    pub empty_step: crate::ton::bytes,
}
impl Eq for EmptyStepRequest {}
impl crate::BareSerialize for EmptyStepRequest {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xdd6c0307)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let EmptyStepRequest { id, empty_step } = self;
        _ser.write_bare::<crate::ton::long>(id)?;
        _ser.write_bare::<crate::ton::bytes>(empty_step)?;
        Ok(())
    }
}
impl crate::BareDeserialize for EmptyStepRequest {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_bare::<crate::ton::long>()?;
            let empty_step = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { id, empty_step })
        }
    }
}
impl crate::IntoBoxed for EmptyStepRequest {
    type Boxed = crate::ton::ton_engine::NetworkProtocol;

    fn into_boxed(self) -> crate::ton::ton_engine::NetworkProtocol {
        crate::ton::ton_engine::NetworkProtocol::TonEngine_NetworkProtocol_EmptyStepRequest(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonEngine.networkProtocol.error`\n\n```text\ntonEngine.networkProtocol.error\n    err_code:int \n    msg:string \n= tonEngine.NetworkProtocol;\n```\n"]
pub struct Error {
    pub err_code: crate::ton::int,
    pub msg: crate::ton::string,
}
impl Eq for Error {}
impl crate::BareSerialize for Error {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xee45fe5f)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Error { err_code, msg } = self;
        _ser.write_bare::<crate::ton::int>(err_code)?;
        _ser.write_bare::<crate::ton::string>(msg)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Error {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let err_code = _de.read_bare::<crate::ton::int>()?;
            let msg = _de.read_bare::<crate::ton::string>()?;
            Ok(Self { err_code, msg })
        }
    }
}
impl crate::IntoBoxed for Error {
    type Boxed = crate::ton::ton_engine::NetworkProtocol;

    fn into_boxed(self) -> crate::ton::ton_engine::NetworkProtocol {
        crate::ton::ton_engine::NetworkProtocol::TonEngine_NetworkProtocol_Error(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonEngine.networkProtocol.rawData`\n\n```text\ntonEngine.networkProtocol.rawData id:long data:bytes = tonEngine.NetworkProtocol;\n```\n"]
pub struct RawData {
    pub id: crate::ton::long,
    pub data: crate::ton::bytes,
}
impl Eq for RawData {}
impl crate::BareSerialize for RawData {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x6ae8a01b)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let RawData { id, data } = self;
        _ser.write_bare::<crate::ton::long>(id)?;
        _ser.write_bare::<crate::ton::bytes>(data)?;
        Ok(())
    }
}
impl crate::BareDeserialize for RawData {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_bare::<crate::ton::long>()?;
            let data = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { id, data })
        }
    }
}
impl crate::IntoBoxed for RawData {
    type Boxed = crate::ton::ton_engine::NetworkProtocol;

    fn into_boxed(self) -> crate::ton::ton_engine::NetworkProtocol {
        crate::ton::ton_engine::NetworkProtocol::TonEngine_NetworkProtocol_RawData(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonEngine.networkProtocol.reflectToDbRequest`\n\n```text\ntonEngine.networkProtocol.reflectToDbRequest \n    id:long\n    transaction:bytes\n    account:bytes\n= tonEngine.NetworkProtocol;\n```\n"]
pub struct ReflectToDbRequest {
    pub id: crate::ton::long,
    pub transaction: crate::ton::bytes,
    pub account: crate::ton::bytes,
}
impl Eq for ReflectToDbRequest {}
impl crate::BareSerialize for ReflectToDbRequest {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x291ada1e)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let ReflectToDbRequest { id, transaction, account } = self;
        _ser.write_bare::<crate::ton::long>(id)?;
        _ser.write_bare::<crate::ton::bytes>(transaction)?;
        _ser.write_bare::<crate::ton::bytes>(account)?;
        Ok(())
    }
}
impl crate::BareDeserialize for ReflectToDbRequest {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_bare::<crate::ton::long>()?;
            let transaction = _de.read_bare::<crate::ton::bytes>()?;
            let account = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { id, transaction, account })
        }
    }
}
impl crate::IntoBoxed for ReflectToDbRequest {
    type Boxed = crate::ton::ton_engine::NetworkProtocol;

    fn into_boxed(self) -> crate::ton::ton_engine::NetworkProtocol {
        crate::ton::ton_engine::NetworkProtocol::TonEngine_NetworkProtocol_ReflectToDbRequest(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonEngine.networkProtocol.reflectToDbResponse`\n\n```text\ntonEngine.networkProtocol.reflectToDbResponse \n    id:long\n    result:long\n= tonEngine.NetworkProtocol;\n```\n"]
pub struct ReflectToDbResponse {
    pub id: crate::ton::long,
    pub result: crate::ton::long,
}
impl Eq for ReflectToDbResponse {}
impl crate::BareSerialize for ReflectToDbResponse {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xaacd0ece)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let ReflectToDbResponse { id, result } = self;
        _ser.write_bare::<crate::ton::long>(id)?;
        _ser.write_bare::<crate::ton::long>(result)?;
        Ok(())
    }
}
impl crate::BareDeserialize for ReflectToDbResponse {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_bare::<crate::ton::long>()?;
            let result = _de.read_bare::<crate::ton::long>()?;
            Ok(Self { id, result })
        }
    }
}
impl crate::IntoBoxed for ReflectToDbResponse {
    type Boxed = crate::ton::ton_engine::NetworkProtocol;

    fn into_boxed(self) -> crate::ton::ton_engine::NetworkProtocol {
        crate::ton::ton_engine::NetworkProtocol::TonEngine_NetworkProtocol_ReflectToDbResponse(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonEngine.networkProtocol.requestBlockByNumber`\n\n```text\ntonEngine.networkProtocol.requestBlockByNumber \n    id:long\n    seq_no:int\n    vert_seq_no:int\n= tonEngine.NetworkProtocol;\n```\n"]
pub struct RequestBlockByNumber {
    pub id: crate::ton::long,
    pub seq_no: crate::ton::int,
    pub vert_seq_no: crate::ton::int,
}
impl Eq for RequestBlockByNumber {}
impl crate::BareSerialize for RequestBlockByNumber {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xfc5c9887)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let RequestBlockByNumber { id, seq_no, vert_seq_no } = self;
        _ser.write_bare::<crate::ton::long>(id)?;
        _ser.write_bare::<crate::ton::int>(seq_no)?;
        _ser.write_bare::<crate::ton::int>(vert_seq_no)?;
        Ok(())
    }
}
impl crate::BareDeserialize for RequestBlockByNumber {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_bare::<crate::ton::long>()?;
            let seq_no = _de.read_bare::<crate::ton::int>()?;
            let vert_seq_no = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { id, seq_no, vert_seq_no })
        }
    }
}
impl crate::IntoBoxed for RequestBlockByNumber {
    type Boxed = crate::ton::ton_engine::NetworkProtocol;

    fn into_boxed(self) -> crate::ton::ton_engine::NetworkProtocol {
        crate::ton::ton_engine::NetworkProtocol::TonEngine_NetworkProtocol_RequestBlockByNumber(
            self,
        )
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonEngine.networkProtocol.requestLastEqualShard`\n\n```text\ntonEngine.networkProtocol.requestLastEqualShard \n    id:long\n    shard_hash:int256\n= tonEngine.NetworkProtocol;\n```\n"]
pub struct RequestLastEqualShard {
    pub id: crate::ton::long,
    pub shard_hash: crate::ton::int256,
}
impl Eq for RequestLastEqualShard {}
impl crate::BareSerialize for RequestLastEqualShard {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xba2b9a5a)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let RequestLastEqualShard { id, shard_hash } = self;
        _ser.write_bare::<crate::ton::long>(id)?;
        _ser.write_bare::<crate::ton::int256>(shard_hash)?;
        Ok(())
    }
}
impl crate::BareDeserialize for RequestLastEqualShard {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_bare::<crate::ton::long>()?;
            let shard_hash = _de.read_bare::<crate::ton::int256>()?;
            Ok(Self { id, shard_hash })
        }
    }
}
impl crate::IntoBoxed for RequestLastEqualShard {
    type Boxed = crate::ton::ton_engine::NetworkProtocol;

    fn into_boxed(self) -> crate::ton::ton_engine::NetworkProtocol {
        crate::ton::ton_engine::NetworkProtocol::TonEngine_NetworkProtocol_RequestLastEqualShard(
            self,
        )
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonEngine.networkProtocol.requestNodeInfo`\n\n```text\ntonEngine.networkProtocol.requestNodeInfo \n    id:long\n= tonEngine.NetworkProtocol;\n```\n"]
pub struct RequestNodeInfo {
    pub id: crate::ton::long,
}
impl Eq for RequestNodeInfo {}
impl crate::BareSerialize for RequestNodeInfo {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x11d0a25b)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let RequestNodeInfo { id } = self;
        _ser.write_bare::<crate::ton::long>(id)?;
        Ok(())
    }
}
impl crate::BareDeserialize for RequestNodeInfo {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_bare::<crate::ton::long>()?;
            Ok(Self { id })
        }
    }
}
impl crate::IntoBoxed for RequestNodeInfo {
    type Boxed = crate::ton::ton_engine::NetworkProtocol;

    fn into_boxed(self) -> crate::ton::ton_engine::NetworkProtocol {
        crate::ton::ton_engine::NetworkProtocol::TonEngine_NetworkProtocol_RequestNodeInfo(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonEngine.networkProtocol.responceBlock`\n\n```text\ntonEngine.networkProtocol.responceBlock \n    id:long\n    signed_block:bytes\n= tonEngine.NetworkProtocol;\n```\n"]
pub struct ResponceBlock {
    pub id: crate::ton::long,
    pub signed_block: crate::ton::bytes,
}
impl Eq for ResponceBlock {}
impl crate::BareSerialize for ResponceBlock {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x5d721f0d)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let ResponceBlock { id, signed_block } = self;
        _ser.write_bare::<crate::ton::long>(id)?;
        _ser.write_bare::<crate::ton::bytes>(signed_block)?;
        Ok(())
    }
}
impl crate::BareDeserialize for ResponceBlock {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_bare::<crate::ton::long>()?;
            let signed_block = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { id, signed_block })
        }
    }
}
impl crate::IntoBoxed for ResponceBlock {
    type Boxed = crate::ton::ton_engine::NetworkProtocol;

    fn into_boxed(self) -> crate::ton::ton_engine::NetworkProtocol {
        crate::ton::ton_engine::NetworkProtocol::TonEngine_NetworkProtocol_ResponceBlock(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonEngine.networkProtocol.responseLastEqualShard`\n\n```text\ntonEngine.networkProtocol.responseLastEqualShard \n    id:long\n    seq_no:int\n    vert_seq_no:int\n= tonEngine.NetworkProtocol;\n```\n"]
pub struct ResponseLastEqualShard {
    pub id: crate::ton::long,
    pub seq_no: crate::ton::int,
    pub vert_seq_no: crate::ton::int,
}
impl Eq for ResponseLastEqualShard {}
impl crate::BareSerialize for ResponseLastEqualShard {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x7f5d7f88)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let ResponseLastEqualShard { id, seq_no, vert_seq_no } = self;
        _ser.write_bare::<crate::ton::long>(id)?;
        _ser.write_bare::<crate::ton::int>(seq_no)?;
        _ser.write_bare::<crate::ton::int>(vert_seq_no)?;
        Ok(())
    }
}
impl crate::BareDeserialize for ResponseLastEqualShard {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_bare::<crate::ton::long>()?;
            let seq_no = _de.read_bare::<crate::ton::int>()?;
            let vert_seq_no = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { id, seq_no, vert_seq_no })
        }
    }
}
impl crate::IntoBoxed for ResponseLastEqualShard {
    type Boxed = crate::ton::ton_engine::NetworkProtocol;

    fn into_boxed(self) -> crate::ton::ton_engine::NetworkProtocol {
        crate::ton::ton_engine::NetworkProtocol::TonEngine_NetworkProtocol_ResponseLastEqualShard(
            self,
        )
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonEngine.networkProtocol.responseNodeInfo`\n\n```text\ntonEngine.networkProtocol.responseNodeInfo \n    id:long\n    validator_no:int\n    workchain:int\n    shard_prefix:long\n    shard_pfx_len:int\n= tonEngine.NetworkProtocol;\n```\n"]
pub struct ResponseNodeInfo {
    pub id: crate::ton::long,
    pub validator_no: crate::ton::int,
    pub workchain: crate::ton::int,
    pub shard_prefix: crate::ton::long,
    pub shard_pfx_len: crate::ton::int,
}
impl Eq for ResponseNodeInfo {}
impl crate::BareSerialize for ResponseNodeInfo {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x23b74379)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let ResponseNodeInfo { id, validator_no, workchain, shard_prefix, shard_pfx_len } = self;
        _ser.write_bare::<crate::ton::long>(id)?;
        _ser.write_bare::<crate::ton::int>(validator_no)?;
        _ser.write_bare::<crate::ton::int>(workchain)?;
        _ser.write_bare::<crate::ton::long>(shard_prefix)?;
        _ser.write_bare::<crate::ton::int>(shard_pfx_len)?;
        Ok(())
    }
}
impl crate::BareDeserialize for ResponseNodeInfo {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_bare::<crate::ton::long>()?;
            let validator_no = _de.read_bare::<crate::ton::int>()?;
            let workchain = _de.read_bare::<crate::ton::int>()?;
            let shard_prefix = _de.read_bare::<crate::ton::long>()?;
            let shard_pfx_len = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { id, validator_no, workchain, shard_prefix, shard_pfx_len })
        }
    }
}
impl crate::IntoBoxed for ResponseNodeInfo {
    type Boxed = crate::ton::ton_engine::NetworkProtocol;

    fn into_boxed(self) -> crate::ton::ton_engine::NetworkProtocol {
        crate::ton::ton_engine::NetworkProtocol::TonEngine_NetworkProtocol_ResponseNodeInfo(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonEngine.networkProtocol.sendMessageRequest`\n\n```text\ntonEngine.networkProtocol.sendMessageRequest \n    id:long\n    message:bytes\n= tonEngine.NetworkProtocol;\n```\n"]
pub struct SendMessageRequest {
    pub id: crate::ton::long,
    pub message: crate::ton::bytes,
}
impl Eq for SendMessageRequest {}
impl crate::BareSerialize for SendMessageRequest {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x7a70345a)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let SendMessageRequest { id, message } = self;
        _ser.write_bare::<crate::ton::long>(id)?;
        _ser.write_bare::<crate::ton::bytes>(message)?;
        Ok(())
    }
}
impl crate::BareDeserialize for SendMessageRequest {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_bare::<crate::ton::long>()?;
            let message = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { id, message })
        }
    }
}
impl crate::IntoBoxed for SendMessageRequest {
    type Boxed = crate::ton::ton_engine::NetworkProtocol;

    fn into_boxed(self) -> crate::ton::ton_engine::NetworkProtocol {
        crate::ton::ton_engine::NetworkProtocol::TonEngine_NetworkProtocol_SendMessageRequest(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonEngine.networkProtocol.sendMessageResponse`\n\n```text\ntonEngine.networkProtocol.sendMessageResponse \n    id:long\n    result:long\n= tonEngine.NetworkProtocol;\n```\n"]
pub struct SendMessageResponse {
    pub id: crate::ton::long,
    pub result: crate::ton::long,
}
impl Eq for SendMessageResponse {}
impl crate::BareSerialize for SendMessageResponse {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x0a1055ed)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let SendMessageResponse { id, result } = self;
        _ser.write_bare::<crate::ton::long>(id)?;
        _ser.write_bare::<crate::ton::long>(result)?;
        Ok(())
    }
}
impl crate::BareDeserialize for SendMessageResponse {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_bare::<crate::ton::long>()?;
            let result = _de.read_bare::<crate::ton::long>()?;
            Ok(Self { id, result })
        }
    }
}
impl crate::IntoBoxed for SendMessageResponse {
    type Boxed = crate::ton::ton_engine::NetworkProtocol;

    fn into_boxed(self) -> crate::ton::ton_engine::NetworkProtocol {
        crate::ton::ton_engine::NetworkProtocol::TonEngine_NetworkProtocol_SendMessageResponse(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonEngine.networkProtocol.test1`\n\n```text\ntonEngine.networkProtocol.test1 id:long hash:int256 = tonEngine.NetworkProtocol;\n```\n"]
pub struct Test1 {
    pub id: crate::ton::long,
    pub hash: crate::ton::int256,
}
impl Eq for Test1 {}
impl crate::BareSerialize for Test1 {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x7afa2679)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Test1 { id, hash } = self;
        _ser.write_bare::<crate::ton::long>(id)?;
        _ser.write_bare::<crate::ton::int256>(hash)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Test1 {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_bare::<crate::ton::long>()?;
            let hash = _de.read_bare::<crate::ton::int256>()?;
            Ok(Self { id, hash })
        }
    }
}
impl crate::IntoBoxed for Test1 {
    type Boxed = crate::ton::ton_engine::NetworkProtocol;

    fn into_boxed(self) -> crate::ton::ton_engine::NetworkProtocol {
        crate::ton::ton_engine::NetworkProtocol::TonEngine_NetworkProtocol_Test1(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonEngine.networkProtocol.test2`\n\n```text\ntonEngine.networkProtocol.test2 id:long flag:Bool = tonEngine.NetworkProtocol;\n```\n"]
pub struct Test2 {
    pub id: crate::ton::long,
    pub flag: crate::ton::Bool,
}
impl Eq for Test2 {}
impl crate::BareSerialize for Test2 {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xc745311c)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Test2 { id, flag } = self;
        _ser.write_bare::<crate::ton::long>(id)?;
        _ser.write_boxed::<crate::ton::Bool>(flag)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Test2 {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_bare::<crate::ton::long>()?;
            let flag = _de.read_boxed::<crate::ton::Bool>()?;
            Ok(Self { id, flag })
        }
    }
}
impl crate::IntoBoxed for Test2 {
    type Boxed = crate::ton::ton_engine::NetworkProtocol;

    fn into_boxed(self) -> crate::ton::ton_engine::NetworkProtocol {
        crate::ton::ton_engine::NetworkProtocol::TonEngine_NetworkProtocol_Test2(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonEngine.networkProtocol.test3`\n\n```text\ntonEngine.networkProtocol.test3 id:long port:int = tonEngine.NetworkProtocol;\n```\n"]
pub struct Test3 {
    pub id: crate::ton::long,
    pub port: crate::ton::int,
}
impl Eq for Test3 {}
impl crate::BareSerialize for Test3 {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xee49ed91)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Test3 { id, port } = self;
        _ser.write_bare::<crate::ton::long>(id)?;
        _ser.write_bare::<crate::ton::int>(port)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Test3 {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_bare::<crate::ton::long>()?;
            let port = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { id, port })
        }
    }
}
impl crate::IntoBoxed for Test3 {
    type Boxed = crate::ton::ton_engine::NetworkProtocol;

    fn into_boxed(self) -> crate::ton::ton_engine::NetworkProtocol {
        crate::ton::ton_engine::NetworkProtocol::TonEngine_NetworkProtocol_Test3(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonEngine.networkProtocol.validationRequest`\n\n```text\ntonEngine.networkProtocol.validationRequest \n    id:long\n    signed_block:bytes\n= tonEngine.NetworkProtocol;\n```\n"]
pub struct ValidationRequest {
    pub id: crate::ton::long,
    pub signed_block: crate::ton::bytes,
}
impl Eq for ValidationRequest {}
impl crate::BareSerialize for ValidationRequest {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xdc0928a1)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let ValidationRequest { id, signed_block } = self;
        _ser.write_bare::<crate::ton::long>(id)?;
        _ser.write_bare::<crate::ton::bytes>(signed_block)?;
        Ok(())
    }
}
impl crate::BareDeserialize for ValidationRequest {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_bare::<crate::ton::long>()?;
            let signed_block = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { id, signed_block })
        }
    }
}
impl crate::IntoBoxed for ValidationRequest {
    type Boxed = crate::ton::ton_engine::NetworkProtocol;

    fn into_boxed(self) -> crate::ton::ton_engine::NetworkProtocol {
        crate::ton::ton_engine::NetworkProtocol::TonEngine_NetworkProtocol_ValidationRequest(self)
    }
}
