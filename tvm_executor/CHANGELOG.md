# Release Notes

All notable changes to this project will be documented in this file.

## [Unreleased]

### Added

- Engine `1.0.8` enables State V2 execution rules. Earlier engines, including
  `1.0.7`, retain SDK 3.0.6 balance import, outbound internal-message DApp
  handling and bounceable timeout behavior, and reject new cross-DApp headers.
  `ExecuteParams::default()` selects `1.0.8`; callers replaying old blocks must
  supply their source engine version explicitly.

### Fixed

- Bounceable cross-DApp messages no longer leave foreign VMShell spendable
  at the receiver, including during outbound actions. Attached VMShell may pay
  compute; its unreturned remainder is burned. Exchange flag 16 applies to
  incoming SHELL for either header type, and an unfunded return preserves the
  post-compute balance instead of leaving SHELL unexchanged or losing ECC.
  Invalid StateInit on existing Uninit accounts and failing constructors also
  preserve retained funds after actual gas fees. Old receiver ECC is untouched;
  successful bounces return original currencies and legacy engines are unchanged.

- ECC settlement after an unfunded bounce applies only to State V2 engine
  1.0.8 and newer; earlier engines retain their legacy behavior.

- Unfunded bounce settlement preserves full incoming extra-currency amounts,
  including SHELL values above `u128::MAX` in internal and cross-DApp headers.

- Cross-DApp headers targeting the sender's own DApp now retain incoming
  VMShell. A bounce that cannot pay return forwarding fees now retains ECC on
  a new Uninit destination even when an attached StateInit cannot deploy.
  Flag 16 exchanges retained incoming ECC SHELL after the failed return, and
  successful returns keep the original currencies. Failed deployments with
  `bounce=false` retain funds according to the DApp boundary and exchange
  rules. Successful contract activation and legacy execution are unchanged.

- Cross-DApp funding messages with `bounce=false` now discard original incoming
  VMShell before credit and execution. Incoming ECC SHELL is retained, or
  converted 1:1 to receiver-local VMShell when exchange flag 16 is set (including
  send flag 17). Existing receiver funds are preserved. VMShell-only funding
  no longer creates an empty uninitialized destination account.

- Internal VMShell transfers within the destination DApp can now create an
  uninitialized account instead of treating the missing account as a
  cross-DApp recipient and burning the transferred value.

## [3.0.6] - 2026-09-03

### Fixed
- Four `ExecutorError` variants reported `0` in place of the value they were built with, their `#[error]` attributes interpolating a bare integer literal rather than the variant's own field: `Transaction executor internal error`, `VM Exception, code: <n>`, `Contract did not accept message, exit code: <n>` and `Compute phase skipped ... with reason <r>`. The exit code a contract refused a message with, and the VM exception code behind a failed transaction, are now reported as themselves.

## Version 1.16.85

- Deny ChangeLibrary action when CapSetLibCode is unset

## Version 1.16.40

- Disable debug symbols by default

## Version 1.16.0

- Skiped compute phase for suspended addresses

## Version 1.15.196

- Removed extra crates bas64
- Minor refactoring

## Version 1.15.191

- Supported ever-types version 2.0

## Version 1.15.190

- Add test for CapFeeInGasUnits

## Version: 1.15.188

### News

- check capability for calculating forward and storage fees

## Version: 1.15.183

### Fixes

- check gas limit and credit for overflow

## Version: 1.15.177

### New

- capability CapBounceAfterFailedAction: if transaction fails on Action phase,
bounced message will be produced 

## Version: 1.15.128

### New

- add common submodule

### Fixes

- minor refactor for clippy

## Version: 1.15.121

### Fixes

- support other libs changes
## Version: 1.15.75

### New

- backward compatibility to prev nodes in bounced fee calculating

## Version: 1.5.73

### New

- support behavior modifier mechanism for TVM
