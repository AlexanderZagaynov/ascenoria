//! Main menu screen implementation inspired by classic Ascendancy.
//!
//! Displays a title and menu buttons for game actions like New Game, Load, Save, and Exit.

use bevy::prelude::*;

mod colors;
mod components;
mod systems;

use systems::{setup_main_menu, cleanup_main_menu, button_system, menu_action_system};

/// Plugin that manages the main menu screen.
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
#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    GameOptions,
    GameSummary,
    InGame,
    StarSystem,
    PlanetView,
    Settings,
}
