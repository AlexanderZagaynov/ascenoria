use crate::data_types::entities::*;
use bevy::prelude::Resource;

/// Aggregated game data loaded from TOML assets.
#[derive(Debug, Resource)]
pub struct GameData {
    pub(crate) surface_cell_types: Vec<SurfaceCellType>,
    pub(crate) surface_buildings: Vec<SurfaceBuilding>,
    pub(crate) technologies: Vec<Technology>,
    pub(crate) victory_conditions: Vec<VictoryCondition>,
    pub(crate) scenarios: Vec<Scenario>,
}
