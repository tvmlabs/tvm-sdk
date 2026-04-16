
```
% tvm-debugger --help
Helper tool, that allows you to run Acki-Nacki virtual machine, get VM trace, output messages and update contract state offchain

Usage: tvm-debugger <COMMAND>

Commands:
  run             Run contract localy with specified parameters
  boc-encode      Encodes given parameters in JSON into a BOC
  boc-decode      Decodes BOC into JSON as a set of provided parameters
  boc-hash        Read BOC string from stdin and print its hash
  state-encode    Encodes initial contract state from code, data, libraries ans special options
  state-decode    Decodes initial contract state into code, data, libraries ans special options
  account-encode  Creates account state BOC
  help            Print this message or the help of the given subcommand(s)
```


## Examples:

Go to repo root, build debugger and copy it to test folder

```
cargo build --release
cd tvm_debugget/test/contract
cp ../../../target/release/tvm-debugger ./
```


### boc-encode
```
./tvm-debugger boc-encode --data everwallet.data.json --params everwallet.params.json 
```

output:
```
{"boc":"te6ccgEBAQEAKgAAUBBNJAZaaPnf8kV8+nQT9uegjrBVtC+/J9FK0mWWRwg2AAAAAEmWAtI="}
```

### boc-decode
```
boc-decode --boc "te6ccgEBAQEAKgAAUBBNJAZaaPnf8kV8+nQT9uegjrBVtC+/J9FK0mWWRwg2AAAAAEmWAtI="  --params everwallet.params.json
```

output:
```
{"data":{"_pubkey":"0x104d24065a68f9dff2457cfa7413f6e7a08eb055b42fbf27d14ad26596470836","_timestamp":"1234567890"}}
```


### state-encode
```
./tvm-debugger state-encode --code contract.code.base64 --data "te6ccgEBAQEAKgAAUBBNJAZaaPnf8kV8+nQT9uegjrBVtC+/J9FK0mWWRwg2AAAAAEmWAtI="
```

output:

```
{"state_init":"te6ccgECGgEAAvQAAgE0AgEAUBBNJAZaaPnf8kV8+nQT9uegjrBVtC+/J9FK0mWWRwg2AAAAAEmWAtIEJIrtUyDjAyDA/+MCIMD+4wLyCxcEAxkCsCHbPNMAAY4fgwjXGCD4KMjOzsn5AAHTAAGU0/9QM5MC+ELi+RDyqJXTAAHyeuLTPwH4QyG58rQg+COBA+iogggbd0CgufK0+GPTHwH4I7zyudMfAds88jwVBQNS7UTQgQFA1yHXCgD4ZiLQ1wsDqTgA3CHHAOMCIdcNH/K8IeMDAds88jwWFgUEUCCCEEwofna74wIgghBr8n7Du+MCIIIQeAYBa7rjAiCCEHwSFjC64wINCgcGAVAw0ds8+EohjhyNBHAAAAAAAAAAAAAAAAA/BIWMIMjOy//JcPsA3vIAFQIaMPhG8uBM0ds84wDyAAkIACjtRNDT/9M/MfhDWMjL/8s/zsntVABU+ABw+CjIz4WIzoKYHMS0AAAAAAAAAAAAAAAAAAAyJTvdzwumy//JcPsAAiggghBotV8/uuMCIIIQa/J+w7rjAgwLAVAw0ds8+EshjhyNBHAAAAAAAAAAAAAAAAA6/J+w4MjOyx/JcPsA3vIAFQIuMPhCbuMAcPhqcPhr+Ebyc9H4ANs88gAVEwM8IIIQL2/POrrjAiCCEDIlO9264wIgghBMKH52uuMCEhAOAyQw+Eby4Ez4Qm7jANHbPNs88gAVDxMADPgA+C34awM0MPhG8uBM+EJu4wAhk9TR0N7T/9HbPNs88gAVERMAhvgAIMEGjjpwkyDBBI4x+Er4JKD4aiGk+CjIz4WIzoKYHMS0AAAAAAAAAAAAAAAAAAAyJTvdzwumy//JcPsApOgw3jADJDD4RvLgTPhCbuMA0ds82zzyABUUEwAs+Ev4SvhD+ELIy//LP8+Dy//LH8ntVAAO+AD4SqT4agAw7UTQ0//TP9MA0//TH9H4a/hq+Gb4Y/hiAAr4RvLgTAIQ9KQg9L3ywE4ZGAAUc29sIDAuNzMuMAAA"}
```


### state-decode
From tvc file: 

```
./tvm-debugger state-decode --state-init contract.tvc 
```

From base64 file:
```
./tvm-debugger state-decode --state-init contract.tvc.base64 
```

