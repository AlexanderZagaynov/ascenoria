use serde::Deserialize;
use crate::data::localization::LocalizedText;
use crate::{impl_has_id, impl_localized_entity};

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

impl_localized_entity!(PlanetSize);
impl_localized_entity!(PlanetSurfaceType);
impl_localized_entity!(PlanetaryItem);
impl_localized_entity!(PlanetaryProject);

impl_has_id!(PlanetSize);
impl_has_id!(PlanetSurfaceType);
impl_has_id!(PlanetaryItem);
impl_has_id!(PlanetaryProject);
