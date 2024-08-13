use tvm_block::{StateInit, Deserializable, Serializable};
use tvm_types::{HashmapE, SliceData};
use tvm_vm::{int, SmartContractInfo};
use tvm_vm::error::tvm_exception;
use tvm_vm::executor::Engine;
use tvm_vm::executor::gas::gas_state::Gas;
use tvm_vm::stack::savelist::SaveList;
use tvm_vm::stack::{Stack, StackItem};
use tvm_vm::stack::integer::IntegerData;
use crate::Args;
use crate::decode::decode_actions;
use crate::helper::{capabilities, config_params, contract_balance, get_dest_address, get_now, load_code_and_data_from_state_init, trace_callback};
use crate::message::generate_message;

pub(crate) fn execute(args: &Args) -> anyhow::Result<()> {
    let mut contract_state_init = StateInit::construct_from_file(&args.input_file)
        .map_err(|e| anyhow::format_err!("Failed to load state init from input file {:?}: {e}",
            args.input_file))?;

    let (code, data) = load_code_and_data_from_state_init(&contract_state_init);

    let registers = initialize_registers(args, code.clone(), data.clone())?;
    let stack = prepare_stack(args)?;
    let gas = Gas::test();
    let library_map = HashmapE::with_hashmap(256, contract_state_init.library.root().cloned());

    let mut engine = Engine::with_capabilities(capabilities(args))
        .setup_with_libraries(
            code, Some(registers), Some(stack), Some(gas), vec![library_map]
        );
    engine.set_trace(0);
    if args.trace {
        engine.set_trace_callback(move |engine, info| { trace_callback(engine, info, true); })
    }

    let exit_code = engine.execute().unwrap_or_else(|error| match tvm_exception(error) {
        Ok(exception) => {
            println!("Unhandled exception: {}", exception);
            exception.exception_or_custom_code()
        }
        _ => -1
    });

    let is_vm_success = engine.get_committed_state().is_committed();
    println!("TVM terminated with exit code {}", exit_code);
    println!("Computing phase is success: {}", is_vm_success);
    println!("Gas used: {}", engine.get_gas().get_gas_used());
    println!();
    println!("{}", engine.dump_stack("Post-execution stack state", false));
    println!("{}", engine.dump_ctrls(false));


    if is_vm_success {
        decode_actions(engine.get_actions(), &mut contract_state_init, args)?;

        contract_state_init.data = match engine.get_committed_state().get_root() {
            StackItem::Cell(root_cell) => Some(root_cell.clone()),
            _ => panic!("cannot get root data: c4 register is not a cell."),
        };
        contract_state_init.write_to_file(&args.input_file)
            .map_err(|e| anyhow::format_err!("Failed to save state init after execution: {e}"))?;

        println!("Contract persistent data updated");
    }

    println!("EXECUTION COMPLETED");

    Ok(())
}

fn initialize_registers(args: &Args, code: SliceData, data: SliceData) -> anyhow::Result<SaveList> {
    let mut ctrls = SaveList::new();
    let address = get_dest_address(args)?.get_address();
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
    ctrls.put(4, &mut StackItem::Cell(data.into_cell()))
        .map_err(|e| anyhow::format_err!("Failed to init register: {e}"))?;
    ctrls.put(7, &mut info.into_temp_data_item())
        .map_err(|e| anyhow::format_err!("Failed to init register: {e}"))?;
    Ok(ctrls)
}

fn prepare_stack(args: &Args) -> anyhow::Result<Stack> {
    let mut stack = Stack::new();
    let (message, body) = generate_message(args)?;
    let msg_value = args.message_value.map(|v| v as u64).unwrap_or(0);
    let contract_balance = contract_balance(args).grams.as_u64().unwrap();

    stack
        .push(int!(contract_balance))
        .push(int!(msg_value))
        .push(StackItem::Cell(message.serialize().map_err(|e| anyhow::format_err!("Failed to serialize message: {e}"))?))
        .push(StackItem::Slice(body))
        .push(int!(if args.internal { 0 } else { -1 }));
    Ok(stack)
}
