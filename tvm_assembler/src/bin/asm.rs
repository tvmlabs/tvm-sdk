// Copyright (C) 2022 TON Labs. All Rights Reserved.
//
// Licensed under the SOFTWARE EVALUATION License (the "License"); you may not
// use this file except in compliance with the License.
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific TON DEV software governing permissions and
// limitations under the License.

use std::error::Error;
use std::io::Write;
use std::process::ExitCode;

use clap::Parser;
use tvm_assembler::DbgInfo;
use tvm_assembler::Engine;
use tvm_assembler::Units;
use tvm_types::Cell;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input assembly sources
    #[arg(required = true)]
    inputs: Vec<String>,
    /// Output boc filename ("output.boc" by default)
    #[arg(short, long)]
    boc: Option<String>,
    /// Output debug map filename ("output.debug.json" by default)
    #[arg(short, long)]
    dbg: Option<String>,
}

fn main() -> ExitCode {
    if let Err(e) = main_impl() {
        eprintln!("{e}");
        ExitCode::from(1)
    } else {
        ExitCode::from(0)
    }
}

fn main_impl() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let output = args.boc.unwrap_or("output.boc".to_string());
    let dbgmap = args.dbg.unwrap_or("output.debug.json".to_string());

    let mut engine = Engine::new("");

    let mut units = Units::new();
    for input in args.inputs {
        let code = std::fs::read_to_string(input.clone())?;
        engine.reset(input);
        units = engine.compile_toplevel(&code).map_err(|e| e.to_string())?;
    }
    let (b, d) = units.finalize();

    let c = b.into_cell()?;
    write_boc(&c, &output)?;

    let dbg = DbgInfo::from(c, d);
    write_dbg(dbg, &dbgmap)?;

    Ok(())
}

fn write_boc(cell: &Cell, output: &str) -> Result<(), Box<dyn Error>> {
    let bytes = tvm_types::write_boc(cell)?;
    let mut file = std::fs::File::create(output)?;
    file.write_all(&bytes)?;
    Ok(())
}

fn write_dbg(dbg: DbgInfo, output: &str) -> Result<(), Box<dyn Error>> {
    let json = serde_json::to_string_pretty(&dbg)?;
    let mut file = std::fs::File::create(output)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}
