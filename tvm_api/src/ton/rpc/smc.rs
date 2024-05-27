use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `smc.getCode`\n\n```text\nsmc.getCode id:int53 = tvm.Cell;\n```\n"]
pub struct GetCode {
    pub id: crate::ton::int53,
}
impl Eq for GetCode {}
impl crate::BareSerialize for GetCode {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x81e61b98)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let GetCode { id } = self;
        _ser.write_bare::<crate::ton::int53>(id)?;
        Ok(())
    }
}
impl crate::BareDeserialize for GetCode {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_bare::<crate::ton::int53>()?;
            Ok(Self { id })
        }
    }
}
impl crate::BoxedDeserialize for GetCode {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x81e61b98)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x81e61b98) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for GetCode {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x81e61b98), self)
    }
}
impl crate::Function for GetCode {
    type Reply = crate::ton::tvm::Cell;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `smc.getData`\n\n```text\nsmc.getData id:int53 = tvm.Cell;\n```\n"]
pub struct GetData {
    pub id: crate::ton::int53,
}
impl Eq for GetData {}
impl crate::BareSerialize for GetData {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xe6835349)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let GetData { id } = self;
        _ser.write_bare::<crate::ton::int53>(id)?;
        Ok(())
    }
}
impl crate::BareDeserialize for GetData {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_bare::<crate::ton::int53>()?;
            Ok(Self { id })
        }
    }
}
impl crate::BoxedDeserialize for GetData {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xe6835349)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0xe6835349) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for GetData {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0xe6835349), self)
    }
}
impl crate::Function for GetData {
    type Reply = crate::ton::tvm::Cell;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `smc.getState`\n\n```text\nsmc.getState id:int53 = tvm.Cell;\n```\n"]
pub struct GetState {
    pub id: crate::ton::int53,
}
impl Eq for GetState {}
impl crate::BareSerialize for GetState {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xf338a9eb)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let GetState { id } = self;
        _ser.write_bare::<crate::ton::int53>(id)?;
        Ok(())
    }
}
impl crate::BareDeserialize for GetState {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_bare::<crate::ton::int53>()?;
            Ok(Self { id })
        }
    }
}
impl crate::BoxedDeserialize for GetState {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xf338a9eb)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0xf338a9eb) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for GetState {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0xf338a9eb), self)
    }
}
impl crate::Function for GetState {
    type Reply = crate::ton::tvm::Cell;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `smc.load`\n\n```text\nsmc.load account_address:accountAddress = smc.Info;\n```\n"]
pub struct Load {
    pub account_address: crate::ton::accountaddress::AccountAddress,
}
impl Eq for Load {}
impl crate::BareSerialize for Load {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xca25d03f)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Load { account_address } = self;
        _ser.write_bare::<crate::ton::accountaddress::AccountAddress>(account_address)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Load {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let account_address = _de.read_bare::<crate::ton::accountaddress::AccountAddress>()?;
            Ok(Self { account_address })
        }
    }
}
impl crate::BoxedDeserialize for Load {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xca25d03f)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0xca25d03f) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for Load {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0xca25d03f), self)
    }
}
impl crate::Function for Load {
    type Reply = crate::ton::smc::Info;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `smc.runGetMethod`\n\n```text\nsmc.runGetMethod id:int53 method:smc.MethodId stack:vector<tvm.StackEntry> = smc.RunResult;\n```\n"]
pub struct RunGetMethod {
    pub id: crate::ton::int53,
    pub method: crate::ton::smc::MethodId,
    pub stack: crate::ton::vector<crate::ton::Boxed, crate::ton::tvm::StackEntry>,
}
impl Eq for RunGetMethod {}
impl crate::BareSerialize for RunGetMethod {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xf0c905aa)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let RunGetMethod { id, method, stack } = self;
        _ser.write_bare::<crate::ton::int53>(id)?;
        _ser.write_boxed::<crate::ton::smc::MethodId>(method)?;
        _ser.write_bare::<crate::ton::vector<crate::ton::Boxed, crate::ton::tvm::StackEntry>>(
            stack,
        )?;
        Ok(())
    }
}
impl crate::BareDeserialize for RunGetMethod {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_bare::<crate::ton::int53>()?;
            let method = _de.read_boxed::<crate::ton::smc::MethodId>()?;
            let stack = _de
                .read_bare::<crate::ton::vector<crate::ton::Boxed, crate::ton::tvm::StackEntry>>(
                )?;
            Ok(Self { id, method, stack })
        }
    }
}
impl crate::BoxedDeserialize for RunGetMethod {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xf0c905aa)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0xf0c905aa) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for RunGetMethod {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0xf0c905aa), self)
    }
}
impl crate::Function for RunGetMethod {
    type Reply = crate::ton::smc::RunResult;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `smc.runTvm`\n\n```text\nsmc.runTvm mode:# account_address:accountAddress stack:vector<tvm.StackEntry> = smc.RunTvmResult;\n```\n"]
pub struct RunTvm {
    pub mode: crate::ton::int,
    pub account_address: crate::ton::accountaddress::AccountAddress,
    pub stack: crate::ton::vector<crate::ton::Boxed, crate::ton::tvm::StackEntry>,
}
impl Eq for RunTvm {}
impl crate::BareSerialize for RunTvm {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xa83be941)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let RunTvm { mode, account_address, stack } = self;
        _ser.write_bare::<crate::ton::int>(mode)?;
        _ser.write_bare::<crate::ton::accountaddress::AccountAddress>(account_address)?;
        _ser.write_bare::<crate::ton::vector<crate::ton::Boxed, crate::ton::tvm::StackEntry>>(
            stack,
        )?;
        Ok(())
    }
}
impl crate::BareDeserialize for RunTvm {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let mode = _de.read_bare::<crate::ton::int>()?;
            let account_address = _de.read_bare::<crate::ton::accountaddress::AccountAddress>()?;
            let stack = _de
                .read_bare::<crate::ton::vector<crate::ton::Boxed, crate::ton::tvm::StackEntry>>(
                )?;
            Ok(Self { mode, account_address, stack })
        }
    }
}
impl crate::BoxedDeserialize for RunTvm {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xa83be941)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0xa83be941) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for RunTvm {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0xa83be941), self)
    }
}
impl crate::Function for RunTvm {
    type Reply = crate::ton::smc::RunTvmResult;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `smc.runTvmByBlock`\n\n```text\nsmc.runTvmByBlock mode:# account_id:int256 block_root_hash:int256 stack:vector<tvm.StackEntry> = smc.RunTvmResult;\n```\n"]
pub struct RunTvmByBlock {
    pub mode: crate::ton::int,
    pub account_id: crate::ton::int256,
    pub block_root_hash: crate::ton::int256,
    pub stack: crate::ton::vector<crate::ton::Boxed, crate::ton::tvm::StackEntry>,
}
impl Eq for RunTvmByBlock {}
impl crate::BareSerialize for RunTvmByBlock {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x607c4db1)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let RunTvmByBlock { mode, account_id, block_root_hash, stack } = self;
        _ser.write_bare::<crate::ton::int>(mode)?;
        _ser.write_bare::<crate::ton::int256>(account_id)?;
        _ser.write_bare::<crate::ton::int256>(block_root_hash)?;
        _ser.write_bare::<crate::ton::vector<crate::ton::Boxed, crate::ton::tvm::StackEntry>>(
            stack,
        )?;
        Ok(())
    }
}
impl crate::BareDeserialize for RunTvmByBlock {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let mode = _de.read_bare::<crate::ton::int>()?;
            let account_id = _de.read_bare::<crate::ton::int256>()?;
            let block_root_hash = _de.read_bare::<crate::ton::int256>()?;
            let stack = _de
                .read_bare::<crate::ton::vector<crate::ton::Boxed, crate::ton::tvm::StackEntry>>(
                )?;
            Ok(Self { mode, account_id, block_root_hash, stack })
        }
    }
}
impl crate::BoxedDeserialize for RunTvmByBlock {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x607c4db1)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x607c4db1) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for RunTvmByBlock {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x607c4db1), self)
    }
}
impl crate::Function for RunTvmByBlock {
    type Reply = crate::ton::smc::RunTvmResult;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `smc.runTvmMsg`\n\n```text\nsmc.runTvmMsg mode:# message:bytes = smc.RunTvmResult;\n```\n"]
pub struct RunTvmMsg {
    pub mode: crate::ton::int,
    pub message: crate::ton::bytes,
}
impl Eq for RunTvmMsg {}
impl crate::BareSerialize for RunTvmMsg {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xef831db1)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let RunTvmMsg { mode, message } = self;
        _ser.write_bare::<crate::ton::int>(mode)?;
        _ser.write_bare::<crate::ton::bytes>(message)?;
        Ok(())
    }
}
impl crate::BareDeserialize for RunTvmMsg {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let mode = _de.read_bare::<crate::ton::int>()?;
            let message = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { mode, message })
        }
    }
}
impl crate::BoxedDeserialize for RunTvmMsg {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xef831db1)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0xef831db1) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for RunTvmMsg {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0xef831db1), self)
    }
}
impl crate::Function for RunTvmMsg {
    type Reply = crate::ton::smc::RunTvmResult;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `smc.runTvmMsgByBlock`\n\n```text\nsmc.runTvmMsgByBlock mode:# block_root_hash:int256 message:bytes = smc.RunTvmResult;\n```\n"]
pub struct RunTvmMsgByBlock {
    pub mode: crate::ton::int,
    pub block_root_hash: crate::ton::int256,
    pub message: crate::ton::bytes,
}
impl Eq for RunTvmMsgByBlock {}
impl crate::BareSerialize for RunTvmMsgByBlock {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x03758f4e)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let RunTvmMsgByBlock { mode, block_root_hash, message } = self;
        _ser.write_bare::<crate::ton::int>(mode)?;
        _ser.write_bare::<crate::ton::int256>(block_root_hash)?;
        _ser.write_bare::<crate::ton::bytes>(message)?;
        Ok(())
    }
}
impl crate::BareDeserialize for RunTvmMsgByBlock {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let mode = _de.read_bare::<crate::ton::int>()?;
            let block_root_hash = _de.read_bare::<crate::ton::int256>()?;
            let message = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { mode, block_root_hash, message })
        }
    }
}
impl crate::BoxedDeserialize for RunTvmMsgByBlock {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x03758f4e)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x03758f4e) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for RunTvmMsgByBlock {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x03758f4e), self)
    }
}
impl crate::Function for RunTvmMsgByBlock {
    type Reply = crate::ton::smc::RunTvmResult;
}
