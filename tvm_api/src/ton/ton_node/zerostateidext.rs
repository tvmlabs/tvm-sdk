use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.zeroStateIdExt`\n\n```text\ntonNode.zeroStateIdExt workchain:int root_hash:int256 file_hash:int256 = tonNode.ZeroStateIdExt;\n```\n"]
pub struct ZeroStateIdExt {
    pub workchain: crate::ton::int,
    pub root_hash: crate::ton::int256,
    pub file_hash: crate::ton::int256,
}
impl Eq for ZeroStateIdExt {}
impl crate::BareSerialize for ZeroStateIdExt {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x1d7235ae)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let ZeroStateIdExt { workchain, root_hash, file_hash } = self;
        _ser.write_bare::<crate::ton::int>(workchain)?;
        _ser.write_bare::<crate::ton::int256>(root_hash)?;
        _ser.write_bare::<crate::ton::int256>(file_hash)?;
        Ok(())
    }
}
impl crate::BareDeserialize for ZeroStateIdExt {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let workchain = _de.read_bare::<crate::ton::int>()?;
            let root_hash = _de.read_bare::<crate::ton::int256>()?;
            let file_hash = _de.read_bare::<crate::ton::int256>()?;
            Ok(Self { workchain, root_hash, file_hash })
        }
    }
}
impl crate::IntoBoxed for ZeroStateIdExt {
    type Boxed = crate::ton::ton_node::ZeroStateIdExt;

    fn into_boxed(self) -> crate::ton::ton_node::ZeroStateIdExt {
        crate::ton::ton_node::ZeroStateIdExt::TonNode_ZeroStateIdExt(self)
    }
}
