use crate::data::entities::{
    Engine, HullClass, PlanetSize, PlanetSurfaceType, PlanetaryItem, PlanetaryProject, Scanner,
    Shield, SpecialModule, Species, Tech, TechEdge, VictoryCondition, VictoryRules, Weapon,
};

pub(crate) struct ModDatasets {
    pub species: Vec<Species>,
    pub planet_sizes: Vec<PlanetSize>,
    pub planet_surface_types: Vec<PlanetSurfaceType>,
    pub surface_items: Vec<PlanetaryItem>,
    pub orbital_items: Vec<PlanetaryItem>,
    pub planetary_projects: Vec<PlanetaryProject>,
    pub hull_classes: Vec<HullClass>,
    pub engines: Vec<Engine>,
    pub weapons: Vec<Weapon>,
    pub shields: Vec<Shield>,
    pub scanners: Vec<Scanner>,
    pub special_modules: Vec<SpecialModule>,
    pub techs: Vec<Tech>,
    pub tech_edges: Vec<TechEdge>,
    pub victories: Vec<VictoryCondition>,
    pub victory_rules: Option<VictoryRules>,
    pub min_schema_version: u32,
}

impl Default for ModDatasets {
    fn default() -> Self {
        Self {
            species: Vec::new(),
            planet_sizes: Vec::new(),
            planet_surface_types: Vec::new(),
            surface_items: Vec::new(),
            orbital_items: Vec::new(),
            planetary_projects: Vec::new(),
            hull_classes: Vec::new(),
            engines: Vec::new(),
            weapons: Vec::new(),
            shields: Vec::new(),
            scanners: Vec::new(),
            special_modules: Vec::new(),
            techs: Vec::new(),
            tech_edges: Vec::new(),
            victories: Vec::new(),
            victory_rules: None,
            min_schema_version: crate::data::loaders::DATA_SCHEMA_VERSION,
        }
    }
}
