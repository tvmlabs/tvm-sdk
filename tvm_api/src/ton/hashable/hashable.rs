use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `hashable.blockCandidate`\n\n```text\nhashable.blockCandidate block:int approved:int = Hashable;\n```\n"]
pub struct BlockCandidate {
    pub block: crate::ton::int,
    pub approved: crate::ton::int,
}
impl Eq for BlockCandidate {}
impl crate::BareSerialize for BlockCandidate {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x0ba9b10d)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let BlockCandidate { block, approved } = self;
        _ser.write_bare::<crate::ton::int>(block)?;
        _ser.write_bare::<crate::ton::int>(approved)?;
        Ok(())
    }
}
impl crate::BareDeserialize for BlockCandidate {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let block = _de.read_bare::<crate::ton::int>()?;
            let approved = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { block, approved })
        }
    }
}
impl crate::IntoBoxed for BlockCandidate {
    type Boxed = crate::ton::Hashable;

    fn into_boxed(self) -> crate::ton::Hashable {
        crate::ton::Hashable::Hashable_BlockCandidate(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `hashable.blockCandidateAttempt`\n\n```text\nhashable.blockCandidateAttempt block:int votes:int = Hashable;\n```\n"]
pub struct BlockCandidateAttempt {
    pub block: crate::ton::int,
    pub votes: crate::ton::int,
}
impl Eq for BlockCandidateAttempt {}
impl crate::BareSerialize for BlockCandidateAttempt {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x3f5c7d0b)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let BlockCandidateAttempt { block, votes } = self;
        _ser.write_bare::<crate::ton::int>(block)?;
        _ser.write_bare::<crate::ton::int>(votes)?;
        Ok(())
    }
}
impl crate::BareDeserialize for BlockCandidateAttempt {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let block = _de.read_bare::<crate::ton::int>()?;
            let votes = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { block, votes })
        }
    }
}
impl crate::IntoBoxed for BlockCandidateAttempt {
    type Boxed = crate::ton::Hashable;

    fn into_boxed(self) -> crate::ton::Hashable {
        crate::ton::Hashable::Hashable_BlockCandidateAttempt(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `hashable.blockSignature`\n\n```text\nhashable.blockSignature signature:int = Hashable;\n```\n"]
pub struct BlockSignature {
    pub signature: crate::ton::int,
}
impl Eq for BlockSignature {}
impl crate::BareSerialize for BlockSignature {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x37e192a2)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let BlockSignature { signature } = self;
        _ser.write_bare::<crate::ton::int>(signature)?;
        Ok(())
    }
}
impl crate::BareDeserialize for BlockSignature {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let signature = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { signature })
        }
    }
}
impl crate::IntoBoxed for BlockSignature {
    type Boxed = crate::ton::Hashable;

    fn into_boxed(self) -> crate::ton::Hashable {
        crate::ton::Hashable::Hashable_BlockSignature(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `hashable.blockVoteCandidate`\n\n```text\nhashable.blockVoteCandidate block:int approved:int = Hashable;\n```\n"]
pub struct BlockVoteCandidate {
    pub block: crate::ton::int,
    pub approved: crate::ton::int,
}
impl Eq for BlockVoteCandidate {}
impl crate::BareSerialize for BlockVoteCandidate {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xcf0d6fe5)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let BlockVoteCandidate { block, approved } = self;
        _ser.write_bare::<crate::ton::int>(block)?;
        _ser.write_bare::<crate::ton::int>(approved)?;
        Ok(())
    }
}
impl crate::BareDeserialize for BlockVoteCandidate {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let block = _de.read_bare::<crate::ton::int>()?;
            let approved = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { block, approved })
        }
    }
}
impl crate::IntoBoxed for BlockVoteCandidate {
    type Boxed = crate::ton::Hashable;

