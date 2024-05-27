use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `catchain.block`\n\n```text\ncatchain.block incarnation:int256 src:int height:int data:catchain.block.data signature:bytes = catchain.Block;\n```\n"]
pub struct Block {
    pub incarnation: crate::ton::int256,
    pub src: crate::ton::int,
    pub height: crate::ton::int,
    pub data: crate::ton::catchain::block::data::Data,
    pub signature: crate::ton::bytes,
}
impl Eq for Block {}
impl crate::Signing for Block {
    fn signature_mut(&mut self) -> &mut crate::ton::bytes {
        &mut self.signature
    }
}
impl crate::BareSerialize for Block {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xd6554174)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Block { incarnation, src, height, data, signature } = self;
        _ser.write_bare::<crate::ton::int256>(incarnation)?;
        _ser.write_bare::<crate::ton::int>(src)?;
        _ser.write_bare::<crate::ton::int>(height)?;
        _ser.write_bare::<crate::ton::catchain::block::data::Data>(data)?;
        _ser.write_bare::<crate::ton::bytes>(signature)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Block {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let incarnation = _de.read_bare::<crate::ton::int256>()?;
            let src = _de.read_bare::<crate::ton::int>()?;
            let height = _de.read_bare::<crate::ton::int>()?;
            let data = _de.read_bare::<crate::ton::catchain::block::data::Data>()?;
            let signature = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { incarnation, src, height, data, signature })
        }
    }
}
impl crate::IntoBoxed for Block {
    type Boxed = crate::ton::catchain::Block;

    fn into_boxed(self) -> crate::ton::catchain::Block {
        crate::ton::catchain::Block::Catchain_Block(self)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `catchain.block.Data`\n\n```text\ncatchain.block.data prev:catchain.block.dep deps:(vector catchain.block.dep) = catchain.block.Data;\n```\n"]
pub enum Data {
    Catchain_Block_Data(crate::ton::catchain::block::data::Data),
}
impl Data {
    pub fn deps(
        &self,
    ) -> &crate::ton::vector<crate::ton::Bare, crate::ton::catchain::block::dep::Dep> {
        match self {
            Data::Catchain_Block_Data(ref x) => &x.deps,
        }
    }

    pub fn prev(&self) -> &crate::ton::catchain::block::dep::Dep {
        match self {
            Data::Catchain_Block_Data(ref x) => &x.prev,
        }
    }

    pub fn only(self) -> crate::ton::catchain::block::data::Data {
        match self {
            Data::Catchain_Block_Data(x) => x,
        }
    }
}
impl Eq for Data {}
impl Default for Data {
    fn default() -> Self {
        Data::Catchain_Block_Data(crate::ton::catchain::block::data::Data::default())
    }
}
impl crate::BoxedSerialize for Data {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Data::Catchain_Block_Data(x) => (crate::ConstructorNumber(0xf8aca620), x),
        }
    }
}
impl crate::BoxedDeserialize for Data {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xf8aca620)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xf8aca620) => Ok(Data::Catchain_Block_Data(
                _de.read_bare::<crate::ton::catchain::block::data::Data>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `catchain.block.Dep`\n\n```text\ncatchain.block.dep src:int height:int data_hash:int256 signature:bytes = catchain.block.Dep;\n```\n"]
pub enum Dep {
    Catchain_Block_Dep(crate::ton::catchain::block::dep::Dep),
}
impl Dep {
    pub fn data_hash(&self) -> &crate::ton::int256 {
        match self {
            Dep::Catchain_Block_Dep(ref x) => &x.data_hash,
        }
    }

    pub fn height(&self) -> &crate::ton::int {
        match self {
            Dep::Catchain_Block_Dep(ref x) => &x.height,
        }
    }

    pub fn signature(&self) -> &crate::ton::bytes {
        match self {
            Dep::Catchain_Block_Dep(ref x) => &x.signature,
        }
    }

    pub fn src(&self) -> &crate::ton::int {
        match self {
            Dep::Catchain_Block_Dep(ref x) => &x.src,
        }
    }

    pub fn only(self) -> crate::ton::catchain::block::dep::Dep {
        match self {
            Dep::Catchain_Block_Dep(x) => x,
        }
    }
}
impl Eq for Dep {}
impl Default for Dep {
    fn default() -> Self {
        Dep::Catchain_Block_Dep(crate::ton::catchain::block::dep::Dep::default())
    }
}
impl crate::BoxedSerialize for Dep {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Dep::Catchain_Block_Dep(x) => (crate::ConstructorNumber(0x5a1ad14f), x),
        }
    }
}
impl crate::BoxedDeserialize for Dep {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x5a1ad14f)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x5a1ad14f) => Ok(Dep::Catchain_Block_Dep(
                _de.read_bare::<crate::ton::catchain::block::dep::Dep>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `catchain.block.Id`\n\n```text\ncatchain.block.id incarnation:int256 src:int256 height:int data_hash:int256 = catchain.block.Id;\n```\n"]
pub enum Id {
    Catchain_Block_Id(crate::ton::catchain::block::id::Id),
}
impl Id {
    pub fn data_hash(&self) -> &crate::ton::int256 {
        match self {
            Id::Catchain_Block_Id(ref x) => &x.data_hash,
        }
    }

    pub fn height(&self) -> &crate::ton::int {
        match self {
            Id::Catchain_Block_Id(ref x) => &x.height,
        }
    }

    pub fn incarnation(&self) -> &crate::ton::int256 {
        match self {
            Id::Catchain_Block_Id(ref x) => &x.incarnation,
        }
    }

    pub fn src(&self) -> &crate::ton::int256 {
        match self {
            Id::Catchain_Block_Id(ref x) => &x.src,
        }
    }

    pub fn only(self) -> crate::ton::catchain::block::id::Id {
        match self {
            Id::Catchain_Block_Id(x) => x,
        }
    }
}
impl Eq for Id {}
impl Default for Id {
    fn default() -> Self {
        Id::Catchain_Block_Id(crate::ton::catchain::block::id::Id::default())
    }
}
impl crate::BoxedSerialize for Id {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Id::Catchain_Block_Id(x) => (crate::ConstructorNumber(0x24fe98ba), x),
        }
    }
}
impl crate::BoxedDeserialize for Id {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x24fe98ba)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x24fe98ba) => {
                Ok(Id::Catchain_Block_Id(_de.read_bare::<crate::ton::catchain::block::id::Id>()?))
            }
            id => _invalid_id!(id),
        }
    }
}
pub mod data;
pub mod dep;
pub mod id;
pub mod inner;
