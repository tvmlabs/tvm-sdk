use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `validatorSession.config`\n\n```text\nvalidatorSession.config catchain_idle_timeout:double catchain_max_deps:int round_candidates:int next_candidate_delay:double round_attempt_duration:int\n        max_round_attempts:int max_block_size:int max_collated_data_size:int = validatorSession.Config;\n```\n"]
pub struct Config {
    pub catchain_idle_timeout: crate::ton::double,
    pub catchain_max_deps: crate::ton::int,
    pub round_candidates: crate::ton::int,
    pub next_candidate_delay: crate::ton::double,
    pub round_attempt_duration: crate::ton::int,
    pub max_round_attempts: crate::ton::int,
    pub max_block_size: crate::ton::int,
    pub max_collated_data_size: crate::ton::int,
}
impl Eq for Config {}
impl crate::BareSerialize for Config {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xb661fdc3)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Config {
            catchain_idle_timeout,
            catchain_max_deps,
            round_candidates,
            next_candidate_delay,
            round_attempt_duration,
            max_round_attempts,
            max_block_size,
            max_collated_data_size,
        } = self;
        _ser.write_bare::<crate::ton::double>(catchain_idle_timeout)?;
        _ser.write_bare::<crate::ton::int>(catchain_max_deps)?;
        _ser.write_bare::<crate::ton::int>(round_candidates)?;
        _ser.write_bare::<crate::ton::double>(next_candidate_delay)?;
        _ser.write_bare::<crate::ton::int>(round_attempt_duration)?;
        _ser.write_bare::<crate::ton::int>(max_round_attempts)?;
        _ser.write_bare::<crate::ton::int>(max_block_size)?;
        _ser.write_bare::<crate::ton::int>(max_collated_data_size)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Config {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let catchain_idle_timeout = _de.read_bare::<crate::ton::double>()?;
            let catchain_max_deps = _de.read_bare::<crate::ton::int>()?;
            let round_candidates = _de.read_bare::<crate::ton::int>()?;
            let next_candidate_delay = _de.read_bare::<crate::ton::double>()?;
            let round_attempt_duration = _de.read_bare::<crate::ton::int>()?;
            let max_round_attempts = _de.read_bare::<crate::ton::int>()?;
            let max_block_size = _de.read_bare::<crate::ton::int>()?;
            let max_collated_data_size = _de.read_bare::<crate::ton::int>()?;
            Ok(Self {
                catchain_idle_timeout,
                catchain_max_deps,
                round_candidates,
                next_candidate_delay,
                round_attempt_duration,
                max_round_attempts,
                max_block_size,
                max_collated_data_size,
            })
        }
    }
}
impl crate::IntoBoxed for Config {
    type Boxed = crate::ton::validator_session::Config;

    fn into_boxed(self) -> crate::ton::validator_session::Config {
        crate::ton::validator_session::Config::ValidatorSession_Config(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `validatorSession.configNew`\n\n```text\nvalidatorSession.configNew catchain_idle_timeout:double catchain_max_deps:int round_candidates:int next_candidate_delay:double round_attempt_duration:int\n        max_round_attempts:int max_block_size:int max_collated_data_size:int new_catchain_ids:Bool = validatorSession.Config;\n```\n"]
pub struct ConfigNew {
    pub catchain_idle_timeout: crate::ton::double,
    pub catchain_max_deps: crate::ton::int,
    pub round_candidates: crate::ton::int,
    pub next_candidate_delay: crate::ton::double,
    pub round_attempt_duration: crate::ton::int,
    pub max_round_attempts: crate::ton::int,
    pub max_block_size: crate::ton::int,
    pub max_collated_data_size: crate::ton::int,
    pub new_catchain_ids: crate::ton::Bool,
}
impl Eq for ConfigNew {}
impl crate::BareSerialize for ConfigNew {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xf7afa99c)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let ConfigNew {
            catchain_idle_timeout,
            catchain_max_deps,
            round_candidates,
            next_candidate_delay,
            round_attempt_duration,
            max_round_attempts,
            max_block_size,
            max_collated_data_size,
            new_catchain_ids,
        } = self;
        _ser.write_bare::<crate::ton::double>(catchain_idle_timeout)?;
        _ser.write_bare::<crate::ton::int>(catchain_max_deps)?;
        _ser.write_bare::<crate::ton::int>(round_candidates)?;
        _ser.write_bare::<crate::ton::double>(next_candidate_delay)?;
        _ser.write_bare::<crate::ton::int>(round_attempt_duration)?;
        _ser.write_bare::<crate::ton::int>(max_round_attempts)?;
        _ser.write_bare::<crate::ton::int>(max_block_size)?;
        _ser.write_bare::<crate::ton::int>(max_collated_data_size)?;
        _ser.write_boxed::<crate::ton::Bool>(new_catchain_ids)?;
        Ok(())
    }
}
impl crate::BareDeserialize for ConfigNew {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let catchain_idle_timeout = _de.read_bare::<crate::ton::double>()?;
            let catchain_max_deps = _de.read_bare::<crate::ton::int>()?;
            let round_candidates = _de.read_bare::<crate::ton::int>()?;
            let next_candidate_delay = _de.read_bare::<crate::ton::double>()?;
            let round_attempt_duration = _de.read_bare::<crate::ton::int>()?;
            let max_round_attempts = _de.read_bare::<crate::ton::int>()?;
            let max_block_size = _de.read_bare::<crate::ton::int>()?;
            let max_collated_data_size = _de.read_bare::<crate::ton::int>()?;
            let new_catchain_ids = _de.read_boxed::<crate::ton::Bool>()?;
            Ok(Self {
                catchain_idle_timeout,
                catchain_max_deps,
                round_candidates,
                next_candidate_delay,
                round_attempt_duration,
                max_round_attempts,
                max_block_size,
                max_collated_data_size,
                new_catchain_ids,
            })
        }
    }
}
impl crate::IntoBoxed for ConfigNew {
    type Boxed = crate::ton::validator_session::Config;

    fn into_boxed(self) -> crate::ton::validator_session::Config {
        crate::ton::validator_session::Config::ValidatorSession_ConfigNew(self)
    }
}
