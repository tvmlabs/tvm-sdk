
---types---

tonEngine.networkProtocol.error
    err_code:int 
    msg:string 
= tonEngine.NetworkProtocol;

tonEngine.networkProtocol.validationRequest 
    id:long
    signed_block:bytes
= tonEngine.NetworkProtocol;

tonEngine.networkProtocol.emptyStepRequest 
    id:long
    empty_step:bytes
= tonEngine.NetworkProtocol;


tonEngine.networkProtocol.confirmValidation 
    id:long 
    peer:int 
    result:long
    block_seq_no:int
    block_start_lt:long
    block_end_lt:long
    block_gen_utime:int
= tonEngine.NetworkProtocol;

tonEngine.networkProtocol.requestBlockByNumber 
    id:long
    seq_no:int
    vert_seq_no:int
= tonEngine.NetworkProtocol;

tonEngine.networkProtocol.responceBlock 
    id:long
    signed_block:bytes
= tonEngine.NetworkProtocol;

tonEngine.networkProtocol.requestLastEqualShard 
    id:long
    shard_hash:int256
= tonEngine.NetworkProtocol;

tonEngine.networkProtocol.responseLastEqualShard 
    id:long
    seq_no:int
    vert_seq_no:int
= tonEngine.NetworkProtocol;


tonEngine.networkProtocol.requestNodeInfo 
    id:long
= tonEngine.NetworkProtocol;

tonEngine.networkProtocol.responseNodeInfo 
    id:long
    validator_no:int
    workchain:int
    shard_prefix:long
    shard_pfx_len:int
= tonEngine.NetworkProtocol;

tonEngine.networkProtocol.sendMessageRequest 
    id:long
    message:bytes
= tonEngine.NetworkProtocol;

tonEngine.networkProtocol.sendMessageResponse 
    id:long
    result:long
= tonEngine.NetworkProtocol;

tonEngine.networkProtocol.reflectToDbRequest 
    id:long
    transaction:bytes
    account:bytes
= tonEngine.NetworkProtocol;

tonEngine.networkProtocol.reflectToDbResponse 
    id:long
    result:long
= tonEngine.NetworkProtocol;

tonEngine.networkProtocol.rawData id:long data:bytes = tonEngine.NetworkProtocol;
tonEngine.networkProtocol.test1 id:long hash:int256 = tonEngine.NetworkProtocol;
tonEngine.networkProtocol.test2 id:long flag:Bool = tonEngine.NetworkProtocol;
tonEngine.networkProtocol.test3 id:long port:int = tonEngine.NetworkProtocol;

---functions---

engine.validator.getBundle block_id:tonNode.blockIdExt = engine.validator.Success;
engine.validator.getFutureBundle prev_block_ids:(vector tonNode.blockIdExt) = engine.validator.Success;
