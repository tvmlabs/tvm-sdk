// 2022-2024 (c) Copyright Contributors to the GOSH DAO. All rights reserved.
//
pragma ever-solidity >=0.66.0;
pragma AbiHeader expire;
pragma AbiHeader pubkey;

contract Tester {
    uint public counter = 0;
    uint32 public seq = 0;

    constructor() {
        tvm.accept();
    }

    function add() public pure {
        tvm.accept();
        this.iterate(0);
    }

    function iterate(uint index) public {
        tvm.accept();
        if (index < 6) {
            for(uint i = 0; i < 4; i++) {
                uint delta = block.logicaltime;
                counter += delta;
                this.iterate(index + 1);
            }
        }
    }

    function inc() public {
        tvm.accept();
        counter += 1;
    }

    function seq_no() public {
        tvm.accept();
        seq = block.seqno;
    }
}