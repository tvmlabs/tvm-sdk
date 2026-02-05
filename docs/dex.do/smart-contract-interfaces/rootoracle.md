---
description: (Work in progress) RootOracle Contract Interface Documentation
---

# RootOracle

{% file src="../../.gitbook/assets/RootOracle.abi.json" %}

## Overview

**RootOracle** is the system root contract responsible for deploying and managing Oracle contracts.\
It also supports contract code upgrades and emits events related to Oracle deployment.

***

## Events

### OracleDeployed

Emitted when a new Oracle contract is successfully deployed.

```solidity
event OracleDeployed(address oracle, uint256 pubkey, string name);
```

* `oracle` — address of the deployed Oracle contract
* `pubkey` — public key associated with the Oracle
* `name` — name of the Oracle

***

## Public & External Interface

### Deploy Oracle

#### **`deployOracle(uint256 oraclePubkey, string oracleName)`**

Deploys a new Oracle contract.

```solidity
function deployOracle(uint256 oraclePubkey, string oracleName) public view accept;
```

**Access:** public\
**Modifiers:** `accept`

**Parameters:**

* `oraclePubkey` — public key of the Oracle
* `oracleName` — human-readable name of the Oracle

**Behavior:**

* Ensures the root contract has the minimum required native balance
* Builds `stateInit` for the Oracle using `DexLib.buildOracleStateInit`
* Deploys a new `Oracle` contract with:
  * `value: 60 vmshell`
  * `flag: 1`
* Passes the following arguments to the Oracle constructor:
  * `oraclePubkey`
  * `_oracleEventListCode`
  * `_PrivateNoteCode`
  * `_pmpCode`
* Emits an `OracleDeployed` event to an external address

***

## View Functions

### **`getVersion()`**

Returns the contract version identifier.

```solidity
function getVersion() external pure returns (string);
```

**Returns:**

* Contract name: `"RootOracle"`
