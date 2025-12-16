//! Ascenoria - A data-driven 4X strategy game inspired by Ascendancy.
//!
//! This is the main entry point that registers all game plugins and starts the Bevy app.

use bevy::{asset::AssetPlugin, prelude::*};

use ascenoria::game_data::GameDataPlugin;
use ascenoria::main_menu::{GameState, MainMenuPlugin};
use ascenoria::planet_view::PlanetViewPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(AssetPlugin {
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

/// Handle ESC key to return to main menu from in-game state.
fn return_to_menu_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::MainMenu);
    }
}
