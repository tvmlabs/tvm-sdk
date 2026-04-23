use std::fmt::Display;
use std::str::FromStr;

use serde::Deserialize;
use serde::Deserializer;
use serde::de;

use super::action::DAction;
use crate::encoding::decode_abi_number;

pub const STATE_ZERO: u8 = 0;
pub const STATE_CURRENT: u8 = 253;
pub const STATE_PREV: u8 = 254;
pub const STATE_EXIT: u8 = 255;

/// Debot Context. Consists of several actions.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone)]
pub struct DContext {
    #[serde(deserialize_with = "from_abi_num")]
    pub id: u8,
    #[serde(deserialize_with = "from_hex_to_utf8_str")]
    pub desc: String,
    pub actions: Vec<DAction>,
}

impl DContext {
    #[allow(dead_code)]
    pub fn new(desc: String, actions: Vec<DAction>, id: u8) -> Self {
        DContext { desc, actions, id }
    }

    pub fn new_quit() -> Self {
        DContext::new(String::new(), vec![], STATE_EXIT)
    }
}

pub(super) fn from_abi_num<'de, D>(des: D) -> Result<u8, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(des)?;
    decode_abi_number(&s).map_err(de::Error::custom)
}

pub(super) fn str_hex_to_utf8(s: &str) -> Option<String> {
    String::from_utf8(hex::decode(s).ok()?).ok()
}

pub(super) fn from_hex_to_utf8_str<'de, S, D>(des: D) -> Result<S, D::Error>
where
    S: FromStr,
    S::Err: Display,
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(des)?;
    let s =
        str_hex_to_utf8(&s).ok_or("failed to convert bytes to utf8 string".to_string()).unwrap();
    S::from_str(&s).map_err(de::Error::custom)
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;
    use serde_json::json;

    use super::*;

    fn hex_str(value: &str) -> String {
        hex::encode(value.as_bytes())
    }

    #[test]
    fn context_helpers_decode_strings_and_states() {
        let context = DContext::new("menu".into(), vec![], 3);
        assert_eq!(context.desc, "menu");
        assert_eq!(context.id, 3);

        let quit = DContext::new_quit();
        assert_eq!(quit.id, STATE_EXIT);
        assert!(quit.desc.is_empty());
        assert!(quit.actions.is_empty());

        assert_eq!(str_hex_to_utf8(&hex_str("hello")).as_deref(), Some("hello"));
        assert_eq!(str_hex_to_utf8("zz"), None);
    }

    #[test]
    fn context_deserializes_numeric_and_hex_fields() {
        let value = json!({
            "id": "253",
            "desc": hex_str("current"),
            "actions": [],
        });

        let context: DContext = serde_json::from_value(value).unwrap();
        assert_eq!(context.id, STATE_CURRENT);
        assert_eq!(context.desc, "current");
        assert!(context.actions.is_empty());
    }

    #[test]
    #[should_panic(expected = "failed to convert bytes to utf8 string")]
    fn invalid_hex_panics_in_from_hex_to_utf8_str() {
        #[derive(Deserialize)]
        struct Wrapper {
            #[serde(deserialize_with = "from_hex_to_utf8_str")]
            value: String,
        }

        let _: Wrapper = serde_json::from_value(json!({ "value": "not-hex" })).unwrap();
    }
}
