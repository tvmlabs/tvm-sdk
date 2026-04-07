# API reference

Complete reference for all public/external functions, events, and error codes across the Accumulator contract system.

## ShellAccumulatorRootUSDC

Source: `contracts/accumulator/ShellAccumulatorRootUSDC.sol` · Version: 1.0.2\
&#xNAN;_&#x49;t will be available after the next node release_

### Entry points

#### `buyShellFor(address buyer)`

```solidity
function buyShellFor(address buyer) public
```

Accepts ECC USDC attached to the message and processes a buy on behalf of `buyer`. Used by Exchange to forward purchases. Does **not** check for multi-currency messages — only verifies ECC USDC is present.

#### `claimUSDC(uint16 D, uint64 orderId, address seller)`

```solidity
function claimUSDC(uint16 D, uint64 orderId, address seller) public
```

Called by a SellOrderLot to claim its USDC payout. Verifies caller address deterministically, checks the order is sold (`orderId <= soldPrefix[D]`), sends ECC USDC to `seller`, then calls `onReceiveUSDC` on the lot.

### Admin

#### `setPubkey(uint256 pubkey)`

```solidity
function setPubkey(uint256 pubkey) public onlyOwnerPubkey accept
```

Replaces the owner public key. Only callable by the current owner (verified via `msg.pubkey()`).

### Getters

#### `getQueueState(uint16 D)`

```solidity
function getQueueState(uint16 D) external view
    returns (uint64 nextId, uint64 available, uint64 soldPrefix, uint64 owedCount)
```

Returns the FIFO queue state for denomination `D` (1, 10, 100, or 1000).

* `nextId` — next order ID to assign (1-based)
* `available` — lots waiting to be matched by a buyer
* `soldPrefix` — contiguous count of sold lots from the start
* `owedCount` — sold lots that haven't been claimed yet

#### `getDetails()`

```solidity
function getDetails() external view
    returns (uint256 ownerPubkey, uint128 sellerShellPool, uint128 usdcBalance, uint128 owedTotal)
```

Returns high-level contract state.

* `sellerShellPool` — total ECC SHELL held from seller deposits (nanoSHELL)
* `usdcBalance` — total ECC USDC tracked by the contract (microUSDC)
* `owedTotal` — total USDC owed to sellers awaiting claim (microUSDC)

#### `getSellOrderAddress(uint16 D, uint64 orderId)`

```solidity
function getSellOrderAddress(uint16 D, uint64 orderId) external view
    returns (address sellOrderAddr)
```

Computes the deterministic address of a lot contract given its denomination and order ID. Useful for off-chain address resolution without deploying.

#### `owedUsdcTotal()`

```solidity
function owedUsdcTotal() external view returns (uint128)
```

Returns total USDC owed to all sellers across all denominations (microUSDC).

#### `getSellerShellPool()`

```solidity
function getSellerShellPool() external view returns (uint128)
```

Returns total ECC SHELL in the seller pool (nanoSHELL).

#### `getUsdcBalance()`

```solidity
function getUsdcBalance() external view returns (uint128)
```

Returns the USDC balance tracked by the contract (microUSDC). This is the accounting balance, not necessarily the on-chain ECC balance.

#### `getNacklInfo()`

```solidity
function getNacklInfo() external view
    returns (uint128 supply, uint128 burned, uint32 unixstart)
```

Returns NACKL emission state.

* `supply` — current `M(t)` from the emission curve (nanoNACKL)
* `burned` — total NACKL burned via `redeemNACKL` to date (nanoNACKL)
* `unixstart` — emission start timestamp (Unix seconds)

The effective circulating supply is `supply - burned`.

#### `getVersion()`

```solidity
function getVersion() external pure returns (string version, string name)
```

Returns `("1.0.2", "ShellAccumulatorRootUSDC")`.

***

## ShellSellOrderLot

