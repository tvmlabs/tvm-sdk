use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `catchain.block.dep`\n\n```text\ncatchain.block.dep src:int height:int data_hash:int256 signature:bytes = catchain.block.Dep;\n```\n"]
pub struct Dep {
    pub src: crate::ton::int,
    pub height: crate::ton::int,
    pub data_hash: crate::ton::int256,
    pub signature: crate::ton::bytes,
}
impl Eq for Dep {}
impl crate::Signing for Dep {
    fn signature_mut(&mut self) -> &mut crate::ton::bytes {
        &mut self.signature
    }
}
impl crate::BareSerialize for Dep {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x5a1ad14f)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Dep { src, height, data_hash, signature } = self;
        _ser.write_bare::<crate::ton::int>(src)?;
        _ser.write_bare::<crate::ton::int>(height)?;
        _ser.write_bare::<crate::ton::int256>(data_hash)?;
        _ser.write_bare::<crate::ton::bytes>(signature)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Dep {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let src = _de.read_bare::<crate::ton::int>()?;
            let height = _de.read_bare::<crate::ton::int>()?;
            let data_hash = _de.read_bare::<crate::ton::int256>()?;
            let signature = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { src, height, data_hash, signature })
        }
    }
}
impl crate::IntoBoxed for Dep {
    type Boxed = crate::ton::catchain::block::Dep;

    fn into_boxed(self) -> crate::ton::catchain::block::Dep {
        crate::ton::catchain::block::Dep::Catchain_Block_Dep(self)
    }
}
