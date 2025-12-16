use std::collections::HashMap;
use crate::data_types::game_data::GameData;
use crate::data_types::entities::{
    Engine, HullClass, PlanetSize, PlanetSurfaceType, PlanetaryItem, PlanetaryProject, Scanner,
    Shield, SpecialModule, Species, Tech, VictoryCondition, Weapon,
};
use crate::data_types::ids::{
    EngineId, HullClassId, PlanetSizeId, PlanetSurfaceTypeId, PlanetaryItemId, PlanetaryProjectId,
    ScannerId, ShieldId, SpecialModuleId, SpeciesId, TechId, VictoryConditionId, WeaponId,
};
use super::GameRegistry;

impl GameRegistry {
    fn resolve<'a, I: Eq + std::hash::Hash, T>(
        items: &'a [T],
        index: &HashMap<I, usize>,
        id: impl Into<I>,
    ) -> Option<&'a T> {
        index.get(&id.into()).and_then(|idx| items.get(*idx))
    }

    /// Resolve a species by identifier.
    pub fn species<'a>(&self, data: &'a GameData, id: impl Into<SpeciesId>) -> Option<&'a Species> {
        Self::resolve(data.species(), &self.species_by_id, id)
    }

    /// Resolve a planet size by identifier.
    pub fn planet_size<'a>(
        &self,
        data: &'a GameData,
        id: impl Into<PlanetSizeId>,
    ) -> Option<&'a PlanetSize> {
        Self::resolve(data.planet_sizes(), &self.planet_size_by_id, id)
    }

    /// Resolve a planet surface type by identifier.
    pub fn planet_surface_type<'a>(
        &self,
        data: &'a GameData,
        id: impl Into<PlanetSurfaceTypeId>,
    ) -> Option<&'a PlanetSurfaceType> {
        Self::resolve(
            data.planet_surface_types(),
            &self.planet_surface_type_by_id,
            id,
        )
    }

    /// Resolve a surface building by identifier.
    pub fn surface_item<'a>(
        &self,
        data: &'a GameData,
        id: impl Into<PlanetaryItemId>,
    ) -> Option<&'a PlanetaryItem> {
        Self::resolve(data.surface_items(), &self.surface_item_by_id, id)
    }

    /// Resolve an orbital building by identifier.
    pub fn orbital_item<'a>(
        &self,
        data: &'a GameData,
        id: impl Into<PlanetaryItemId>,
    ) -> Option<&'a PlanetaryItem> {
        Self::resolve(data.orbital_items(), &self.orbital_item_by_id, id)
    }

    /// Resolve a planetary project by identifier.
    pub fn planetary_project<'a>(
        &self,
        data: &'a GameData,
        id: impl Into<PlanetaryProjectId>,
    ) -> Option<&'a PlanetaryProject> {
        Self::resolve(data.planetary_projects(), &self.planetary_project_by_id, id)
    }

    /// Resolve a hull class by identifier.
    pub fn hull_class<'a>(
        &self,
        data: &'a GameData,
        id: impl Into<HullClassId>,
    ) -> Option<&'a HullClass> {
        Self::resolve(data.hull_classes(), &self.hull_class_by_id, id)
    }

    /// Resolve an engine by identifier.
    pub fn engine<'a>(&self, data: &'a GameData, id: impl Into<EngineId>) -> Option<&'a Engine> {
        Self::resolve(data.engines(), &self.engine_by_id, id)
    }

    /// Resolve a weapon by identifier.
    pub fn weapon<'a>(&self, data: &'a GameData, id: impl Into<WeaponId>) -> Option<&'a Weapon> {
        Self::resolve(data.weapons(), &self.weapon_by_id, id)
    }

    /// Resolve a shield by identifier.
    pub fn shield<'a>(&self, data: &'a GameData, id: impl Into<ShieldId>) -> Option<&'a Shield> {
        Self::resolve(data.shields(), &self.shield_by_id, id)
    }

    /// Resolve a scanner by identifier.
    pub fn scanner<'a>(&self, data: &'a GameData, id: impl Into<ScannerId>) -> Option<&'a Scanner> {
        Self::resolve(data.scanners(), &self.scanner_by_id, id)
    }

    /// Resolve a special module by identifier.
    pub fn special_module<'a>(
        &self,
        data: &'a GameData,
        id: impl Into<SpecialModuleId>,
    ) -> Option<&'a SpecialModule> {
        Self::resolve(data.special_modules(), &self.special_module_by_id, id)
    }

    /// Resolve a technology by identifier.
    pub fn tech<'a>(&self, data: &'a GameData, id: impl Into<TechId>) -> Option<&'a Tech> {
        Self::resolve(data.techs(), &self.tech_by_id, id)
    }

    /// Resolve a victory condition by identifier.
    pub fn victory_condition<'a>(
        &self,
        data: &'a GameData,
        id: impl Into<VictoryConditionId>,
    ) -> Option<&'a VictoryCondition> {
        Self::resolve(data.victory_conditions(), &self.victory_condition_by_id, id)
    }
}
