use crate::data::entities::*;
use super::definition::GameData;

impl GameData {
    /// Mutable access to research graph (for loader).
    pub(crate) fn set_research_graph(&mut self, graph: ResearchGraph) {
        self.research_graph = graph;
    }

    /// Mutable access to victory rules (for loader).
    pub(crate) fn set_victory_rules(&mut self, rules: VictoryRules) {
        self.victory_rules = rules;
    }

    // Mutable accessors for mod loading

    pub(crate) fn species_mut(&mut self) -> &mut Vec<Species> {
        &mut self.species
    }

    pub(crate) fn planet_sizes_mut(&mut self) -> &mut Vec<PlanetSize> {
        &mut self.planet_sizes
    }

    pub(crate) fn planet_surface_types_mut(&mut self) -> &mut Vec<PlanetSurfaceType> {
        &mut self.planet_surface_types
    }

    pub(crate) fn surface_items_mut(&mut self) -> &mut Vec<PlanetaryItem> {
        &mut self.surface_items
    }

    pub(crate) fn orbital_items_mut(&mut self) -> &mut Vec<PlanetaryItem> {
        &mut self.orbital_items
    }

    pub(crate) fn planetary_projects_mut(&mut self) -> &mut Vec<PlanetaryProject> {
        &mut self.planetary_projects
    }

    pub(crate) fn hull_classes_mut(&mut self) -> &mut Vec<HullClass> {
        &mut self.hull_classes
    }

    pub(crate) fn engines_mut(&mut self) -> &mut Vec<Engine> {
        &mut self.engines
    }

    pub(crate) fn weapons_mut(&mut self) -> &mut Vec<Weapon> {
        &mut self.weapons
    }

    pub(crate) fn shields_mut(&mut self) -> &mut Vec<Shield> {
        &mut self.shields
    }

    pub(crate) fn scanners_mut(&mut self) -> &mut Vec<Scanner> {
        &mut self.scanners
    }

    pub(crate) fn special_modules_mut(&mut self) -> &mut Vec<SpecialModule> {
        &mut self.special_modules
    }

    pub(crate) fn techs_mut(&mut self) -> &mut Vec<Tech> {
        &mut self.techs
    }

    pub(crate) fn victory_conditions_mut(&mut self) -> &mut Vec<VictoryCondition> {
        &mut self.victory_conditions
    }
}
