//! Planet view screen implementation inspired by classic Ascendancy.
//!
//! Displays a 3D rotating planet globe with surface tiles rendered on the sphere,
//! buildings, population, and orbital structures. Accessed by clicking on a planet
//! in the star system view.

mod modal;
mod rendering;
mod setup;
mod systems;
mod types;
mod ui;

use bevy::prelude::*;

use crate::main_menu::GameState;

// Re-export public types
pub use modal::PlanetInfoModalState;
pub use types::PlanetViewState;

/// Plugin that manages the planet view screen.
pub struct PlanetViewPlugin;

impl Plugin for PlanetViewPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlanetViewState>()
            .init_resource::<PlanetInfoModalState>()
            .add_systems(OnEnter(GameState::PlanetView), setup::setup_planet_view)
            .add_systems(OnExit(GameState::PlanetView), systems::cleanup_planet_view)
            .add_systems(
                Update,
                (
                    systems::keyboard_navigation_system,
                    systems::panel_button_system,
                    modal::planet_info_modal_system,
                    modal::planet_info_modal_button_system,
                )
                    .run_if(in_state(GameState::PlanetView)),
            );
    }
}
