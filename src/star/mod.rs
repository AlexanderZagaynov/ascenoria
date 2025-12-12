//! Star system view screen inspired by classic Ascendancy.
//!
//! Displays a 3D isometric view of a planetary system with:
//! - Planets on vertical poles/stalks
//! - Blue grid plane for depth reference
//! - Right-side control panel with navigation buttons
//! - Planet info display when selected

mod generation;
mod setup;
mod systems;
mod types;
mod ui;

use bevy::prelude::*;

use crate::main_menu::GameState;

// Re-export public types
pub use types::StarState;

/// Plugin that manages the star system view screen.
pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StarState>()
            .add_systems(OnEnter(GameState::StarView), setup::setup_star)
            .add_systems(OnExit(GameState::StarView), systems::cleanup_star)
            .add_systems(
                Update,
                (
                    systems::planet_hover_system,
                    systems::panel_button_system,
                    systems::camera_control_system,
                    systems::keyboard_navigation_system,
                )
                    .run_if(in_state(GameState::StarView)),
            );
    }
}
