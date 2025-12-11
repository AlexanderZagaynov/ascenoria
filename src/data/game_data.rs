//! GameData: the main data container for all loaded game content.
//!
//! This struct holds all game content loaded from TOML files: species, planets,
//! ships, technologies, etc. It's stored as a Bevy `Resource` so ECS systems
//! can query it.

use bevy::prelude::Resource;

use super::compute::GameDataComputed;
use super::entities::*;

/// Aggregated game data loaded from TOML assets.
#[derive(Debug, Resource)]
pub struct GameData {
    /// Playable and AI species.
    pub(crate) species: Vec<Species>,
    /// Planet size definitions.
    pub(crate) planet_sizes: Vec<PlanetSize>,
    /// Surface type distributions.
    pub(crate) planet_surface_types: Vec<PlanetSurfaceType>,
    /// Surface installation templates.
    pub(crate) surface_items: Vec<PlanetaryItem>,
    /// Orbital installation templates.
    pub(crate) orbital_items: Vec<PlanetaryItem>,
    /// Planetary projects definitions.
    pub(crate) planetary_projects: Vec<PlanetaryProject>,
    /// Hull classes available to the ship designer.
    pub(crate) hull_classes: Vec<HullClass>,
    /// Engine modules.
    pub(crate) engines: Vec<Engine>,
    /// Weapon modules.
    pub(crate) weapons: Vec<Weapon>,
    /// Shield modules.
    pub(crate) shields: Vec<Shield>,
    /// Scanner modules.
    pub(crate) scanners: Vec<Scanner>,
    /// Special modules.
    pub(crate) special_modules: Vec<SpecialModule>,
    /// Technologies.
    pub(crate) techs: Vec<Tech>,
    /// Research graph edges.
    pub(crate) research_graph: ResearchGraph,
    /// Victory condition archetypes.
    pub(crate) victory_conditions: Vec<VictoryCondition>,
    /// Tunable parameters for victory checks.
    pub(crate) victory_rules: VictoryRules,
}

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

    /// Get all species definitions.
    pub fn species(&self) -> &[Species] {
        &self.species
    }

    /// Get all planet size archetypes.
    pub fn planet_sizes(&self) -> &[PlanetSize] {
        &self.planet_sizes
    }

    /// Get surface composition distributions.
    pub fn planet_surface_types(&self) -> &[PlanetSurfaceType] {
        &self.planet_surface_types
    }

    /// Get all surface building templates.
    pub fn surface_items(&self) -> &[PlanetaryItem] {
        &self.surface_items
    }

    /// Get all orbital building templates.
    pub fn orbital_items(&self) -> &[PlanetaryItem] {
        &self.orbital_items
    }

    /// Get planetary project definitions.
    pub fn planetary_projects(&self) -> &[PlanetaryProject] {
        &self.planetary_projects
    }

    /// Get ship hull templates.
    pub fn hull_classes(&self) -> &[HullClass] {
        &self.hull_classes
    }

    /// Get engine module definitions.
    pub fn engines(&self) -> &[Engine] {
        &self.engines
    }

    /// Get weapon module definitions.
    pub fn weapons(&self) -> &[Weapon] {
        &self.weapons
    }

    /// Get shield module definitions.
    pub fn shields(&self) -> &[Shield] {
        &self.shields
    }

    /// Get scanner module definitions.
    pub fn scanners(&self) -> &[Scanner] {
        &self.scanners
    }

    /// Get special module definitions.
    pub fn special_modules(&self) -> &[SpecialModule] {
        &self.special_modules
    }

    /// Get all technologies.
    pub fn techs(&self) -> &[Tech] {
        &self.techs
    }

    /// Get prerequisites for a technology id.
    pub fn tech_prereqs(&self, id: &str) -> &[String] {
        self.research_graph
            .prereqs
            .get(id)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    /// Get unlocks for a technology id.
    pub fn tech_unlocks(&self, id: &str) -> &[String] {
        self.research_graph
            .unlocks
            .get(id)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    /// Get victory condition archetypes.
    pub fn victory_conditions(&self) -> &[VictoryCondition] {
        &self.victory_conditions
    }

    /// Get victory rules configuration.
    pub fn victory_rules(&self) -> &VictoryRules {
        &self.victory_rules
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
