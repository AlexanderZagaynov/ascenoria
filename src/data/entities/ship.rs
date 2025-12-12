use serde::Deserialize;
use crate::data::localization::LocalizedText;
use crate::{impl_has_id, impl_localized_entity};

/// Hull class template used by the ship designer.
#[derive(Debug, Clone, Deserialize)]
pub struct HullClass {
    /// Stable identifier.
    pub id: String,
    /// Localized name.
    pub name: LocalizedText,
    /// Localized description.
    pub description: LocalizedText,
    /// Size index used for balancing.
    pub size_index: i32,
    /// Maximum module count supported by the hull.
    pub max_items: i32,
}

/// Engine module definition.
#[derive(Debug, Clone, Deserialize)]
pub struct Engine {
    /// Stable identifier.
    pub id: String,
    /// Localized name.
    pub name: LocalizedText,
    /// Localized description.
    pub description: LocalizedText,
    /// Power draw of the engine.
    pub power_use: i32,
    /// Thrust rating used for movement calculations.
    pub thrust_rating: f32,
    /// Industry cost to build.
    pub industry_cost: i32,
}

/// Weapon module definition.
#[derive(Debug, Clone, Deserialize)]
pub struct Weapon {
    /// Stable identifier.
    pub id: String,
    /// Localized name.
    pub name: LocalizedText,
    /// Localized description.
    pub description: LocalizedText,
    /// Power draw.
    pub power_use: i32,
    /// Weapon range.
    pub range: i32,
    /// Damage strength.
    pub strength: f32,
    /// Uses per turn.
    pub uses_per_turn: i32,
    /// Industry cost to build.
    pub industry_cost: i32,
    /// Required tech index to unlock.
    pub tech_index: i32,
}

/// Shield module definition.
#[derive(Debug, Clone, Deserialize)]
pub struct Shield {
    /// Stable identifier.
    pub id: String,
    /// Localized name.
    pub name: LocalizedText,
    /// Localized description.
    pub description: LocalizedText,
    /// Shield strength.
    pub strength: f32,
    /// Industry cost to build.
    pub industry_cost: i32,
}

/// Scanner module definition.
#[derive(Debug, Clone, Deserialize)]
pub struct Scanner {
    /// Stable identifier.
    pub id: String,
    /// Localized name.
    pub name: LocalizedText,
    /// Localized description.
    pub description: LocalizedText,
    /// Scanner range.
    pub range: i32,
    /// Scanner strength.
    pub strength: f32,
    /// Industry cost to build.
    pub industry_cost: i32,
}

/// Special module definition with bespoke effects.
#[derive(Debug, Clone, Deserialize)]
pub struct SpecialModule {
    /// Stable identifier.
    pub id: String,
    /// Localized name.
    pub name: LocalizedText,
    /// Localized description.
    pub description: LocalizedText,
    /// Power draw of the special module.
    pub power_use: i32,
    /// Effective range.
    pub range: i32,
    /// Industry cost to build.
    pub industry_cost: i32,
}

impl_localized_entity!(HullClass);
impl_localized_entity!(Engine);
impl_localized_entity!(Weapon);
impl_localized_entity!(Shield);
impl_localized_entity!(Scanner);
impl_localized_entity!(SpecialModule);

impl_has_id!(HullClass);
impl_has_id!(Engine);
impl_has_id!(Weapon);
impl_has_id!(Shield);
impl_has_id!(Scanner);
impl_has_id!(SpecialModule);
