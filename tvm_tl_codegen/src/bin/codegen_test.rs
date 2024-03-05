// Copyright (C) 2019-2023 EverX. All Rights Reserved.
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
        fs::File::open(&file).unwrap().read_to_string(&mut input).unwrap();
    }

    tvm_tl_codegen::generate_code_for(None, &input, Path::new(OUTPUT_DIR));
}
