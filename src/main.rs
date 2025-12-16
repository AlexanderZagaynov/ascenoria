//! Ascenoria - A data-driven 4X strategy game inspired by Ascendancy.
//!
//! This is the main entry point that registers all game plugins and starts the Bevy app.
//!
//! # Game Structure
//!
//! The game is organized as a collection of Bevy plugins:
//!
//! - [`GameDataPlugin`] - Loads RON data files and creates the `GameData` and `GameRegistry` resources
//! - [`MainMenuPlugin`] - Main menu screen and `GameState` state machine
//! - [`PlanetViewPlugin`] - Planet surface management screen
//!
//! # State Machine
//!
//! Game flow is controlled by the `GameState` enum:
//! - `MainMenu` → `PlanetView` (when player starts game)
//! - `PlanetView` → `MainMenu` (when player presses ESC)

use bevy::{asset::AssetPlugin, prelude::*};

use ascenoria::game_data::GameDataPlugin;
use ascenoria::main_menu::{GameState, MainMenuPlugin};
use ascenoria::planet_view::PlanetViewPlugin;

/// Application entry point.
///
/// Configures and runs the Bevy app with:
/// - Default Bevy plugins (windowing, rendering, input, etc.)
/// - Asset hot-reloading enabled for development
/// - Game-specific plugins for data, menus, and gameplay
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(AssetPlugin {
                // Enable hot-reloading of assets during development
                watch_for_changes_override: Some(true),
                ..default()
            }),
            GameDataPlugin::default(),
            MainMenuPlugin,
            PlanetViewPlugin,
        ))
        .add_systems(
            Update,
            return_to_menu_input.run_if(in_state(GameState::PlanetView)),
        )
        .run();
}

/// Handle ESC key to return to main menu from planet view.
///
/// This system runs only when in `GameState::PlanetView` and allows
/// the player to exit back to the main menu at any time.
fn return_to_menu_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::MainMenu);
    }
}
