use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `engine.validator.config`\n\n```text\nengine.validator.config out_port:int addrs:(vector engine.Addr) adnl:(vector engine.adnl) \n        dht:(vector engine.dht)\n        validators:(vector engine.validator) fullnode:int256 fullnodeslaves:(vector engine.validator.fullNodeSlave)\n        fullnodemasters:(vector engine.validator.fullNodeMaster)\n        liteservers:(vector engine.liteServer) control:(vector engine.controlInterface)\n        gc:engine.gc = engine.validator.Config;\n```\n"]
pub struct Config {
    pub out_port: crate::ton::int,
    pub addrs: crate::ton::vector<crate::ton::engine::Addr>,
    pub adnl: crate::ton::vector<crate::ton::engine::adnl::Adnl>,
    pub dht: crate::ton::vector<crate::ton::engine::dht::Dht>,
    pub validators: crate::ton::vector<crate::ton::engine::validator::Validator>,
    pub fullnode: crate::ton::int256,
    pub fullnodeslaves:
        crate::ton::vector<crate::ton::engine::validator::fullnodeslave::FullNodeSlave>,
    pub fullnodemasters:
        crate::ton::vector<crate::ton::engine::validator::fullnodemaster::FullNodeMaster>,
    pub liteservers: crate::ton::vector<crate::ton::engine::liteserver::LiteServer>,
    pub control: crate::ton::vector<crate::ton::engine::controlinterface::ControlInterface>,
    pub gc: crate::ton::engine::gc::Gc,
}
impl Eq for Config {}
impl crate::BareSerialize for Config {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xcec219a4)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Config {
            out_port,
            addrs,
            adnl,
            dht,
            validators,
            fullnode,
            fullnodeslaves,
            fullnodemasters,
            liteservers,
            control,
            gc,
        } = self;
        _ser.write_bare::<crate::ton::int>(out_port)?;
        (addrs as &dyn crate::ton::VectoredBoxed<crate::ton::engine::Addr>).serialize(_ser)?;
        (adnl as &dyn crate::ton::VectoredBare<crate::ton::engine::adnl::Adnl>).serialize(_ser)?;
        (dht as &dyn crate::ton::VectoredBare<crate::ton::engine::dht::Dht>).serialize(_ser)?;
        (validators as &dyn crate::ton::VectoredBare<crate::ton::engine::validator::Validator>)
            .serialize(_ser)?;
        _ser.write_bare::<crate::ton::int256>(fullnode)?;
        (fullnodeslaves
            as &dyn crate::ton::VectoredBare<
                crate::ton::engine::validator::fullnodeslave::FullNodeSlave,
            >)
            .serialize(_ser)?;
        (fullnodemasters
            as &dyn crate::ton::VectoredBare<
                crate::ton::engine::validator::fullnodemaster::FullNodeMaster,
            >)
            .serialize(_ser)?;
        (liteservers as &dyn crate::ton::VectoredBare<crate::ton::engine::liteserver::LiteServer>)
            .serialize(_ser)?;
        (control as & dyn crate :: ton :: VectoredBare < crate :: ton :: engine :: controlinterface :: ControlInterface >) . serialize (_ser) ? ;
        _ser.write_bare::<crate::ton::engine::gc::Gc>(gc)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Config {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let out_port = _de.read_bare::<crate::ton::int>()?;
            let addrs = <Vec<crate::ton::engine::Addr> as crate::ton::VectoredBoxed<
                crate::ton::engine::Addr,
            >>::deserialize(_de)?;
            let adnl = <Vec<crate::ton::engine::adnl::Adnl> as crate::ton::VectoredBare<
                crate::ton::engine::adnl::Adnl,
            >>::deserialize(_de)?;
            let dht = <Vec<crate::ton::engine::dht::Dht> as crate::ton::VectoredBare<
                crate::ton::engine::dht::Dht,
            >>::deserialize(_de)?;
            let validators =
                <Vec<crate::ton::engine::validator::Validator> as crate::ton::VectoredBare<
                    crate::ton::engine::validator::Validator,
                >>::deserialize(_de)?;
            let fullnode = _de.read_bare::<crate::ton::int256>()?;
            let fullnodeslaves = < Vec < crate :: ton :: engine :: validator :: fullnodeslave :: FullNodeSlave > as crate :: ton :: VectoredBare < crate :: ton :: engine :: validator :: fullnodeslave :: FullNodeSlave >> :: deserialize (_de) ? ;
            let fullnodemasters = <Vec<
                crate::ton::engine::validator::fullnodemaster::FullNodeMaster,
            > as crate::ton::VectoredBare<
                crate::ton::engine::validator::fullnodemaster::FullNodeMaster,
            >>::deserialize(_de)?;
            let liteservers =
                <Vec<crate::ton::engine::liteserver::LiteServer> as crate::ton::VectoredBare<
                    crate::ton::engine::liteserver::LiteServer,
                >>::deserialize(_de)?;
            let control = < Vec < crate :: ton :: engine :: controlinterface :: ControlInterface > as crate :: ton :: VectoredBare < crate :: ton :: engine :: controlinterface :: ControlInterface >> :: deserialize (_de) ? ;
            let gc = _de.read_bare::<crate::ton::engine::gc::Gc>()?;
            Ok(Self {
                out_port,
                addrs,
                adnl,
                dht,
                validators,
                fullnode,
                fullnodeslaves,
                fullnodemasters,
                liteservers,
                control,
                gc,
            })
        }
    }
}
impl crate::IntoBoxed for Config {
    type Boxed = crate::ton::engine::validator::Config;

    fn into_boxed(self) -> crate::ton::engine::validator::Config {
        crate::ton::engine::validator::Config::Engine_Validator_Config(self)
    }
}
