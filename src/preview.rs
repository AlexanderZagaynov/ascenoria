//! Preview and debug resources for game state visualization.
//!
//! Contains resources for planet/galaxy preview, construction state,
//! and tech unlocking during development and testing.

use bevy::prelude::*;
use std::collections::HashSet;

use crate::data::{GameData, HasId, Language, NO_TECH_REQUIREMENT};
use crate::galaxy::Galaxy;
use crate::industry::PlanetIndustry;
use crate::planet::{
    GeneratedPlanet, OrbitalPreview, PlanetOrbitals, PlanetSurface, SurfacePreview,
};
use crate::research::ResearchState;
use crate::victory::{DominationConfig, VictoryState};

/// Current language selection for UI rendering.
#[derive(Resource, Default)]
pub struct LocalizationSettings {
    pub language: Language,
}

impl LocalizationSettings {
    pub fn toggle(&mut self) {
        self.language = match self.language {
            Language::En => Language::Ru,
            Language::Ru => Language::En,
        };
    }
}

/// Holds a generated planet for debug visualization.
#[derive(Resource, Default)]
pub struct PlanetPreview {
    pub planet: Option<GeneratedPlanet>,
}

/// Holds a generated galaxy snapshot.
#[derive(Resource)]
pub struct GalaxyPreview {
    pub galaxy: Galaxy,
}

impl Default for GalaxyPreview {
    fn default() -> Self {
        Self {
            galaxy: Galaxy {
                systems: Vec::new(),
            },
        }
    }
}

/// Placeholder industry/build queue preview.
#[derive(Resource, Default)]
pub struct IndustryPreview {
    pub industry: PlanetIndustry,
}

/// Tracks domination victory progress.
#[derive(Resource)]
pub struct VictoryPreview {
    pub state: VictoryState,
}

impl Default for VictoryPreview {
    fn default() -> Self {
        Self {
            state: VictoryState::new(0, DominationConfig::default()),
        }
    }
}

/// Research progress and selection.
#[derive(Resource)]
pub struct ResearchPreview {
    pub state: ResearchState,
}

impl Default for ResearchPreview {
    fn default() -> Self {
        Self {
            state: ResearchState::new(1),
        }
    }
}

/// Tracks unlocked technologies by index for filtering build options.
#[derive(Resource, Default)]
pub struct TechState {
    pub completed: HashSet<String>,
}

impl TechState {
    pub fn is_unlocked(&self, tech_index: i32, techs: &[crate::data::Tech]) -> bool {
        if tech_index == NO_TECH_REQUIREMENT {
            return true;
        }
        techs
            .get(tech_index as usize)
            .map(|t| self.completed.contains(&t.id))
            .unwrap_or(false)
    }
}

/// Surface construction preview and selection state.
#[derive(Resource, Default)]
pub struct SurfaceConstruction {
    pub surface: Option<PlanetSurface>,
    pub selected_building: usize,
    pub preview: Option<SurfacePreview>,
}

impl SurfaceConstruction {
    pub fn with_planet(planet: Option<GeneratedPlanet>) -> Self {
        Self {
            surface: planet.as_ref().map(PlanetSurface::from),
            selected_building: 0,
            preview: None,
        }
    }
}

/// Orbital construction preview and selection state.
#[derive(Resource, Default)]
pub struct OrbitalConstruction {
    pub orbitals: Option<PlanetOrbitals>,
    pub selected_building: usize,
    pub preview: Option<OrbitalPreview>,
}

impl OrbitalConstruction {
    pub fn with_planet(planet: Option<GeneratedPlanet>) -> Self {
        Self {
            orbitals: planet.as_ref().map(PlanetOrbitals::from),
            selected_building: 0,
            preview: None,
        }
    }
}

/// Refresh surface construction preview based on current selection.
pub fn refresh_surface_preview(
    surface_construction: &mut SurfaceConstruction,
    game_data: &GameData,
    tech_state: &TechState,
) {
    let available_buildings: Vec<_> = game_data
        .surface_items()
        .iter()
        .filter(|item| tech_state.is_unlocked(item.tech_index, game_data.techs()))
        .collect();

    if available_buildings.is_empty() {
        surface_construction.preview = None;
        surface_construction.selected_building = 0;
        return;
    }

    if surface_construction.selected_building >= available_buildings.len() {
        surface_construction.selected_building = available_buildings.len() - 1;
    }

    if let Some(surface) = &surface_construction.surface {
        if let Some(item) = available_buildings.get(surface_construction.selected_building) {
            surface_construction.preview = surface.preview_placement(item).ok();
        }
    } else {
        surface_construction.preview = None;
    }
}

/// Refresh orbital construction preview based on current selection.
pub fn refresh_orbital_preview(
    orbital_construction: &mut OrbitalConstruction,
    game_data: &GameData,
    tech_state: &TechState,
) {
    let available_buildings: Vec<_> = game_data
        .orbital_items()
        .iter()
        .filter(|item| tech_state.is_unlocked(item.tech_index, game_data.techs()))
        .collect();

    if available_buildings.is_empty() {
        orbital_construction.preview = None;
        orbital_construction.selected_building = 0;
        return;
    }

    if orbital_construction.selected_building >= available_buildings.len() {
        orbital_construction.selected_building = available_buildings.len() - 1;
    }

    if let Some(orbitals) = &orbital_construction.orbitals {
        if let Some(item) = available_buildings.get(orbital_construction.selected_building) {
            orbital_construction.preview = orbitals.preview_placement(item).ok();
        }
    } else {
        orbital_construction.preview = None;
    }
}
