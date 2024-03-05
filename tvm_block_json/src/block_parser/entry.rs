use serde_json::Map;
use serde_json::Value;
use tvm_types::Result;

use crate::block_parser::BlockParsingError;
use crate::block_parser::JsonReducer;
use crate::EntryConfig;

#[derive(Clone)]
pub struct ParsedEntry {
    pub id: String,
    pub body: Map<String, Value>,
    pub partition: Option<u32>,
}

impl ParsedEntry {
    pub fn new(body: Map<String, Value>, partition: Option<u32>) -> Result<Self> {
        Ok(Self {
            id: body["id"]
                .as_str()
                .ok_or_else(|| BlockParsingError::InvalidData("Doc has no `id` field".to_owned()))?
                .to_owned(),
            body,
            partition,
        })
    }

    pub fn reduced<R: JsonReducer>(
        body: Map<String, Value>,
        partition: Option<u32>,
        config: &Option<EntryConfig<R>>,
    ) -> Result<Self> {
        if let Some(config) = config {
            if let Some(reducer) = &config.reducer {
                return Self::new(reducer.reduce(body)?, partition);
            }
        }
        Self::new(body, partition)
    }
}

pub(crate) fn get_sharding_depth<R: JsonReducer>(config: &Option<EntryConfig<R>>) -> u32 {
    config.as_ref().map_or(0, |x| x.sharding_depth.unwrap_or(0))
}
