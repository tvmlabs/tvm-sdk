---
description: (Work in progress) RootOracle Contract Interface Documentation
---

# RootOracle

{% file src="../../.gitbook/assets/RootOracle.abi (1).json" %}

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

### **`deployOracle`**

Deploys a new Oracle contract.

```solidity
function deployOracle(uint256 oraclePubkey, string oracleName) public view accept;
```

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

### **`getOracleAddress`**

Returns the deterministic address of an `Oracle` contract by its name.

```solidity
function getOracleAddress(string name) external view returns(address oracleAddress)
```

**Parameters:**

* `name` — unique Oracle name used during deployment

**Returns:**

* `oracleAddress` — deterministic address of the Oracle contract associated with the given name

### **`getVersion`**

Returns the contract version identifier.

```solidity
function getVersion() external pure returns (string, string)
```

**Returns:**

* semantic version string
* Contract name: `"RootOracle"`
