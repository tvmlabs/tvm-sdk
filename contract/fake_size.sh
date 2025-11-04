# FOR THIS ONE YOU NEED TO DO produce_examples.sh
#
# this one shows content and size of a cell
# first one is NOT forged - it will show full content
# second one IS forged - it will show only first value and reports that the depth of the cell is 1, though it will return a remainder of a cell with unread content

set -o xtrace
./file_to_json_file.sh
sold contract.sol
FILE=non_forged.json
./tvm-debugger run --input-file contract.tvc -a contract.abi.json -m get_content_of_cell --call-parameters-file $FILE
FILE=forged.json
./tvm-debugger run --input-file contract.tvc -a contract.abi.json -m get_content_of_cell --call-parameters-file $FILE
