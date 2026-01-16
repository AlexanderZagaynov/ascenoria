//! Setup systems for the Planet View screen.
//!
//! This module coordinates the initialization of the planet view,
//! spawning both the 3D scene and the 2D UI overlay.
//!
//! - [`scene`] - 3D camera, lights, tile grid, and buildings
//! - [`overlay`] - 2D UI with resource bars, controls, and victory message

mod overlay;
mod scene;

use crate::planet_data::generate_planet;
use crate::planet_view::types::PlanetViewState;
use crate::planet_view::logic::update_connectivity;
use crate::data_types::GameData;
use crate::data_types::GameRegistry;
use bevy::prelude::*;

use self::overlay::setup_ui_overlay;
use self::scene::setup_scene;

/// Main setup system for the Planet View screen.
///
/// This system runs on entering `GameState::PlanetView` and:
/// 1. Generates a new planet surface with a fixed seed
/// 2. Calculates initial resource yields from the Base building
/// 3. Initializes connectivity (determines which tiles are "powered")
/// 4. Spawns the 3D scene (camera, lights, tiles, buildings)
/// 5. Spawns the 2D UI overlay (resource bars, controls)
///
/// # Resource Initialization
/// The Base building provides initial yields:
/// - Food: +1
/// - Housing: +3
/// - Production: +1
/// - Science: +1
pub fn setup_planet_view(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut planet_state: ResMut<PlanetViewState>,
    mut ambient_light: ResMut<GlobalAmbientLight>,
    game_data: Res<GameData>,
    registry: Res<GameRegistry>,
) {
    // Initialize Game State
    let mut surface = generate_planet(12345); // Fixed seed for MVP

    // Calculate initial yields from Base
    let mut food = 0;
    let mut housing = 0;
    let mut production = 0;
    let mut science = 0;

    // Base provides: Food +1, Housing +3, Production +1, Science +1
    // Since we just generated it, we know there is one Base.
    food += 1;
    housing += 3;
    production += 1;
    science += 1;
// Calculate initial connectivity
    update_connectivity(&mut surface, &game_data, &registry);

    *planet_state = PlanetViewState {
        surface: Some(surface.clone()),
        turn: 1,
        food,
        housing,
        production,
        science,
        research_progress: 0,
        terraforming_unlocked: false,
        victory: false,
        production_queue: Default::default(),
        build_menu_open: false,
        build_menu_target_tile: None,
    };

    // Setup Scene (Grid)
    setup_scene(&mut commands, &mut meshes, &mut materials, &surface, &mut ambient_light, &game_data);

    // Setup UI
    setup_ui_overlay(&mut commands);
}
