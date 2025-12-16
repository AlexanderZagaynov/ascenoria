use super::GameRegistry;
use crate::data_types::entities::*;
use crate::data_types::game_data::GameData;
use crate::data_types::ids::*;

impl GameRegistry {
    fn resolve<'a, I: Eq + std::hash::Hash, T>(
        &self,
        map: &std::collections::HashMap<I, usize>,
        data_slice: &'a [T],
        id: I,
    ) -> Option<&'a T> {
        map.get(&id).map(|&idx| &data_slice[idx])
    }

    pub fn surface_cell_type<'a>(
        &self,
        data: &'a GameData,
        id: impl Into<SurfaceCellTypeId>,
    ) -> Option<&'a SurfaceCellType> {
        self.resolve(
            &self.surface_cell_type_by_id,
            data.surface_cell_types(),
            id.into(),
        )
    }

    pub fn surface_building<'a>(
        &self,
        data: &'a GameData,
        id: impl Into<SurfaceBuildingId>,
    ) -> Option<&'a SurfaceBuilding> {
        self.resolve(
            &self.surface_building_by_id,
            data.surface_buildings(),
            id.into(),
        )
    }

    pub fn technology<'a>(
        &self,
        data: &'a GameData,
        id: impl Into<TechnologyId>,
    ) -> Option<&'a Technology> {
        self.resolve(&self.technology_by_id, data.technologies(), id.into())
    }

    pub fn victory_condition<'a>(
        &self,
        data: &'a GameData,
        id: impl Into<VictoryConditionId>,
    ) -> Option<&'a VictoryCondition> {
        self.resolve(
            &self.victory_condition_by_id,
            data.victory_conditions(),
            id.into(),
        )
    }

    pub fn scenario<'a>(
        &self,
        data: &'a GameData,
        id: impl Into<ScenarioId>,
    ) -> Option<&'a Scenario> {
        self.resolve(&self.scenario_by_id, data.scenarios(), id.into())
    }
}
