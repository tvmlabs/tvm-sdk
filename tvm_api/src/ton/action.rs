use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `actionDns`\n\n```text\nactionDns actions:vector<dns.Action> = Action;\n```\n"]
pub struct ActionDns {
    pub actions: crate::ton::vector<crate::ton::Boxed, crate::ton::dns::Action>,
}
impl Eq for ActionDns {}
impl crate::BareSerialize for ActionDns {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x47273021)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let ActionDns { actions } = self;
        _ser.write_bare::<crate::ton::vector<crate::ton::Boxed, crate::ton::dns::Action>>(actions)?;
        Ok(())
    }
}
impl crate::BareDeserialize for ActionDns {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let actions =
                _de.read_bare::<crate::ton::vector<crate::ton::Boxed, crate::ton::dns::Action>>()?;
            Ok(Self { actions })
        }
    }
}
impl crate::IntoBoxed for ActionDns {
    type Boxed = crate::ton::Action;

    fn into_boxed(self) -> crate::ton::Action {
        crate::ton::Action::ActionDns(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `actionMsg`\n\n```text\nactionMsg messages:vector<msg.message> allow_send_to_uninited:Bool = Action;\n```\n"]
pub struct ActionMsg {
    pub messages: crate::ton::vector<crate::ton::Bare, crate::ton::msg::message::Message>,
    pub allow_send_to_uninited: crate::ton::Bool,
}
impl Eq for ActionMsg {}
impl crate::BareSerialize for ActionMsg {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x0eb67750)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let ActionMsg { messages, allow_send_to_uninited } = self;
        _ser.write_bare::<crate::ton::vector<crate::ton::Bare, crate::ton::msg::message::Message>>(
            messages,
        )?;
        _ser.write_boxed::<crate::ton::Bool>(allow_send_to_uninited)?;
        Ok(())
    }
}
impl crate::BareDeserialize for ActionMsg {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let messages = _de . read_bare :: < crate :: ton :: vector < crate :: ton :: Bare , crate :: ton :: msg :: message :: Message > > () ? ;
            let allow_send_to_uninited = _de.read_boxed::<crate::ton::Bool>()?;
            Ok(Self { messages, allow_send_to_uninited })
        }
    }
}
impl crate::IntoBoxed for ActionMsg {
    type Boxed = crate::ton::Action;

    fn into_boxed(self) -> crate::ton::Action {
        crate::ton::Action::ActionMsg(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `actionPchan`\n\n```text\nactionPchan action:pchan.Action = Action;\n```\n"]
pub struct ActionPchan {
    pub action: crate::ton::pchan::Action,
}
impl Eq for ActionPchan {}
impl crate::BareSerialize for ActionPchan {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xa72dc5e1)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let ActionPchan { action } = self;
        _ser.write_boxed::<crate::ton::pchan::Action>(action)?;
        Ok(())
    }
}
impl crate::BareDeserialize for ActionPchan {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let action = _de.read_boxed::<crate::ton::pchan::Action>()?;
            Ok(Self { action })
        }
    }
}
impl crate::IntoBoxed for ActionPchan {
    type Boxed = crate::ton::Action;

    fn into_boxed(self) -> crate::ton::Action {
        crate::ton::Action::ActionPchan(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `actionRwallet`\n\n```text\nactionRwallet action:rwallet.actionInit = Action;\n```\n"]
pub struct ActionRwallet {
    pub action: crate::ton::rwallet::actioninit::ActionInit,
}
impl Eq for ActionRwallet {}
impl crate::BareSerialize for ActionRwallet {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xf90237c5)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let ActionRwallet { action } = self;
        _ser.write_bare::<crate::ton::rwallet::actioninit::ActionInit>(action)?;
        Ok(())
    }
}
impl crate::BareDeserialize for ActionRwallet {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let action = _de.read_bare::<crate::ton::rwallet::actioninit::ActionInit>()?;
            Ok(Self { action })
        }
    }
}
impl crate::IntoBoxed for ActionRwallet {
    type Boxed = crate::ton::Action;

    fn into_boxed(self) -> crate::ton::Action {
        crate::ton::Action::ActionRwallet(self)
    }
}
