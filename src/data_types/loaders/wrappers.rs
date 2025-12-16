use serde::Deserialize;

use crate::data_types::entities::{
    Engine, HullClass, PlanetSize, PlanetSurfaceType, PlanetaryItem, PlanetaryProject, Scanner,
    Shield, SpecialModule, Species, Tech, TechEdge, VictoryCondition, VictoryRules, Weapon,
};

#[derive(Deserialize)]
pub(crate) struct SpeciesData {
    pub species: Vec<Species>,
}

#[derive(Deserialize)]
pub(crate) struct PlanetSizesData {
    pub planet_size: Vec<PlanetSize>,
}

#[derive(Deserialize)]
pub(crate) struct PlanetSurfaceTypesData {
    pub planet_surface_type: Vec<PlanetSurfaceType>,
}

#[derive(Deserialize)]
pub(crate) struct PlanetarySurfaceData {
    pub surface_item: Vec<PlanetaryItem>,
}

#[derive(Deserialize)]
pub(crate) struct PlanetaryOrbitalData {
    pub orbital_item: Vec<PlanetaryItem>,
}

#[derive(Deserialize)]
pub(crate) struct PlanetaryProjectsData {
    pub planetary_project: Vec<PlanetaryProject>,
}

#[derive(Deserialize)]
pub(crate) struct HullClassesData {
    pub hull_class: Vec<HullClass>,
}

#[derive(Deserialize)]
pub(crate) struct EnginesData {
    pub engine: Vec<Engine>,
}

#[derive(Deserialize)]
pub(crate) struct WeaponsData {
    pub weapon: Vec<Weapon>,
}

#[derive(Deserialize)]
pub(crate) struct ShieldsData {
    pub shield: Vec<Shield>,
}

#[derive(Deserialize)]
pub(crate) struct ScannersData {
    pub scanner: Vec<Scanner>,
}

#[derive(Deserialize)]
pub(crate) struct SpecialModulesData {
    pub special_module: Vec<SpecialModule>,
}

#[derive(Deserialize)]
pub(crate) struct TechsData {
    pub tech: Vec<Tech>,
}

#[derive(Deserialize)]
pub(crate) struct TechEdgesData {
    pub tech_edge: Vec<TechEdge>,
}

#[derive(Deserialize)]
pub(crate) struct VictoryConditionsData {
    pub victory_condition: Vec<VictoryCondition>,
}

#[derive(Deserialize)]
pub(crate) struct DataManifest {
    pub data_schema_version: Option<u32>,
}

#[derive(Default, Deserialize)]
pub(crate) struct ModManifest {
    #[serde(default)]
    pub priority: i32,
    pub data_schema_version: Option<u32>,
}

#[derive(Deserialize)]
pub(crate) struct VictoryRulesData {
    pub victory_rules: VictoryRules,
}