    fn into_boxed(self) -> crate::ton::Hashable {
        crate::ton::Hashable::Hashable_BlockVoteCandidate(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `hashable.bool`\n\n```text\nhashable.bool value:Bool = Hashable;\n```\n"]
pub struct Bool {
    pub value: crate::ton::Bool,
}
impl Eq for Bool {}
impl crate::BareSerialize for Bool {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xcf61441c)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Bool { value } = self;
        _ser.write_boxed::<crate::ton::Bool>(value)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Bool {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let value = _de.read_boxed::<crate::ton::Bool>()?;
            Ok(Self { value })
        }
    }
}
impl crate::IntoBoxed for Bool {
    type Boxed = crate::ton::Hashable;

    fn into_boxed(self) -> crate::ton::Hashable {
        crate::ton::Hashable::Hashable_Bool(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `hashable.bytes`\n\n```text\nhashable.bytes value:bytes = Hashable;\n```\n"]
pub struct Bytes {
    pub value: crate::ton::bytes,
}
impl Eq for Bytes {}
impl crate::BareSerialize for Bytes {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x0713de12)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Bytes { value } = self;
        _ser.write_bare::<crate::ton::bytes>(value)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Bytes {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let value = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { value })
        }
    }
}
impl crate::IntoBoxed for Bytes {
    type Boxed = crate::ton::Hashable;

    fn into_boxed(self) -> crate::ton::Hashable {
        crate::ton::Hashable::Hashable_Bytes(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `hashable.cntSortedVector`\n\n```text\nhashable.cntSortedVector data:int = Hashable;\n```\n"]
pub struct CntSortedVector {
    pub data: crate::ton::int,
}
impl Eq for CntSortedVector {}
impl crate::BareSerialize for CntSortedVector {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x7b964659)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let CntSortedVector { data } = self;
        _ser.write_bare::<crate::ton::int>(data)?;
        Ok(())
    }
}
impl crate::BareDeserialize for CntSortedVector {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let data = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { data })
        }
    }
}
impl crate::IntoBoxed for CntSortedVector {
    type Boxed = crate::ton::Hashable;

    fn into_boxed(self) -> crate::ton::Hashable {
        crate::ton::Hashable::Hashable_CntSortedVector(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `hashable.cntVector`\n\n```text\nhashable.cntVector data:int = Hashable;\n```\n"]
pub struct CntVector {
    pub data: crate::ton::int,
}
impl Eq for CntVector {}
impl crate::BareSerialize for CntVector {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x0b286f38)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let CntVector { data } = self;
        _ser.write_bare::<crate::ton::int>(data)?;
        Ok(())
    }
}
impl crate::BareDeserialize for CntVector {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let data = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { data })
        }
    }
}
impl crate::IntoBoxed for CntVector {
    type Boxed = crate::ton::Hashable;

    fn into_boxed(self) -> crate::ton::Hashable {
        crate::ton::Hashable::Hashable_CntVector(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `hashable.int256`\n\n```text\nhashable.int256 value:int256 = Hashable;\n```\n"]
pub struct Int256 {
    pub value: crate::ton::int256,
}
impl Eq for Int256 {}
impl crate::BareSerialize for Int256 {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x3a2313cf)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Int256 { value } = self;
        _ser.write_bare::<crate::ton::int256>(value)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Int256 {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let value = _de.read_bare::<crate::ton::int256>()?;
            Ok(Self { value })
        }
    }
}
impl crate::IntoBoxed for Int256 {
    type Boxed = crate::ton::Hashable;

    fn into_boxed(self) -> crate::ton::Hashable {
        crate::ton::Hashable::Hashable_Int256(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `hashable.int32`\n\n```text\nhashable.int32 value:int = Hashable;\n```\n"]
pub struct Int32 {
    pub value: crate::ton::int,
}
impl Eq for Int32 {}
impl crate::BareSerialize for Int32 {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xd3b59356)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Int32 { value } = self;
        _ser.write_bare::<crate::ton::int>(value)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Int32 {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let value = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { value })
        }
    }
}
impl crate::IntoBoxed for Int32 {
    type Boxed = crate::ton::Hashable;

