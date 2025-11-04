# FOR THIS ONE YOU NEED TO DO produce_huge_examples.sh
# this one shows that ordinary cell (ord.json) of size bigger than limits
# will fail
# but huge boc3 will process as if there were no limits

set -o xtrace
./file_to_json_file.sh
sold contract.sol
FILE=ord.json
./tvm-debugger run --input-file contract.tvc -a contract.abi.json -m get_content_of_cell --call-parameters-file $FILE
FILE=non_forged.json
./tvm-debugger run --input-file contract.tvc -a contract.abi.json -m get_content_of_cell --call-parameters-file $FILE
