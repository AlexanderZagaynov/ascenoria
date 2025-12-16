use serde::Deserialize;

use crate::data_types::entities::{
    Scenario, SurfaceBuilding, SurfaceCellType, Technology, VictoryCondition,
};

#[derive(Deserialize)]
pub(crate) struct SurfaceCellTypesData {
    pub surface_cell_type: Vec<SurfaceCellType>,
}

#[derive(Deserialize)]
pub(crate) struct SurfaceBuildingsData {
    pub surface_building: Vec<SurfaceBuilding>,
}

#[derive(Deserialize)]
pub(crate) struct TechnologiesData {
    pub technology: Vec<Technology>,
}

#[derive(Deserialize)]
pub(crate) struct VictoryConditionsData {
    pub victory_condition: Vec<VictoryCondition>,
}

#[derive(Deserialize)]
pub(crate) struct ScenariosData {
    pub scenario: Vec<Scenario>,
}