    fn into_boxed(self) -> crate::ton::Hashable {
        crate::ton::Hashable::Hashable_Int32(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `hashable.int64`\n\n```text\nhashable.int64 value:long = Hashable;\n```\n"]
pub struct Int64 {
    pub value: crate::ton::long,
}
impl Eq for Int64 {}
impl crate::BareSerialize for Int64 {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xe7da8e42)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Int64 { value } = self;
        _ser.write_bare::<crate::ton::long>(value)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Int64 {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let value = _de.read_bare::<crate::ton::long>()?;
            Ok(Self { value })
        }
    }
}
impl crate::IntoBoxed for Int64 {
    type Boxed = crate::ton::Hashable;

    fn into_boxed(self) -> crate::ton::Hashable {
        crate::ton::Hashable::Hashable_Int64(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `hashable.pair`\n\n```text\nhashable.pair left:int right:int = Hashable;\n```\n"]
pub struct Pair {
    pub left: crate::ton::int,
    pub right: crate::ton::int,
}
impl Eq for Pair {}
impl crate::BareSerialize for Pair {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xc7e56895)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Pair { left, right } = self;
        _ser.write_bare::<crate::ton::int>(left)?;
        _ser.write_bare::<crate::ton::int>(right)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Pair {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let left = _de.read_bare::<crate::ton::int>()?;
            let right = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { left, right })
        }
    }
}
impl crate::IntoBoxed for Pair {
    type Boxed = crate::ton::Hashable;

    fn into_boxed(self) -> crate::ton::Hashable {
        crate::ton::Hashable::Hashable_Pair(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `hashable.sentBlock`\n\n```text\nhashable.sentBlock src:int root_hash:int file_hash:int collated_data_file_hash:int = Hashable;\n```\n"]
pub struct SentBlock {
    pub src: crate::ton::int,
    pub root_hash: crate::ton::int,
    pub file_hash: crate::ton::int,
    pub collated_data_file_hash: crate::ton::int,
}
impl Eq for SentBlock {}
impl crate::BareSerialize for SentBlock {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xbdb9952b)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let SentBlock { src, root_hash, file_hash, collated_data_file_hash } = self;
        _ser.write_bare::<crate::ton::int>(src)?;
        _ser.write_bare::<crate::ton::int>(root_hash)?;
        _ser.write_bare::<crate::ton::int>(file_hash)?;
        _ser.write_bare::<crate::ton::int>(collated_data_file_hash)?;
        Ok(())
    }
}
impl crate::BareDeserialize for SentBlock {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let src = _de.read_bare::<crate::ton::int>()?;
            let root_hash = _de.read_bare::<crate::ton::int>()?;
            let file_hash = _de.read_bare::<crate::ton::int>()?;
            let collated_data_file_hash = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { src, root_hash, file_hash, collated_data_file_hash })
        }
    }
}
impl crate::IntoBoxed for SentBlock {
    type Boxed = crate::ton::Hashable;

    fn into_boxed(self) -> crate::ton::Hashable {
        crate::ton::Hashable::Hashable_SentBlock(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `hashable.validatorSession`\n\n```text\nhashable.validatorSession ts:int old_rounds:int cur_round:int = Hashable;\n```\n"]
pub struct ValidatorSession {
    pub ts: crate::ton::int,
    pub old_rounds: crate::ton::int,
    pub cur_round: crate::ton::int,
}
impl Eq for ValidatorSession {}
impl crate::BareSerialize for ValidatorSession {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x681263d5)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let ValidatorSession { ts, old_rounds, cur_round } = self;
        _ser.write_bare::<crate::ton::int>(ts)?;
        _ser.write_bare::<crate::ton::int>(old_rounds)?;
        _ser.write_bare::<crate::ton::int>(cur_round)?;
        Ok(())
    }
}
impl crate::BareDeserialize for ValidatorSession {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let ts = _de.read_bare::<crate::ton::int>()?;
            let old_rounds = _de.read_bare::<crate::ton::int>()?;
            let cur_round = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { ts, old_rounds, cur_round })
        }
    }
}
impl crate::IntoBoxed for ValidatorSession {
    type Boxed = crate::ton::Hashable;

