//! Game summary screen.
//!
//! Shows a summary of the chosen/loaded game context. When starting a new game,
//! it presents the selected species details and a short briefing.

mod briefing;
mod setup;
mod systems;
mod types;

use bevy::prelude::*;

use crate::main_menu::GameState;

/// Plugin for the game summary screen.
pub struct GameSummaryPlugin;

impl Plugin for GameSummaryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameSummary), setup::setup_game_summary)
            .add_systems(
                OnExit(GameState::GameSummary),
                systems::cleanup_game_summary,
            )
            .add_systems(
                Update,
                (
                    systems::continue_system,
                    systems::keyboard_navigation_system,
                    systems::summary_scroll_system,
                )
                    .run_if(in_state(GameState::GameSummary)),
            );
    }
}
