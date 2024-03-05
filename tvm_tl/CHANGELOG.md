# Release Notes

All notable changes to this project will be documented in this file.

## Version 0.3.47

- Added BLS data structures 

## Version 0.3.42

- Added control queries TL declaration `smc.runTvm`, `smc.runTvmByBlock`, `smc.runTvmMsg` and 
`smc.runTvmMsgByBlock`

## Version 0.3.29

- Added control queries TL declaration `raw.getShardAccountMeta`, `raw.getAccountByBlock`,
`raw.getAccountMetaByBlock`, `raw.getAppliedShardsInfo`

## Version 0.3.22

- Modify conversion API for TL public key
- Increase package version

## Version 0.3.7

- Add GetSelectedStats control query

## Version 0.3.1

- Added Signing trait for structures which have signature field

## Version 0.3.0

- Use bytes types as alias to Vec<u8>

## Version 0.2.198

- Updated datatypes to allow unique message id tracing in REMP

## Version 0.2.195

- Added datatypes for msg queue updates

## Version 0.2.190

- Removed using extra crate base64
- Added conversion between KeyOption and ton::PublicKey
- Minor refactoring for modern rust language
