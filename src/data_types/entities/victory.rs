use serde::Deserialize;
use crate::data_types::localization::LocalizedText;
use crate::{impl_has_id, impl_localized_entity};

/// Victory condition archetype.
#[derive(Debug, Clone, Deserialize)]
pub struct VictoryCondition {
    /// Stable identifier.
    pub id: String,
    /// Localized name.
    pub name: LocalizedText,
    /// Localized description.
    pub description: LocalizedText,
}

/// Tunable parameters for victory checks.
#[derive(Debug, Clone, Deserialize)]
pub struct VictoryRules {
    /// Fraction of systems required to claim domination.
    pub domination_threshold: f32,
}

impl Default for VictoryRules {
    fn default() -> Self {
        Self {
            domination_threshold: 0.5,
        }
    }
}

impl_localized_entity!(VictoryCondition);
impl_has_id!(VictoryCondition);
