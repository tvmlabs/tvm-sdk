use serde_json::Map;
use serde_json::Value;
use tvm_types::Result;

use crate::BlockParsingError;
use crate::JsonReducer;

#[derive(Clone, Debug, PartialEq)]
enum Field {
    Scalar(String),
    Object(String, Vec<Field>),
}

#[derive(Clone, Debug, PartialEq)]
struct ReduceConfig {
    pub fields: Vec<Field>,
}

impl ReduceConfig {
    pub fn parse_str(config: &str) -> Result<ReduceConfig> {
        let config = config.trim_start().trim_end();
        let config = if config.starts_with('{') {
            config.trim_start_matches('{').to_string()
        } else {
            format!("{}}}", config)
        };
        let spaced = config.replace('{', " { ").replace('}', " } ");

        let fields = Self::collect_selection(&mut spaced.split_whitespace())?;
        Ok(ReduceConfig { fields })
    }

    fn collect_selection<'a>(iter: &mut impl Iterator<Item = &'a str>) -> Result<Vec<Field>> {
        let mut prev_name = "";
        let mut selection = vec![];
        while let Some(field) = iter.next() {
            if field == "{" {
                if prev_name.is_empty() {
                    return Err(BlockParsingError::InvalidData(
                        "unnamed subset selection".to_string(),
                    )
                    .into());
                }
                let inner = Self::collect_selection(iter)?;
                selection.push(Field::Object(prev_name.to_string(), inner));
                prev_name = "";
            } else if field == "}" {
                if !prev_name.is_empty() {
                    selection.push(Field::Scalar(prev_name.to_string()));
                }
                if selection.is_empty() {
                    return Err(
                        BlockParsingError::InvalidData("empty selection".to_string()).into()
                    );
                }
                return Ok(selection);
            } else {
                if field
                    .find(|char: char| {
                        !(char.is_ascii_alphanumeric() || char == '-' || char == '_')
                    })
                    .is_some()
                {
                    return Err(BlockParsingError::InvalidData(format!(
                        "invalid field name {}",
                        field
                    ))
                    .into());
                }

                if !prev_name.is_empty() {
                    selection.push(Field::Scalar(prev_name.to_string()));
                }
                prev_name = field;
            }
        }
        Err(BlockParsingError::InvalidData("mismatched square angle".to_string()).into())
    }
}

pub struct JsonFieldsReducer {
    config: ReduceConfig,
}

impl JsonReducer for JsonFieldsReducer {
    fn reduce(&self, json: Map<String, Value>) -> Result<Map<String, Value>> {
        self.reduce(json)
    }
}

impl JsonFieldsReducer {
    pub fn with_config(config: &str) -> Result<Self> {
        let config = ReduceConfig::parse_str(config)?;
        Ok(Self { config })
    }

    pub fn reduce(&self, json: Map<String, Value>) -> Result<Map<String, Value>> {
        Self::reduce_fields(&self.config.fields, json)
    }

    fn reduce_fields(fields: &[Field], json: Map<String, Value>) -> Result<Map<String, Value>> {
        let mut map = json;
        let mut result = Map::new();
        for field in fields {
            match field {
                Field::Scalar(name) => {
                    map.remove(name).map(|x| result.insert(name.clone(), x));
                }
                Field::Object(name, inner_fields) => {
                    if let Some(mut value) = map.remove(name) {
                        if let Some(array) = value.as_array_mut() {
                            for arr_value in array {
                                *arr_value = Self::reduce_value(inner_fields, arr_value.take())?;
                            }
                            result.insert(name.clone(), value);
                        } else {
                            result.insert(name.clone(), Self::reduce_value(inner_fields, value)?);
                        }
                    }
                }
            }
        }

        Ok(result)
    }

    fn reduce_value(fields: &[Field], value: Value) -> Result<Value> {
        Ok(match value {
            Value::Null => value,
            Value::Object(map) => Value::Object(Self::reduce_fields(fields, map)?),
            _ => Err(BlockParsingError::InvalidData(format!(
                "JSON object expected, received value: {}",
                value
            )))?,
        })
    }
}

#[cfg(test)]
#[path = "../tests/test_reducers.rs"]
mod tests;
