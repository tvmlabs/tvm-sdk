// Copyright (C) 2019-2021 TON Labs. All Rights Reserved.
//
// Licensed under the SOFTWARE EVALUATION License (the "License"); you may not
// use this file except in compliance with the License.
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific TON DEV software governing permissions and
// limitations under the License.

mod param_type_tests {
    use crate::Param;
    use crate::ParamType;
    use crate::contract::ABI_VERSION_1_0;
    use crate::contract::ABI_VERSION_2_0;
    use crate::contract::ABI_VERSION_2_1;
    use crate::contract::ABI_VERSION_2_4;

    #[test]
    fn test_param_type_signature() {
        assert_eq!(ParamType::Uint(256).type_signature(), "uint256".to_owned());
        assert_eq!(ParamType::Int(64).type_signature(), "int64".to_owned());
        assert_eq!(ParamType::Bool.type_signature(), "bool".to_owned());

        assert_eq!(
            ParamType::Array(Box::new(ParamType::Cell)).type_signature(),
            "cell[]".to_owned()
        );

        assert_eq!(
            ParamType::FixedArray(Box::new(ParamType::Int(33)), 2).type_signature(),
            "int33[2]".to_owned()
        );

        assert_eq!(
            ParamType::FixedArray(Box::new(ParamType::Array(Box::new(ParamType::Bytes))), 2)
                .type_signature(),
            "bytes[][2]".to_owned()
        );

        let tuple_params = vec![
            Param { name: "a".to_owned(), kind: ParamType::Uint(123) },
            Param { name: "b".to_owned(), kind: ParamType::Int(8) },
        ];

        let tuple_with_tuple = vec![
            Param { name: "a".to_owned(), kind: ParamType::Tuple(tuple_params.clone()) },
            Param { name: "b".to_owned(), kind: ParamType::Token },
        ];

        assert_eq!(
            ParamType::Tuple(tuple_params.clone()).type_signature(),
            "(uint123,int8)".to_owned()
        );

        assert_eq!(
            ParamType::Array(Box::new(ParamType::Tuple(tuple_with_tuple))).type_signature(),
            "((uint123,int8),gram)[]".to_owned()
        );

        assert_eq!(
            ParamType::FixedArray(Box::new(ParamType::Tuple(tuple_params)), 4).type_signature(),
            "(uint123,int8)[4]".to_owned()
        );

        assert_eq!(
            ParamType::Map(Box::new(ParamType::Int(456)), Box::new(ParamType::Address))
                .type_signature(),
            "map(int456,address)".to_owned()
        );

        assert_eq!(ParamType::String.type_signature(), "string".to_owned());

        assert_eq!(ParamType::VarUint(16).type_signature(), "varuint16".to_owned());
        assert_eq!(ParamType::VarInt(32).type_signature(), "varint32".to_owned());

        assert_eq!(
            ParamType::Optional(Box::new(ParamType::Int(123))).type_signature(),
            "optional(int123)".to_owned()
        );
        assert_eq!(
            ParamType::Ref(Box::new(ParamType::Uint(123))).type_signature(),
            "ref(uint123)".to_owned()
        );
        assert_eq!(format!("{}", ParamType::Expire), "expire");
    }

