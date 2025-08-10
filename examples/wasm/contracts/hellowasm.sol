pragma tvm-solidity >=0.76.1;
pragma AbiHeader expire;

interface IHelloWorld {
    function touch() external;
}


// This is class that describes you smart contract.
contract helloWorld {
    // Contract can have an instance variables.
    // In this example instance variable `timestamp` is used to store the time of `constructor` or `touch`
    // function call
    uint32 public timestamp;

    // The contract can have a `constructor` – a function that is called when the contract is deployed to the blockchain.
    // Parameter `value` represents the number of SHELL tokens to be converted to VMSHELL to pay the transaction fee.
    // In this example, the constructor stores the current timestamp in an instance variable.
    // All contracts need to call `tvm.accept()` for a successful deployment.
    constructor(uint64 value) {
        // Call the VM command to convert SHELL tokens to VMSHELL tokens to pay the transaction fee.
        gosh.cnvrtshellq(value);

        // Ensure that the contract's public key is set.
        require(tvm.pubkey() != 0, 101);

        // The current smart contract agrees to buy some gas to complete the
        // current transaction. This action is required to process external
        // messages, which carry no value (and therefore no gas).
        tvm.accept();

        // Set the instance variable to the current block timestamp.
        timestamp = block.timestamp;
    }

    // Converts SHELL to VMSHELL for payment of transaction fees
    // Parameter `value`- the amount of SHELL tokens that will be exchanged 1-to-1 into VMSHELL tokens.
    function exchangeToken(uint64 value) public pure {
        tvm.accept();
        getTokens();
        gosh.cnvrtshellq(value);
    }

    // Returns a static message, "helloWorld".
    // This function serves as a basic example of returning a fixed string in Solidity.
    function renderHelloWorld () public pure returns (string) {
        return 'helloWorld';
    }

    // Returns a static message, "helloWorld".
    // This function serves as a basic example of returning a fixed string in Solidity.
    function runWasm (bytes wasmBinary, string wasmModule, string wasmFunction, bytes wasmArgs) public returns (bytes) {
        
        tvm.accept(); 
        getTokens();
        // uint8[] wasmBinaryArr = Convert.FromBase64String(wasmBinary);
        // uint8[] wasmArgsArr = Convert.FromBase64String(wasmArgs);
        // TODO: somehow pack everythign into cells
        //                                              [1,2]
        TvmCell wasmResultCell = gosh.runwasm(abi.encode(wasmArgs), abi.encode(wasmFunction), abi.encode(wasmModule), abi.encode(wasmBinary));
        tvm.commit();
        tvm.log("result acquired");
        // TvmCell eoncoded = abi.encode(wasmArgs);
        // bytes decoded = abi.decode(eoncoded, (string));
        bytes wasmResult = abi.decode(wasmResultCell, bytes);
        // uint8[3]
        //              cell      string        string      cell    
        return wasmResult;
        
    }

    // Updates the `timestamp` variable with the current blockchain time.
    // We will use this function to modify the data in the contract.
    // Сalled by an external message.
    function touch() external {
        // Informs the TVM that we accept this message.
        tvm.accept();
        getTokens();
        // Update the timestamp variable with the current block timestamp.
        timestamp = block.timestamp;
    }

    // Used to call the touch method of a contract via an internal message.
    // Parameter 'addr' - the address of the contract where the 'touch' will be invoked.
    function callExtTouch(address addr) public view {
        // Each function that accepts an external message must check that
        // the message is correctly signed.
        require(msg.pubkey() == tvm.pubkey(), 102);
        tvm.accept();
        getTokens();
        IHelloWorld(addr).touch();
    }

    // Sends VMSHELL to another contract with the same Dapp ID.
    // Parameter `dest` - the target address within the same Dapp ID to receive the transfer.
    // Parameter `value`- the amount of VMSHELL tokens to transfer.
    // Parameter `bounce` - Bounce flag. Set true if need to transfer funds to existing account;
    // set false to create new account.
    function sendVMShell(address dest, uint128 amount, bool bounce) public view {
        require(msg.pubkey() == tvm.pubkey(), 102);
        tvm.accept();
        getTokens();
        // Enables a transfer with arbitrary settings
        dest.transfer(varuint16(amount), bounce, 0);
    }

    // Allows transferring SHELL tokens within the same Dapp ID and to other Dapp IDs.
    // Parameter `dest` - the target address to receive the transfer.
    // Parameter `value`- the amount of SHELL tokens to transfer.
    function sendShell(address dest, uint128 value) public view {
        require(msg.pubkey() == tvm.pubkey(), 102);
        tvm.accept();
        getTokens();

        TvmCell payload;
        mapping(uint32 => varuint32) cc;
        cc[2] = varuint32(value);
        // Executes transfer to target address
        dest.transfer(0, true, 1, payload, cc);
    }

    // Deploys a new contract within its Dapp.
    // The address of the new contract is calculated as a hash of its initial state.
    // The owner's public key is part of the initial state.
    // Parameter `stateInit` - the contract code plus data.
    // Parameter `initialBalance` - the amount of funds to transfer. 
    // Parameter `payload` - a tree of cells used as the body of the outbound internal message.
    function deployNewContract(
        TvmCell stateInit,
        uint128 initialBalance,
        TvmCell payload
    )
        public pure
    {
        // Runtime function to deploy contract with prepared msg body for constructor call.
        tvm.accept();
        getTokens();
        address addr = address.makeAddrStd(0, tvm.hash(stateInit));
        addr.transfer({stateInit: stateInit, body: payload, value: varuint16(initialBalance)});
    }
    
    // Checks the contract balance
    // and if it is below the specified limit, mints VMSHELL.
    // The amounts are specified in nanotokens.
    // Used to enable automatic balance replenishment.
    function getTokens() private pure {
        if (address(this).balance > 100000000000) {     // 100 VMSHELL
            return; 
        }
        gosh.mintshell(100000000000);                   // 100 VMSHELL
    }

}

