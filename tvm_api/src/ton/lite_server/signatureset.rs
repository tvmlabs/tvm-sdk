use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `liteServer.signatureSet`\n\n```text\nliteServer.signatureSet validator_set_hash:int catchain_seqno:int signatures:(vector liteServer.signature) = liteServer.SignatureSet;\n```\n"]
pub struct SignatureSet {
    pub validator_set_hash: crate::ton::int,
    pub catchain_seqno: crate::ton::int,
    pub signatures:
        crate::ton::vector<crate::ton::Bare, crate::ton::lite_server::signature::Signature>,
}
impl Eq for SignatureSet {}
impl crate::BareSerialize for SignatureSet {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xf644a6e6)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let SignatureSet { validator_set_hash, catchain_seqno, signatures } = self;
        _ser.write_bare::<crate::ton::int>(validator_set_hash)?;
        _ser.write_bare::<crate::ton::int>(catchain_seqno)?;
        _ser . write_bare :: < crate :: ton :: vector < crate :: ton :: Bare , crate :: ton :: lite_server :: signature :: Signature > > (signatures) ? ;
        Ok(())
    }
}
impl crate::BareDeserialize for SignatureSet {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let validator_set_hash = _de.read_bare::<crate::ton::int>()?;
            let catchain_seqno = _de.read_bare::<crate::ton::int>()?;
            let signatures = _de.read_bare::<crate::ton::vector<
                crate::ton::Bare,
                crate::ton::lite_server::signature::Signature,
            >>()?;
            Ok(Self { validator_set_hash, catchain_seqno, signatures })
        }
    }
}
impl crate::IntoBoxed for SignatureSet {
    type Boxed = crate::ton::lite_server::SignatureSet;

    fn into_boxed(self) -> crate::ton::lite_server::SignatureSet {
        crate::ton::lite_server::SignatureSet::LiteServer_SignatureSet(self)
    }
}
