use std::path::PathBuf;
use std::str::FromStr;

use tvm_abi::decode_function_response;
use tvm_block::CommonMsgInfo;
use tvm_block::CurrencyCollection;
use tvm_block::Deserializable;
use tvm_block::Grams;
use tvm_block::Message;
use tvm_block::MsgAddressInt;
use tvm_block::OutAction;
use tvm_block::OutActions;
use tvm_block::Serializable;
use tvm_block::StateInit;
use tvm_types::BuilderData;
use tvm_types::Cell;
use tvm_types::SliceData;
use tvm_types::base64_encode;
use tvm_types::write_boc;
use tvm_vm::stack::StackItem;

use crate::ExecutionResult;
use crate::RunArgs;
use crate::helper::load_abi_as_string;

pub(crate) fn decode_body(
    abi_file: &PathBuf,
    function: &str,
    body: SliceData,
    internal: bool,
    res: &mut ExecutionResult,
) -> anyhow::Result<()> {
    let response =
        decode_function_response(&load_abi_as_string(abi_file)?, function, body, internal, false)
            .map_err(|e| anyhow::format_err!("Failed to decode function response: {e}"))?;
    res.response(response);
    Ok(())
}

pub(crate) fn decode_actions(
    actions: StackItem,
    state: &mut StateInit,
    args: &RunArgs,
    res: &mut ExecutionResult,
) -> anyhow::Result<()> {
    let abi_file = args.abi_file.as_ref();
    let function_name = args.function_name.as_ref();
    let address =
        args.address.as_ref().map(|s| MsgAddressInt::from_str(s).unwrap()).unwrap_or_default();
    if let StackItem::Cell(ref cell) = actions {
        let actions: OutActions = OutActions::construct_from(
            &mut SliceData::load_cell(cell.clone())
                .map_err(|e| anyhow::format_err!("SliceData::load_cell: {e}"))?,
        )
        .map_err(|e| anyhow::format_err!("OutActions::construct_from: {e}"))?;
        res.log("Output actions:\n----------------".to_string());
        let mut created_lt = 1;
        for act in actions {
            match act {
                OutAction::SendMsg { mode: _, mut out_msg } => {
                    if out_msg.is_internal() {
                    if out_msg.is_internal() || out_msg.is_cross_dapp() {
                        out_msg.set_src_address(address.clone());
                        out_msg.set_at_and_lt(0, created_lt);
                        created_lt += 1;
                    }
                    res.add_out_message(out_msg.clone());
                    res.log(format!("Action(SendMsg):\n{}", msg_printer(&out_msg)?));
                    if let Some(b) = out_msg.body() {
                        if abi_file.is_some() && function_name.is_some() && !(out_msg.is_internal() || out_msg.is_cross_dapp()){
                            decode_body(
                                abi_file.unwrap(),
                                function_name.unwrap(),
                                b,
                                out_msg.is_internal(),
                                res,
                            )?;
                        }
                    }
                }
                OutAction::SetCode { new_code: code } => {
                    res.log("Action(SetCode)".to_string());
                    state.code = Some(code);
                }
                OutAction::ReserveCurrency { .. } => {
                    res.log("Action(ReserveCurrency)".to_string());
                }
                OutAction::ChangeLibrary { .. } => {
                    res.log("Action(ChangeLibrary)".to_string());
                }
                _ => res.log("Action(Unknown)".to_string()),
            };
        }
    }
    Ok(())
}

pub fn msg_printer(msg: &Message) -> anyhow::Result<String> {
    let mut b = BuilderData::new();
    msg.write_to(&mut b).map_err(|e| anyhow::format_err!("{e}"))?;
    let bytes = write_boc(&b.into_cell().map_err(|e| anyhow::format_err!("{e}"))?)
        .map_err(|e| anyhow::format_err!("{e}"))?;
    Ok(format!(
        "message header\n{}init  : {}\nbody  : {}\nbody_hex: {}\nbody_base64: {}\nboc_base64: {}\n",
        print_msg_header(msg.header()),
        msg.state_init()
            .as_ref()
            .map(|x| { state_init_printer(x) })
            .unwrap_or_else(|| "None".to_string()),
        match msg.body() {
            Some(slice) => format!("{:.2}", slice.into_cell()),
            None => "None".to_string(),
        },
        msg.body()
            .map(|b| hex::encode(write_boc(&b.into_cell()).unwrap()))
            .unwrap_or_else(|| "None".to_string()),
        tree_of_cells_into_base64(msg.body().map(|slice| slice.into_cell()).as_ref(),),
        base64_encode(bytes),
    ))
}

pub fn state_init_printer(state: &StateInit) -> String {
    let serialized_data = state.write_to_bytes().unwrap();
    format!(
        "StateInit\n serialized_data: {}\n split_depth: {}\n special: {}\n data: {}\n code: {}\n code_hash: {}\n data_hash: {}\n code_depth: {}\n data_depth: {}\n version: {}\n lib:  {}\n",
        hex::encode(&serialized_data),
        state.split_depth.as_ref().map_or("None".to_string(), |x| x.as_u32().to_string()),
        state.special.as_ref().map_or("None".to_string(), ToString::to_string),
        tree_of_cells_into_base64(state.data.as_ref()),
        tree_of_cells_into_base64(state.code.as_ref()),
        state
            .code
            .as_ref()
            .map(|code| code.repr_hash().to_hex_string())
            .unwrap_or_else(|| "None".to_string()),
        state
            .data
            .as_ref()
            .map(|code| code.repr_hash().to_hex_string())
            .unwrap_or_else(|| "None".to_string()),
        state
            .code
            .as_ref()
            .map(|code| code.repr_depth().to_string())
            .unwrap_or_else(|| "None".to_string()),
        state
            .data
            .as_ref()
            .map(|code| code.repr_depth().to_string())
            .unwrap_or_else(|| "None".to_string()),
        get_version_mycode_aware(state.code.as_ref()).unwrap_or_else(|_| "None".to_string()),
        tree_of_cells_into_base64(state.library.root()),
    )
}

pub fn tree_of_cells_into_base64(root_cell: Option<&Cell>) -> String {
    match root_cell {
        Some(cell) => match write_boc(cell) {
            Ok(bytes) => base64_encode(bytes),
            Err(_) => "None".to_string(),
        },
        None => "None".to_string(),
    }
}

pub fn get_version_mycode_aware(root: Option<&Cell>) -> anyhow::Result<String> {
    let root = root.ok_or_else(|| anyhow::format_err!("not found (empty root)"))?;
    match get_version(root) {
        Ok(res) => Ok(res),
        Err(_) => {
            let root = root.reference(1).map_err(|e| anyhow::format_err!("{e}"))?;
            get_version(&root)
        }
    }
}

fn get_version(root: &Cell) -> anyhow::Result<String> {
    let cell1 = root.reference(0).map_err(|e| anyhow::format_err!("not found ({})", e))?;
    let cell2 = cell1.reference(1).map_err(|e| anyhow::format_err!("not found ({})", e))?;
    let bytes = cell2.data();
    match String::from_utf8(bytes.to_vec()) {
        Ok(string) => {
            if string.is_empty() {
                Ok("<empty>".to_string())
            } else {
                Ok(string)
            }
        }
        Err(e) => Err(anyhow::format_err!("decoding failed ({})", e)),
    }
}

fn print_msg_header(header: &CommonMsgInfo) -> String {
    match header {
        CommonMsgInfo::IntMsgInfo(header) => {
            format!("   ihr_disabled: {}\n", header.ihr_disabled)
                + &format!("   bounce      : {}\n", header.bounce)
                + &format!("   bounced     : {}\n", header.bounced)
                + &format!("   source      : {}\n", &header.src)
                + &format!("   destination : {}\n", &header.dst)
                + &format!("   value       : {}\n", print_cc(&header.value))
                + &format!("   ihr_fee     : {}\n", print_grams(&header.ihr_fee))
                + &format!("   fwd_fee     : {}\n", print_grams(&header.fwd_fee))
                + &format!("   created_lt  : {}\n", header.created_lt)
                + &format!("   created_at  : {}\n", header.created_at)
        }
        CommonMsgInfo::CrossDappMessageInfo(header) => {
            format!("   bounce      : {}\n", header.bounce)
                + &format!("   bounced     : {}\n", header.bounced)
                + &format!("   source      : {}\n", &header.src)
                + &format!("   source_dapp : {}\n", &header.src_dapp_id)
                + &format!("   destination : {}\n", &header.dst)
                + &format!("   dest_dapp   : {}\n", &header.dest_dapp_id)
                + &format!("   value       : {}\n", print_cc(&header.value))
                + &format!("   ihr_fee     : {}\n", print_grams(&header.ihr_fee))
                + &format!("   fwd_fee     : {}\n", print_grams(&header.fwd_fee))
                + &format!("   created_lt  : {}\n", header.created_lt)
                + &format!("   created_at  : {}\n", header.created_at)
        }
        CommonMsgInfo::ExtInMsgInfo(header) => {
            format!("   source      : {}\n", &header.src)
                + &format!("   destination : {}\n", &header.dst)
                + &format!("   import_fee  : {}\n", print_grams(&header.import_fee))
        }
        CommonMsgInfo::ExtOutMsgInfo(header) => {
            format!("   source      : {}\n", &header.src)
                + &format!("   destination : {}\n", &header.dst)
                + &format!("   created_lt  : {}\n", header.created_lt)
                + &format!("   created_at  : {}\n", header.created_at)
        }
    }
}

fn print_grams(grams: &Grams) -> String {
    grams.to_string()
}

fn print_cc(cc: &CurrencyCollection) -> String {
    let mut result = print_grams(&cc.grams);
    if !cc.other.is_empty() {
        result += " other: {";
        cc.other
            .iterate_with_keys(|key: u32, value| {
                result += &format!(" \"{}\": \"{}\",", key, value);
                Ok(true)
            })
            .ok();
        result.pop(); // remove extra comma
        result += " }";
    }
    result
}