Source: `contracts/accumulator/ShellSellOrderLot.sol` · Version: 1.0.2\
&#xNAN;_&#x49;t will be available after the next node release_

### Entry points

#### `claim()`

```solidity
function claim() public
```

Initiates USDC payout claim. Sets `_claimed = true` and calls `Root.claimUSDC(denom, orderId, owner)`. If the root rejects (order not yet sold), the bounced message resets `_claimed = false` via `onBounce`.

Can be called by anyone (no `msg.sender` check), but the payout always goes to `_owner` (the original seller).

#### `onReceiveUSDC(uint128 amount)`

```solidity
function onReceiveUSDC(uint128 amount) public senderIs(_root) accept
```

Callback from the Root confirming payout was sent. Verifies `amount == _denom * USDC_DECIMALS_FACTOR`, emits `OrderDestroyed`, and self-destructs.

### Getters

#### `getDetails()`

```solidity
function getDetails() external view
    returns (address root, address owner, uint16 denom, uint64 orderId, bool claimed)
```

Returns all lot metadata.

* `root` — parent Accumulator address
* `owner` — seller address (receives USDC payout)
* `denom` — lot denomination (1, 10, 100, 1000)
* `orderId` — FIFO position within the denomination queue
* `claimed` — `true` if `claim()` was called and is pending or completed

#### `getVersion()`

```solidity
function getVersion() external pure returns (string version, string name)
```

Returns `("1.0.2", "ShellSellOrderLot")`.

***

## Exchange

Source: `contracts/exchange/Exchange.sol` · Version: 1.0.4\
&#xNAN;_&#x49;t will be available after the next node release_

### Entry points

#### `onTransferReceived(address from, address to, uint128 value, uint128 balance)`

```solidity
function onTransferReceived(address from, address, uint128 value, uint128) external override
```

ISubscriber callback from the Exchange's TIP-3 USDC wallet. Mints equivalent ECC USDC and sends it to `from` (the depositor). Only callable by `_usdcWallet`.

#### `mintAndSend(address recipient, uint128 value, uint64 nonce)`

```solidity
function mintAndSend(address recipient, uint128 value, uint64 nonce) public onlyOwnerPubkey accept
```

Admin-only. Mints ECC USDC and sends to `recipient`. Requires `nonce == _mintNonce + 1`.

#### `mintAndSendAccumulator(address buyer, uint128 value, uint64 nonce)`

```solidity
function mintAndSendAccumulator(address buyer, uint128 value, uint64 nonce) public onlyOwnerPubkey accept
```

Admin-only. Mints ECC USDC and calls `Accumulator.buyShellFor(buyer)` with the minted USDC attached. Requires whole USDC units and `nonce == _mintAccumulatorNonce + 1`. Uses separate nonce space from `mintAndSend`.

### Admin

#### `setPubkey(uint256 pubkey)`

```solidity
function setPubkey(uint256 pubkey) public onlyOwnerPubkey accept
```

Replaces the owner public key.

#### `triggerTransaction(address txAddr)`

```solidity
function triggerTransaction(address txAddr) public view onlyOwnerPubkey accept
```

Sends 1 vmshell to `txAddr`. Used to trigger Transaction contracts for wallet setup (e.g., `SET_SUBSCRIBER_TYPE`).

### Getters

#### `getUsdcWallet()`

```solidity
function getUsdcWallet() external view returns (address)
```

Returns the TIP-3 USDC TokenWallet address used for the bridge.

#### `getOwnerPubkey()`

```solidity
function getOwnerPubkey() external view returns (uint256)
```

Returns the current owner public key.

#### `getTotalMinted()`

```solidity
function getTotalMinted() external view returns (uint128)
```

Returns total ECC USDC minted by this contract across all methods (microUSDC).

#### `getNonces()`

```solidity
function getNonces() external view returns (uint64 mintNonce, uint64 mintAccumulatorNonce)
```

