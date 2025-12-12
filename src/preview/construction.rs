use bevy::prelude::*;
use crate::data::GameData;
use crate::planet::{GeneratedPlanet, OrbitalPreview, PlanetOrbitals, PlanetSurface, SurfacePreview};
use super::tech::TechState;

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
