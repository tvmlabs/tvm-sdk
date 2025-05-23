// Copyright 2023 TON DEV SOLUTIONS LTD.
//
// Licensed under the SOFTWARE EVALUATION License (the "License"); you may not
// use this file except in compliance with the License.
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific TON DEV software governing permissions and
// limitations under the License.

use tvm_types::Cell;
use tvm_types::Result;
use tvm_types::SliceData;

use super::codedict::DelimitedHashmapE;
use super::types::Code;
use super::types::Instruction;
use super::types::InstructionParameter;

pub fn print_tree_of_cells(toc: &Cell) {
    fn print_tree_of_cells(cell: &Cell, prefix: String, last: bool) {
        let indent = if last { "└ " } else { "├ " };
        let mut hex = cell.to_hex_string(true);
        if !hex.is_empty() {
            let mut first = true;
            let indent_next = if !last { "│ " } else { "  " };
            while hex.len() > 64 {
                let tail = hex.split_off(64);
                println!("{}{}{}…", prefix, if first { indent } else { indent_next }, hex);
                hex = tail;
                first = false;
            }
            println!("{}{}{}", prefix, if first { indent } else { indent_next }, hex);
        } else {
            println!("{prefix}{indent}8_");
        }

        let prefix_child = if last { "  " } else { "│ " };
        let prefix = prefix + prefix_child;
        if cell.references_count() > 0 {
            let last_child = cell.references_count() - 1;
            for i in 0..cell.references_count() {
                let child = cell.reference(i).unwrap();
                print_tree_of_cells(&child, prefix.to_string(), i == last_child);
            }
        }
    }
    print_tree_of_cells(toc, "".to_string(), true);
}

fn print_code_dict(cell: &Cell, key_size: usize, indent: &str) -> Result<String> {
    let mut map = DelimitedHashmapE::new(cell.clone(), key_size);
    map.mark()?;
    Ok(map.print(indent))
}

fn print_dictpushconst(insn: &Instruction, indent: &str) -> String {
    let key_length = if let Some(InstructionParameter::Length(l)) = insn.params().first() {
        *l
    } else {
        unreachable!()
    };
    let cell = if let Some(InstructionParameter::Cell { cell, collapsed }) = insn.params().get(1) {
        assert!(collapsed == &false);
        cell.as_ref()
    } else {
        unreachable!()
    };
    if let Some(cell) = cell {
        let text = if let Some(InstructionParameter::CodeDictMarker) = insn.params().get(2) {
            print_code_dict(cell, key_length, indent)
                .unwrap_or_else(|_| print_cell(cell, indent, true))
        } else {
            print_cell(cell, indent, true)
        };
        format!("{} {}\n{}", insn.name(), key_length, text)
    } else {
        format!("{} {} ;; missing dict ref\n", insn.name(), key_length)
    }
}

fn print_cell(cell: &Cell, indent: &str, dot_cell: bool) -> String {
    let mut text = String::new();
    if dot_cell {
        text += &format!("{indent}.cell ");
    }
    text += &format!("{{ ;; #{}\n", cell.repr_hash().to_hex_string());
    let inner_indent = String::from("  ") + indent;
    if cell.bit_length() > 0 {
        text += &format!("{}.blob x{}\n", inner_indent, cell.to_hex_string(true));
    }
    let refs = cell.references_count();
    for i in 0..refs {
        text += &print_cell(&cell.reference(i).unwrap(), &inner_indent, true);
    }
    text += &format!("{indent}}}");
    if dot_cell {
        text += "\n";
    }
    text
}

fn truncate(s: String, n: usize) -> String {
    match s.char_indices().nth(n) {
        None => s,
        Some((idx, _)) => String::from(&s[..idx]),
    }
}

fn print_bytecode(slice: Option<(&SliceData, usize)>, bytecode_width: usize) -> String {
    let mut text = String::new();
    if bytecode_width > 0 {
        let mut bytecode = String::new();
        if let Some((slice, refs)) = slice {
            let mut b = slice.to_hex_string();
            if refs > 0 {
                b += &format!(" {{{refs}r}}");
            }
            bytecode = truncate(b, bytecode_width);
        }
        text += &format!("{bytecode:<bytecode_width$} │ ");
    }
    text
}

