//! ID-based lookup registry for game data.
//!
//! `GameRegistry` holds hash maps from typed IDs (e.g., `SpeciesId`, `TechId`)
//! to their index in the corresponding `GameData` vector. This allows O(1) lookups
//! by ID instead of scanning through arrays.

use bevy::prelude::*;
use std::collections::HashMap;

use super::entities::{
    Engine, HullClass, PlanetSize, PlanetSurfaceType, PlanetaryItem, PlanetaryProject, Scanner,
    Shield, SpecialModule, Species, Tech, VictoryCondition, Weapon,
};
use super::errors::DataLoadError;
use super::game_data::GameData;
use super::ids::{
    EngineId, HullClassId, PlanetSizeId, PlanetSurfaceTypeId, PlanetaryItemId, PlanetaryProjectId,
    ScannerId, ShieldId, SpecialModuleId, SpeciesId, TechId, VictoryConditionId, WeaponId,
};

/// Read-only lookup tables for game data identifiers.
#[derive(Debug, Resource, Default)]
pub struct GameRegistry {
    species_by_id: HashMap<SpeciesId, usize>,
    planet_size_by_id: HashMap<PlanetSizeId, usize>,
    planet_surface_type_by_id: HashMap<PlanetSurfaceTypeId, usize>,
    surface_item_by_id: HashMap<PlanetaryItemId, usize>,
    orbital_item_by_id: HashMap<PlanetaryItemId, usize>,
    planetary_project_by_id: HashMap<PlanetaryProjectId, usize>,
    hull_class_by_id: HashMap<HullClassId, usize>,
    engine_by_id: HashMap<EngineId, usize>,
    weapon_by_id: HashMap<WeaponId, usize>,
    shield_by_id: HashMap<ShieldId, usize>,
    scanner_by_id: HashMap<ScannerId, usize>,
    special_module_by_id: HashMap<SpecialModuleId, usize>,
    tech_by_id: HashMap<TechId, usize>,
    victory_condition_by_id: HashMap<VictoryConditionId, usize>,
}

impl GameRegistry {
    fn resolve<'a, I: Eq + std::hash::Hash, T>(
        items: &'a [T],
        index: &HashMap<I, usize>,
        id: impl Into<I>,
    ) -> Option<&'a T> {
        index.get(&id.into()).and_then(|idx| items.get(*idx))
    }

    /// Build lookup tables from loaded game data.
    pub fn from_game_data(data: &GameData) -> Result<Self, DataLoadError> {
        Ok(Self {
            species_by_id: build_typed_index("species", data.species(), |s| {
                SpeciesId::from(s.id.clone())
            })?,
            planet_size_by_id: build_typed_index("planet_size", data.planet_sizes(), |p| {
                PlanetSizeId::from(p.id.clone())
            })?,
            planet_surface_type_by_id: build_typed_index(
                "planet_surface_type",
                data.planet_surface_types(),
                |p| PlanetSurfaceTypeId::from(p.id.clone()),
            )?,
            surface_item_by_id: build_typed_index("surface_item", data.surface_items(), |i| {
                PlanetaryItemId::from(i.id.clone())
            })?,
            orbital_item_by_id: build_typed_index("orbital_item", data.orbital_items(), |i| {
                PlanetaryItemId::from(i.id.clone())
            })?,
            planetary_project_by_id: build_typed_index(
                "planetary_project",
                data.planetary_projects(),
                |p| PlanetaryProjectId::from(p.id.clone()),
            )?,
            hull_class_by_id: build_typed_index("hull_class", data.hull_classes(), |h| {
                HullClassId::from(h.id.clone())
            })?,
            engine_by_id: build_typed_index("engine", data.engines(), |e| {
                EngineId::from(e.id.clone())
            })?,
            weapon_by_id: build_typed_index("weapon", data.weapons(), |w| {
                WeaponId::from(w.id.clone())
            })?,
            shield_by_id: build_typed_index("shield", data.shields(), |s| {
                ShieldId::from(s.id.clone())
            })?,
            scanner_by_id: build_typed_index("scanner", data.scanners(), |s| {
                ScannerId::from(s.id.clone())
            })?,
            special_module_by_id: build_typed_index(
                "special_module",
                data.special_modules(),
                |s| SpecialModuleId::from(s.id.clone()),
            )?,
            tech_by_id: build_typed_index("tech", data.techs(), |t| TechId::from(t.id.clone()))?,
            victory_condition_by_id: build_typed_index(
                "victory_condition",
                data.victory_conditions(),
                |v| VictoryConditionId::from(v.id.clone()),
            )?,
        })
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

fn build_typed_index<T, F, I>(
    kind: &'static str,
    items: &[T],
    id_fn: F,
) -> Result<HashMap<I, usize>, DataLoadError>
where
    F: Fn(&T) -> I,
    I: Eq + std::hash::Hash + Clone + Into<String>,
{
    let mut index = HashMap::new();
    for (i, item) in items.iter().enumerate() {
        let id = id_fn(item);
        if index.insert(id.clone(), i).is_some() {
            return Err(DataLoadError::DuplicateId {
                kind,
                id: id.into(),
            });
        }
    }
    Ok(index)
}
