# Acki Nacki V1 TVM transfer findings

Status: V1 fixes are in the local SDK branch; nothing has been pushed. The transfer matrix matched 32/32 cases. The full call suite passed 80/82 cases on the pre-wait CLI; its two F13 discrepancies passed in a focused rerun with the CLI and harness corrections (F13a–F13c). A separate 41-case V1/MSIG suite passed the 30 transfer/gas cases, all 10 MSIG 0.81 cases, and MSIG 0.82 M1. Its next 0.82 setup drain timed out, and an automatic review blocked repeating a mode-128 drain to a random sink.

## Test baseline

- Acki Nacki dev: `3e1833152`.
- SDK: `v3.1.0.an`, commit `fd76a8a` (the SDK `main` commit used as the branch base).
- Legacy engine: `1.0.3`; network initialized from zerostate.
- Transfer table: V1 `UpdateCustodianMultisigWallet` 0.79.3, 32 cases.
- Full report: `/home/sehor/Downloads/v1_dev_report.html`; transfer-only extract: `/home/sehor/Downloads/v1_transfers_table.html`.
- The full report's night section summarizes 136 rows; some scenarios repeat calls (including 200-call batches). Results are historical and were not rerun for this note.

## Findings

### Confirmed SDK defects addressed in the local branch

- Cross-DApp messages without `tvm.accept()` were reserving the unused gas-limit remainder through the action phase. Gas fees are already removed during compute, so this blocked output actions with action errors 34/37. The local change charges no more than the incoming VM/SHELL budget and restores any fee shortfall to the existing account balance.
- Send flag 64 (`SENDMSG_REMAINING_MSG_BALANCE`) was not applied. The implementation now carries the remaining inbound value, less compute gas when forwarding fees are not separate, into the output message.
- Failed send actions carrying flag 2 (`SENDMSG_IGNORE_ERROR`) were still aborting the action phase. The local change skips that failed action and continues processing later actions.
- When a failed bounce could not be funded for a new account, ECC was discarded. The local change retains it on an `Uninit` account with zero VM balance.

The call suite passes F2a/F10 (flag 64) and AP6 (ignored second send) both between and within DApps. The rebuilt node image also passed all six cases together (F2a, F10, AP6 across both routes).

### Confirmed SDK runtime defect: cross-DApp VM double burn (P7)

When a cross-DApp message completes compute successfully and then aborts, `need_to_burn` can be debited once in the compute-success/abort path and again in the later abort path. The receiver loses the imported VM amount on top of gas. The recorded example with 1 VM loses 1.001 VM instead of 0.001 VM. The V1 night run reports the extra burn for several action failures and bounce settings; it does not occur for same-DApp delivery or external messages.

Relevant code in this baseline: `tvm_executor/src/ordinary_transaction.rs:470-472` and `:553-557`. The V1 report records the initial burn setup at `:135-159`.

### Network configuration behavior: failed action does not bounce (P4)

For action-phase failures such as exit 37/38, bounce creation depends on `CapBounceAfterFailedAction`. The tested dev config did not enable this capability, so a message with `bounce=true` can leave ECC at the receiver after an action failure. Compute-phase failures were observed to bounce when the message had enough VM to pay for the return.

This is a network capability/configuration setting, not by itself an SDK implementation defect. Recheck the target network configuration before applying the observation elsewhere.

### Bounce with zero VM cannot return attached ECC (P5)

Transfer cases #19–24 sent ECC with `bounce=true`, zero VM, to a nonexistent address. The sender was debited and no bounce returned. The table's state columns show the ECC at the destination's `Uninit` address; they do not prove that ECC was burned or irrecoverable. The operational problem is that the sender receives no automatic refund and the destination has not been initialized.

### Failed cross-DApp call can charge the receiver's VM before `tvm.accept()` (P2)

The V1 report observed that an incoming cross-DApp VM is burned and compute gas can be charged to the receiver's own VM balance even when the call fails before `tvm.accept()`. Same-DApp calls use incoming VM for gas in the tested cases. Record this as a protocol/accounting behavior to compare with the intended network rule; the report alone does not establish that all such charging is an SDK bug.

### Client reports success for a rejected external message (P6)

For an external message rejected without `tvm.accept()`, the node created no transaction and did not change state, while the `tvm-cli` response still included message, block, and transaction hashes as if successful. This is a client/API reporting issue; callers should verify that a transaction was actually included before treating the operation as successful.

The local CLI now checks GraphQL for the returned transaction hash before reporting success. In the focused F13 rerun, F13a (no accept) returned an error, F13b (accept then exception) produced a transaction, and F13c rejected a replayed message. The `sendfile` harness now supplies the required `--dst-dapp-id`.

### SHELL exchange with no VM and bounce requested (P8)

In night case N6, an exchange send using flag 17 with zero VM and `bounce=true` failed without creating a bounce because no VM remained to fund the return. SHELL remained credited as ECC at the receiver, while receiver-owned VM paid gas in the tested failure cases. This is an edge case of exchange plus failed delivery, not one of the 32 multisig transfer rows.

### Failed constructor leaves an activated account (P9)

In night case N8, deployment to a nonexistent address activated the account and installed code/data before the constructor failed. With `bounce=false`, VM and ECC remained on the active account; with `bounce=true`, the values returned but the account remained active with zero balance and a constructor marker showing it did not complete. The report flags this for protocol/product review rather than identifying a confirmed SDK regression.

## Other observations

- A receiver with no VM and no accepted incoming gas may fail for lack of execution budget; the test setup treated this as expected.
- The zero-VM bounce cases above must not be described as globally burned ECC: the test table records ECC on the destination's `Uninit` account.
