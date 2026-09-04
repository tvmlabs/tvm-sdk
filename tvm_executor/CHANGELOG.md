# Release Notes

All notable changes to this project will be documented in this file.

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
