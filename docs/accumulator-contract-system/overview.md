# Overview

Shell Accumulator is an on-chain exchange system on the GOSH network that lets users trade **ECC SHELL** for **eccUSDC** at a fixed rate of **100 SHELL = 1 eccUSDC**.

## What it does

Sellers deposit SHELL into fixed-size lots (denominated in 1, 10, 100, or 1000 eccUSDC). Buyers deposit eccUSDC and receive SHELL — first matched against existing seller lots in FIFO order, then minted if no sellers are available. When a buyer's eccUSDC matches a seller's lot, the seller can claim their eccUSDC payout.

A separate token, **NACKL**, can be redeemed (burned) against the "free reserve" — the eccUSDC balance not owed to any seller.

## Tokens (ECC IDs)

| Token   | ECC ID | Decimals  | Role                            |
| ------- | ------ | --------- | ------------------------------- |
| NACKL   | 1      | 9 (nano)  | Value storage and staking       |
| SHELL   | 2      | 9 (nano)  | Utility token, sold by sellers  |
| eccUSDC | 3      | 6 (micro) | Stablecoin, deposited by buyers |

All token amounts in the contracts use their smallest unit: nanoSHELL, nanoNACKL, microeccUSDC.

## Contracts

| Contract                   | Source                                               | Role                                                                                                                                                                     |
| -------------------------- | ---------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `ShellAccumulatorRootUSDC` | `contracts/accumulator/ShellAccumulatorRootUSDC.sol` | The central Accumulator contract: accepts eccUSDC, SHELL, and NACKL, manages FIFO seller queues, sells SHELL instantly, and handles eccUSDC payouts for claims and burn. |
| `ShellSellOrderLot`        | `contracts/accumulator/ShellSellOrderLot.sol`        | One per seller position; carries claim metadata                                                                                                                          |
| `AccumulatorLib`           | `contracts/accumulator/libraries/AccumulatorLib.sol` | Deterministic address derivation for lots                                                                                                                                |
| `Exchange`                 | `contracts/exchange/Exchange.sol`                    | TIP-3 USDC bridge and admin mint entry point                                                                                                                             |

## Key design decisions

**Fixed denominations.**

Lots come in exactly 4 sizes: 1, 10, 100, 1000 eccUSDC. There are no partial lots. A 154 eccUSDC buy matches 1×100 + 5×10 + 4×1 lots.

**FIFO guarantee.**

Each denomination has its own queue. Lots are matched strictly in order of creation — earlier sellers get paid first.

**Deterministic lot addresses.**

Lot contract addresses are derived from `(code, root, denom, orderId)` using `AccumulatorLib`. This means the Root can verify any caller claiming to be a lot by recomputing the expected address, without storing a mapping.

**Directed events.**

Events are emitted to hardcoded external addresses (610–617) so that off-chain backends can subscribe to specific event streams without parsing all contract messages. See [Events ](api-reference.md#events)for the full list.
