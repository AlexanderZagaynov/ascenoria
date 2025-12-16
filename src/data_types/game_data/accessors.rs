use super::definition::GameData;
use crate::data_types::entities::*;

impl GameData {
    pub fn surface_cell_types(&self) -> &[SurfaceCellType] {
        &self.surface_cell_types
    }

    pub fn surface_buildings(&self) -> &[SurfaceBuilding] {
        &self.surface_buildings
    }

    pub fn technologies(&self) -> &[Technology] {
        &self.technologies
    }

    pub fn victory_conditions(&self) -> &[VictoryCondition] {
        &self.victory_conditions
    }

    pub fn scenarios(&self) -> &[Scenario] {
        &self.scenarios
    }
}
