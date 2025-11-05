# FOR THIS ONE YOU NEED TO DO produce_examples.sh

# this one will write BOC3 cell into state
# so every single call after that will fail, unable to read state

# set -o xtrace
# ./file_to_json_file.sh
# sold contract.sol
# FILE=non_forged.json
# ./tvm-debugger run --input-file contract.tvc -a contract.abi.json -m just_write_cell --call-parameters-file $FILE
# ./tvm-debugger run --input-file contract.tvc -a contract.abi.json -m doNothing -p '{}'

import os
import sys
import shutil
import time
from dataclasses import dataclass

#sys.path.append("tests")
from helper import common


GIVER_ADDRESS = "0:1111111111111111111111111111111111111111111111111111111111111111"
EPHEMERAL_KEY_PATH = "epk.keys.json"
TEST_KEY_PATH = "test_key.keys.json"
TEST_KEY_2_PATH = "test_key_2.keys.json"
TEST_KEY_3_PATH = "test_key_3.keys.json"
DEST_KEY_PATH =  "dest_key.keys.json"
GIVER_ABI = "tests/GiverV3.abi.json"
GIVER_KEY_PATH = "tests/GiverV3.keys.json"

WALLET_INIT_BALANCE = 10000000000000
WALLET_INIT_CC = 100000000000000
ECC_KEY = "2"
INDEX_MOD_4 = 1

DUMMY_PUB_RECOVERY_KEY = 100000000000000
DUMMY_PUB_RECOVERY_KEY_IN_STRING_HEX = "0x00000000000000000000000000000000000000000000000000005af3107a4000"
ZKADDR = "19014913083821391659776715405561365555081978517533943138452652822856495767863"

ERR_CONTRACT_EXEC = 414

PARAMS_SHORT_1 = {"cell":"rMOnKAAAAAEAAAD9EAIj2GJsSpBGnbAghcuvbunaisCBjH9MB+lw5W8WQUgOcQAAGAAAAAEAAAAIEQLUlphkKPfJdSsmrx7H3owVuPzsV7D2MZwwDhiv/EOoeAABKgAAAAwAAAABAAAACBECOL5JNyAw52Fu4b4hW9uRNJBjiq7DB4q1OfvJMdEnOt8AAioAAAA5AAAAAQAAAAgRAmJCc3w3zIcZ7LfazHp6hu7a3ruLUbgs+vdzBIeiJ3ZGAAMqAAAAagAAAAEAAAAIEQL64Hjc7Vo4jmAzHBcTHiYcNhwTGqs2hGxFlcsObIhvtAAEKgAAAJsAAAABAAAACBECs7JBqhCm1FprjcPyvJiFX7QO0kh8z6BLykBNVWTwfXYABSoAAADMAAAAAQAAAAg="}


def test_boc_recursive_cells_on_deployed():
    MULTIFACTOR_CONTRACT_PATH = "contracts/wasm/boc3_tester"
    MULTIFACTOR_ABI = "boc3_tester.abi.json"
    KEY_PATH = "contracts/wasm/boc3_tester.keys.json"

    def init(zkaddr, pub_recovery_key):
        print("start")
        common.set_config({"async_call": "false"})
        common.setup()
        address = common.generate_address(MULTIFACTOR_CONTRACT_PATH, KEY_PATH)
        print(address)
        print("Address of contract " + address)
        params = {"dest": address, "value": WALLET_INIT_BALANCE, "flag": 16, "ecc": {ECC_KEY: WALLET_INIT_CC}}
        res = common.call_contract(GIVER_ADDRESS, GIVER_ABI, GIVER_KEY_PATH, "sendCurrencyWithFlag", params)
        print(res)
        common.wait_account_uninit(address)

        key = common.read_public_key(KEY_PATH)
        params = common.format_params({"owners_pubkey":[f"0x{key}"], "owners_address": [],"reqConfirms":1,"reqConfirmsData":1,"value":100000000})
        res = common.execute_cli_cmd(f"deployx --abi {MULTIFACTOR_CONTRACT_PATH}.abi.json --keys {KEY_PATH} {MULTIFACTOR_CONTRACT_PATH}.tvc {params}")
        print(res)
        time.sleep(2)
        common.wait_account_active(address)
        result = common.is_account_active(address)
        assert result == True
        return address

    address = init(ZKADDR, DUMMY_PUB_RECOVERY_KEY)
    #print(common.execute_cmd_without_exit("pwd"))
    params = common.format_params(PARAMS_SHORT_1)
    res = common.execute_cli_cmd_without_exit(f"callx --addr {address} --keys {KEY_PATH} --abi {MULTIFACTOR_CONTRACT_PATH}.abi.json -m just_write_cell {params}")
    print(res)
  
    time.sleep(10)
    res = common.execute_cli_cmd_without_exit(f"callx --addr {address} --keys {KEY_PATH} --abi {MULTIFACTOR_CONTRACT_PATH}.abi.json -m doNothing {params}")
    print(res)
  
    #res = common.run_getter(address, f"{MULTIFACTOR_CONTRACT_PATH}.abi.json", "data")
    print(res)
    assert res.get('Error', None).get('code', None) == ERR_CONTRACT_EXEC

    print("Finish test_boc_recursive_cells_on_deployed")


if __name__ == "__main__":
    test_boc_recursive_cells_on_deployed()