    fn into_boxed(self) -> crate::ton::Hashable {
        crate::ton::Hashable::Hashable_ValidatorSession(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `hashable.validatorSessionOldRound`\n\n```text\nhashable.validatorSessionOldRound seqno:int block:int signatures:int approve_signatures:int = Hashable;\n```\n"]
pub struct ValidatorSessionOldRound {
    pub seqno: crate::ton::int,
    pub block: crate::ton::int,
    pub signatures: crate::ton::int,
    pub approve_signatures: crate::ton::int,
}
impl Eq for ValidatorSessionOldRound {}
impl crate::BareSerialize for ValidatorSessionOldRound {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x478b67a9)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let ValidatorSessionOldRound { seqno, block, signatures, approve_signatures } = self;
        _ser.write_bare::<crate::ton::int>(seqno)?;
        _ser.write_bare::<crate::ton::int>(block)?;
        _ser.write_bare::<crate::ton::int>(signatures)?;
        _ser.write_bare::<crate::ton::int>(approve_signatures)?;
        Ok(())
    }
}
impl crate::BareDeserialize for ValidatorSessionOldRound {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let seqno = _de.read_bare::<crate::ton::int>()?;
            let block = _de.read_bare::<crate::ton::int>()?;
            let signatures = _de.read_bare::<crate::ton::int>()?;
            let approve_signatures = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { seqno, block, signatures, approve_signatures })
        }
    }
}
impl crate::IntoBoxed for ValidatorSessionOldRound {
    type Boxed = crate::ton::Hashable;

    fn into_boxed(self) -> crate::ton::Hashable {
        crate::ton::Hashable::Hashable_ValidatorSessionOldRound(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `hashable.validatorSessionRound`\n\n```text\nhashable.validatorSessionRound locked_round:int locked_block:int seqno:int precommitted:Bool\n          first_attempt:int approved_blocks:int signatures:int attempts:int = Hashable;\n```\n"]
pub struct ValidatorSessionRound {
    pub locked_round: crate::ton::int,
    pub locked_block: crate::ton::int,
    pub seqno: crate::ton::int,
    pub precommitted: crate::ton::Bool,
    pub first_attempt: crate::ton::int,
    pub approved_blocks: crate::ton::int,
    pub signatures: crate::ton::int,
    pub attempts: crate::ton::int,
}
impl Eq for ValidatorSessionRound {}
impl crate::BareSerialize for ValidatorSessionRound {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x35774fe3)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let ValidatorSessionRound {
            locked_round,
            locked_block,
            seqno,
            precommitted,
            first_attempt,
            approved_blocks,
            signatures,
            attempts,
        } = self;
        _ser.write_bare::<crate::ton::int>(locked_round)?;
        _ser.write_bare::<crate::ton::int>(locked_block)?;
        _ser.write_bare::<crate::ton::int>(seqno)?;
        _ser.write_boxed::<crate::ton::Bool>(precommitted)?;
        _ser.write_bare::<crate::ton::int>(first_attempt)?;
        _ser.write_bare::<crate::ton::int>(approved_blocks)?;
        _ser.write_bare::<crate::ton::int>(signatures)?;
        _ser.write_bare::<crate::ton::int>(attempts)?;
        Ok(())
    }
}
impl crate::BareDeserialize for ValidatorSessionRound {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let locked_round = _de.read_bare::<crate::ton::int>()?;
            let locked_block = _de.read_bare::<crate::ton::int>()?;
            let seqno = _de.read_bare::<crate::ton::int>()?;
            let precommitted = _de.read_boxed::<crate::ton::Bool>()?;
            let first_attempt = _de.read_bare::<crate::ton::int>()?;
            let approved_blocks = _de.read_bare::<crate::ton::int>()?;
            let signatures = _de.read_bare::<crate::ton::int>()?;
            let attempts = _de.read_bare::<crate::ton::int>()?;
            Ok(Self {
                locked_round,
                locked_block,
                seqno,
                precommitted,
                first_attempt,
                approved_blocks,
                signatures,
                attempts,
            })
        }
    }
}
impl crate::IntoBoxed for ValidatorSessionRound {
    type Boxed = crate::ton::Hashable;

