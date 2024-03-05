# Release Notes

All notable changes to this project will be documented in this file.

## Version 0.7.196

- BLS structures support

## Version 0.7.189

### Fixed

- Block parser: parsing was failed if block had empty shard state update.
- Block parser: optimized deserialization accounts from merkle update.  

## Version 0.7.188

- Added block parser  implements common block parsing strategy (with accounts, transactions, messages etc.).
  It is a generalized parsing algorithm based on three sources (ever-node, parser-service, evernode-se). 

## Version 0.7.179

- Parse block proof

## Version 0.7.170

- Pruned cells are not serialized to BOC, only hash is written

## Version 0.7.120

- Parse config parameter 44

## Version 0.7.118

- Supported new enum variant "ComputeSkipReason::Suspended"

## Version 0.7.109

- Supported ever-types version 2.0
