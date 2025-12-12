use crate::game_options::systems::{interaction, setup, species};
use crate::main_menu::GameState;
use bevy::prelude::*;

/// Plugin that manages the species selection screen.
pub struct GameOptionsPlugin;

impl Plugin for GameOptionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameOptions), setup::setup_game_options)
            .add_systems(OnExit(GameState::GameOptions), setup::cleanup_game_options)
            .add_systems(
                Update,
                (
                    interaction::button_system,
                    species::species_list_system,
                    interaction::settings_button_system,
                    interaction::begin_game_system,
                    species::keyboard_navigation_system,
                    species::species_scroll_system.after(species::keyboard_navigation_system),
                )
                    .run_if(in_state(GameState::GameOptions)),
            );
    }
}
