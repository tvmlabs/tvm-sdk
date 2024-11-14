---
hidden: true
---

# VM Instructions specifications

## CNVRTSHELLQ (С727)&#x20;

Converts  SHELL to VMSHELL  at a 1:1  ratio.

Q in the end stands for ‘quiet’ which means that if there is not enough Shell, it will not throw an exception.

If the account balance does not have the required number of tokens, the exchange will be made for the entire available amount. That is, MIN(available\_tokens, want\_cnt\_to\_convert).

[https://github.com/tvmlabs/tvm-sdk/blob/a58e859e68e14a17a8acd2f142d260127a0a3f2d/tvm\_assembler/src/simple.rs#L841](https://github.com/tvmlabs/tvm-sdk/blob/a58e859e68e14a17a8acd2f142d260127a0a3f2d/tvm\_assembler/src/simple.rs#L841)&#x20;
