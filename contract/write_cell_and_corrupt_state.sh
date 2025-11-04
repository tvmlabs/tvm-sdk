# FOR THIS ONE YOU NEED TO DO produce_examples.sh

# this one will write BOC3 cell into state
# so every single call after that will fail, unable to read state

set -o xtrace
./file_to_json_file.sh
sold contract.sol
FILE=non_forged.json
./tvm-debugger run --input-file contract.tvc -a contract.abi.json -m just_write_cell --call-parameters-file $FILE
./tvm-debugger run --input-file contract.tvc -a contract.abi.json -m doNothing -p '{}'
