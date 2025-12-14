//! Main menu screen implementation inspired by classic Ascendancy.
//!
//! Displays a title and menu buttons for game actions like New Game, Load, Save, and Exit.

use bevy::prelude::*;

mod colors;
mod components;
mod systems;

use crate::shared::AppState;
use systems::{button_system, cleanup_main_menu, menu_action_system, setup_main_menu};

/// Plugin that manages the main menu screen.
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), setup_main_menu)
            .add_systems(OnExit(AppState::MainMenu), cleanup_main_menu)
            .add_systems(
                Update,
                (button_system, menu_action_system).run_if(in_state(AppState::MainMenu)),
            );
    }
}
