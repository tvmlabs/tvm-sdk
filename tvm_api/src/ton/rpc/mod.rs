use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `addLogMessage`\n\n```text\naddLogMessage verbosity_level:int32 text:string = Ok;\n```\n"]
pub struct AddLogMessage {
    pub verbosity_level: crate::ton::int32,
    pub text: crate::ton::string,
}
impl Eq for AddLogMessage {}
impl crate::BareSerialize for AddLogMessage {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x5f36cfec)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let AddLogMessage { verbosity_level, text } = self;
        _ser.write_bare::<crate::ton::int32>(verbosity_level)?;
        _ser.write_bare::<crate::ton::string>(text)?;
        Ok(())
    }
}
impl crate::BareDeserialize for AddLogMessage {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let verbosity_level = _de.read_bare::<crate::ton::int32>()?;
            let text = _de.read_bare::<crate::ton::string>()?;
            Ok(Self { verbosity_level, text })
        }
    }
}
impl crate::BoxedDeserialize for AddLogMessage {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x5f36cfec)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x5f36cfec) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for AddLogMessage {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x5f36cfec), self)
    }
}
impl crate::Function for AddLogMessage {
    type Reply = crate::ton::Ok;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `changeLocalPassword`\n\n```text\nchangeLocalPassword input_key:InputKey new_local_password:secureBytes = Key;\n```\n"]
pub struct ChangeLocalPassword {
    pub input_key: crate::ton::InputKey,
    pub new_local_password: crate::ton::secureBytes,
}
impl Eq for ChangeLocalPassword {}
impl crate::BareSerialize for ChangeLocalPassword {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xe81037bf)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let ChangeLocalPassword { input_key, new_local_password } = self;
        _ser.write_boxed::<crate::ton::InputKey>(input_key)?;
        _ser.write_bare::<crate::ton::secureBytes>(new_local_password)?;
        Ok(())
    }
}
impl crate::BareDeserialize for ChangeLocalPassword {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let input_key = _de.read_boxed::<crate::ton::InputKey>()?;
            let new_local_password = _de.read_bare::<crate::ton::secureBytes>()?;
            Ok(Self { input_key, new_local_password })
        }
    }
}
impl crate::BoxedDeserialize for ChangeLocalPassword {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xe81037bf)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0xe81037bf) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for ChangeLocalPassword {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0xe81037bf), self)
    }
}
impl crate::Function for ChangeLocalPassword {
    type Reply = crate::ton::Key;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `close`\n\n```text\nclose = Ok;\n```\n"]
pub struct Close;
impl Eq for Close {}
impl crate::BareSerialize for Close {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xb933e17f)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        Ok(())
    }
}
impl crate::BareDeserialize for Close {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        Ok(Self {})
    }
}
impl crate::BoxedDeserialize for Close {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xb933e17f)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0xb933e17f) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for Close {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0xb933e17f), self)
    }
}
impl crate::Function for Close {
    type Reply = crate::ton::Ok;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `createNewKey`\n\n```text\ncreateNewKey local_password:secureBytes mnemonic_password:secureBytes random_extra_seed:secureBytes = Key;\n```\n"]
pub struct CreateNewKey {
    pub local_password: crate::ton::secureBytes,
    pub mnemonic_password: crate::ton::secureBytes,
    pub random_extra_seed: crate::ton::secureBytes,
}
impl Eq for CreateNewKey {}
impl crate::BareSerialize for CreateNewKey {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x910d8210)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let CreateNewKey { local_password, mnemonic_password, random_extra_seed } = self;
        _ser.write_bare::<crate::ton::secureBytes>(local_password)?;
        _ser.write_bare::<crate::ton::secureBytes>(mnemonic_password)?;
        _ser.write_bare::<crate::ton::secureBytes>(random_extra_seed)?;
        Ok(())
    }
}
impl crate::BareDeserialize for CreateNewKey {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let local_password = _de.read_bare::<crate::ton::secureBytes>()?;
            let mnemonic_password = _de.read_bare::<crate::ton::secureBytes>()?;
            let random_extra_seed = _de.read_bare::<crate::ton::secureBytes>()?;
            Ok(Self { local_password, mnemonic_password, random_extra_seed })
        }
    }
}
impl crate::BoxedDeserialize for CreateNewKey {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x910d8210)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x910d8210) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for CreateNewKey {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x910d8210), self)
    }
}
impl crate::Function for CreateNewKey {
    type Reply = crate::ton::Key;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `createQuery`\n\n```text\ncreateQuery private_key:InputKey address:accountAddress timeout:int32 action:Action initial_account_state:InitialAccountState = query.Info;\n```\n"]
pub struct CreateQuery {
    pub private_key: crate::ton::InputKey,
    pub address: crate::ton::accountaddress::AccountAddress,
    pub timeout: crate::ton::int32,
    pub action: crate::ton::Action,
    pub initial_account_state: crate::ton::InitialAccountState,
}
impl Eq for CreateQuery {}
impl crate::BareSerialize for CreateQuery {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xf18b20c5)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let CreateQuery { private_key, address, timeout, action, initial_account_state } = self;
        _ser.write_boxed::<crate::ton::InputKey>(private_key)?;
        _ser.write_bare::<crate::ton::accountaddress::AccountAddress>(address)?;
        _ser.write_bare::<crate::ton::int32>(timeout)?;
        _ser.write_boxed::<crate::ton::Action>(action)?;
        _ser.write_boxed::<crate::ton::InitialAccountState>(initial_account_state)?;
        Ok(())
    }
}
impl crate::BareDeserialize for CreateQuery {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let private_key = _de.read_boxed::<crate::ton::InputKey>()?;
            let address = _de.read_bare::<crate::ton::accountaddress::AccountAddress>()?;
            let timeout = _de.read_bare::<crate::ton::int32>()?;
            let action = _de.read_boxed::<crate::ton::Action>()?;
            let initial_account_state = _de.read_boxed::<crate::ton::InitialAccountState>()?;
            Ok(Self { private_key, address, timeout, action, initial_account_state })
        }
    }
}
impl crate::BoxedDeserialize for CreateQuery {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xf18b20c5)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0xf18b20c5) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for CreateQuery {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0xf18b20c5), self)
    }
}
impl crate::Function for CreateQuery {
    type Reply = crate::ton::query::Info;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `decrypt`\n\n```text\ndecrypt encrypted_data:secureBytes secret:secureBytes = Data;\n```\n"]
pub struct Decrypt {
    pub encrypted_data: crate::ton::secureBytes,
    pub secret: crate::ton::secureBytes,
}
impl Eq for Decrypt {}
impl crate::BareSerialize for Decrypt {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x155685ae)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Decrypt { encrypted_data, secret } = self;
        _ser.write_bare::<crate::ton::secureBytes>(encrypted_data)?;
        _ser.write_bare::<crate::ton::secureBytes>(secret)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Decrypt {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let encrypted_data = _de.read_bare::<crate::ton::secureBytes>()?;
            let secret = _de.read_bare::<crate::ton::secureBytes>()?;
            Ok(Self { encrypted_data, secret })
        }
    }
}
impl crate::BoxedDeserialize for Decrypt {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x155685ae)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x155685ae) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for Decrypt {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x155685ae), self)
    }
}
impl crate::Function for Decrypt {
    type Reply = crate::ton::Data;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `deleteAllKeys`\n\n```text\ndeleteAllKeys = Ok;\n```\n"]
pub struct DeleteAllKeys;
impl Eq for DeleteAllKeys {}
impl crate::BareSerialize for DeleteAllKeys {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x5fe3fb23)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        Ok(())
    }
}
impl crate::BareDeserialize for DeleteAllKeys {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        Ok(Self {})
    }
}
impl crate::BoxedDeserialize for DeleteAllKeys {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x5fe3fb23)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x5fe3fb23) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for DeleteAllKeys {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x5fe3fb23), self)
    }
}
impl crate::Function for DeleteAllKeys {
    type Reply = crate::ton::Ok;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `deleteKey`\n\n```text\ndeleteKey key:key = Ok;\n```\n"]
pub struct DeleteKey {
    pub key: crate::ton::key::Key,
}
impl Eq for DeleteKey {}
impl crate::BareSerialize for DeleteKey {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xa1d948cd)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let DeleteKey { key } = self;
        _ser.write_bare::<crate::ton::key::Key>(key)?;
        Ok(())
    }
}
impl crate::BareDeserialize for DeleteKey {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let key = _de.read_bare::<crate::ton::key::Key>()?;
            Ok(Self { key })
        }
    }
}
impl crate::BoxedDeserialize for DeleteKey {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xa1d948cd)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0xa1d948cd) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for DeleteKey {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0xa1d948cd), self)
    }
}
impl crate::Function for DeleteKey {
    type Reply = crate::ton::Ok;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `encrypt`\n\n```text\nencrypt decrypted_data:secureBytes secret:secureBytes = Data;\n```\n"]
pub struct Encrypt {
    pub decrypted_data: crate::ton::secureBytes,
    pub secret: crate::ton::secureBytes,
}
impl Eq for Encrypt {}
impl crate::BareSerialize for Encrypt {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x936f4b1c)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Encrypt { decrypted_data, secret } = self;
        _ser.write_bare::<crate::ton::secureBytes>(decrypted_data)?;
        _ser.write_bare::<crate::ton::secureBytes>(secret)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Encrypt {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let decrypted_data = _de.read_bare::<crate::ton::secureBytes>()?;
            let secret = _de.read_bare::<crate::ton::secureBytes>()?;
            Ok(Self { decrypted_data, secret })
        }
    }
}
impl crate::BoxedDeserialize for Encrypt {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x936f4b1c)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x936f4b1c) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for Encrypt {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x936f4b1c), self)
    }
}
impl crate::Function for Encrypt {
    type Reply = crate::ton::Data;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `exportEncryptedKey`\n\n```text\nexportEncryptedKey input_key:InputKey key_password:secureBytes = ExportedEncryptedKey;\n```\n"]
pub struct ExportEncryptedKey {
    pub input_key: crate::ton::InputKey,
    pub key_password: crate::ton::secureBytes,
}
impl Eq for ExportEncryptedKey {}
impl crate::BareSerialize for ExportEncryptedKey {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x0d02097f)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let ExportEncryptedKey { input_key, key_password } = self;
        _ser.write_boxed::<crate::ton::InputKey>(input_key)?;
        _ser.write_bare::<crate::ton::secureBytes>(key_password)?;
        Ok(())
    }
}
impl crate::BareDeserialize for ExportEncryptedKey {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let input_key = _de.read_boxed::<crate::ton::InputKey>()?;
            let key_password = _de.read_bare::<crate::ton::secureBytes>()?;
            Ok(Self { input_key, key_password })
        }
    }
}
impl crate::BoxedDeserialize for ExportEncryptedKey {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x0d02097f)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x0d02097f) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for ExportEncryptedKey {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x0d02097f), self)
    }
}
impl crate::Function for ExportEncryptedKey {
    type Reply = crate::ton::ExportedEncryptedKey;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `exportKey`\n\n```text\nexportKey input_key:InputKey = ExportedKey;\n```\n"]
pub struct ExportKey {
    pub input_key: crate::ton::InputKey,
}
impl Eq for ExportKey {}
impl crate::BareSerialize for ExportKey {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x9f4cd973)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let ExportKey { input_key } = self;
        _ser.write_boxed::<crate::ton::InputKey>(input_key)?;
        Ok(())
    }
}
impl crate::BareDeserialize for ExportKey {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let input_key = _de.read_boxed::<crate::ton::InputKey>()?;
            Ok(Self { input_key })
        }
    }
}
impl crate::BoxedDeserialize for ExportKey {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x9f4cd973)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x9f4cd973) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for ExportKey {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x9f4cd973), self)
    }
}
impl crate::Function for ExportKey {
    type Reply = crate::ton::ExportedKey;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `exportPemKey`\n\n```text\nexportPemKey input_key:InputKey key_password:secureBytes = ExportedPemKey;\n```\n"]
pub struct ExportPemKey {
    pub input_key: crate::ton::InputKey,
    pub key_password: crate::ton::secureBytes,
}
impl Eq for ExportPemKey {}
impl crate::BareSerialize for ExportPemKey {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xd9a8a3ba)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let ExportPemKey { input_key, key_password } = self;
        _ser.write_boxed::<crate::ton::InputKey>(input_key)?;
        _ser.write_bare::<crate::ton::secureBytes>(key_password)?;
        Ok(())
    }
}
impl crate::BareDeserialize for ExportPemKey {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let input_key = _de.read_boxed::<crate::ton::InputKey>()?;
            let key_password = _de.read_bare::<crate::ton::secureBytes>()?;
            Ok(Self { input_key, key_password })
        }
    }
}
impl crate::BoxedDeserialize for ExportPemKey {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xd9a8a3ba)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0xd9a8a3ba) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for ExportPemKey {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0xd9a8a3ba), self)
    }
}
impl crate::Function for ExportPemKey {
    type Reply = crate::ton::ExportedPemKey;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `exportUnencryptedKey`\n\n```text\nexportUnencryptedKey input_key:InputKey = ExportedUnencryptedKey;\n```\n"]
pub struct ExportUnencryptedKey {
    pub input_key: crate::ton::InputKey,
}
impl Eq for ExportUnencryptedKey {}
impl crate::BareSerialize for ExportUnencryptedKey {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xda2bc740)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let ExportUnencryptedKey { input_key } = self;
        _ser.write_boxed::<crate::ton::InputKey>(input_key)?;
        Ok(())
    }
}
impl crate::BareDeserialize for ExportUnencryptedKey {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let input_key = _de.read_boxed::<crate::ton::InputKey>()?;
            Ok(Self { input_key })
        }
    }
}
impl crate::BoxedDeserialize for ExportUnencryptedKey {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xda2bc740)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0xda2bc740) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for ExportUnencryptedKey {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0xda2bc740), self)
    }
}
impl crate::Function for ExportUnencryptedKey {
    type Reply = crate::ton::ExportedUnencryptedKey;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `getAccountAddress`\n\n```text\ngetAccountAddress initial_account_state:InitialAccountState revision:int32 workchain_id:int32 = AccountAddress;\n```\n"]
pub struct GetAccountAddress {
    pub initial_account_state: crate::ton::InitialAccountState,
    pub revision: crate::ton::int32,
    pub workchain_id: crate::ton::int32,
}
impl Eq for GetAccountAddress {}
impl crate::BareSerialize for GetAccountAddress {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x1e8ba5c8)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let GetAccountAddress { initial_account_state, revision, workchain_id } = self;
        _ser.write_boxed::<crate::ton::InitialAccountState>(initial_account_state)?;
        _ser.write_bare::<crate::ton::int32>(revision)?;
        _ser.write_bare::<crate::ton::int32>(workchain_id)?;
        Ok(())
    }
}
impl crate::BareDeserialize for GetAccountAddress {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let initial_account_state = _de.read_boxed::<crate::ton::InitialAccountState>()?;
            let revision = _de.read_bare::<crate::ton::int32>()?;
            let workchain_id = _de.read_bare::<crate::ton::int32>()?;
            Ok(Self { initial_account_state, revision, workchain_id })
        }
    }
}
impl crate::BoxedDeserialize for GetAccountAddress {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x1e8ba5c8)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x1e8ba5c8) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for GetAccountAddress {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x1e8ba5c8), self)
    }
}
impl crate::Function for GetAccountAddress {
    type Reply = crate::ton::AccountAddress;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `getAccountState`\n\n```text\ngetAccountState account_address:accountAddress = FullAccountState;\n```\n"]
pub struct GetAccountState {
    pub account_address: crate::ton::accountaddress::AccountAddress,
}
impl Eq for GetAccountState {}
impl crate::BareSerialize for GetAccountState {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x81daf446)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let GetAccountState { account_address } = self;
        _ser.write_bare::<crate::ton::accountaddress::AccountAddress>(account_address)?;
        Ok(())
    }
}
impl crate::BareDeserialize for GetAccountState {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let account_address = _de.read_bare::<crate::ton::accountaddress::AccountAddress>()?;
            Ok(Self { account_address })
        }
    }
}
impl crate::BoxedDeserialize for GetAccountState {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x81daf446)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x81daf446) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for GetAccountState {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x81daf446), self)
    }
}
impl crate::Function for GetAccountState {
    type Reply = crate::ton::FullAccountState;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `getBip39Hints`\n\n```text\ngetBip39Hints prefix:string = Bip39Hints;\n```\n"]
pub struct GetBip39Hints {
    pub prefix: crate::ton::string,
}
impl Eq for GetBip39Hints {}
impl crate::BareSerialize for GetBip39Hints {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x8f5e5dea)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let GetBip39Hints { prefix } = self;
        _ser.write_bare::<crate::ton::string>(prefix)?;
        Ok(())
    }
}
impl crate::BareDeserialize for GetBip39Hints {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let prefix = _de.read_bare::<crate::ton::string>()?;
            Ok(Self { prefix })
        }
    }
}
impl crate::BoxedDeserialize for GetBip39Hints {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x8f5e5dea)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x8f5e5dea) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for GetBip39Hints {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x8f5e5dea), self)
    }
}
impl crate::Function for GetBip39Hints {
    type Reply = crate::ton::Bip39Hints;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `getLogStream`\n\n```text\ngetLogStream = LogStream;\n```\n"]
pub struct GetLogStream;
impl Eq for GetLogStream {}
impl crate::BareSerialize for GetLogStream {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x45984b5b)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        Ok(())
    }
}
impl crate::BareDeserialize for GetLogStream {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        Ok(Self {})
    }
}
impl crate::BoxedDeserialize for GetLogStream {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x45984b5b)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x45984b5b) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for GetLogStream {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x45984b5b), self)
    }
}
impl crate::Function for GetLogStream {
    type Reply = crate::ton::LogStream;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `getLogTagVerbosityLevel`\n\n```text\ngetLogTagVerbosityLevel tag:string = LogVerbosityLevel;\n```\n"]
pub struct GetLogTagVerbosityLevel {
    pub tag: crate::ton::string,
}
impl Eq for GetLogTagVerbosityLevel {}
impl crate::BareSerialize for GetLogTagVerbosityLevel {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x38af2d83)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let GetLogTagVerbosityLevel { tag } = self;
        _ser.write_bare::<crate::ton::string>(tag)?;
        Ok(())
    }
}
impl crate::BareDeserialize for GetLogTagVerbosityLevel {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let tag = _de.read_bare::<crate::ton::string>()?;
            Ok(Self { tag })
        }
    }
}
impl crate::BoxedDeserialize for GetLogTagVerbosityLevel {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x38af2d83)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x38af2d83) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for GetLogTagVerbosityLevel {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x38af2d83), self)
    }
}
impl crate::Function for GetLogTagVerbosityLevel {
    type Reply = crate::ton::LogVerbosityLevel;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `getLogTags`\n\n```text\ngetLogTags = LogTags;\n```\n"]
pub struct GetLogTags;
impl Eq for GetLogTags {}
impl crate::BareSerialize for GetLogTags {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xf0d569da)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        Ok(())
    }
}
impl crate::BareDeserialize for GetLogTags {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        Ok(Self {})
    }
}
impl crate::BoxedDeserialize for GetLogTags {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xf0d569da)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0xf0d569da) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for GetLogTags {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0xf0d569da), self)
    }
}
impl crate::Function for GetLogTags {
    type Reply = crate::ton::LogTags;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `getLogVerbosityLevel`\n\n```text\ngetLogVerbosityLevel = LogVerbosityLevel;\n```\n"]
pub struct GetLogVerbosityLevel;
impl Eq for GetLogVerbosityLevel {}
impl crate::BareSerialize for GetLogVerbosityLevel {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x23689ae4)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        Ok(())
    }
}
impl crate::BareDeserialize for GetLogVerbosityLevel {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        Ok(Self {})
    }
}
impl crate::BoxedDeserialize for GetLogVerbosityLevel {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x23689ae4)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x23689ae4) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for GetLogVerbosityLevel {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x23689ae4), self)
    }
}
impl crate::Function for GetLogVerbosityLevel {
    type Reply = crate::ton::LogVerbosityLevel;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `guessAccount`\n\n```text\nguessAccount public_key:string rwallet_init_public_key:string = AccountRevisionList;\n```\n"]
pub struct GuessAccount {
    pub public_key: crate::ton::string,
    pub rwallet_init_public_key: crate::ton::string,
}
impl Eq for GuessAccount {}
impl crate::BareSerialize for GuessAccount {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x986d6c60)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let GuessAccount { public_key, rwallet_init_public_key } = self;
        _ser.write_bare::<crate::ton::string>(public_key)?;
        _ser.write_bare::<crate::ton::string>(rwallet_init_public_key)?;
        Ok(())
    }
}
impl crate::BareDeserialize for GuessAccount {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let public_key = _de.read_bare::<crate::ton::string>()?;
            let rwallet_init_public_key = _de.read_bare::<crate::ton::string>()?;
            Ok(Self { public_key, rwallet_init_public_key })
        }
    }
}
impl crate::BoxedDeserialize for GuessAccount {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x986d6c60)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x986d6c60) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for GuessAccount {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x986d6c60), self)
    }
}
impl crate::Function for GuessAccount {
    type Reply = crate::ton::AccountRevisionList;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `guessAccountRevision`\n\n```text\nguessAccountRevision initial_account_state:InitialAccountState workchain_id:int32 = AccountRevisionList;\n```\n"]
pub struct GuessAccountRevision {
    pub initial_account_state: crate::ton::InitialAccountState,
    pub workchain_id: crate::ton::int32,
}
impl Eq for GuessAccountRevision {}
impl crate::BareSerialize for GuessAccountRevision {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x6eb892a2)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let GuessAccountRevision { initial_account_state, workchain_id } = self;
        _ser.write_boxed::<crate::ton::InitialAccountState>(initial_account_state)?;
        _ser.write_bare::<crate::ton::int32>(workchain_id)?;
        Ok(())
    }
}
impl crate::BareDeserialize for GuessAccountRevision {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let initial_account_state = _de.read_boxed::<crate::ton::InitialAccountState>()?;
            let workchain_id = _de.read_bare::<crate::ton::int32>()?;
            Ok(Self { initial_account_state, workchain_id })
        }
    }
}
impl crate::BoxedDeserialize for GuessAccountRevision {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x6eb892a2)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x6eb892a2) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for GuessAccountRevision {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x6eb892a2), self)
    }
}
impl crate::Function for GuessAccountRevision {
    type Reply = crate::ton::AccountRevisionList;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `importEncryptedKey`\n\n```text\nimportEncryptedKey local_password:secureBytes key_password:secureBytes exported_encrypted_key:exportedEncryptedKey = Key;\n```\n"]
pub struct ImportEncryptedKey {
    pub local_password: crate::ton::secureBytes,
    pub key_password: crate::ton::secureBytes,
    pub exported_encrypted_key: crate::ton::exportedencryptedkey::ExportedEncryptedKey,
}
impl Eq for ImportEncryptedKey {}
impl crate::BareSerialize for ImportEncryptedKey {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x2724d3de)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let ImportEncryptedKey { local_password, key_password, exported_encrypted_key } = self;
        _ser.write_bare::<crate::ton::secureBytes>(local_password)?;
        _ser.write_bare::<crate::ton::secureBytes>(key_password)?;
        _ser.write_bare::<crate::ton::exportedencryptedkey::ExportedEncryptedKey>(
            exported_encrypted_key,
        )?;
        Ok(())
    }
}
impl crate::BareDeserialize for ImportEncryptedKey {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let local_password = _de.read_bare::<crate::ton::secureBytes>()?;
            let key_password = _de.read_bare::<crate::ton::secureBytes>()?;
            let exported_encrypted_key =
                _de.read_bare::<crate::ton::exportedencryptedkey::ExportedEncryptedKey>()?;
            Ok(Self { local_password, key_password, exported_encrypted_key })
        }
    }
}
impl crate::BoxedDeserialize for ImportEncryptedKey {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x2724d3de)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x2724d3de) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for ImportEncryptedKey {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x2724d3de), self)
    }
}
impl crate::Function for ImportEncryptedKey {
    type Reply = crate::ton::Key;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `importKey`\n\n```text\nimportKey local_password:secureBytes mnemonic_password:secureBytes exported_key:exportedKey = Key;\n```\n"]
pub struct ImportKey {
    pub local_password: crate::ton::secureBytes,
    pub mnemonic_password: crate::ton::secureBytes,
    pub exported_key: crate::ton::exportedkey::ExportedKey,
}
impl Eq for ImportKey {}
impl crate::BareSerialize for ImportKey {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xa0296119)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let ImportKey { local_password, mnemonic_password, exported_key } = self;
        _ser.write_bare::<crate::ton::secureBytes>(local_password)?;
        _ser.write_bare::<crate::ton::secureBytes>(mnemonic_password)?;
        _ser.write_bare::<crate::ton::exportedkey::ExportedKey>(exported_key)?;
        Ok(())
    }
}
impl crate::BareDeserialize for ImportKey {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let local_password = _de.read_bare::<crate::ton::secureBytes>()?;
            let mnemonic_password = _de.read_bare::<crate::ton::secureBytes>()?;
            let exported_key = _de.read_bare::<crate::ton::exportedkey::ExportedKey>()?;
            Ok(Self { local_password, mnemonic_password, exported_key })
        }
    }
}
impl crate::BoxedDeserialize for ImportKey {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xa0296119)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0xa0296119) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for ImportKey {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0xa0296119), self)
    }
}
impl crate::Function for ImportKey {
    type Reply = crate::ton::Key;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `importPemKey`\n\n```text\nimportPemKey local_password:secureBytes key_password:secureBytes exported_key:exportedPemKey = Key;\n```\n"]
pub struct ImportPemKey {
    pub local_password: crate::ton::secureBytes,
    pub key_password: crate::ton::secureBytes,
    pub exported_key: crate::ton::exportedpemkey::ExportedPemKey,
}
impl Eq for ImportPemKey {}
impl crate::BareSerialize for ImportPemKey {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x048d8d51)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let ImportPemKey { local_password, key_password, exported_key } = self;
        _ser.write_bare::<crate::ton::secureBytes>(local_password)?;
        _ser.write_bare::<crate::ton::secureBytes>(key_password)?;
        _ser.write_bare::<crate::ton::exportedpemkey::ExportedPemKey>(exported_key)?;
        Ok(())
    }
}
impl crate::BareDeserialize for ImportPemKey {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let local_password = _de.read_bare::<crate::ton::secureBytes>()?;
            let key_password = _de.read_bare::<crate::ton::secureBytes>()?;
            let exported_key = _de.read_bare::<crate::ton::exportedpemkey::ExportedPemKey>()?;
            Ok(Self { local_password, key_password, exported_key })
        }
    }
}
impl crate::BoxedDeserialize for ImportPemKey {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x048d8d51)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x048d8d51) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for ImportPemKey {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x048d8d51), self)
    }
}
impl crate::Function for ImportPemKey {
    type Reply = crate::ton::Key;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `importUnencryptedKey`\n\n```text\nimportUnencryptedKey local_password:secureBytes  exported_unencrypted_key:exportedUnencryptedKey = Key;\n```\n"]
pub struct ImportUnencryptedKey {
    pub local_password: crate::ton::secureBytes,
    pub exported_unencrypted_key: crate::ton::exportedunencryptedkey::ExportedUnencryptedKey,
}
impl Eq for ImportUnencryptedKey {}
impl crate::BareSerialize for ImportUnencryptedKey {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xb9635915)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let ImportUnencryptedKey { local_password, exported_unencrypted_key } = self;
        _ser.write_bare::<crate::ton::secureBytes>(local_password)?;
        _ser.write_bare::<crate::ton::exportedunencryptedkey::ExportedUnencryptedKey>(
            exported_unencrypted_key,
        )?;
        Ok(())
    }
}
impl crate::BareDeserialize for ImportUnencryptedKey {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let local_password = _de.read_bare::<crate::ton::secureBytes>()?;
            let exported_unencrypted_key =
                _de.read_bare::<crate::ton::exportedunencryptedkey::ExportedUnencryptedKey>()?;
            Ok(Self { local_password, exported_unencrypted_key })
        }
    }
}
impl crate::BoxedDeserialize for ImportUnencryptedKey {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xb9635915)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0xb9635915) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for ImportUnencryptedKey {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0xb9635915), self)
    }
}
impl crate::Function for ImportUnencryptedKey {
    type Reply = crate::ton::Key;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `init`\n\n```text\ninit options:options = options.Info;\n```\n"]
pub struct Init {
    pub options: crate::ton::options::Options,
}
impl Eq for Init {}
impl crate::BareSerialize for Init {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xc45c22b6)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Init { options } = self;
        _ser.write_bare::<crate::ton::options::Options>(options)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Init {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let options = _de.read_bare::<crate::ton::options::Options>()?;
            Ok(Self { options })
        }
    }
}
impl crate::BoxedDeserialize for Init {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xc45c22b6)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0xc45c22b6) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for Init {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0xc45c22b6), self)
    }
}
impl crate::Function for Init {
    type Reply = crate::ton::options::Info;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `kdf`\n\n```text\nkdf password:secureBytes salt:secureBytes iterations:int32 = Data;\n```\n"]
pub struct Kdf {
    pub password: crate::ton::secureBytes,
    pub salt: crate::ton::secureBytes,
    pub iterations: crate::ton::int32,
}
impl Eq for Kdf {}
impl crate::BareSerialize for Kdf {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x9c96737d)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Kdf { password, salt, iterations } = self;
        _ser.write_bare::<crate::ton::secureBytes>(password)?;
        _ser.write_bare::<crate::ton::secureBytes>(salt)?;
        _ser.write_bare::<crate::ton::int32>(iterations)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Kdf {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let password = _de.read_bare::<crate::ton::secureBytes>()?;
            let salt = _de.read_bare::<crate::ton::secureBytes>()?;
            let iterations = _de.read_bare::<crate::ton::int32>()?;
            Ok(Self { password, salt, iterations })
        }
    }
}
impl crate::BoxedDeserialize for Kdf {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x9c96737d)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x9c96737d) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for Kdf {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x9c96737d), self)
    }
}
impl crate::Function for Kdf {
    type Reply = crate::ton::Data;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `onLiteServerQueryError`\n\n```text\nonLiteServerQueryError id:int64 error:error = Ok;\n```\n"]
pub struct OnLiteServerQueryError {
    pub id: crate::ton::int64,
    pub error: crate::ton::error::Error,
}
impl Eq for OnLiteServerQueryError {}
impl crate::BareSerialize for OnLiteServerQueryError {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xd79f46b3)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let OnLiteServerQueryError { id, error } = self;
        _ser.write_bare::<crate::ton::int64>(id)?;
        _ser.write_bare::<crate::ton::error::Error>(error)?;
        Ok(())
    }
}
impl crate::BareDeserialize for OnLiteServerQueryError {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_bare::<crate::ton::int64>()?;
            let error = _de.read_bare::<crate::ton::error::Error>()?;
            Ok(Self { id, error })
        }
    }
}
impl crate::BoxedDeserialize for OnLiteServerQueryError {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xd79f46b3)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0xd79f46b3) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for OnLiteServerQueryError {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0xd79f46b3), self)
    }
}
impl crate::Function for OnLiteServerQueryError {
    type Reply = crate::ton::Ok;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `onLiteServerQueryResult`\n\n```text\nonLiteServerQueryResult id:int64 bytes:bytes = Ok;\n```\n"]
pub struct OnLiteServerQueryResult {
    pub id: crate::ton::int64,
    pub bytes: crate::ton::bytes,
}
impl Eq for OnLiteServerQueryResult {}
impl crate::BareSerialize for OnLiteServerQueryResult {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x7a92da5e)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let OnLiteServerQueryResult { id, bytes: ref bytes_ } = self;
        _ser.write_bare::<crate::ton::int64>(id)?;
        _ser.write_bare::<crate::ton::bytes>(bytes_)?;
        Ok(())
    }
}
impl crate::BareDeserialize for OnLiteServerQueryResult {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_bare::<crate::ton::int64>()?;
            let bytes = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { id, bytes })
        }
    }
}
impl crate::BoxedDeserialize for OnLiteServerQueryResult {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x7a92da5e)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x7a92da5e) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for OnLiteServerQueryResult {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x7a92da5e), self)
    }
}
impl crate::Function for OnLiteServerQueryResult {
    type Reply = crate::ton::Ok;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `packAccountAddress`\n\n```text\npackAccountAddress account_address:unpackedAccountAddress = AccountAddress;\n```\n"]
pub struct PackAccountAddress {
    pub account_address: crate::ton::unpackedaccountaddress::UnpackedAccountAddress,
}
impl Eq for PackAccountAddress {}
impl crate::BareSerialize for PackAccountAddress {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xad3c39ec)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let PackAccountAddress { account_address } = self;
        _ser.write_bare::<crate::ton::unpackedaccountaddress::UnpackedAccountAddress>(
            account_address,
        )?;
        Ok(())
    }
}
impl crate::BareDeserialize for PackAccountAddress {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let account_address =
                _de.read_bare::<crate::ton::unpackedaccountaddress::UnpackedAccountAddress>()?;
            Ok(Self { account_address })
        }
    }
}
impl crate::BoxedDeserialize for PackAccountAddress {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xad3c39ec)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0xad3c39ec) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for PackAccountAddress {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0xad3c39ec), self)
    }
}
impl crate::Function for PackAccountAddress {
    type Reply = crate::ton::AccountAddress;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `runTests`\n\n```text\nrunTests dir:string = Ok;\n```\n"]
pub struct RunTests {
    pub dir: crate::ton::string,
}
impl Eq for RunTests {}
impl crate::BareSerialize for RunTests {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x8669354d)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let RunTests { dir } = self;
        _ser.write_bare::<crate::ton::string>(dir)?;
        Ok(())
    }
}
impl crate::BareDeserialize for RunTests {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let dir = _de.read_bare::<crate::ton::string>()?;
            Ok(Self { dir })
        }
    }
}
impl crate::BoxedDeserialize for RunTests {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x8669354d)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x8669354d) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for RunTests {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x8669354d), self)
    }
}
impl crate::Function for RunTests {
    type Reply = crate::ton::Ok;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `setLogStream`\n\n```text\nsetLogStream log_stream:LogStream = Ok;\n```\n"]
pub struct SetLogStream {
    pub log_stream: crate::ton::LogStream,
}
impl Eq for SetLogStream {}
impl crate::BareSerialize for SetLogStream {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xaeaff791)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let SetLogStream { log_stream } = self;
        _ser.write_boxed::<crate::ton::LogStream>(log_stream)?;
        Ok(())
    }
}
impl crate::BareDeserialize for SetLogStream {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let log_stream = _de.read_boxed::<crate::ton::LogStream>()?;
            Ok(Self { log_stream })
        }
    }
}
impl crate::BoxedDeserialize for SetLogStream {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xaeaff791)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0xaeaff791) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for SetLogStream {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0xaeaff791), self)
    }
}
impl crate::Function for SetLogStream {
    type Reply = crate::ton::Ok;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `setLogTagVerbosityLevel`\n\n```text\nsetLogTagVerbosityLevel tag:string new_verbosity_level:int32 = Ok;\n```\n"]
pub struct SetLogTagVerbosityLevel {
    pub tag: crate::ton::string,
    pub new_verbosity_level: crate::ton::int32,
}
impl Eq for SetLogTagVerbosityLevel {}
impl crate::BareSerialize for SetLogTagVerbosityLevel {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x8317d696)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let SetLogTagVerbosityLevel { tag, new_verbosity_level } = self;
        _ser.write_bare::<crate::ton::string>(tag)?;
        _ser.write_bare::<crate::ton::int32>(new_verbosity_level)?;
        Ok(())
    }
}
impl crate::BareDeserialize for SetLogTagVerbosityLevel {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let tag = _de.read_bare::<crate::ton::string>()?;
            let new_verbosity_level = _de.read_bare::<crate::ton::int32>()?;
            Ok(Self { tag, new_verbosity_level })
        }
    }
}
impl crate::BoxedDeserialize for SetLogTagVerbosityLevel {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x8317d696)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x8317d696) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for SetLogTagVerbosityLevel {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x8317d696), self)
    }
}
impl crate::Function for SetLogTagVerbosityLevel {
    type Reply = crate::ton::Ok;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `setLogVerbosityLevel`\n\n```text\nsetLogVerbosityLevel new_verbosity_level:int32 = Ok;\n```\n"]
pub struct SetLogVerbosityLevel {
    pub new_verbosity_level: crate::ton::int32,
}
impl Eq for SetLogVerbosityLevel {}
impl crate::BareSerialize for SetLogVerbosityLevel {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xedea07d2)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let SetLogVerbosityLevel { new_verbosity_level } = self;
        _ser.write_bare::<crate::ton::int32>(new_verbosity_level)?;
        Ok(())
    }
}
impl crate::BareDeserialize for SetLogVerbosityLevel {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let new_verbosity_level = _de.read_bare::<crate::ton::int32>()?;
            Ok(Self { new_verbosity_level })
        }
    }
}
impl crate::BoxedDeserialize for SetLogVerbosityLevel {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xedea07d2)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0xedea07d2) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for SetLogVerbosityLevel {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0xedea07d2), self)
    }
}
impl crate::Function for SetLogVerbosityLevel {
    type Reply = crate::ton::Ok;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `sync`\n\n```text\nsync = tonNode.BlockIdExt;\n```\n"]
pub struct Sync;
impl Eq for Sync {}
impl crate::BareSerialize for Sync {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xf89f182b)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        Ok(())
    }
}
impl crate::BareDeserialize for Sync {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        Ok(Self {})
    }
}
impl crate::BoxedDeserialize for Sync {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xf89f182b)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0xf89f182b) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for Sync {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0xf89f182b), self)
    }
}
impl crate::Function for Sync {
    type Reply = crate::ton::ton_node::BlockIdExt;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `unpackAccountAddress`\n\n```text\nunpackAccountAddress account_address:string = UnpackedAccountAddress;\n```\n"]
pub struct UnpackAccountAddress {
    pub account_address: crate::ton::string,
}
impl Eq for UnpackAccountAddress {}
impl crate::BareSerialize for UnpackAccountAddress {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xd7528049)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let UnpackAccountAddress { account_address } = self;
        _ser.write_bare::<crate::ton::string>(account_address)?;
        Ok(())
    }
}
impl crate::BareDeserialize for UnpackAccountAddress {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let account_address = _de.read_bare::<crate::ton::string>()?;
            Ok(Self { account_address })
        }
    }
}
impl crate::BoxedDeserialize for UnpackAccountAddress {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xd7528049)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0xd7528049) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for UnpackAccountAddress {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0xd7528049), self)
    }
}
impl crate::Function for UnpackAccountAddress {
    type Reply = crate::ton::UnpackedAccountAddress;
}
pub mod adnl;
pub mod catchain;
pub mod dht;
pub mod dns;
pub mod engine;
pub mod http;
pub mod lite_server;
pub mod mbpp;
pub mod msg;
pub mod options;
pub mod overlay;
pub mod pchan;
pub mod query;
pub mod raw;
pub mod smc;
pub mod tcp;
pub mod ton_node;
pub mod validator_session;
