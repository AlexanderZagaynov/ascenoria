use crate::data::entities::*;
use super::definition::GameData;

impl GameData {
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
}
