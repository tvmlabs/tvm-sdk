---
description: (Work in progress) Nullifier Contract Interface Documentation
---

# Nullifier

{% file src="../../.gitbook/assets/Nullifier.abi.json" %}

## Overview

`Nullifier` is a contract that stores a **static nullifier hash** and provides a method to retrieve the contract version.

During deployment, the constructor verifies that the deployer is the `RootPN` contract and transfers a small amount of funds to a specified address.

## View Function

### **`getVersion`**

Returns the contract version identifier.

```solidity
function getVersion() external pure returns (string, string)
```

**Returns:**

* semantic version string
* Contract name: `"Nullifier"`&#x20;
