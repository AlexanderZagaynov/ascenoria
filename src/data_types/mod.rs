//! Data types for loading and validating RON game data files.
//!
//! This module provides the data-driven foundation for Ascenoria,
//! loading game content from RON files in `assets/data/`.
//!
//! # Module Structure
//!
//! - [`entities`] - Core data structures (`SurfaceBuilding`, `Technology`, etc.)
//! - [`errors`] - Error types for data loading and validation
//! - [`game_data`] - `GameData` resource containing all loaded data
//! - [`ids`] - Strongly-typed ID types for type-safe lookups
//! - [`loaders`] - RON file parsing and validation
//! - [`registry`] - `GameRegistry` for O(1) ID-based lookups
//!
//! # Usage
//!
//! ```ignore
//! use ascenoria::data_types::{load_game_data, GameData, GameRegistry};
//!
//! let (game_data, registry) = load_game_data("assets/data")?;
//! let building = registry.get_surface_building("building_farm_1");
//! ```

mod entities;
mod errors;
mod game_data;
mod ids;
mod loaders;
mod registry;

#[cfg(test)]
#[path = "tests/mod.rs"]
mod tests;

pub use entities::{
    BuildableOn, GenerationMode, Scenario, SpecialBehavior, SurfaceBuilding, SurfaceCellType,
    Technology, VictoryCondition, VictoryType,
};
pub use errors::DataLoadError;
pub use game_data::GameData;
pub use ids::{ScenarioId, SurfaceBuildingId, SurfaceCellTypeId, TechnologyId, VictoryConditionId};
pub use loaders::load_game_data;
pub use registry::GameRegistry;
