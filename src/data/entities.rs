//! Entity struct definitions for game data.
//!
//! Contains all the TOML-deserializable data types used throughout the game:
//! species, planets, ships, research, victory conditions, etc.
//! These are pure data holders - no logic, no loading, no validation.

use std::collections::HashMap;

use serde::Deserialize;

use super::localization::LocalizedText;
use crate::{impl_has_id, impl_localized_entity};

// ---------------------------------------------------------------------------
// Species
// ---------------------------------------------------------------------------

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

// ---------------------------------------------------------------------------
// Planets
// ---------------------------------------------------------------------------

/// Planet size definition describing available slots.
#[derive(Debug, Clone, Deserialize)]
pub struct PlanetSize {
    /// Stable identifier.
    pub id: String,
    /// Localized name.
    pub name: LocalizedText,
    /// Localized description.
    pub description: LocalizedText,
    /// Available surface building slots.
    pub surface_slots: u32,
    /// Available orbital building slots.
    pub orbital_slots: u32,
}

/// Distribution of surface tiles by color.
#[derive(Debug, Clone, Deserialize)]
pub struct TileDistribution {
    /// Hostile tiles (black).
    pub black: u32,
    /// Neutral tiles (white).
    pub white: u32,
    /// Red tiles.
    pub red: u32,
    /// Green tiles.
    pub green: u32,
    /// Blue tiles.
    pub blue: u32,
}

/// Surface composition archetype for a planet.
#[derive(Debug, Clone, Deserialize)]
pub struct PlanetSurfaceType {
    /// Stable identifier.
    pub id: String,
    /// Localized name.
    pub name: LocalizedText,
    /// Localized description.
    pub description: LocalizedText,
    /// Percent distribution of tile colors.
    pub tile_distribution: TileDistribution,
}

/// Shared structure for both surface and orbital installations.
#[derive(Debug, Clone, Deserialize)]
pub struct PlanetaryItem {
    /// Stable identifier.
    pub id: String,
    /// Localized name.
    pub name: LocalizedText,
    /// Localized description.
    pub description: LocalizedText,
    /// Industry output bonus.
    pub industry_bonus: i32,
    /// Research output bonus.
    pub research_bonus: i32,
    /// Prosperity output bonus.
    pub prosperity_bonus: i32,
    /// Maximum population bonus.
    pub max_population_bonus: i32,
    /// Slot size requirement for surface tiles.
    pub slot_size: i32,
    /// Industry cost to build.
    pub industry_cost: i32,
    /// Tech index required to unlock.
    pub tech_index: i32,
}

/// Long-running planetary project definition.
#[derive(Debug, Clone, Deserialize)]
pub struct PlanetaryProject {
    /// Stable identifier.
    pub id: String,
    /// Localized name.
    pub name: LocalizedText,
    /// Localized description.
    pub description: LocalizedText,
    /// Industry cost to complete.
    pub industry_cost: i32,
}

// ---------------------------------------------------------------------------
// Ships
// ---------------------------------------------------------------------------

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

// ---------------------------------------------------------------------------
// Research
// ---------------------------------------------------------------------------

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

// ---------------------------------------------------------------------------
// Victory
// ---------------------------------------------------------------------------

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

// ---------------------------------------------------------------------------
// Trait implementations
// ---------------------------------------------------------------------------

impl_localized_entity!(Species);
impl_localized_entity!(PlanetSize);
impl_localized_entity!(PlanetSurfaceType);
impl_localized_entity!(PlanetaryItem);
impl_localized_entity!(PlanetaryProject);
impl_localized_entity!(HullClass);
impl_localized_entity!(Engine);
impl_localized_entity!(Weapon);
impl_localized_entity!(Shield);
impl_localized_entity!(Scanner);
impl_localized_entity!(SpecialModule);
impl_localized_entity!(Tech);
impl_localized_entity!(VictoryCondition);

impl_has_id!(Species);
impl_has_id!(PlanetSize);
impl_has_id!(PlanetSurfaceType);
impl_has_id!(PlanetaryItem);
impl_has_id!(PlanetaryProject);
impl_has_id!(HullClass);
impl_has_id!(Engine);
impl_has_id!(Weapon);
impl_has_id!(Shield);
impl_has_id!(Scanner);
impl_has_id!(SpecialModule);
impl_has_id!(Tech);
impl_has_id!(VictoryCondition);
