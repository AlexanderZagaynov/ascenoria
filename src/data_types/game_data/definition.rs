use bevy::prelude::Resource;
use crate::data_types::entities::*;

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