    fn into_boxed(self) -> crate::ton::Hashable {
        crate::ton::Hashable::Hashable_ValidatorSessionRound(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `hashable.validatorSessionRoundAttempt`\n\n```text\nhashable.validatorSessionRoundAttempt seqno:int votes:int precommitted:int vote_for_inited:int vote_for:int = Hashable;\n```\n"]
pub struct ValidatorSessionRoundAttempt {
    pub seqno: crate::ton::int,
    pub votes: crate::ton::int,
    pub precommitted: crate::ton::int,
    pub vote_for_inited: crate::ton::int,
    pub vote_for: crate::ton::int,
}
impl Eq for ValidatorSessionRoundAttempt {}
impl crate::BareSerialize for ValidatorSessionRoundAttempt {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x4c11ffad)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let ValidatorSessionRoundAttempt { seqno, votes, precommitted, vote_for_inited, vote_for } =
            self;
        _ser.write_bare::<crate::ton::int>(seqno)?;
        _ser.write_bare::<crate::ton::int>(votes)?;
        _ser.write_bare::<crate::ton::int>(precommitted)?;
        _ser.write_bare::<crate::ton::int>(vote_for_inited)?;
        _ser.write_bare::<crate::ton::int>(vote_for)?;
        Ok(())
    }
}
impl crate::BareDeserialize for ValidatorSessionRoundAttempt {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let seqno = _de.read_bare::<crate::ton::int>()?;
            let votes = _de.read_bare::<crate::ton::int>()?;
            let precommitted = _de.read_bare::<crate::ton::int>()?;
            let vote_for_inited = _de.read_bare::<crate::ton::int>()?;
            let vote_for = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { seqno, votes, precommitted, vote_for_inited, vote_for })
        }
    }
}
impl crate::IntoBoxed for ValidatorSessionRoundAttempt {
    type Boxed = crate::ton::Hashable;

    fn into_boxed(self) -> crate::ton::Hashable {
        crate::ton::Hashable::Hashable_ValidatorSessionRoundAttempt(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `hashable.vector`\n\n```text\nhashable.vector value:(vector int) = Hashable;\n```\n"]
pub struct Vector {
    pub value: crate::ton::vector<crate::ton::Bare, crate::ton::int>,
}
impl Eq for Vector {}
impl crate::BareSerialize for Vector {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xdf34c36d)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Vector { value } = self;
        _ser.write_bare::<crate::ton::vector<crate::ton::Bare, crate::ton::int>>(value)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Vector {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let value = _de.read_bare::<crate::ton::vector<crate::ton::Bare, crate::ton::int>>()?;
            Ok(Self { value })
        }
    }
}
impl crate::IntoBoxed for Vector {
    type Boxed = crate::ton::Hashable;

    fn into_boxed(self) -> crate::ton::Hashable {
        crate::ton::Hashable::Hashable_Vector(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `hashable.vote`\n\n```text\nhashable.vote block:int node:int = Hashable;\n```\n"]
pub struct Vote {
    pub block: crate::ton::int,
    pub node: crate::ton::int,
}
impl Eq for Vote {}
impl crate::BareSerialize for Vote {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xaebf2bc5)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Vote { block, node } = self;
        _ser.write_bare::<crate::ton::int>(block)?;
        _ser.write_bare::<crate::ton::int>(node)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Vote {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let block = _de.read_bare::<crate::ton::int>()?;
            let node = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { block, node })
        }
    }
}
impl crate::IntoBoxed for Vote {
    type Boxed = crate::ton::Hashable;

    fn into_boxed(self) -> crate::ton::Hashable {
        crate::ton::Hashable::Hashable_Vote(self)
    }
}
