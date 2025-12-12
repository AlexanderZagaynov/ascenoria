//! ID-based lookup registry for game data.
//!
//! `GameRegistry` holds hash maps from typed IDs (e.g., `SpeciesId`, `TechId`)
//! to their index in the corresponding `GameData` vector. This allows O(1) lookups
//! by ID instead of scanning through arrays.

use bevy::prelude::*;
use std::collections::HashMap;

use crate::data::errors::DataLoadError;
use crate::data::game_data::GameData;
use crate::data::ids::{
    EngineId, HullClassId, PlanetSizeId, PlanetSurfaceTypeId, PlanetaryItemId, PlanetaryProjectId,
    ScannerId, ShieldId, SpecialModuleId, SpeciesId, TechId, VictoryConditionId, WeaponId,
};

mod accessors;
mod builder;

use builder::build_typed_index;

/// Read-only lookup tables for game data identifiers.
#[derive(Debug, Resource, Default)]
pub struct GameRegistry {
    pub(crate) species_by_id: HashMap<SpeciesId, usize>,
    pub(crate) planet_size_by_id: HashMap<PlanetSizeId, usize>,
    pub(crate) planet_surface_type_by_id: HashMap<PlanetSurfaceTypeId, usize>,
    pub(crate) surface_item_by_id: HashMap<PlanetaryItemId, usize>,
    pub(crate) orbital_item_by_id: HashMap<PlanetaryItemId, usize>,
    pub(crate) planetary_project_by_id: HashMap<PlanetaryProjectId, usize>,
    pub(crate) hull_class_by_id: HashMap<HullClassId, usize>,
    pub(crate) engine_by_id: HashMap<EngineId, usize>,
    pub(crate) weapon_by_id: HashMap<WeaponId, usize>,
    pub(crate) shield_by_id: HashMap<ShieldId, usize>,
    pub(crate) scanner_by_id: HashMap<ScannerId, usize>,
    pub(crate) special_module_by_id: HashMap<SpecialModuleId, usize>,
    pub(crate) tech_by_id: HashMap<TechId, usize>,
    pub(crate) victory_condition_by_id: HashMap<VictoryConditionId, usize>,
}

impl GameRegistry {
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
}
