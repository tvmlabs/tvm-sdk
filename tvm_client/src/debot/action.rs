use serde::Deserialize;
use serde::Deserializer;
use serde::Serializer;
use serde::de;

use super::context::from_abi_num;
use super::context::from_hex_to_utf8_str;
use crate::encoding::decode_abi_number;

#[derive(Clone, Default)]
pub enum AcType {
    #[default]
    Empty = 0,
    RunAction = 1,
    RunMethod = 2,
    SendMsg = 3,
    Invoke = 4,
    Print = 5,
    Goto = 6,
    CallEngine = 10,
    Unknown = 255,
}

impl From<u8> for AcType {
    fn from(ac_type: u8) -> Self {
        match ac_type {
            0 => AcType::Empty,
            1 => AcType::RunAction,
            2 => AcType::RunMethod,
            3 => AcType::SendMsg,
            4 => AcType::Invoke,
            5 => AcType::Print,
            6 => AcType::Goto,
            10 => AcType::CallEngine,
            _ => AcType::Unknown,
        }
    }
}

/// Describes a debot action in a Debot Context.
#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct DAction {
    /// A short action description. Should be used by Debot Browser as name of
    /// menu item.
    #[serde(deserialize_with = "from_hex_to_utf8_str")]
    pub desc: String,
    /// Depends on action type. Can be a debot function name or a print string
    /// (for Print Action).
    #[serde(deserialize_with = "from_hex_to_utf8_str")]
    pub name: String,
    /// Action type.
    #[serde(deserialize_with = "str_to_actype")]
    #[serde(serialize_with = "actype_to_str")]
    pub action_type: AcType,
    /// ID of debot context to switch after action execution.
    #[serde(deserialize_with = "from_abi_num")]
    pub to: u8,
    /// Action attributes. In the form of "param=value,flag".
    /// attribute example: instant, args, fargs, sign.
    #[serde(deserialize_with = "from_hex_to_utf8_str")]
    pub attrs: String,
    /// Some internal action data. Used by debot only.
    pub misc: String,
}

impl DAction {
    #[allow(dead_code)]
    pub fn empty() -> Self {
        DAction {
            desc: String::new(),
            name: String::new(),
            action_type: AcType::Empty,
            to: 0,
            attrs: String::new(),
            misc: String::new(),
        }
    }

    #[allow(dead_code)]
    pub fn new(desc: String, name: String, action_type: u8, to: u8) -> Self {
        DAction {
            desc,
            name,
            action_type: action_type.into(),
            to,
            attrs: String::new(),
            misc: String::new(),
        }
    }

    pub fn is_engine_call(&self) -> bool {
        matches!(self.action_type, AcType::CallEngine)
    }

    pub fn is_invoke(&self) -> bool {
        matches!(self.action_type, AcType::Invoke)
    }

    pub fn is_instant(&self) -> bool {
        self.attrs.split(',').find(|val| *val == "instant").map(|_| true).unwrap_or(false)
    }

    pub fn func_attr(&self) -> Option<String> {
        self.attr_value("func")
    }

    pub fn args_attr(&self) -> Option<String> {
        self.attr_value("args")
    }

    pub fn sign_by_user(&self) -> bool {
        self.attr_value("sign").map(|s| s == "by_user").unwrap_or(false)
    }

    pub fn format_args(&self) -> Option<String> {
        self.attr_value("fargs")
    }

    fn attr_value(&self, name: &str) -> Option<String> {
        let name = name.to_owned() + "=";
        self.attrs.split(',').find(|val| val.starts_with(&name)).map(|val| {
            let vec: Vec<&str> = val.split('=').collect();
            vec[1].to_owned()
        })
    }
}

fn str_to_actype<'de, D>(des: D) -> Result<AcType, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(des)?;
    decode_abi_number::<u8>(&s).map_err(de::Error::custom).map(|t| t.into())
}

fn actype_to_str<S>(a: &AcType, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let num: u8 = match a {
        AcType::Empty => 0,
        AcType::RunAction => 1,
        AcType::RunMethod => 2,
        AcType::SendMsg => 3,
        AcType::Invoke => 4,
        AcType::Print => 5,
        AcType::Goto => 6,
        AcType::CallEngine => 10,
        AcType::Unknown => 255,
    };

    s.serialize_str(&format!("{:x}", num))
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    fn hex_str(value: &str) -> String {
        hex::encode(value.as_bytes())
    }

    #[test]
    fn ac_type_from_maps_known_and_unknown_values() {
        assert!(matches!(AcType::from(0), AcType::Empty));
        assert!(matches!(AcType::from(1), AcType::RunAction));
        assert!(matches!(AcType::from(2), AcType::RunMethod));
        assert!(matches!(AcType::from(3), AcType::SendMsg));
        assert!(matches!(AcType::from(4), AcType::Invoke));
        assert!(matches!(AcType::from(5), AcType::Print));
        assert!(matches!(AcType::from(6), AcType::Goto));
        assert!(matches!(AcType::from(10), AcType::CallEngine));
        assert!(matches!(AcType::from(42), AcType::Unknown));
    }

    #[test]
    fn action_helpers_extract_attributes() {
        let action = DAction {
            desc: "show".into(),
            name: "run".into(),
            action_type: AcType::CallEngine,
            to: 7,
            attrs: "instant,func=invoke,args=foo,sign=by_user,fargs=bar".into(),
            misc: "misc".into(),
        };

        assert!(action.is_engine_call());
        assert!(!action.is_invoke());
        assert!(action.is_instant());
        assert_eq!(action.func_attr().as_deref(), Some("invoke"));
        assert_eq!(action.args_attr().as_deref(), Some("foo"));
        assert!(action.sign_by_user());
        assert_eq!(action.format_args().as_deref(), Some("bar"));
    }

    #[test]
    fn action_constructors_keep_expected_defaults() {
        let empty = DAction::empty();
        assert!(matches!(empty.action_type, AcType::Empty));
        assert!(empty.desc.is_empty());
        assert!(empty.name.is_empty());
        assert!(empty.attrs.is_empty());

        let unknown = DAction::new("desc".into(), "name".into(), 255, 9);
        assert!(matches!(unknown.action_type, AcType::Unknown));
        assert_eq!(unknown.to, 9);
        assert!(unknown.attrs.is_empty());
        assert!(unknown.misc.is_empty());
    }

    #[test]
    fn serde_roundtrip_uses_hex_strings_and_numeric_action_type() {
        let value = json!({
            "desc": hex_str("menu item"),
            "name": hex_str("call"),
            "actionType": "10",
            "to": "15",
            "attrs": hex_str("instant,func=go"),
            "misc": "opaque",
        });

        let action: DAction = serde_json::from_value(value).unwrap();
        assert_eq!(action.desc, "menu item");
        assert_eq!(action.name, "call");
        assert!(matches!(action.action_type, AcType::CallEngine));
        assert_eq!(action.to, 15);
        assert_eq!(action.attrs, "instant,func=go");
        assert_eq!(action.misc, "opaque");

        let serialized = serde_json::to_value(&action).unwrap();
        assert_eq!(serialized["actionType"], "a");
    }

    #[test]
    fn serde_rejects_invalid_action_type() {
        let value = json!({
            "desc": hex_str("menu item"),
            "name": hex_str("call"),
            "actionType": "not-a-number",
            "to": "1",
            "attrs": hex_str(""),
            "misc": "",
        });

        let err = serde_json::from_value::<DAction>(value).err().unwrap().to_string();
        assert!(err.contains("not-a-number"));
    }
}
