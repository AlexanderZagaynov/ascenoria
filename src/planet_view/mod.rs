//! Planet view screen implementation inspired by classic Ascendancy.
//!
//! This module implements the planet management screen where players can:
//! - View the planet's surface as a 3D grid of tiles
//! - Place buildings on connected tiles
//! - Manage the production queue
//! - Track resource yields (Food, Housing, Production, Science)
//!
//! # Module Structure
//!
//! - [`logic`] - Pure game logic (connectivity algorithm)
//! - [`setup`] - Scene initialization (3D meshes, UI layout)
//! - [`systems`] - Bevy ECS systems (input, rendering, game loop)
//! - [`types`] - Data structures and component definitions
//! - [`ui`] - UI components (build menu, panels, top bar)
//!
//! # Game Flow
//!
//! 1. Player enters planet view from star system
//! 2. Surface is generated/loaded with Base building
//! 3. Player clicks connected tiles to open build menu
//! 4. Buildings are added to production queue
//! 5. "End Turn" processes yields and advances construction
//! 6. Victory when all tiles are occupied (MVP condition)

mod logic;
mod setup;
mod systems;
mod types;
pub mod ui;

use crate::main_menu::GameState;

use crate::planet_view::types::{PlanetViewState, TileUpdateEvent};
use bevy::prelude::*;

/// Plugin that manages the planet view screen.
///
/// Registers all resources, events, and systems needed for planet management.
/// Systems only run when the game is in `GameState::PlanetView`.
pub struct PlanetViewPlugin;

impl Plugin for PlanetViewPlugin {
    fn build(&self, app: &mut App) {
        app
            // Initialize the planet state resource with defaults
            .init_resource::<PlanetViewState>()
            // Register the tile update event for visual refresh
            .add_message::<TileUpdateEvent>()
            // Setup: Run once when entering planet view
            .add_systems(OnEnter(GameState::PlanetView), setup::setup_planet_view)
            // Cleanup: Run once when leaving planet view
            .add_systems(OnExit(GameState::PlanetView), systems::cleanup_planet_view)
            // Update: Run every frame while in planet view
            .add_systems(
                Update,
                (
                    systems::ui_action_system,           // Handle button clicks
                    systems::tile_interaction_system,    // Handle tile clicks/hover
                    systems::update_visuals_system,      // Refresh tile meshes
                    systems::update_connectivity_system, // Recalculate power grid
                    systems::update_ui_system,           // Update stat display
                    systems::update_production_queue_ui, // Update queue display
                    ui::build_menu::update_build_menu,   // Show/hide build menu
                    ui::build_menu::build_menu_interaction, // Handle menu clicks
                    systems::configure_ui_camera,        // Layer UI over 3D
                )
                    .run_if(in_state(GameState::PlanetView)),
            );
    }
}
