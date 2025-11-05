pragma gosh-solidity >=0.76.1;
pragma AbiHeader expire;
pragma AbiHeader pubkey;

contract C {

  constructor(uint64 value) {
        // Call the VM command to convert SHELL tokens to VMSHELL tokens to pay the transaction fee.
        gosh.cnvrtshellq(value);

        // Ensure that the contract's public key is set.
        require(tvm.pubkey() != 0, 101);

        // The current smart contract agrees to buy some gas to complete the
        // current transaction. This action is required to process external
        // messages, which carry no value (and therefore no gas).
        tvm.accept();
  }

  TvmCell c;

  function just_write_cell(TvmCell cell) public returns (uint cells_, uint bits_, uint refs_){
    c = cell;
    return cell.dataSize(1024);
  }

  function get_content_of_cell(TvmCell cell) public pure returns (uint8 u_, TvmCell cell_, mapping (int => uint8 m_), int size_){
    (uint8 u, TvmCell c1) = abi.decode(cell, (uint8, TvmCell));
    mapping(int=>uint8) m;
    int i = 0;
    for(; i < cell.depth() - 1; ++i) {
	m[i] = u;
	(u, c1) = abi.decode(c1, (uint8, TvmCell));
    }
    uint8 uu = abi.decode(c1, (uint8));
    m[i] = uu;
    return (uu, c1, m, cell.depth());
  }

  function doNothing() public {}
}
