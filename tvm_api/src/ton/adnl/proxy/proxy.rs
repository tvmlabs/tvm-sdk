use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `adnl.proxy.fast`\n\n```text\nadnl.proxy.fast id:int256 shared_secret:bytes = adnl.Proxy;\n```\n"]
pub struct Fast {
    pub id: crate::ton::int256,
    pub shared_secret: crate::ton::bytes,
}
impl Eq for Fast {}
impl crate::BareSerialize for Fast {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x3a8b45b5)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Fast { id, shared_secret } = self;
        _ser.write_bare::<crate::ton::int256>(id)?;
        _ser.write_bare::<crate::ton::bytes>(shared_secret)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Fast {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_bare::<crate::ton::int256>()?;
            let shared_secret = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { id, shared_secret })
        }
    }
}
impl crate::IntoBoxed for Fast {
    type Boxed = crate::ton::adnl::Proxy;

    fn into_boxed(self) -> crate::ton::adnl::Proxy {
        crate::ton::adnl::Proxy::Adnl_Proxy_Fast(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `adnl.proxy.none`\n\n```text\nadnl.proxy.none id:int256 = adnl.Proxy;\n```\n"]
pub struct None {
    pub id: crate::ton::int256,
}
impl Eq for None {}
impl crate::BareSerialize for None {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x3532487b)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let None { id } = self;
        _ser.write_bare::<crate::ton::int256>(id)?;
        Ok(())
    }
}
impl crate::BareDeserialize for None {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_bare::<crate::ton::int256>()?;
            Ok(Self { id })
        }
    }
}
impl crate::IntoBoxed for None {
    type Boxed = crate::ton::adnl::Proxy;

    fn into_boxed(self) -> crate::ton::adnl::Proxy {
        crate::ton::adnl::Proxy::Adnl_Proxy_None(self)
    }
}
