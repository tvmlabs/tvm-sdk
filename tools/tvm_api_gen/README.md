# TVM SDK command line tool

`tvm_api_gen` is a command line interface utility designed to work with TVM SDK.

## How to build

```bash
cargo build [--release]
```

## How to test
```bash
cargo test
```

## How to run

```bash
> tvm_api_gen [-n network] command parameters
```
Where `network` is a network address. By default, utility connects to `shellnet.ackinacki.org` network.

