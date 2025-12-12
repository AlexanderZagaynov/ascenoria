mod overlay;
mod scene;

use bevy::prelude::*;
use crate::{GalaxyPreview, planet::PlanetSurface, star::StarState};
use crate::planet_view::types::PlanetViewState;

use self::overlay::setup_ui_overlay;
use self::scene::setup_3d_scene;

/// Set up the planet view screen.
pub fn setup_planet_view(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    star_state: Res<StarState>,
    galaxy_preview: Res<GalaxyPreview>,
    mut _planet_state: ResMut<PlanetViewState>,
) {
// Get current planet info from star system state
    let star_index = star_state.star_index;
    let planet_index = star_state.selected_planet.unwrap_or(0);

    // Get planet data if available
    let (planet_name, surface_type, planet_size, surface_slots, orbital_slots, tiles, row_width) =
        galaxy_preview
            .galaxy
            .systems
            .get(star_index)
            .and_then(|s| s.planets.get(planet_index))
            .map(|p| {
                let _surface = PlanetSurface::from(p);
                (
                    format!("Planet {}", planet_index + 1),
                    p.surface_type_id.clone(),
                    p.size_id.clone(),
                    p.surface_slots,
                    p.orbital_slots,
                    p.tiles.clone(),
                    p.row_width,
                )
            })
            .unwrap_or_else(|| {
                (
                    "Unknown Planet".to_string(),
                    "unknown".to_string(),
                    "unknown".to_string(),
                    0,
                    0,
                    Vec::new(),
                    1,
                )
            });

    let num_planets = galaxy_preview
        .galaxy
        .systems
        .get(star_index)
        .map(|s| s.planets.len())
        .unwrap_or(0);

    // =========================================================================
    // 3D Scene Setup
    // =========================================================================

    setup_3d_scene(
        &mut commands,
        &mut meshes,
        &mut materials,
        &tiles,
        row_width,
        &surface_type,
    );

    // =========================================================================
    // 2D UI Overlay
    // =========================================================================

    setup_ui_overlay(
        &mut commands,
        num_planets,
        planet_index,
        star_index,
        &galaxy_preview,
        &planet_name,
        &surface_type,
        &planet_size,
        surface_slots as usize,
        orbital_slots as usize,
    );
}