output:
```
{"code":"te6ccgECGAEAAsUABCSK7VMg4wMgwP/jAiDA/uMC8gsVAgEXArAh2zzTAAGOH4MI1xgg+CjIzs7J+QAB0wABlNP/UDOTAvhC4vkQ8qiV0wAB8nri0z8B+EMhufK0IPgjgQPoqIIIG3dAoLnytPhj0x8B+CO88rnTHwHbPPI8EwMDUu1E0IEBQNch1woA+GYi0NcLA6k4ANwhxwDjAiHXDR/yvCHjAwHbPPI8FBQDBFAgghBMKH52u+MCIIIQa/J+w7vjAiCCEHgGAWu64wIgghB8EhYwuuMCCwgFBAFQMNHbPPhKIY4cjQRwAAAAAAAAAAAAAAAAPwSFjCDIzsv/yXD7AN7yABMCGjD4RvLgTNHbPOMA8gAHBgAo7UTQ0//TPzH4Q1jIy//LP87J7VQAVPgAcPgoyM+FiM6CmBzEtAAAAAAAAAAAAAAAAAAAMiU73c8Lpsv/yXD7AAIoIIIQaLVfP7rjAiCCEGvyfsO64wIKCQFQMNHbPPhLIY4cjQRwAAAAAAAAAAAAAAAAOvyfsODIzssfyXD7AN7yABMCLjD4Qm7jAHD4anD4a/hG8nPR+ADbPPIAExEDPCCCEC9vzzq64wIgghAyJTvduuMCIIIQTCh+drrjAhAODAMkMPhG8uBM+EJu4wDR2zzbPPIAEw0RAAz4APgt+GsDNDD4RvLgTPhCbuMAIZPU0dDe0//R2zzbPPIAEw8RAIb4ACDBBo46cJMgwQSOMfhK+CSg+GohpPgoyM+FiM6CmBzEtAAAAAAAAAAAAAAAAAAAMiU73c8Lpsv/yXD7AKToMN4wAyQw+Eby4Ez4Qm7jANHbPNs88gATEhEALPhL+Er4Q/hCyMv/yz/Pg8v/yx/J7VQADvgA+Eqk+GoAMO1E0NP/0z/TANP/0x/R+Gv4avhm+GP4YgAK+Eby4EwCEPSkIPS98sBOFxYAFHNvbCAwLjczLjAAAA==","code_hash":"7e8cb4cf15f08ac9ddfefa3e5b237253bbf43133ac069c629c824cb69638040e","code_depth":5,"data":"te6ccgEBAQEATwAAmUZwR+gdaoRh0jd9rKUTxUo+LDi6IC1Q6uAMv9e/P1dwAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABA","data_hash":"5c00170d89e7700b1ab33bbd438ae613a868bc7b2c5fbf66a97f061a1ee98281","data_depth":0,"library":null,"tick":null,"tock":null,"split_depth":null,"compiler_version":"sol 0.73.0"}
```

### account-encode

```
./tvm-debugger account-encode contract.tvc.base64 --balance 1000000000 --last-trans-lt 100 --last-paid 1740490043
```
output:
```
{"account":"te6ccgECHAEAA1QAAgHAGwECJQAAAAAAAAAAAAAAABkQ7msoATQDAgCZRnBH6B1qhGHSN32spRPFSj4sOLogLVDq4Ay/178/V3AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAEAEJIrtUyDjAyDA/+MCIMD+4wLyCxgFBBoCsCHbPNMAAY4fgwjXGCD4KMjOzsn5AAHTAAGU0/9QM5MC+ELi+RDyqJXTAAHyeuLTPwH4QyG58rQg+COBA+iogggbd0CgufK0+GPTHwH4I7zyudMfAds88jwWBgNS7UTQgQFA1yHXCgD4ZiLQ1wsDqTgA3CHHAOMCIdcNH/K8IeMDAds88jwXFwYEUCCCEEwofna74wIgghBr8n7Du+MCIIIQeAYBa7rjAiCCEHwSFjC64wIOCwgHAVAw0ds8+EohjhyNBHAAAAAAAAAAAAAAAAA/BIWMIMjOy//JcPsA3vIAFgIaMPhG8uBM0ds84wDyAAoJACjtRNDT/9M/MfhDWMjL/8s/zsntVABU+ABw+CjIz4WIzoKYHMS0AAAAAAAAAAAAAAAAAAAyJTvdzwumy//JcPsAAiggghBotV8/uuMCIIIQa/J+w7rjAg0MAVAw0ds8+EshjhyNBHAAAAAAAAAAAAAAAAA6/J+w4MjOyx/JcPsA3vIAFgIuMPhCbuMAcPhqcPhr+Ebyc9H4ANs88gAWFAM8IIIQL2/POrrjAiCCEDIlO9264wIgghBMKH52uuMCExEPAyQw+Eby4Ez4Qm7jANHbPNs88gAWEBQADPgA+C34awM0MPhG8uBM+EJu4wAhk9TR0N7T/9HbPNs88gAWEhQAhvgAIMEGjjpwkyDBBI4x+Er4JKD4aiGk+CjIz4WIzoKYHMS0AAAAAAAAAAAAAAAAAAAyJTvdzwumy//JcPsApOgw3jADJDD4RvLgTPhCbuMA0ds82zzyABYVFAAs+Ev4SvhD+ELIy//LP8+Dy//LH8ntVAAO+AD4SqT4agAw7UTQ0//TP9MA0//TH9H4a/hq+Gb4Y/hiAAr4RvLgTAIQ9KQg9L3ywE4aGQAUc29sIDAuNzMuMAAAAEOAGoc8WEYbfqVCtdMgkd8ZzK3oOhQLoh7wV2Z1up7qBXyo","id":"d439e2c230dbf52a15ae99048ef8ce656f41d0a05d10f782bb33add4f7502be5"}
```