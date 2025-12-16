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

#[derive(Debug, Resource, Default)]
pub struct GameRegistry {
    pub(crate) surface_cell_type_by_id: HashMap<SurfaceCellTypeId, usize>,
    pub(crate) surface_building_by_id: HashMap<SurfaceBuildingId, usize>,
    pub(crate) technology_by_id: HashMap<TechnologyId, usize>,
    pub(crate) victory_condition_by_id: HashMap<VictoryConditionId, usize>,
    pub(crate) scenario_by_id: HashMap<ScenarioId, usize>,
}

impl GameRegistry {
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