impl Code {
    pub fn print(&self, indent: &str, full: bool, bytecode_width: usize) -> String {
        let mut text = String::new();
        for insn in self.iter() {
            text += &print_bytecode(insn.bytecode().map(|v| (v, insn.refs())), bytecode_width);
            text += indent;
            if full {
                match insn.name() {
                    "DICTPUSHCONST" | "PFXDICTSWITCH" => {
                        // TODO better improve assembler for these two insns
                        text += &print_dictpushconst(insn, indent);
                        continue;
                    }
                    "IMPLICIT-JMP" => {
                        if let Some(InstructionParameter::Code { code, cell }) =
                            insn.params().first()
                        {
                            let hash = cell.as_ref().unwrap().repr_hash().to_hex_string();
                            text += &format!(".cell {{ ;; #{hash}\n");
                            let inner_indent = String::from("  ") + indent;
                            text += &code.print(&inner_indent, full, bytecode_width);
                            text += indent;
                            text += "}\n";
                        } else {
                            unreachable!()
                        }
                        continue;
                    }
                    _ => (),
                }
            }
            text += insn.name();
            if insn.is_quiet() {
                text += "Q";
            }
            text += &print_insn_params(insn.params(), indent, full, bytecode_width);
            if let Some(comment) = insn.comment() {
                text += &format!(" ;; {comment}");
            }
            text += "\n";
        }
        text
    }
}

fn print_insn_params(
    params: &[InstructionParameter],
    indent: &str,
    full: bool,
    bytecode_width: usize,
) -> String {
    use InstructionParameter::*;

    let mut text = String::new();
    let len = params.len();
    if len > 0 {
        text += " ";
    }
    for (index, param) in params.iter().enumerate() {
        let last = len == (index + 1);
        let mut curr_is_block = false;
        match param {
            BigInteger(i) => {
                text += &format!("{i}");
            }
            ControlRegister(c) => {
                text += &format!("c{c}");
            }
            Integer(i) => {
                text += &format!("{i}");
            }
            Length(l) => {
                text += &format!("{l}");
            }
            LengthAndIndex(l, i) => {
                text += &format!("{l}, {i}");
            }
            Nargs(n) => {
                text += &format!("{n}");
            }
            Pargs(p) => {
                text += &format!("{p}");
            }
            Rargs(r) => {
                text += &format!("{r}");
            }
            Slice(s) => {
                // TODO slice may have references
                debug_assert!(s.remaining_references() == 0);
                text += &format!("x{}", s.to_hex_string());
            }
            StackRegister(r) => {
                text += &format!("s{r}");
            }
            StackRegisterPair(ra, rb) => {
                text += &format!("s{ra}, s{rb}");
            }
            StackRegisterTriple(ra, rb, rc) => {
                text += &format!("s{ra}, s{rb}, s{rc}");
            }
            Code { code, cell } => {
                if full {
                    if let Some(cell) = cell {
                        text += &format!("{{ ;; #{}\n", cell.repr_hash().to_hex_string());
                    } else {
                        text += "{\n";
                    }
                    let inner_indent = String::from("  ") + indent;
                    text += &code.print(&inner_indent, full, bytecode_width);
                    text += &print_bytecode(None, bytecode_width);
                    text += indent;
                    text += "}";
                    curr_is_block = true;
                }
            }
            Cell { cell, collapsed } => {
                if full {
                    if *collapsed {
                        text += "<collapsed>";
                    } else if let Some(cell) = cell {
                        text += &print_cell(cell, indent, false);
                    } else {
                        text += "{\n";
                        text += &print_bytecode(None, bytecode_width);
                        text += &format!("{indent}  ;; missing cell\n");
                        text += &print_bytecode(None, bytecode_width);
                        text += indent;
                        text += "}";
                    }
                    curr_is_block = true;
                }
            }
            CodeDictMarker => {
                // markers must have been already eliminated
                unreachable!()
            }
        }
        if !last && !curr_is_block {
            text += ", ";
        }
    }
    text
}
