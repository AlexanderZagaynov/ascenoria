//! Main menu screen implementation inspired by classic Ascendancy.
//!
//! Displays a title and menu buttons for game actions like New Game, Load, Save, and Exit.
//!
//! # Module Structure
//! - [`colors`] - Color palette for the menu UI
//! - [`components`] - Marker components for menu entities
//! - [`systems`] - Setup, interaction, and cleanup systems

use bevy::prelude::*;

mod colors;
mod components;
mod systems;

use systems::{button_system, cleanup_main_menu, menu_action_system, setup_main_menu};

/// Plugin that manages the main menu screen.
///
/// # Systems
/// - `setup_main_menu` - Spawns UI on `OnEnter(GameState::MainMenu)`
/// - `cleanup_main_menu` - Despawns UI on `OnExit(GameState::MainMenu)`
/// - `button_system` - Handles hover highlighting
/// - `menu_action_system` - Handles button clicks to navigate or exit
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_systems(OnEnter(GameState::MainMenu), setup_main_menu)
            .add_systems(OnExit(GameState::MainMenu), cleanup_main_menu)
            .add_systems(
                Update,
                (button_system, menu_action_system).run_if(in_state(GameState::MainMenu)),
            );
    }
}

/// Game state machine for managing screens.
///
/// Each variant represents a distinct game screen. The Bevy state machine
/// handles transitions, running `OnEnter` and `OnExit` systems automatically.
///
/// # States
/// - `MainMenu` - Initial state, shows title and menu buttons
/// - `PlanetView` - Planet surface management screen
///
/// # Transitions
/// - `MainMenu` → `PlanetView`: Player clicks "New Game"
/// - `PlanetView` → `MainMenu`: Player presses ESC
#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    /// Main menu screen (default starting state).
    #[default]
    MainMenu,
    /// Planet surface management screen.
    PlanetView,
}
