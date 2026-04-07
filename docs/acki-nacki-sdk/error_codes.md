# Error Codes

You can find error codes with descriptions on this page

* [SDK Errors](error_codes.md#sdk-errors)
* [Solidity Runtime Errors](error_codes.md#solidity-runtime-errors)
* [TVM Virtual Machine Runtime Errors](error_codes.md#tvm-virtual-machine-runtime-errors)
  * [Action phase errors](error_codes.md#action-phase-errors)

## SDK Errors

[Client Error codes (1-99)](https://dev.ackinacki.com/reference/types-and-methods/mod_client#clienterrorcode)

[Crypto Error codes (100-199)](https://dev.ackinacki.com/reference/types-and-methods/mod_crypto#cryptoerrorcode)

[Boc error codes(200-299)](https://dev.ackinacki.com/reference/types-and-methods/mod_boc#bocerrorcode)

[Abi Error codes (300-399)](https://dev.ackinacki.com/reference/types-and-methods/mod_abi#abierrorcode)

[TVM Error codes (400-499)](https://dev.ackinacki.com/reference/types-and-methods/mod_tvm#tvmerrorcode)

[Processing Error codes (500-599)](https://dev.ackinacki.com/reference/types-and-methods/mod_processing#processingerrorcode)

[Net Error Codes (600-699)](https://dev.ackinacki.com/reference/types-and-methods/mod_net#neterrorcode)

[DeBot Error Codes (800-899)](https://dev.ackinacki.com/reference/types-and-methods/mod_debot#deboterrorcode)

## Solidity Runtime Errors

[https://github.com/gosh-sh/TVM-Solidity-Compiler/blob/master/API.md#solidity-runtime-errors](https://github.com/gosh-sh/TVM-Solidity-Compiler/blob/master/API.md#solidity-runtime-errors)

## TVM Virtual Machine Runtime Errors

`0` TVM terminated successfully

`1` TVM terminated successfully: alternative code

`2` Stack underflow

`3` Stack overflow

`4` Integer overflow

`5` Range check error

`6` Invalid opcode

`7` Type check error

`8` Cell overflow

`9` Cell underflow

`10` Dictionary error

`11` Unknown error

`12` Fatal error

`-14` Out of gas: the contract is either low on gas, or its limit is exceeded

`17`  Execution timeout: the transaction execution time limit for the Virtual Machine has been reached.

### Action phase errors

`32` Action list invalid

`33` Too many actions

`34` Unsupported action

`35` Invalid source address

`36` Invalid destination address

`37` Too low balance to send outbound message (37) at action

`38` Too low extra to send outbound message (38) at action

`39` Message does not fit in buffer

`40` Message too large

`41` Library not found

`42` Library delete error
