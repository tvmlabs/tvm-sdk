use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `unpackedAccountAddress`\n\n```text\nunpackedAccountAddress workchain_id:int32 bounceable:Bool testnet:Bool addr:bytes = UnpackedAccountAddress;\n```\n"]
pub struct UnpackedAccountAddress {
    pub workchain_id: crate::ton::int32,
    pub bounceable: crate::ton::Bool,
    pub testnet: crate::ton::Bool,
    pub addr: crate::ton::bytes,
}
impl Eq for UnpackedAccountAddress {}
impl crate::BareSerialize for UnpackedAccountAddress {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x70d41436)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let UnpackedAccountAddress { workchain_id, bounceable, testnet, addr } = self;
        _ser.write_bare::<crate::ton::int32>(workchain_id)?;
        _ser.write_boxed::<crate::ton::Bool>(bounceable)?;
        _ser.write_boxed::<crate::ton::Bool>(testnet)?;
        _ser.write_bare::<crate::ton::bytes>(addr)?;
        Ok(())
    }
}
impl crate::BareDeserialize for UnpackedAccountAddress {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let workchain_id = _de.read_bare::<crate::ton::int32>()?;
            let bounceable = _de.read_boxed::<crate::ton::Bool>()?;
            let testnet = _de.read_boxed::<crate::ton::Bool>()?;
            let addr = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { workchain_id, bounceable, testnet, addr })
        }
    }
}
impl crate::IntoBoxed for UnpackedAccountAddress {
    type Boxed = crate::ton::UnpackedAccountAddress;

    fn into_boxed(self) -> crate::ton::UnpackedAccountAddress {
        crate::ton::UnpackedAccountAddress::UnpackedAccountAddress(self)
    }
}
