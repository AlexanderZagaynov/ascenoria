use crate::data_types::compute::GameDataComputed;
use crate::data_types::entities::*;
use super::definition::GameData;

impl GameData {
    /// Create a new GameData with the given collections.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        species: Vec<Species>,
        planet_sizes: Vec<PlanetSize>,
        planet_surface_types: Vec<PlanetSurfaceType>,
        surface_items: Vec<PlanetaryItem>,
        orbital_items: Vec<PlanetaryItem>,
        planetary_projects: Vec<PlanetaryProject>,
        hull_classes: Vec<HullClass>,
        engines: Vec<Engine>,
        weapons: Vec<Weapon>,
        shields: Vec<Shield>,
        scanners: Vec<Scanner>,
        special_modules: Vec<SpecialModule>,
        techs: Vec<Tech>,
        research_graph: ResearchGraph,
        victory_conditions: Vec<VictoryCondition>,
        victory_rules: VictoryRules,
    ) -> Self {
        Self {
            species,
            planet_sizes,
            planet_surface_types,
            surface_items,
            orbital_items,
            planetary_projects,
            hull_classes,
            engines,
            weapons,
            shields,
            scanners,
            special_modules,
            techs,
            research_graph,
            victory_conditions,
            victory_rules,
        }
    }

    /// Compute derived stats for frequently used entities.
    pub fn compute(&self) -> GameDataComputed {
        GameDataComputed::from_data(
            &self.weapons,
            &self.engines,
            &self.surface_items,
            &self.orbital_items,
        )
    }
}
