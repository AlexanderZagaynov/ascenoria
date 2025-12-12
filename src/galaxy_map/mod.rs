//! Galaxy map screen implementation inspired by classic Ascendancy.
//!
//! Displays a 3D rotatable star map with clickable star systems and a right-side control panel.
//! Stars are distributed in a spherical volume and the view can be rotated by dragging.

mod generation;
mod modal;
mod setup;
mod systems;
mod types;
mod ui;

use bevy::prelude::*;

use crate::main_menu::GameState;

// Re-export public types
pub use modal::InfoModalState;
pub use types::GalaxyMapState;

/// Colors for the galaxy map UI.
pub mod colors {
    use bevy::prelude::*;

    /// Black space background.
    pub const SPACE_BLACK: Color = Color::srgb(0.0, 0.0, 0.0);
    /// Gray textured panel background.
    pub const PANEL_BG: Color = Color::srgb(0.35, 0.38, 0.42);
    /// Darker panel sections.
    pub const PANEL_DARK: Color = Color::srgb(0.25, 0.28, 0.32);
    /// Panel border color.
    pub const PANEL_BORDER: Color = Color::srgb(0.2, 0.22, 0.25);
    /// Bright green for player-owned systems.
    pub const STAR_PLAYER: Color = Color::srgb(0.2, 0.9, 0.3);
    /// Orange for enemy systems.
    pub const STAR_ENEMY: Color = Color::srgb(0.9, 0.4, 0.1);
    /// White/yellow for neutral stars.
    pub const STAR_NEUTRAL: Color = Color::srgb(0.95, 0.9, 0.7);
    /// Red giant stars.
    pub const STAR_RED: Color = Color::srgb(0.9, 0.3, 0.2);
    /// Blue stars.
    pub const STAR_BLUE: Color = Color::srgb(0.4, 0.6, 0.95);
    /// Cyan for selection highlight.
    pub const SELECTION_CYAN: Color = Color::srgb(0.2, 0.8, 0.8);
    /// Green ring indicators.
    pub const RING_GREEN: Color = Color::srgb(0.3, 0.7, 0.4);
    /// Text on panels.
    pub const PANEL_TEXT: Color = Color::srgb(0.85, 0.85, 0.85);
    /// Dim text.
    pub const PANEL_TEXT_DIM: Color = Color::srgb(0.6, 0.6, 0.6);
}

/// Plugin that manages the galaxy map screen.
pub struct GalaxyMapPlugin;

impl Plugin for GalaxyMapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GalaxyMapState>()
            .init_resource::<InfoModalState>()
            .add_systems(OnEnter(GameState::GalaxyView), setup::setup_galaxy_map)
            .add_systems(OnExit(GameState::GalaxyView), systems::cleanup_galaxy_map)
            .add_systems(
                Update,
                (
                    systems::galaxy_rotation_system,
                    systems::star_click_system,
                    systems::panel_button_system,
                    systems::turn_control_system,
                    modal::info_modal_system,
                    modal::info_modal_button_system,
                )
                    .run_if(in_state(GameState::GalaxyView)),
            );
    }
}
