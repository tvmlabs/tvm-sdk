use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `fec.online`\n\n```text\nfec.online data_size:int symbol_size:int symbols_count:int = fec.Type;\n```\n"]
pub struct Online {
    pub data_size: crate::ton::int,
    pub symbol_size: crate::ton::int,
    pub symbols_count: crate::ton::int,
}
impl Eq for Online {}
impl crate::BareSerialize for Online {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x0127660c)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Online { data_size, symbol_size, symbols_count } = self;
        _ser.write_bare::<crate::ton::int>(data_size)?;
        _ser.write_bare::<crate::ton::int>(symbol_size)?;
        _ser.write_bare::<crate::ton::int>(symbols_count)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Online {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let data_size = _de.read_bare::<crate::ton::int>()?;
            let symbol_size = _de.read_bare::<crate::ton::int>()?;
            let symbols_count = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { data_size, symbol_size, symbols_count })
        }
    }
}
impl crate::IntoBoxed for Online {
    type Boxed = crate::ton::fec::Type;

    fn into_boxed(self) -> crate::ton::fec::Type {
        crate::ton::fec::Type::Fec_Online(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `fec.raptorQ`\n\n```text\nfec.raptorQ data_size:int symbol_size:int symbols_count:int = fec.Type;\n```\n"]
pub struct RaptorQ {
    pub data_size: crate::ton::int,
    pub symbol_size: crate::ton::int,
    pub symbols_count: crate::ton::int,
}
impl Eq for RaptorQ {}
impl crate::BareSerialize for RaptorQ {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x8b93a7e0)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let RaptorQ { data_size, symbol_size, symbols_count } = self;
        _ser.write_bare::<crate::ton::int>(data_size)?;
        _ser.write_bare::<crate::ton::int>(symbol_size)?;
        _ser.write_bare::<crate::ton::int>(symbols_count)?;
        Ok(())
    }
}
impl crate::BareDeserialize for RaptorQ {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let data_size = _de.read_bare::<crate::ton::int>()?;
            let symbol_size = _de.read_bare::<crate::ton::int>()?;
            let symbols_count = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { data_size, symbol_size, symbols_count })
        }
    }
}
impl crate::IntoBoxed for RaptorQ {
    type Boxed = crate::ton::fec::Type;

    fn into_boxed(self) -> crate::ton::fec::Type {
        crate::ton::fec::Type::Fec_RaptorQ(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `fec.roundRobin`\n\n```text\nfec.roundRobin data_size:int symbol_size:int symbols_count:int = fec.Type;\n```\n"]
pub struct RoundRobin {
    pub data_size: crate::ton::int,
    pub symbol_size: crate::ton::int,
    pub symbols_count: crate::ton::int,
}
impl Eq for RoundRobin {}
impl crate::BareSerialize for RoundRobin {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x32f528e4)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let RoundRobin { data_size, symbol_size, symbols_count } = self;
        _ser.write_bare::<crate::ton::int>(data_size)?;
        _ser.write_bare::<crate::ton::int>(symbol_size)?;
        _ser.write_bare::<crate::ton::int>(symbols_count)?;
        Ok(())
    }
}
impl crate::BareDeserialize for RoundRobin {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let data_size = _de.read_bare::<crate::ton::int>()?;
            let symbol_size = _de.read_bare::<crate::ton::int>()?;
            let symbols_count = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { data_size, symbol_size, symbols_count })
        }
    }
}
impl crate::IntoBoxed for RoundRobin {
    type Boxed = crate::ton::fec::Type;

    fn into_boxed(self) -> crate::ton::fec::Type {
        crate::ton::fec::Type::Fec_RoundRobin(self)
    }
}