    #[test]
    fn set_components_and_supported_versions_cover_error_paths() {
        let tuple_components = vec![Param::new("value", ParamType::Uint(32))];
        let mut tuple = ParamType::Tuple(vec![]);
        tuple.set_components(tuple_components.clone()).unwrap();
        assert_eq!(tuple, ParamType::Tuple(tuple_components));

        let err = ParamType::Tuple(vec![]).set_components(vec![]).unwrap_err();
        assert!(
            err.to_string()
                .contains("Tuple description should contain non empty `components` field")
        );

        let err = ParamType::Bool
            .set_components(vec![Param::new("value", ParamType::Uint(8))])
            .unwrap_err();
        assert!(err.to_string().contains(
            "Type description contains non empty `components` field but it is not a tuple"
        ));

        assert!(!ParamType::Time.is_supported(&ABI_VERSION_1_0));
        assert!(ParamType::Time.is_supported(&ABI_VERSION_2_0));
        assert!(!ParamType::String.is_supported(&ABI_VERSION_2_0));
        assert!(ParamType::String.is_supported(&ABI_VERSION_2_1));
        assert!(!ParamType::Ref(Box::new(ParamType::Bool)).is_supported(&ABI_VERSION_2_1));
        assert!(ParamType::Ref(Box::new(ParamType::Bool)).is_supported(&ABI_VERSION_2_4));
    }
}

mod deserialize_tests {
    use serde::Deserialize;
    use serde::de::value::StringDeserializer;

    use crate::ParamType;
    use crate::param_type::deserialize::read_type;

    #[test]
    fn param_type_deserialization() {
        let s = r#"["uint256", "int64", "bool", "bool[]", "int33[2]", "bool[][2]",
            "tuple", "tuple[]", "tuple[4]", "cell", "map(int3,bool)", "map(uint1023,tuple[][5])",
            "address", "bytes", "fixedbytes32", "token", "time", "expire", "pubkey", "string",
            "varuint16", "varint32", "optional(bytes)", "ref(bool)"]"#;
        let deserialized: Vec<ParamType> = serde_json::from_str(s).unwrap();
        assert_eq!(
            deserialized,
            vec![
                ParamType::Uint(256),
                ParamType::Int(64),
                ParamType::Bool,
                ParamType::Array(Box::new(ParamType::Bool)),
                ParamType::FixedArray(Box::new(ParamType::Int(33)), 2),
                ParamType::FixedArray(Box::new(ParamType::Array(Box::new(ParamType::Bool))), 2),
                ParamType::Tuple(vec![]),
                ParamType::Array(Box::new(ParamType::Tuple(vec![]))),
                ParamType::FixedArray(Box::new(ParamType::Tuple(vec![])), 4),
                ParamType::Cell,
                ParamType::Map(Box::new(ParamType::Int(3)), Box::new(ParamType::Bool)),
                ParamType::Map(
                    Box::new(ParamType::Uint(1023)),
                    Box::new(ParamType::FixedArray(
                        Box::new(ParamType::Array(Box::new(ParamType::Tuple(vec![])))),
                        5
                    ))
                ),
                ParamType::Address,
                ParamType::Bytes,
                ParamType::FixedBytes(32),
                ParamType::Token,
                ParamType::Time,
                ParamType::Expire,
                ParamType::PublicKey,
                ParamType::String,
                ParamType::VarUint(16),
                ParamType::VarInt(32),
                ParamType::Optional(Box::new(ParamType::Bytes)),
                ParamType::Ref(Box::new(ParamType::Bool)),
            ]
        );
    }

    #[test]
    fn param_type_deserialization_covers_string_deserializer_and_errors() {
        let parsed = ParamType::deserialize(StringDeserializer::<serde_json::Error>::new(
            "bool[]".to_owned(),
        ))
        .unwrap();
        assert_eq!(parsed, ParamType::Array(Box::new(ParamType::Bool)));

        let err = read_type("bool[abc]").unwrap_err();
        assert!(err.to_string().contains("Invalid name:"));

        let err = read_type("map(bool,uint8)").unwrap_err();
        assert!(err.to_string().contains("Only integer and std address values can be map keys"));

        let err = read_type("map(uint8)").unwrap_err();
        assert!(err.to_string().contains("Invalid name:"));

        let err = read_type("wat").unwrap_err();
        assert!(err.to_string().contains("Invalid name:"));

        let err = serde_json::from_str::<ParamType>("123").unwrap_err();
        assert!(err.to_string().contains("a correct name of abi-encodable parameter type"));
    }
}
