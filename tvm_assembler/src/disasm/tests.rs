// Copyright 2018-2023 TON DEV SOLUTIONS LTD.
//
// Licensed under the SOFTWARE EVALUATION License (the "License"); you may not
// use this file except in compliance with the License.
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific TON DEV software governing permissions and
// limitations under the License.

use failure::format_err;
use similar::ChangeTag;
use similar::TextDiff;
use tvm_types::read_boc;
use tvm_types::write_boc;
use tvm_types::SliceData;
use tvm_types::Status;

use crate::disasm::disasm;
use crate::disasm::fmt::print_tree_of_cells;

fn cut_asm_hashes(asm: String) -> String {
    let mut out = String::new();
    for line in asm.lines() {
        if let Some((before, _)) = line.split_once(" ;; #") {
            out += &format!("{}\n", before);
        } else {
            out += &format!("{}\n", line);
        }
    }
    out
}

fn round_trip_test(filename: &str, check_bin: bool) -> Status {
    let bin0 = &std::fs::read(filename)?;
    let toc0 = read_boc(bin0)?.withdraw_single_root()?;
    let mut asm0 = disasm(&mut SliceData::load_cell(toc0.clone())?)?;
    let toc1 = crate::compile_code_to_cell(&asm0.clone()).map_err(|e| anyhow::anyhow!("{}", e))?;
    let mut asm1 = disasm(&mut SliceData::load_cell(toc1.clone())?)?;

    if !check_bin {
        asm0 = cut_asm_hashes(asm0);
        asm1 = cut_asm_hashes(asm1);
    }

    let diff = TextDiff::from_lines(&asm0, &asm1);
    let mut differ = false;
    for change in diff.iter_all_changes() {
        match change.tag() {
            ChangeTag::Delete => {
                print!("-{}", change);
                differ = true;
            }
            ChangeTag::Insert => {
                print!("+{}", change);
                differ = true;
            }
            _ => (),
        }
    }
    assert!(!differ, "roundtrip difference was detected for {}", filename);

    if check_bin {
        let bin1 = write_boc(&toc1)?;
        if bin0 != &bin1 {
            println!("{}", asm0);
            print_tree_of_cells(&toc0);
            print_tree_of_cells(&toc1);
            assert!(false);
        }
    }
    Ok(())
}

#[test]
fn round_trip() {
    let mut indices = (0..130).collect::<Vec<i32>>();
    indices.append(&mut (200..331).collect());
    for n in indices {
        round_trip_test(&format!("src/tests/disasm/roundtrip/{:03}.boc", n), true).unwrap();
    }
}

fn check_fragment(code: &str, text: &str) -> Status {
    let builder = SliceData::from_string(code)?.as_builder();
    let mut slice = SliceData::load_builder(builder)?;
    let text_disasm = disasm(&mut slice)?;
    assert_eq!(text, &text_disasm);
    Ok(())
}

#[test]
fn fragments() -> Status {
    check_fragment("70", "PUSHINT 0\n")?;
    check_fragment("88", "PUSHREF {\n  ;; missing cell\n}\n")?;
    check_fragment("8b04", "PUSHSLICE x4_\n")?;
    check_fragment("8c0800000000", "PUSHSLICE x000000004_\n")?;
    check_fragment("8c40", "PUSHSLICE x4_ ;; missing 1 ref\n")?;
    check_fragment("8c80", "PUSHSLICE x4_ ;; missing 2 refs\n")?;
    check_fragment("8e80", "PUSHCONT {\n} ;; missing 1 ref\n")?;
    check_fragment("8e81", "PUSHCONT {\n} ;; missing 8 bits and 1 ref\n")?;
    check_fragment("920000", "PUSHCONT {\n  NOP\n  NOP\n}\n")?;
    check_fragment("e300", "IFREF {\n  ;; missing cell\n}\n")?;
    check_fragment("e30f", "IFREFELSEREF {\n  ;; missing cell\n}{\n  ;; missing cell\n}\n")?;
    check_fragment("f4a420", "DICTPUSHCONST 32 ;; missing dict ref\n")?;
    check_fragment("ff77", "SETCP 119\n")?;
    Ok(())
}

fn check_code(name: &str) -> Status {
    let inp = std::fs::read_to_string(format!("src/tests/disasm/{}.in", name))?;
    let out = std::fs::read_to_string(format!("src/tests/disasm/{}.out", name))?;
    let mut code = crate::compile_code(&inp).map_err(|e| anyhow::anyhow!("{}", e))?;
    let dis = disasm(&mut code)?;
    if dis == out {
        return Ok(());
    }
    let diff = TextDiff::from_lines(&out, &dis);
    for change in diff.iter_all_changes() {
        match change.tag() {
            ChangeTag::Delete => print!("-{}", change),
            ChangeTag::Insert => print!("+{}", change),
            _ => (),
        }
    }
    Err(anyhow::anyhow!("check failed"))
}

#[test]
fn codes() -> Status {
    check_code("code-dict-1.code")?;
    Ok(())
}
