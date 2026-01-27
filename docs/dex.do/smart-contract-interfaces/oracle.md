---
description: Oracle Contract Interface Documentation
---

# Oracle

## Overview

**Oracle** is a core contract responsible for managing oracle identity and deploying an associated **OracleEventList**.\
It serves as the oracle’s root contract, holding the oracle public key, managing accumulated fees, and exposing metadata for integration with **PrivateNote** and **PMP** **(Prediction Market Pool)** contracts.

Each `Oracle` instance is deployed by `RootPN` and is uniquely identified by its static name.

***

## Contract Metadata

* **Contract name:** `Oracle`
* **Role:** Oracle root & fee manager
* **Deployed by:** `RootPN`
* **Authorization model:** Oracle public key (`_oraclePubkey`)
* **Associated contracts:** `OracleEventList`, `PrivateNote`, `PMP`

***

## Events

### `OracleEventListDeployed`

```solidity
event OracleEventListDeployed(address eventListAddress);
```

Emitted after successful deployment of the `OracleEventList` contract.

**Parameters:**

* `eventListAddress` — Address of the deployed `OracleEventList`

***

### `EventPublished`

```solidity
event EventPublished(uint256 event_id, string event_name);
```

Represents publication of an oracle event.

> ℹ️ **Note:**\
> This event is declared in the `Oracle` contract but not emitted directly in the shown implementation.\
> It may be intended for future use or for emission via delegated logic.

***

## Constructor

#### `constructor`

```solidity
constructor(
    uint256 oraclePubkey,
    TvmCell oracleEventListCode,
    TvmCell PrivateNoteCode,
    TvmCell pmpCode
)
```

Deploys the `Oracle` contract and immediately deploys the associated `OracleEventList`.

**Parameters:**

* `oraclePubkey` — Public key controlling the oracle
* `oracleEventListCode` — Contract code for `OracleEventList`
* `PrivateNoteCode` — `PrivateNote` contract code
* `pmpCode` — `PMP` contract code

**Requirements:**

* Caller must be `ROOT_PN_ADDRESS`

**Side Effects:**

* Deploys a new `OracleEventList`
* Stores oracle configuration and contract codes
* Emits `OracleEventListDeployed`

***

## Fee Management

### `withdrawFees`

```solidity
function withdrawFees(
    address to,
    uint128 amount
) public view onlyOwnerPubkey accept;
```

Withdraws accumulated oracle fees to a specified address.

**Parameters:**

* `to` — Recipient address
* `amount` — Amount of shell tokens to withdraw

**Access Control:**

* Callable only by the oracle owner (`_oraclePubkey`)

**Behavior:**

* Transfers shell tokens using `CURRENCIES_ID_SHELL`

***

## Native Transfers

### `receive`

```solidity
receive() external pure;
```

Accepts incoming native token transfers.

**Behavior:**

* Accepts the message
* Ensures minimal contract balance

***

## View Functions

### `getEventListAddress`

```solidity
function getEventListAddress()
    external view
    returns (address);
```

Returns the address of the associated `OracleEventList` contract.

***

### `getVersion`

```solidity
function getVersion()
    external pure
    returns (string);
```

Returns the contract identifier.

**Returns:**

* `"Oracle"`

***

## Internal Mechanics (Informational)

#### Balance Safety

The contract internally maintains a minimum native balance using a private `ensureBalance()` helper.\
If the balance falls below `MIN_BALANCE`, additional shell tokens are minted.

***

## Access Control Summary

| Function              | Access            |
| --------------------- | ----------------- |
| `withdrawFees`        | Oracle owner only |
| `getEventListAddress` | Public            |
| `getVersion`          | Public            |
| `receive`             | Public            |

***

## Notes & Caveats

* `Oracle` contracts **must** be deployed by `RootPN`.
* Fee withdrawal operates exclusively in **Shell tokens** (`CURRENCIES_ID_SHELL`).
* Event publishing logic is expected to be handled by `OracleEventList`.

