/// First engine version using State V2 message and balance rules.
pub const STATE_V2_ENGINE_VERSION: semver::Version = semver::Version::new(1, 0, 8);

pub fn uses_state_v2_rules(version: &semver::Version) -> bool {
    version >= &STATE_V2_ENGINE_VERSION
}
