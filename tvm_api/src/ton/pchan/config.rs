use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `pchan.config`\n\n```text\npchan.config alice_public_key:string alice_address:accountAddress bob_public_key:string bob_address:accountAddress init_timeout:int32 close_timeout:int32 channel_id:int64 = pchan.Config;\n```\n"]
pub struct Config {
    pub alice_public_key: crate::ton::string,
    pub alice_address: crate::ton::accountaddress::AccountAddress,
    pub bob_public_key: crate::ton::string,
    pub bob_address: crate::ton::accountaddress::AccountAddress,
    pub init_timeout: crate::ton::int32,
    pub close_timeout: crate::ton::int32,
    pub channel_id: crate::ton::int64,
}
impl Eq for Config {}
impl crate::BareSerialize for Config {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x8486f436)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Config {
            alice_public_key,
            alice_address,
            bob_public_key,
            bob_address,
            init_timeout,
            close_timeout,
            channel_id,
        } = self;
        _ser.write_bare::<crate::ton::string>(alice_public_key)?;
        _ser.write_bare::<crate::ton::accountaddress::AccountAddress>(alice_address)?;
        _ser.write_bare::<crate::ton::string>(bob_public_key)?;
        _ser.write_bare::<crate::ton::accountaddress::AccountAddress>(bob_address)?;
        _ser.write_bare::<crate::ton::int32>(init_timeout)?;
        _ser.write_bare::<crate::ton::int32>(close_timeout)?;
        _ser.write_bare::<crate::ton::int64>(channel_id)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Config {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let alice_public_key = _de.read_bare::<crate::ton::string>()?;
            let alice_address = _de.read_bare::<crate::ton::accountaddress::AccountAddress>()?;
            let bob_public_key = _de.read_bare::<crate::ton::string>()?;
            let bob_address = _de.read_bare::<crate::ton::accountaddress::AccountAddress>()?;
            let init_timeout = _de.read_bare::<crate::ton::int32>()?;
            let close_timeout = _de.read_bare::<crate::ton::int32>()?;
            let channel_id = _de.read_bare::<crate::ton::int64>()?;
            Ok(Self {
                alice_public_key,
                alice_address,
                bob_public_key,
                bob_address,
                init_timeout,
                close_timeout,
                channel_id,
            })
        }
    }
}
impl crate::IntoBoxed for Config {
    type Boxed = crate::ton::pchan::Config;
    fn into_boxed(self) -> crate::ton::pchan::Config {
        crate::ton::pchan::Config::Pchan_Config(self)
    }
}
