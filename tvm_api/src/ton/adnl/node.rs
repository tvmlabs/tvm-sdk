use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `adnl.node`\n\n```text\nadnl.node id:PublicKey addr_list:adnl.addressList = adnl.Node;\n```\n"]
pub struct Node {
    pub id: crate::ton::PublicKey,
    pub addr_list: crate::ton::adnl::addresslist::AddressList,
}
impl Eq for Node {}
impl crate::BareSerialize for Node {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x6b561285)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Node { id, addr_list } = self;
        _ser.write_boxed::<crate::ton::PublicKey>(id)?;
        _ser.write_bare::<crate::ton::adnl::addresslist::AddressList>(addr_list)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Node {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_boxed::<crate::ton::PublicKey>()?;
            let addr_list = _de.read_bare::<crate::ton::adnl::addresslist::AddressList>()?;
            Ok(Self { id, addr_list })
        }
    }
}
impl crate::IntoBoxed for Node {
    type Boxed = crate::ton::adnl::Node;
    fn into_boxed(self) -> crate::ton::adnl::Node {
        crate::ton::adnl::Node::Adnl_Node(self)
    }
}