Returns current nonces for both mint paths. The next valid nonce for each path is `current + 1`.

#### `getVersion()`

```solidity
function getVersion() external pure returns (string version, string name)
```

Returns `("1.0.4", "Exchange")`.

***

## AccumulatorLib

Source: `contracts/accumulator/libraries/AccumulatorLib.sol` · Version: 1.0.2\
&#xNAN;_&#x49;t will be available after the next node release_

#### `calculateSellOrderAddress(TvmCell code, address root, uint16 denom, uint64 orderId)`

```solidity
function calculateSellOrderAddress(TvmCell code, address root, uint16 denom, uint64 orderId)
    public returns (address)
```

Computes the deterministic address of a SellOrderLot. The address is `makeAddrStd(0, hash(stateInit))`.

#### `composeSellOrderStateInit(TvmCell code, address root, uint16 denom, uint64 orderId)`

```solidity
function composeSellOrderStateInit(TvmCell code, address root, uint16 denom, uint64 orderId)
    public returns (TvmCell)
```

Builds the full stateInit for a lot: salted code + static variables `_denom` and `_orderId`.

#### `buildSellOrderCode(TvmCell originalCode, address root)`

```solidity
function buildSellOrderCode(TvmCell originalCode, address root)
    public returns (TvmCell)
```

Salts the lot code with `abi.encode(versionLib, root)`. This binds the lot to a specific Root contract and library version.

***

## Events

All events are emitted to **external addresses** (directed events) for off-chain subscription. The external address is constructed as `address.makeAddrExtern(eventId, 256)`.

### Root events

| Event              | Ext Addr                   | Fields                                                                               | Emitted when                                                                     |
| ------------------ | -------------------------- | ------------------------------------------------------------------------------------ | -------------------------------------------------------------------------------- |
| `SellOrderCreated` | **610** + seller's address | `(address seller, uint16 denom, uint64 orderId, uint128 shellAmount)`                | New lot created. Emitted twice: to addr 610 and to the seller's external address |
| `ShellPurchased`   | **611**                    | `(address buyer, uint128 usdcAmount, uint128 shellFromSellers, uint128 shellMinted)` | Buy completed                                                                    |
| `UsdcClaimed`      | **612**                    | `(uint64 orderId, uint16 denom, address seller, uint128 payout)`                     | Seller claimed USDC                                                              |
| `NacklRedeemed`    | **613**                    | `(address recipient, uint128 burnAmount, uint128 payout)`                            | NACKL burned for USDC                                                            |
| `MatchedOrders`    | **617**                    | `(uint64 lastSold1, uint64 lastSold10, uint64 lastSold100, uint64 lastSold1000)`     | Updated soldPrefix values after a buy                                            |

### Lot events

| Event            | Target   | Fields                                           |
| ---------------- | -------- | ------------------------------------------------ |
| `ClaimInitiated` | internal | `(uint64 orderId, uint16 denom, address owner)`  |
| `OrderDestroyed` | internal | `(uint64 orderId, uint16 denom, uint128 amount)` |

### Exchange events

| Event          | Ext Addr | Fields                               | Emitted when                                                            |
| -------------- | -------- | ------------------------------------ | ----------------------------------------------------------------------- |
| `UsdcMigrated` | **615**  | `(address from, uint128 value)`      | TIP-3 USDC bridged to ECC                                               |
| `UsdcMinted`   | **616**  | `(address recipient, uint128 value)` | Admin-minted ECC USDC (from `mintAndSend` and `mintAndSendAccumulator`) |

***

## Error Codes

### Accumulator errors (Root + SellOrderLot)

