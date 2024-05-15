use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `tonEngine.NetworkProtocol`\n\n```text\ntonEngine.networkProtocol.confirmValidation \n    id:long \n    peer:int \n    result:long\n    block_seq_no:int\n    block_start_lt:long\n    block_end_lt:long\n    block_gen_utime:int\n= tonEngine.NetworkProtocol;\n\ntonEngine.networkProtocol.emptyStepRequest \n    id:long\n    empty_step:bytes\n= tonEngine.NetworkProtocol;\n\ntonEngine.networkProtocol.error\n    err_code:int \n    msg:string \n= tonEngine.NetworkProtocol;\n\ntonEngine.networkProtocol.rawData id:long data:bytes = tonEngine.NetworkProtocol;\n\ntonEngine.networkProtocol.reflectToDbRequest \n    id:long\n    transaction:bytes\n    account:bytes\n= tonEngine.NetworkProtocol;\n\ntonEngine.networkProtocol.reflectToDbResponse \n    id:long\n    result:long\n= tonEngine.NetworkProtocol;\n\ntonEngine.networkProtocol.requestBlockByNumber \n    id:long\n    seq_no:int\n    vert_seq_no:int\n= tonEngine.NetworkProtocol;\n\ntonEngine.networkProtocol.requestLastEqualShard \n    id:long\n    shard_hash:int256\n= tonEngine.NetworkProtocol;\n\ntonEngine.networkProtocol.requestNodeInfo \n    id:long\n= tonEngine.NetworkProtocol;\n\ntonEngine.networkProtocol.responceBlock \n    id:long\n    signed_block:bytes\n= tonEngine.NetworkProtocol;\n\ntonEngine.networkProtocol.responseLastEqualShard \n    id:long\n    seq_no:int\n    vert_seq_no:int\n= tonEngine.NetworkProtocol;\n\ntonEngine.networkProtocol.responseNodeInfo \n    id:long\n    validator_no:int\n    workchain:int\n    shard_prefix:long\n    shard_pfx_len:int\n= tonEngine.NetworkProtocol;\n\ntonEngine.networkProtocol.sendMessageRequest \n    id:long\n    message:bytes\n= tonEngine.NetworkProtocol;\n\ntonEngine.networkProtocol.sendMessageResponse \n    id:long\n    result:long\n= tonEngine.NetworkProtocol;\n\ntonEngine.networkProtocol.test1 id:long hash:int256 = tonEngine.NetworkProtocol;\n\ntonEngine.networkProtocol.test2 id:long flag:Bool = tonEngine.NetworkProtocol;\n\ntonEngine.networkProtocol.test3 id:long port:int = tonEngine.NetworkProtocol;\n\ntonEngine.networkProtocol.validationRequest \n    id:long\n    signed_block:bytes\n= tonEngine.NetworkProtocol;\n```\n"]
pub enum NetworkProtocol {
    TonEngine_NetworkProtocol_ConfirmValidation(
        Box<crate::ton::ton_engine::network_protocol::networkprotocol::ConfirmValidation>,
    ),
    TonEngine_NetworkProtocol_EmptyStepRequest(
        crate::ton::ton_engine::network_protocol::networkprotocol::EmptyStepRequest,
    ),
    TonEngine_NetworkProtocol_Error(
        crate::ton::ton_engine::network_protocol::networkprotocol::Error,
    ),
    TonEngine_NetworkProtocol_RawData(
        crate::ton::ton_engine::network_protocol::networkprotocol::RawData,
    ),
    TonEngine_NetworkProtocol_ReflectToDbRequest(
        crate::ton::ton_engine::network_protocol::networkprotocol::ReflectToDbRequest,
    ),
    TonEngine_NetworkProtocol_ReflectToDbResponse(
        crate::ton::ton_engine::network_protocol::networkprotocol::ReflectToDbResponse,
    ),
    TonEngine_NetworkProtocol_RequestBlockByNumber(
        crate::ton::ton_engine::network_protocol::networkprotocol::RequestBlockByNumber,
    ),
    TonEngine_NetworkProtocol_RequestLastEqualShard(
        crate::ton::ton_engine::network_protocol::networkprotocol::RequestLastEqualShard,
    ),
    TonEngine_NetworkProtocol_RequestNodeInfo(
        crate::ton::ton_engine::network_protocol::networkprotocol::RequestNodeInfo,
    ),
    TonEngine_NetworkProtocol_ResponceBlock(
        crate::ton::ton_engine::network_protocol::networkprotocol::ResponceBlock,
    ),
    TonEngine_NetworkProtocol_ResponseLastEqualShard(
        crate::ton::ton_engine::network_protocol::networkprotocol::ResponseLastEqualShard,
    ),
    TonEngine_NetworkProtocol_ResponseNodeInfo(
        crate::ton::ton_engine::network_protocol::networkprotocol::ResponseNodeInfo,
    ),
    TonEngine_NetworkProtocol_SendMessageRequest(
        crate::ton::ton_engine::network_protocol::networkprotocol::SendMessageRequest,
    ),
    TonEngine_NetworkProtocol_SendMessageResponse(
        crate::ton::ton_engine::network_protocol::networkprotocol::SendMessageResponse,
    ),
    TonEngine_NetworkProtocol_Test1(
        crate::ton::ton_engine::network_protocol::networkprotocol::Test1,
    ),
    TonEngine_NetworkProtocol_Test2(
        crate::ton::ton_engine::network_protocol::networkprotocol::Test2,
    ),
    TonEngine_NetworkProtocol_Test3(
        crate::ton::ton_engine::network_protocol::networkprotocol::Test3,
    ),
    TonEngine_NetworkProtocol_ValidationRequest(
        crate::ton::ton_engine::network_protocol::networkprotocol::ValidationRequest,
    ),
}
impl NetworkProtocol {
    pub fn account(&self) -> Option<&crate::ton::bytes> {
        match self {
            NetworkProtocol::TonEngine_NetworkProtocol_ReflectToDbRequest(ref x) => {
                Some(&x.account)
            }
            _ => None,
        }
    }
    pub fn block_end_lt(&self) -> Option<&crate::ton::long> {
        match self {
            NetworkProtocol::TonEngine_NetworkProtocol_ConfirmValidation(ref x) => {
                Some(&x.block_end_lt)
            }
            _ => None,
        }
    }
    pub fn block_gen_utime(&self) -> Option<&crate::ton::int> {
        match self {
            NetworkProtocol::TonEngine_NetworkProtocol_ConfirmValidation(ref x) => {
                Some(&x.block_gen_utime)
            }
            _ => None,
        }
    }
    pub fn block_seq_no(&self) -> Option<&crate::ton::int> {
        match self {
            NetworkProtocol::TonEngine_NetworkProtocol_ConfirmValidation(ref x) => {
                Some(&x.block_seq_no)
            }
            _ => None,
        }
    }
    pub fn block_start_lt(&self) -> Option<&crate::ton::long> {
        match self {
            NetworkProtocol::TonEngine_NetworkProtocol_ConfirmValidation(ref x) => {
                Some(&x.block_start_lt)
            }
            _ => None,
        }
    }
    pub fn data(&self) -> Option<&crate::ton::bytes> {
        match self {
            NetworkProtocol::TonEngine_NetworkProtocol_RawData(ref x) => Some(&x.data),
            _ => None,
        }
    }
    pub fn empty_step(&self) -> Option<&crate::ton::bytes> {
        match self {
            NetworkProtocol::TonEngine_NetworkProtocol_EmptyStepRequest(ref x) => {
                Some(&x.empty_step)
            }
            _ => None,
        }
    }
    pub fn err_code(&self) -> Option<&crate::ton::int> {
        match self {
            NetworkProtocol::TonEngine_NetworkProtocol_Error(ref x) => Some(&x.err_code),
            _ => None,
        }
    }
    pub fn flag(&self) -> Option<&crate::ton::Bool> {
        match self {
            NetworkProtocol::TonEngine_NetworkProtocol_Test2(ref x) => Some(&x.flag),
            _ => None,
        }
    }
    pub fn hash(&self) -> Option<&crate::ton::int256> {
        match self {
            NetworkProtocol::TonEngine_NetworkProtocol_Test1(ref x) => Some(&x.hash),
            _ => None,
        }
    }
    pub fn id(&self) -> Option<&crate::ton::long> {
        match self {
            NetworkProtocol::TonEngine_NetworkProtocol_ConfirmValidation(ref x) => Some(&x.id),
            NetworkProtocol::TonEngine_NetworkProtocol_EmptyStepRequest(ref x) => Some(&x.id),
            NetworkProtocol::TonEngine_NetworkProtocol_RawData(ref x) => Some(&x.id),
            NetworkProtocol::TonEngine_NetworkProtocol_ReflectToDbRequest(ref x) => Some(&x.id),
            NetworkProtocol::TonEngine_NetworkProtocol_ReflectToDbResponse(ref x) => Some(&x.id),
            NetworkProtocol::TonEngine_NetworkProtocol_RequestBlockByNumber(ref x) => Some(&x.id),
            NetworkProtocol::TonEngine_NetworkProtocol_RequestLastEqualShard(ref x) => Some(&x.id),
            NetworkProtocol::TonEngine_NetworkProtocol_RequestNodeInfo(ref x) => Some(&x.id),
            NetworkProtocol::TonEngine_NetworkProtocol_ResponceBlock(ref x) => Some(&x.id),
            NetworkProtocol::TonEngine_NetworkProtocol_ResponseLastEqualShard(ref x) => Some(&x.id),
            NetworkProtocol::TonEngine_NetworkProtocol_ResponseNodeInfo(ref x) => Some(&x.id),
            NetworkProtocol::TonEngine_NetworkProtocol_SendMessageRequest(ref x) => Some(&x.id),
            NetworkProtocol::TonEngine_NetworkProtocol_SendMessageResponse(ref x) => Some(&x.id),
            NetworkProtocol::TonEngine_NetworkProtocol_Test1(ref x) => Some(&x.id),
            NetworkProtocol::TonEngine_NetworkProtocol_Test2(ref x) => Some(&x.id),
            NetworkProtocol::TonEngine_NetworkProtocol_Test3(ref x) => Some(&x.id),
            NetworkProtocol::TonEngine_NetworkProtocol_ValidationRequest(ref x) => Some(&x.id),
            _ => None,
        }
    }
    pub fn message(&self) -> Option<&crate::ton::bytes> {
        match self {
            NetworkProtocol::TonEngine_NetworkProtocol_SendMessageRequest(ref x) => {
                Some(&x.message)
            }
            _ => None,
        }
    }
    pub fn msg(&self) -> Option<&crate::ton::string> {
        match self {
            NetworkProtocol::TonEngine_NetworkProtocol_Error(ref x) => Some(&x.msg),
            _ => None,
        }
    }
    pub fn peer(&self) -> Option<&crate::ton::int> {
        match self {
            NetworkProtocol::TonEngine_NetworkProtocol_ConfirmValidation(ref x) => Some(&x.peer),
            _ => None,
        }
    }
    pub fn port(&self) -> Option<&crate::ton::int> {
        match self {
            NetworkProtocol::TonEngine_NetworkProtocol_Test3(ref x) => Some(&x.port),
            _ => None,
        }
    }
    pub fn result(&self) -> Option<&crate::ton::long> {
        match self {
            NetworkProtocol::TonEngine_NetworkProtocol_ConfirmValidation(ref x) => Some(&x.result),
            NetworkProtocol::TonEngine_NetworkProtocol_ReflectToDbResponse(ref x) => {
                Some(&x.result)
            }
            NetworkProtocol::TonEngine_NetworkProtocol_SendMessageResponse(ref x) => {
                Some(&x.result)
            }
            _ => None,
        }
    }
    pub fn seq_no(&self) -> Option<&crate::ton::int> {
        match self {
            NetworkProtocol::TonEngine_NetworkProtocol_RequestBlockByNumber(ref x) => {
                Some(&x.seq_no)
            }
            NetworkProtocol::TonEngine_NetworkProtocol_ResponseLastEqualShard(ref x) => {
                Some(&x.seq_no)
            }
            _ => None,
        }
    }
    pub fn shard_hash(&self) -> Option<&crate::ton::int256> {
        match self {
            NetworkProtocol::TonEngine_NetworkProtocol_RequestLastEqualShard(ref x) => {
                Some(&x.shard_hash)
            }
            _ => None,
        }
    }
    pub fn shard_pfx_len(&self) -> Option<&crate::ton::int> {
        match self {
            NetworkProtocol::TonEngine_NetworkProtocol_ResponseNodeInfo(ref x) => {
                Some(&x.shard_pfx_len)
            }
            _ => None,
        }
    }
    pub fn shard_prefix(&self) -> Option<&crate::ton::long> {
        match self {
            NetworkProtocol::TonEngine_NetworkProtocol_ResponseNodeInfo(ref x) => {
                Some(&x.shard_prefix)
            }
            _ => None,
        }
    }
    pub fn signed_block(&self) -> Option<&crate::ton::bytes> {
        match self {
            NetworkProtocol::TonEngine_NetworkProtocol_ResponceBlock(ref x) => {
                Some(&x.signed_block)
            }
            NetworkProtocol::TonEngine_NetworkProtocol_ValidationRequest(ref x) => {
                Some(&x.signed_block)
            }
            _ => None,
        }
    }
    pub fn transaction(&self) -> Option<&crate::ton::bytes> {
        match self {
            NetworkProtocol::TonEngine_NetworkProtocol_ReflectToDbRequest(ref x) => {
                Some(&x.transaction)
            }
            _ => None,
        }
    }
    pub fn validator_no(&self) -> Option<&crate::ton::int> {
        match self {
            NetworkProtocol::TonEngine_NetworkProtocol_ResponseNodeInfo(ref x) => {
                Some(&x.validator_no)
            }
            _ => None,
        }
    }
    pub fn vert_seq_no(&self) -> Option<&crate::ton::int> {
        match self {
            NetworkProtocol::TonEngine_NetworkProtocol_RequestBlockByNumber(ref x) => {
                Some(&x.vert_seq_no)
            }
            NetworkProtocol::TonEngine_NetworkProtocol_ResponseLastEqualShard(ref x) => {
                Some(&x.vert_seq_no)
            }
            _ => None,
        }
    }
    pub fn workchain(&self) -> Option<&crate::ton::int> {
        match self {
            NetworkProtocol::TonEngine_NetworkProtocol_ResponseNodeInfo(ref x) => {
                Some(&x.workchain)
            }
            _ => None,
        }
    }
}
impl Eq for NetworkProtocol {}
impl Default for NetworkProtocol {
    fn default() -> Self {
        NetworkProtocol::TonEngine_NetworkProtocol_ConfirmValidation(Box::default())
    }
}
impl crate::BoxedSerialize for NetworkProtocol {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            NetworkProtocol::TonEngine_NetworkProtocol_ConfirmValidation(x) => {
                (crate::ConstructorNumber(0x249e0c6f), x.as_ref())
            }
            NetworkProtocol::TonEngine_NetworkProtocol_EmptyStepRequest(x) => {
                (crate::ConstructorNumber(0xdd6c0307), x)
            }
            NetworkProtocol::TonEngine_NetworkProtocol_Error(x) => {
                (crate::ConstructorNumber(0xee45fe5f), x)
            }
            NetworkProtocol::TonEngine_NetworkProtocol_RawData(x) => {
                (crate::ConstructorNumber(0x6ae8a01b), x)
            }
            NetworkProtocol::TonEngine_NetworkProtocol_ReflectToDbRequest(x) => {
                (crate::ConstructorNumber(0x291ada1e), x)
            }
            NetworkProtocol::TonEngine_NetworkProtocol_ReflectToDbResponse(x) => {
                (crate::ConstructorNumber(0xaacd0ece), x)
            }
            NetworkProtocol::TonEngine_NetworkProtocol_RequestBlockByNumber(x) => {
                (crate::ConstructorNumber(0xfc5c9887), x)
            }
            NetworkProtocol::TonEngine_NetworkProtocol_RequestLastEqualShard(x) => {
                (crate::ConstructorNumber(0xba2b9a5a), x)
            }
            NetworkProtocol::TonEngine_NetworkProtocol_RequestNodeInfo(x) => {
                (crate::ConstructorNumber(0x11d0a25b), x)
            }
            NetworkProtocol::TonEngine_NetworkProtocol_ResponceBlock(x) => {
                (crate::ConstructorNumber(0x5d721f0d), x)
            }
            NetworkProtocol::TonEngine_NetworkProtocol_ResponseLastEqualShard(x) => {
                (crate::ConstructorNumber(0x7f5d7f88), x)
            }
            NetworkProtocol::TonEngine_NetworkProtocol_ResponseNodeInfo(x) => {
                (crate::ConstructorNumber(0x23b74379), x)
            }
            NetworkProtocol::TonEngine_NetworkProtocol_SendMessageRequest(x) => {
                (crate::ConstructorNumber(0x7a70345a), x)
            }
            NetworkProtocol::TonEngine_NetworkProtocol_SendMessageResponse(x) => {
                (crate::ConstructorNumber(0x0a1055ed), x)
            }
            NetworkProtocol::TonEngine_NetworkProtocol_Test1(x) => {
                (crate::ConstructorNumber(0x7afa2679), x)
            }
            NetworkProtocol::TonEngine_NetworkProtocol_Test2(x) => {
                (crate::ConstructorNumber(0xc745311c), x)
            }
            NetworkProtocol::TonEngine_NetworkProtocol_Test3(x) => {
                (crate::ConstructorNumber(0xee49ed91), x)
            }
            NetworkProtocol::TonEngine_NetworkProtocol_ValidationRequest(x) => {
                (crate::ConstructorNumber(0xdc0928a1), x)
            }
        }
    }
}
impl crate::BoxedDeserialize for NetworkProtocol {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![
            crate::ConstructorNumber(0x249e0c6f),
            crate::ConstructorNumber(0xdd6c0307),
            crate::ConstructorNumber(0xee45fe5f),
            crate::ConstructorNumber(0x6ae8a01b),
            crate::ConstructorNumber(0x291ada1e),
            crate::ConstructorNumber(0xaacd0ece),
            crate::ConstructorNumber(0xfc5c9887),
            crate::ConstructorNumber(0xba2b9a5a),
            crate::ConstructorNumber(0x11d0a25b),
            crate::ConstructorNumber(0x5d721f0d),
            crate::ConstructorNumber(0x7f5d7f88),
            crate::ConstructorNumber(0x23b74379),
            crate::ConstructorNumber(0x7a70345a),
            crate::ConstructorNumber(0x0a1055ed),
            crate::ConstructorNumber(0x7afa2679),
            crate::ConstructorNumber(0xc745311c),
            crate::ConstructorNumber(0xee49ed91),
            crate::ConstructorNumber(0xdc0928a1),
        ]
    }
    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id { crate :: ConstructorNumber (0x249e0c6f) => Ok (NetworkProtocol :: TonEngine_NetworkProtocol_ConfirmValidation (Box :: new (_de . read_bare :: < crate :: ton :: ton_engine :: network_protocol :: networkprotocol :: ConfirmValidation > () ?))) , crate :: ConstructorNumber (0xdd6c0307) => Ok (NetworkProtocol :: TonEngine_NetworkProtocol_EmptyStepRequest (_de . read_bare :: < crate :: ton :: ton_engine :: network_protocol :: networkprotocol :: EmptyStepRequest > () ?)) , crate :: ConstructorNumber (0xee45fe5f) => Ok (NetworkProtocol :: TonEngine_NetworkProtocol_Error (_de . read_bare :: < crate :: ton :: ton_engine :: network_protocol :: networkprotocol :: Error > () ?)) , crate :: ConstructorNumber (0x6ae8a01b) => Ok (NetworkProtocol :: TonEngine_NetworkProtocol_RawData (_de . read_bare :: < crate :: ton :: ton_engine :: network_protocol :: networkprotocol :: RawData > () ?)) , crate :: ConstructorNumber (0x291ada1e) => Ok (NetworkProtocol :: TonEngine_NetworkProtocol_ReflectToDbRequest (_de . read_bare :: < crate :: ton :: ton_engine :: network_protocol :: networkprotocol :: ReflectToDbRequest > () ?)) , crate :: ConstructorNumber (0xaacd0ece) => Ok (NetworkProtocol :: TonEngine_NetworkProtocol_ReflectToDbResponse (_de . read_bare :: < crate :: ton :: ton_engine :: network_protocol :: networkprotocol :: ReflectToDbResponse > () ?)) , crate :: ConstructorNumber (0xfc5c9887) => Ok (NetworkProtocol :: TonEngine_NetworkProtocol_RequestBlockByNumber (_de . read_bare :: < crate :: ton :: ton_engine :: network_protocol :: networkprotocol :: RequestBlockByNumber > () ?)) , crate :: ConstructorNumber (0xba2b9a5a) => Ok (NetworkProtocol :: TonEngine_NetworkProtocol_RequestLastEqualShard (_de . read_bare :: < crate :: ton :: ton_engine :: network_protocol :: networkprotocol :: RequestLastEqualShard > () ?)) , crate :: ConstructorNumber (0x11d0a25b) => Ok (NetworkProtocol :: TonEngine_NetworkProtocol_RequestNodeInfo (_de . read_bare :: < crate :: ton :: ton_engine :: network_protocol :: networkprotocol :: RequestNodeInfo > () ?)) , crate :: ConstructorNumber (0x5d721f0d) => Ok (NetworkProtocol :: TonEngine_NetworkProtocol_ResponceBlock (_de . read_bare :: < crate :: ton :: ton_engine :: network_protocol :: networkprotocol :: ResponceBlock > () ?)) , crate :: ConstructorNumber (0x7f5d7f88) => Ok (NetworkProtocol :: TonEngine_NetworkProtocol_ResponseLastEqualShard (_de . read_bare :: < crate :: ton :: ton_engine :: network_protocol :: networkprotocol :: ResponseLastEqualShard > () ?)) , crate :: ConstructorNumber (0x23b74379) => Ok (NetworkProtocol :: TonEngine_NetworkProtocol_ResponseNodeInfo (_de . read_bare :: < crate :: ton :: ton_engine :: network_protocol :: networkprotocol :: ResponseNodeInfo > () ?)) , crate :: ConstructorNumber (0x7a70345a) => Ok (NetworkProtocol :: TonEngine_NetworkProtocol_SendMessageRequest (_de . read_bare :: < crate :: ton :: ton_engine :: network_protocol :: networkprotocol :: SendMessageRequest > () ?)) , crate :: ConstructorNumber (0x0a1055ed) => Ok (NetworkProtocol :: TonEngine_NetworkProtocol_SendMessageResponse (_de . read_bare :: < crate :: ton :: ton_engine :: network_protocol :: networkprotocol :: SendMessageResponse > () ?)) , crate :: ConstructorNumber (0x7afa2679) => Ok (NetworkProtocol :: TonEngine_NetworkProtocol_Test1 (_de . read_bare :: < crate :: ton :: ton_engine :: network_protocol :: networkprotocol :: Test1 > () ?)) , crate :: ConstructorNumber (0xc745311c) => Ok (NetworkProtocol :: TonEngine_NetworkProtocol_Test2 (_de . read_bare :: < crate :: ton :: ton_engine :: network_protocol :: networkprotocol :: Test2 > () ?)) , crate :: ConstructorNumber (0xee49ed91) => Ok (NetworkProtocol :: TonEngine_NetworkProtocol_Test3 (_de . read_bare :: < crate :: ton :: ton_engine :: network_protocol :: networkprotocol :: Test3 > () ?)) , crate :: ConstructorNumber (0xdc0928a1) => Ok (NetworkProtocol :: TonEngine_NetworkProtocol_ValidationRequest (_de . read_bare :: < crate :: ton :: ton_engine :: network_protocol :: networkprotocol :: ValidationRequest > () ?)) , id => _invalid_id ! (id) , }
    }
}
pub mod network_protocol;
