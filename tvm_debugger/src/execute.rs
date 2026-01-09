use tvm_block::Deserializable;
use tvm_block::Serializable;
use tvm_block::StateInit;
use tvm_types::HashmapE;
use tvm_types::SliceData;
use tvm_vm::SmartContractInfo;
use tvm_vm::error::tvm_exception;
use tvm_vm::executor::Engine;
use tvm_vm::executor::gas::gas_state::Gas;
use tvm_vm::int;
use tvm_vm::stack::Stack;
use tvm_vm::stack::StackItem;
use tvm_vm::stack::integer::IntegerData;
use tvm_vm::stack::savelist::SaveList;

use crate::ExecutionResult;
use crate::RunArgs;
use crate::decode::decode_actions;
use crate::helper::capabilities;
use crate::helper::config_params;
use crate::helper::contract_balance;
use crate::helper::get_dest_address;
use crate::helper::get_now;
use crate::helper::load_code_and_data_from_state_init;
use crate::helper::trace_callback;
use crate::message::generate_message;

pub(crate) fn execute(args: &RunArgs, res: &mut ExecutionResult) -> anyhow::Result<()> {
    let mut contract_state_init =
        StateInit::construct_from_file(&args.input_file).map_err(|e| {
            anyhow::format_err!(
                "Failed to load state init from input file {:?}: {e}",
                args.input_file
            )
        })?;

    let (code, data) = load_code_and_data_from_state_init(&contract_state_init);

    let registers = initialize_registers(args, code.clone(), data.clone())?;
    let stack = prepare_stack(args)?;
    let gas = Gas::test();
    let library_map = HashmapE::with_hashmap(256, contract_state_init.library.root().cloned());

    let mut engine = Engine::with_capabilities(capabilities(args)).setup_with_libraries(
        code,
        Some(registers),
        Some(stack),
        Some(gas),
        vec![library_map],
    );
    engine.set_trace(0);
    if args.trace {
        engine.set_trace_callback(move |engine, info| {
            trace_callback(engine, info, true);
        })
    }

    let exit_code = engine.execute().unwrap_or_else(|error| match tvm_exception(error) {
        Ok(exception) => {
            res.log(format!("Unhandled exception: {}", exception));
            exception.exception_or_custom_code()
        }
        _ => -1,
    });

    res.exit_code(exit_code);
    res.vm_success(engine.get_committed_state().is_committed());
    res.gas_used(engine.get_gas().get_gas_used());
    res.log(format!("{}", engine.dump_stack("Post-execution stack state", false)));
    res.log(format!("{}", engine.dump_ctrls(false)));

    if res.is_vm_success {
        decode_actions(engine.get_actions(), &mut contract_state_init, args, res)?;

        contract_state_init.data = match engine.get_committed_state().get_root() {
            StackItem::Cell(root_cell) => Some(root_cell.clone()),
            _ => panic!("cannot get root data: c4 register is not a cell."),
        };
        contract_state_init
            .write_to_file(&args.input_file)
            .map_err(|e| anyhow::format_err!("Failed to save state init after execution: {e}"))?;

        res.log("Contract persistent data updated".to_string());
    }

    res.log("EXECUTION COMPLETED".to_string());

    Ok(())
}

fn initialize_registers(
    args: &RunArgs,
    code: SliceData,
    data: SliceData,
) -> anyhow::Result<SaveList> {
    let mut ctrls = SaveList::new();
    let address = SliceData::load_cell(
        get_dest_address(args)?
            .serialize()
            .map_err(|e| anyhow::format_err!("Failed to serialize address: {e}"))?,
    )
    .unwrap();
    let info = SmartContractInfo {
        capabilities: capabilities(args),
        balance: contract_balance(args),
        myself: address,
        mycode: code.into_cell(),
        unix_time: get_now(args).as_u32(),
        config_params: config_params(args),
        ..Default::default()
    };
    // TODO info.set_init_code_hash()
    ctrls
        .put(4, &mut StackItem::Cell(data.into_cell()))
        .map_err(|e| anyhow::format_err!("Failed to init register: {e}"))?;
    ctrls
        .put(7, &mut info.into_temp_data_item())
        .map_err(|e| anyhow::format_err!("Failed to init register: {e}"))?;
    Ok(ctrls)
}

fn prepare_stack(args: &RunArgs) -> anyhow::Result<Stack> {
    let mut stack = Stack::new();
    let (message, body) = generate_message(args)?;
    let msg_value = args.message_value.map(|v| v as u64).unwrap_or(0);
    let contract_balance = contract_balance(args).grams.as_u64().unwrap();

    stack
        .push(int!(contract_balance))
        .push(int!(msg_value))
        .push(StackItem::Cell(
            message
                .serialize()
                .map_err(|e| anyhow::format_err!("Failed to serialize message: {e}"))?,
        ))
        .push(StackItem::Slice(body))
        .push(int!(if args.internal {
            0
        } else if args.cross_dapp {
            -3
        } else {
            -1
        }));
    Ok(stack)
}
