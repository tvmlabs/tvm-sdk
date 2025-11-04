# Patch Tvm debugger
patch tvm_debugger to allow it to read functions parameters from a file (because they get so big at a point that your bash will give you an error about too long or too mush parameters)

somthing like
```
cd tvm-sdk
git apply tvm_debugger.patch
```

# Cell_generator
## What's that
this is a utility program to generate cells of defined depth (and tweak them)
each leaf, except for the last, contains a uint8 `42` and a ref to the next (intermidiate) leaf
only the last leaf contains just uint8 `24` (not `42`)
## Compiling
put the `cell_generator` folder right inside your `tvm_sdk` folder
patch `Cargo.toml` to include cell_generator as a subproject
then just build with `cargo build -p cell_generator`

# Produce cell examples
`produce_examples.sh` and `produce_huge_examples.sh` create three files with base64 of some cell
1. `ord.txt` contains an ordinary cell
2. `non_forged.txt` contains a BOC3 cell
3. `forged.txt` contains a tweaked BOC3 cell

the only difference between `produce_examples.sh` and `produce_huge_examples.sh` is a depth of a cell, 5 in former and 2046 in latter

# Now you can run scripts in `./contract` folder
## `fake_size.sh`
this one will show that one can fake cell's size in BOC3 format
it make two calls to a function which will ask for cell's `.depth()` and then try to recursively `abi.decode()` it
in the case of unforged cell, it will show correct depth and return a mapping showing correct amount of "values"
but in the case of a forged cell, it will incorrectly `abi.decode` only once and then in the returning value one can see the "remainder" of a cell, which, if you decode and look at it, still contains a lot of "undiscovered" values

## `write_cell_and_corrupt_state.sh`
this one will show that if you write an (unforged even!) BOC3 into the state of a contract, the state become invalid, at least in the eyes of a tvm_debugger
so every call to the contract will fail

## `huge_boc.sh`
for this one you need a `produce_huge_examples.sh` which will create a very deep cell, with a depth of 2046
and script shows that if send a message with cell this big in ordinary format to a contract, contract execution will fail
but! if you send it in boc3 format - everything will be fine in a sense that contract will execute (processing a huge boc, much bigger than what is allowed by the limits)
