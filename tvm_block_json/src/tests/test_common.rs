/*
 * Copyright (C) 2019-2021 TON Labs. All Rights Reserved.
 *
 * Licensed under the SOFTWARE EVALUATION License (the "License"); you may not use
 * this file except in compliance with the License.  You may obtain a copy of the
 * License at:
 *
 * https://www.ton.dev/licenses
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific TON DEV software governing permissions and limitations
 * under the License.
 */

fn assert_json_eq(json: &str, expected: &str, name: &str) {
    let expected = expected.replace("\r", "");
    let expected = if let Some(expected) = expected.strip_suffix("\n") {
        expected.to_string()
    } else {
        expected
    };
    if json != expected {
        std::fs::write(format!("target/{}.json", name), &json).unwrap();
        panic!("json != expected")
    }
}