| Code | Name                          | Meaning                                                  |
| ---- | ----------------------------- | -------------------------------------------------------- |
| 200  | `ERR_INVALID_DENOM`           | Denomination is not 1, 10, 100, or 1000                  |
| 201  | `ERR_WRONG_SHELL_AMOUNT`      | SHELL amount doesn't divide evenly by SHELL\_PER\_USDC   |
| 202  | `ERR_WRONG_USDC_AMOUNT`       | USDC amount mismatch in onReceiveUSDC or balance check   |
| 203  | `ERR_NOT_WHOLE_USDC`          | USDC amount is not a whole number (not divisible by 10⁶) |
| 204  | `ERR_ZERO_AMOUNT`             | Zero amount supplied                                     |
| 205  | `ERR_ORDER_NOT_SOLD`          | Lot's orderId > soldPrefix (not yet matched)             |
| 206  | `ERR_NO_OWED`                 | No owed claims remaining for this denomination           |
| 207  | `ERR_INVALID_SENDER`          | Caller is not the expected contract                      |
| 208  | `ERR_ALREADY_CLAIMED`         | claim() already called on this lot                       |
| 209  | `ERR_NOT_OWNER`               | msg.pubkey() doesn't match owner                         |
| 210  | `ERR_INSUFFICIENT_REDEEMABLE` | Not enough free reserve for NACKL redemption             |
| 211  | `ERR_WRONG_CODE`              | (reserved)                                               |
| 212  | `ERR_WRONG_ADDRESS`           | Caller address doesn't match deterministic lot address   |
| 213  | `ERR_MULTIPLE_CURRENCIES`     | Message carries more than one ECC currency type          |
| 214  | `ERR_OVERFLOW`                | Amount exceeds uint64 max                                |

### Exchange errors

| Code | Name                 | Meaning                          |
| ---- | -------------------- | -------------------------------- |
| 204  | `ERR_ZERO_AMOUNT`    | Zero value                       |
| 207  | `ERR_INVALID_SENDER` | Caller is not the USDC wallet    |
| 209  | `ERR_NOT_OWNER`      | msg.pubkey() doesn't match owner |
| 213  | `ERR_NOT_WHOLE_USDC` | Value not divisible by 10⁶       |
| 214  | `ERR_OVERFLOW`       | Value exceeds uint64 max         |
| 215  | `ERR_INVALID_NONCE`  | Nonce is not current + 1         |

{% hint style="info" %}
Error code 213 means different things in different contracts: `ERR_MULTIPLE_CURRENCIES` in the Accumulator vs `ERR_NOT_WHOLE_USDC` in the Exchange. When debugging failed transactions, check which contract emitted the error.
{% endhint %}

***

## Constants

### Token IDs and decimals

| Constant                | Value                       | Used in        |
| ----------------------- | --------------------------- | -------------- |
| `NACKL_ECC_ID`          | 1                           | Root           |
| `SHELL_ECC_ID`          | 2                           | Root           |
| `USDC_ECC_ID`           | 3                           | Root, Exchange |
| `SHELL_DECIMALS_FACTOR` | 1,000,000,000 (10⁹)         | Root           |
| `USDC_DECIMALS_FACTOR`  | 1,000,000 (10⁶)             | Root, Exchange |
| `SHELL_PER_USDC`        | 100,000,000,000 (100 × 10⁹) | Root           |

### NACKL emission

| Constant         | Value                      | Meaning                        |
| ---------------- | -------------------------- | ------------------------------ |
| `NACKL_T`        | 10,400,000,000,000,000,000 | Max supply cap (nanoNACKL)     |
| `NACKL_T_KM`     | 10,400,104,000,000,000,000 | T × (1 + K\_M), K\_M = 0.00001 |
| `NACKL_U_M_FP18` | 5,756,467,732              | Growth rate × 10¹⁸             |
| `FP18`           | 10¹⁸                       | Fixed-point scaling factor     |
| `INV_E_FP18`     | 367,879,441,171,442,322    | exp(-1) × 10¹⁸                 |

### Denominations

```
DENOM_1    = 1
DENOM_10   = 10
DENOM_100  = 100
DENOM_1000 = 1000
```
