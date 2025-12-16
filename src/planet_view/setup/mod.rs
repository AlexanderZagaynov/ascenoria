mod overlay;
mod scene;

use crate::data_types::GameData;
use crate::data_types::GameRegistry;
use crate::planet_data::generate_planet;
use crate::planet_view::logic::update_connectivity;
use crate::planet_view::types::PlanetViewState;
use bevy::prelude::*;

use self::overlay::setup_ui_overlay;
use self::scene::setup_scene;

/// Set up the planet view screen.
pub fn setup_planet_view(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut planet_state: ResMut<PlanetViewState>,
    mut ambient_light: ResMut<AmbientLight>,
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
    setup_scene(
        &mut commands,
        &mut meshes,
        &mut materials,
        &surface,
        &mut ambient_light,
        &game_data,
    );

    // Setup UI
    setup_ui_overlay(&mut commands);
}
