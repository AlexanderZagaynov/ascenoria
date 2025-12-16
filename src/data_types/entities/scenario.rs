//! Game scenario data structures.
//!
//! Scenarios define starting conditions, map generation parameters,
//! and victory conditions for a game session.

use serde::Deserialize;

/// Planet surface generation algorithms.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum GenerationMode {
    /// Random placement of white and black tiles based on `black_ratio`.
    RandomWhiteBlack,
}

/// A game scenario defining starting conditions and win criteria.
///
/// # RON Example
/// ```ron
/// (
///     id: "scenario_standard",
///     name_en: "Standard Game",
///     grid_width: 10,
///     grid_height: 10,
///     start_building_id: "building_base",
///     generation_mode: random_white_black,
///     black_ratio: 0.2,
///     victory_condition_id: "victory_cover_all",
/// )
/// ```
#[derive(Debug, Clone, Deserialize)]
pub struct Scenario {
    /// Unique identifier.
    pub id: String,
    /// English display name.
    pub name_en: String,
    /// Planet grid width in tiles.
    pub grid_width: u32,
    /// Planet grid height in tiles.
    pub grid_height: u32,
    /// ID of the starting building (usually "building_base").
    pub start_building_id: String,
    /// Algorithm used to generate the planet surface.
    pub generation_mode: GenerationMode,
    /// Fraction of tiles that should be black (unbuildable), 0.0 to 1.0.
    pub black_ratio: f32,
    /// ID of the victory condition to use.
    pub victory_condition_id: String,
}
