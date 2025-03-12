// Copyright (C) 2019-2022 TON Labs. All Rights Reserved.
//
// Licensed under the SOFTWARE EVALUATION License (the "License"); you may not
// use this file except in compliance with the License.
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific TON DEV software governing permissions and
// limitations under the License.

use std::fs;
use std::io::Read;
use std::path;
use std::path::Path;

use tvm_tl_codegen::Config;

const OUTPUT_DIR: &str = "tvm_api/src/ton";
const TL_DIR: &str = "tvm_api/tl";

fn main() {
    let mut files = fs::read_dir(TL_DIR)
        .unwrap_or_else(|_| panic!("Unable to read directory contents: {}", TL_DIR))
        .filter_map(Result::ok)
        .map(|d| d.path())
        .filter(|path| path.to_str().unwrap().ends_with(".tl"))
        .collect::<Vec<path::PathBuf>>();

    assert!(!files.is_empty());
    files.sort();

    let mut input = String::new();
    for file in files {
        if !input.is_empty() {
            input += "---types---\n";
        }
        fs::File::open(&file)
            .unwrap_or_else(|_| {
                panic!("Unable to open file for reading: {}", file.to_string_lossy())
            })
            .read_to_string(&mut input)
            .unwrap_or_else(|_| panic!("Unable to read file contents: {}", file.to_string_lossy()));
    }

    let config_path = Path::new(TL_DIR).join("codegen.json");
    let config: Option<Config> = if config_path.exists() && config_path.is_file() {
        let mut config_string = String::new();
        fs::File::open(&config_path)
            .unwrap_or_else(|_| {
                panic!("Unable to open file for reading: {}", config_path.to_string_lossy())
            })
            .read_to_string(&mut config_string)
            .unwrap_or_else(|_| {
                panic!("Unable to read file contents: {}", config_path.to_string_lossy())
            });
        Some(serde_json::from_str(&config_string).unwrap_or_else(|_| {
            panic!("Unable to parse file as JSON: {}", config_path.to_string_lossy())
        }))
    } else {
        None
    };

    tvm_tl_codegen::generate_code_for(config, &input, Path::new(OUTPUT_DIR));
}
