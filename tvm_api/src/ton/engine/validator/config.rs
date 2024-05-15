use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `engine.validator.config`\n\n```text\nengine.validator.config out_port:int addrs:(vector engine.Addr) adnl:(vector engine.adnl) \n        dht:(vector engine.dht)\n        validators:(vector engine.validator) fullnode:int256 fullnodeslaves:(vector engine.validator.fullNodeSlave)\n        fullnodemasters:(vector engine.validator.fullNodeMaster)\n        liteservers:(vector engine.liteServer) control:(vector engine.controlInterface)\n        gc:engine.gc = engine.validator.Config;\n```\n"]
pub struct Config {
    pub out_port: crate::ton::int,
    pub addrs: crate::ton::vector<crate::ton::Boxed, crate::ton::engine::Addr>,
    pub adnl: crate::ton::vector<crate::ton::Bare, crate::ton::engine::adnl::Adnl>,
    pub dht: crate::ton::vector<crate::ton::Bare, crate::ton::engine::dht::Dht>,
    pub validators: crate::ton::vector<crate::ton::Bare, crate::ton::engine::validator::Validator>,
    pub fullnode: crate::ton::int256,
    pub fullnodeslaves: crate::ton::vector<
        crate::ton::Bare,
        crate::ton::engine::validator::fullnodeslave::FullNodeSlave,
    >,
    pub fullnodemasters: crate::ton::vector<
        crate::ton::Bare,
        crate::ton::engine::validator::fullnodemaster::FullNodeMaster,
    >,
    pub liteservers:
        crate::ton::vector<crate::ton::Bare, crate::ton::engine::liteserver::LiteServer>,
    pub control: crate::ton::vector<
        crate::ton::Bare,
        crate::ton::engine::controlinterface::ControlInterface,
    >,
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
        _ser.write_bare::<crate::ton::vector<crate::ton::Boxed, crate::ton::engine::Addr>>(addrs)?;
        _ser.write_bare::<crate::ton::vector<crate::ton::Bare, crate::ton::engine::adnl::Adnl>>(
            adnl,
        )?;
        _ser.write_bare::<crate::ton::vector<crate::ton::Bare, crate::ton::engine::dht::Dht>>(dht)?;
        _ser . write_bare :: < crate :: ton :: vector < crate :: ton :: Bare , crate :: ton :: engine :: validator :: Validator > > (validators) ? ;
        _ser.write_bare::<crate::ton::int256>(fullnode)?;
        _ser.write_bare::<crate::ton::vector<
            crate::ton::Bare,
            crate::ton::engine::validator::fullnodeslave::FullNodeSlave,
        >>(fullnodeslaves)?;
        _ser.write_bare::<crate::ton::vector<
            crate::ton::Bare,
            crate::ton::engine::validator::fullnodemaster::FullNodeMaster,
        >>(fullnodemasters)?;
        _ser . write_bare :: < crate :: ton :: vector < crate :: ton :: Bare , crate :: ton :: engine :: liteserver :: LiteServer > > (liteservers) ? ;
        _ser.write_bare::<crate::ton::vector<
            crate::ton::Bare,
            crate::ton::engine::controlinterface::ControlInterface,
        >>(control)?;
        _ser.write_bare::<crate::ton::engine::gc::Gc>(gc)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Config {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let out_port = _de.read_bare::<crate::ton::int>()?;
            let addrs =
                _de.read_bare::<crate::ton::vector<crate::ton::Boxed, crate::ton::engine::Addr>>()?;
            let adnl = _de
                .read_bare::<crate::ton::vector<crate::ton::Bare, crate::ton::engine::adnl::Adnl>>(
                )?;
            let dht = _de
                .read_bare::<crate::ton::vector<crate::ton::Bare, crate::ton::engine::dht::Dht>>(
                )?;
            let validators = _de . read_bare :: < crate :: ton :: vector < crate :: ton :: Bare , crate :: ton :: engine :: validator :: Validator > > () ? ;
            let fullnode = _de.read_bare::<crate::ton::int256>()?;
            let fullnodeslaves = _de.read_bare::<crate::ton::vector<
                crate::ton::Bare,
                crate::ton::engine::validator::fullnodeslave::FullNodeSlave,
            >>()?;
            let fullnodemasters = _de.read_bare::<crate::ton::vector<
                crate::ton::Bare,
                crate::ton::engine::validator::fullnodemaster::FullNodeMaster,
            >>()?;
            let liteservers =
                _de.read_bare::<crate::ton::vector<
                    crate::ton::Bare,
                    crate::ton::engine::liteserver::LiteServer,
                >>()?;
            let control = _de.read_bare::<crate::ton::vector<
                crate::ton::Bare,
                crate::ton::engine::controlinterface::ControlInterface,
            >>()?;
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
