use serde::Deserialize;
use crate::data_types::localization::LocalizedText;
use crate::{impl_has_id, impl_localized_entity};

/// Species definition used for selection UI and AI templates.
#[derive(Debug, Clone, Deserialize)]
pub struct Species {
    /// Stable identifier used by references and save games.
    pub id: String,
    /// Localized name for UI presentation.
    pub name: LocalizedText,
    /// Localized description text.
    pub description: LocalizedText,
}

impl_localized_entity!(Species);
impl_has_id!(Species);
