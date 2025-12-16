//! Victory condition data structures.
//!
//! Defines the win and lose conditions for game scenarios.

use serde::Deserialize;

/// Types of victory conditions.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum VictoryType {
    /// Win by covering all white tiles with buildings.
    CoverAllTiles,
}

/// A victory condition definition.
///
/// # RON Example
/// ```ron
/// (
///     id: "victory_cover_all",
///     name_en: "Planetary Domination",
///     type: cover_all_tiles,
/// )
/// ```
#[derive(Debug, Clone, Deserialize)]
pub struct VictoryCondition {
    /// Unique identifier.
    pub id: String,
    /// English display name.
    pub name_en: String,
    /// The type of condition to check.
    #[serde(rename = "type")]
    pub condition_type: VictoryType,
}
