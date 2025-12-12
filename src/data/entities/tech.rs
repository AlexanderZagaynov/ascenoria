use std::collections::HashMap;
use serde::Deserialize;
use crate::data::localization::LocalizedText;
use crate::{impl_has_id, impl_localized_entity};

/// Technology entry with cost and localization.
#[derive(Debug, Clone, Deserialize)]
pub struct Tech {
    /// Stable identifier.
    pub id: String,
    /// Localized name.
    pub name: LocalizedText,
    /// Localized description.
    pub description: LocalizedText,
    /// Research cost to unlock the technology.
    pub research_cost: i32,
}

/// Edge between technologies (from prerequisite to unlock).
#[derive(Debug, Clone, Deserialize)]
pub struct TechEdge {
    /// Prerequisite tech id.
    pub from: String,
    /// Dependent tech id.
    pub to: String,
}

/// Research graph mapping prerequisites and unlocks.
#[derive(Debug, Default, Clone)]
pub struct ResearchGraph {
    /// Maps tech id to its prerequisites.
    pub prereqs: HashMap<String, Vec<String>>,
    /// Maps tech id to technologies it unlocks.
    pub unlocks: HashMap<String, Vec<String>>,
}

impl_localized_entity!(Tech);
impl_has_id!(Tech);
