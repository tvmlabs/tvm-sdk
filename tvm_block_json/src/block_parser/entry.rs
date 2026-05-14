use serde_json::Map;
use serde_json::Value;
use tvm_types::Result;

use crate::EntryConfig;
use crate::block_parser::BlockParsingError;
use crate::block_parser::JsonReducer;

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

#[cfg(test)]
mod tests {
    use serde_json::Map;
    use serde_json::Value;

    use super::ParsedEntry;
    use super::get_sharding_depth;
    use crate::EntryConfig;
    use crate::JsonReducer;

    struct TestReducer;

    impl JsonReducer for TestReducer {
        fn reduce(&self, mut json: Map<String, Value>) -> tvm_types::Result<Map<String, Value>> {
            json.insert("id".to_owned(), Value::String("reduced-id".to_owned()));
            json.insert("reduced".to_owned(), Value::Bool(true));
            Ok(json)
        }
    }

    #[test]
    fn parsed_entry_new_reads_id_and_partition() {
        let mut body = Map::new();
        body.insert("id".to_owned(), Value::String("entry-id".to_owned()));
        body.insert("value".to_owned(), Value::from(1));

        let entry = ParsedEntry::new(body.clone(), Some(7)).unwrap();
        assert_eq!(entry.id, "entry-id");
        assert_eq!(entry.partition, Some(7));
        assert_eq!(entry.body, body);
    }

    #[test]
    fn parsed_entry_new_rejects_missing_or_non_string_id() {
        let panic = std::panic::catch_unwind(|| ParsedEntry::new(Map::new(), None));
        assert!(panic.is_err());

        let mut body = Map::new();
        body.insert("id".to_owned(), Value::from(42));
        assert!(ParsedEntry::new(body, None).is_err());
    }

    #[test]
    fn parsed_entry_reduced_uses_reducer_and_sharding_depth_defaults() {
        let mut body = Map::new();
        body.insert("id".to_owned(), Value::String("entry-id".to_owned()));

        let reducer_config =
            Some(EntryConfig { sharding_depth: Some(5), reducer: Some(TestReducer) });
        let reduced = ParsedEntry::reduced(body.clone(), Some(3), &reducer_config).unwrap();
        assert_eq!(reduced.id, "reduced-id");
        assert_eq!(reduced.partition, Some(3));
        assert_eq!(reduced.body["reduced"], Value::Bool(true));

        let no_reducer = Some(EntryConfig::<TestReducer> { sharding_depth: None, reducer: None });
        let plain = ParsedEntry::reduced(body, None, &no_reducer).unwrap();
        assert_eq!(plain.id, "entry-id");
        assert_eq!(plain.partition, None);

        assert_eq!(get_sharding_depth(&reducer_config), 5);
        assert_eq!(get_sharding_depth(&no_reducer), 0);
        assert_eq!(get_sharding_depth::<TestReducer>(&None), 0);
    }
}
