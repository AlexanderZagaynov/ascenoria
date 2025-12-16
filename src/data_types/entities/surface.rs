//! Surface-related data types loaded from RON files.
//!
//! These structures are deserialized from the game data files and define
//! the properties of surface cells and buildings.

use serde::Deserialize;

/// Definition of a surface cell type (e.g., "plains", "mountains").
///
/// Loaded from `surface_cell_types.ron`.
#[derive(Debug, Clone, Deserialize)]
pub struct SurfaceCellType {
    /// Unique identifier for this cell type.
    pub id: String,
    /// English display name.
    pub name_en: String,
    /// Whether buildings can be placed on this cell type.
    pub is_usable: bool,
}

/// Specifies which tile color a building can be placed on.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BuildableOn {
    /// Building can only be placed on white (usable) tiles.
    White,
    /// Building can only be placed on black tiles (for terraformer).
    Black,
}

/// Special behaviors that buildings can have.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SpecialBehavior {
    /// No special behavior, just provides yields.
    None,
    /// Converts adjacent black tiles to white.
    Terraformer,
}

/// Definition of a building that can be placed on the planet surface.
///
/// Loaded from `surface_buildings.ron`. Contains all the data needed
/// for construction costs, resource yields, and visual representation.
#[derive(Debug, Clone, Deserialize)]
pub struct SurfaceBuilding {
    /// Unique identifier (e.g., "building_farm_1").
    pub id: String,
    /// English display name.
    pub name_en: String,
    /// RGB color for the 3D mesh (0.0 - 1.0 range).
    pub color: (f32, f32, f32),
    /// What type of tile this building can be placed on.
    pub buildable_on_cell_type: BuildableOn,
    /// Whether this building extends the power grid to adjacent tiles.
    pub counts_for_adjacency: bool,
    /// Production points required to construct this building.
    pub production_cost: u32,
    /// Food yield per turn (can be negative for upkeep).
    pub yields_food: i32,
    /// Housing yield per turn.
    pub yields_housing: i32,
    /// Production yield per turn.
    pub yields_production: i32,
    /// Science yield per turn.
    pub yields_science: i32,
    /// Technology ID that must be researched before building (if any).
    pub unlocked_by_tech_id: Option<String>,
    /// Special behavior when this building is placed.
    pub special_behavior: SpecialBehavior,
}
