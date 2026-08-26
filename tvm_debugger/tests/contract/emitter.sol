// 2022-2026 (c) Copyright Contributors to the GOSH DAO. All rights reserved.
//
// Fixture for the ext-out message handling tests: a contract emits its events
// as external outbound messages, the same kind of message an ABI function
// response arrives in.
pragma ever-solidity >=0.66.0;
pragma AbiHeader expire;
pragma AbiHeader pubkey;

contract Emitter {
    uint128 public counter = 0;

    event Bumped(uint128 total, uint128 value);

    constructor() public externalMsg {
        tvm.accept();
    }

    /// Emits an event.
    function bump(uint128 value) public internalMsg {
        tvm.accept();
        counter += value;
        emit Bumped(counter, value);
    }

    /// Emits nothing.
    function bumpQuiet(uint128 value) public internalMsg {
        tvm.accept();
        counter += value;
    }

    /// Emits an event, then fails.
    function boom(uint128 value) public internalMsg {
        tvm.accept();
        counter += value;
        emit Bumped(counter, value);
        require(false, 199);
    }

    /// Refuses any sender but the one it is told to expect, so that the source
    /// address the debugger is given is exercised end to end.
    function bumpFromSender(address expected, uint128 value) public internalMsg {
        require(msg.sender == expected, 200);
        tvm.accept();
        counter += value;
        emit Bumped(counter, value);
    }

    /// Emits an event and returns a value, so both kinds of external outbound
    /// message leave the contract in one run.
    function readAndEmit() public view externalMsg returns (uint128 total) {
        tvm.accept();
        emit Bumped(counter, 0);
        total = counter;
    }
}
