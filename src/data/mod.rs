//! Game data loading, storage, and lookup.
//!
//! This module contains all the data structures and loading logic for the game's
//! TOML-based content system. It handles species, planets, ships, technologies,
//! and victory conditions.
//!
//! ## Quick Start
//!
//! ```ignore
//! use ascenoria::data::{load_game_data, GameData, GameRegistry, Language};
//!
//! let (data, registry) = load_game_data("assets/data").unwrap();
//! let orfa = registry.species(&data, "orfa").unwrap();
//! println!("{}", orfa.name(Language::En));
//! ```

mod compute;
mod entities;
mod errors;
mod game_data;
mod ids;
mod loaders;
mod localization;
mod registry;
mod validation;

#[cfg(test)]
#[path = "tests/mod.rs"]
mod tests;

// Re-export public API

// Core types
pub use compute::{EngineComputed, GameDataComputed, PlanetaryItemComputed, WeaponComputed};
pub use entities::{
    Engine, HullClass, PlanetSize, PlanetSurfaceType, PlanetaryItem, PlanetaryProject,
    ResearchGraph, Scanner, Shield, SpecialModule, Species, Tech, TechEdge, TileDistribution,
    VictoryCondition, VictoryRules, Weapon,
};
pub use errors::DataLoadError;
pub use game_data::GameData;
pub use ids::{
    EngineId, HullClassId, PlanetSizeId, PlanetSurfaceTypeId, PlanetaryItemId, PlanetaryProjectId,
    ScannerId, ShieldId, SpecialModuleId, SpeciesId, TechId, VictoryConditionId, WeaponId,
};
pub use loaders::{DATA_SCHEMA_VERSION, load_game_data};
pub use localization::{
    HasDescription, HasId, Language, LocalizedEntity, LocalizedText, NamedEntity,
};
pub use registry::GameRegistry;
pub use validation::NO_TECH_REQUIREMENT;
