/// First engine version using State V2 message and balance rules.
pub const STATE_V2_ENGINE_VERSION: semver::Version = semver::Version::new(1, 0, 9);

pub fn uses_state_v2_rules(version: &semver::Version) -> bool {
    version >= &STATE_V2_ENGINE_VERSION
}

#[cfg(test)]
mod tests {
    use super::uses_state_v2_rules;

    #[test]
    fn state_v2_rules_start_at_engine_1_0_9() {
        assert!(!uses_state_v2_rules(&semver::Version::new(1, 0, 8)));
        assert!(uses_state_v2_rules(&semver::Version::new(1, 0, 9)));
    }
}
