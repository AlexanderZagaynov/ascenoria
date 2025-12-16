//! Planet view screen implementation inspired by classic Ascendancy.
//!
//! Displays a 3D rotating planet globe with surface tiles rendered on the sphere,
//! buildings, population, and orbital structures. Accessed by clicking on a planet
//! in the star system view.

mod logic;
mod setup;
mod systems;
mod types;
pub mod ui;

use crate::main_menu::GameState;

use crate::planet_view::types::{PlanetViewState, TileUpdateEvent};
use bevy::prelude::*;

/// Plugin that manages the planet view screen.
pub struct PlanetViewPlugin;

impl Plugin for PlanetViewPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlanetViewState>()
            .add_message::<TileUpdateEvent>()
            .add_systems(OnEnter(GameState::PlanetView), setup::setup_planet_view)
            .add_systems(OnExit(GameState::PlanetView), systems::cleanup_planet_view)
            .add_systems(
                Update,
                (
                    systems::ui_action_system,
                    systems::tile_interaction_system,
                    systems::update_visuals_system,
                    systems::update_connectivity_system,
                    systems::update_ui_system,
                    systems::update_production_queue_ui,
                    ui::build_menu::update_build_menu,
                    ui::build_menu::build_menu_interaction,
                    systems::configure_ui_camera,
                )
                    .run_if(in_state(GameState::PlanetView)),
            );
    }
}
