//! GameRegistry: O(1) lookup index for game entities by ID.
//!
//! While `GameData` stores entities in vectors, `GameRegistry` provides
//! hash map indices for fast lookups by ID string.
//!
//! # Example
//! ```ignore
//! let registry = GameRegistry::from_game_data(&game_data)?;
//! let building = registry.get_surface_building(&game_data, "building_farm_1");
//! ```

use bevy::prelude::*;
use std::collections::HashMap;

use crate::data_types::errors::DataLoadError;
use crate::data_types::game_data::GameData;
use crate::data_types::ids::{
    ScenarioId, SurfaceBuildingId, SurfaceCellTypeId, TechnologyId, VictoryConditionId,
};

mod accessors;
mod builder;

use builder::build_typed_index;

/// Registry providing O(1) lookups of game entities by ID.
///
/// Internally stores `HashMap<TypedId, usize>` indices into the
/// corresponding vectors in `GameData`.
///
/// # Construction
/// Use `GameRegistry::from_game_data()` after loading `GameData`
/// to build the lookup indices. This validates uniqueness of all IDs.
#[derive(Debug, Resource, Default)]
pub struct GameRegistry {
    /// Index of surface cell types (terrain) by ID.
    pub(crate) surface_cell_type_by_id: HashMap<SurfaceCellTypeId, usize>,
    /// Index of surface buildings by ID.
    pub(crate) surface_building_by_id: HashMap<SurfaceBuildingId, usize>,
    /// Index of technologies by ID.
    pub(crate) technology_by_id: HashMap<TechnologyId, usize>,
    /// Index of victory conditions by ID.
    pub(crate) victory_condition_by_id: HashMap<VictoryConditionId, usize>,
    /// Index of scenarios by ID.
    pub(crate) scenario_by_id: HashMap<ScenarioId, usize>,
}

impl GameRegistry {
    /// Build the registry from loaded game data.
    ///
    /// # Errors
    /// Returns `DataLoadError::DuplicateId` if any entity type
    /// has duplicate IDs.
    pub fn from_game_data(data: &GameData) -> Result<Self, DataLoadError> {
        Ok(Self {
            surface_cell_type_by_id: build_typed_index(
                "surface_cell_type",
                data.surface_cell_types(),
                |s| SurfaceCellTypeId::from(s.id.clone()),
            )?,
            surface_building_by_id: build_typed_index(
                "surface_building",
                data.surface_buildings(),
                |s| SurfaceBuildingId::from(s.id.clone()),
            )?,
            technology_by_id: build_typed_index("technology", data.technologies(), |t| {
                TechnologyId::from(t.id.clone())
            })?,
            victory_condition_by_id: build_typed_index(
                "victory_condition",
                data.victory_conditions(),
                |v| VictoryConditionId::from(v.id.clone()),
            )?,
            scenario_by_id: build_typed_index("scenario", data.scenarios(), |s| {
                ScenarioId::from(s.id.clone())
            })?,
        })
    }
}
