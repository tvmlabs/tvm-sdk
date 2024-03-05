// Copyright (C) 2019-2021 TON Labs. All Rights Reserved.
//
// Licensed under the SOFTWARE EVALUATION License (the "License"); you may not
// use this file except in compliance with the License.  You may obtain a copy
// of the License at:
//
// https://www.ton.dev/licenses
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific TON DEV software governing permissions and
// limitations under the License.

use serde_json::json;

use super::*;
use crate::block_parser::reducers::JsonFieldsReducer;

#[test]
fn test_config_parser() {
    let minimal = ReduceConfig { fields: vec![Field::Scalar("a".to_owned())] };
    assert_eq!(ReduceConfig::parse_str("{ a }").unwrap(), minimal.clone());

    assert_eq!(
        ReduceConfig::parse_str(
            r#"{
            a
        }"#
        )
        .unwrap(),
        minimal.clone()
    );

    assert_eq!(ReduceConfig::parse_str("a").unwrap(), minimal.clone());

    assert_eq!(
        ReduceConfig::parse_str("{a b {c}}").unwrap(),
        ReduceConfig {
            fields: vec![
                Field::Scalar("a".to_owned()),
                Field::Object("b".to_owned(), vec![Field::Scalar("c".to_owned())])
            ]
        }
    );

    assert_eq!(
        ReduceConfig::parse_str(
            r#"{
            a b{c}
            d
                {
                    e { f g }
                    j
                    k l m { o p q }
                }
            r
        }"#
        )
        .unwrap(),
        ReduceConfig {
            fields: vec![
                Field::Scalar("a".to_owned()),
                Field::Object("b".to_owned(), vec![Field::Scalar("c".to_owned())]),
                Field::Object(
                    "d".to_owned(),
                    vec![
                        Field::Object(
                            "e".to_owned(),
                            vec![Field::Scalar("f".to_owned()), Field::Scalar("g".to_owned()),]
                        ),
                        Field::Scalar("j".to_owned()),
                        Field::Scalar("k".to_owned()),
                        Field::Scalar("l".to_owned()),
                        Field::Object(
                            "m".to_owned(),
                            vec![
                                Field::Scalar("o".to_owned()),
                                Field::Scalar("p".to_owned()),
                                Field::Scalar("q".to_owned()),
                            ]
                        ),
                    ]
                ),
                Field::Scalar("r".to_owned()),
            ]
        }
    );
}

#[test]
fn test_reducer() {
    let json = json!({
        "a": true,
        "b": {
            "c": 123,
        },
        "d": {
            "e": {
                "f": "456",
                "g": null,
            },
            "j": 0,
            "k": false,
            "l": "",
            "m": {
                "o": 7,
                "p": 8,
                "q": 9,
            },
        },
        "r": null,
        "s": [1, 2, 3],
        "t": [{"a": 123, "b": "456"}, {"a": 456, "b": "123"}],
    })
    .as_object()
    .unwrap()
    .clone();

    assert_eq!(
        JsonFieldsReducer::with_config("{a}").unwrap().reduce(json.clone()).unwrap(),
        json!({ "a": true }).as_object().unwrap().clone()
    );

    assert_eq!(
        JsonFieldsReducer::with_config("b").unwrap().reduce(json.clone()).unwrap(),
        json!({
            "b": {
                "c": 123
            }
        })
        .as_object()
        .unwrap()
        .clone()
    );

    assert_eq!(
        JsonFieldsReducer::with_config("{ b { c } d { e { g } k } }")
            .unwrap()
            .reduce(json.clone())
            .unwrap(),
        json!({
            "b": {
                "c": 123,
            },
            "d": {
                "e": {
                    "g": null,
                },
                "k": false,
            }
        })
        .as_object()
        .unwrap()
        .clone()
    );

    assert_eq!(
        JsonFieldsReducer::with_config("{ r { s } }").unwrap().reduce(json.clone()).unwrap(),
        json!({ "r": null }).as_object().unwrap().clone()
    );

    assert_eq!(
        JsonFieldsReducer::with_config("{ z }").unwrap().reduce(json.clone()).unwrap(),
        json!({}).as_object().unwrap().clone()
    );

    assert_eq!(
        JsonFieldsReducer::with_config(
            r#"{
            a b{c}
            d
                {
                    e { f g }
                    j
                    k l m { o p q }
                }
            r
            s
            t { a b }
        }"#
        )
        .unwrap()
        .reduce(json.clone())
        .unwrap(),
        json
    );

    assert_eq!(
        JsonFieldsReducer::with_config("s t { a }").unwrap().reduce(json.clone()).unwrap(),
        json!({
            "s": [1, 2, 3],
            "t": [{"a": 123}, {"a": 456}],
        })
        .as_object()
        .unwrap()
        .clone()
    );
}
