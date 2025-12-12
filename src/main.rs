//! Ascenoria - A data-driven 4X strategy game inspired by Ascendancy.
//!
//! This is the main entry point that registers all game plugins and starts the Bevy app.

mod combat;
mod data;
mod galaxy_data;
mod galaxy_view;
mod game_data_plugin;
mod game_options;
mod game_summary;
mod industry;
mod main_menu;
mod planet_data;
mod planet_view;
mod preview;
mod research;
mod ship_blueprints;
mod ship_design;
mod ship_ui;
mod star_data;
mod victory;

use bevy::{asset::AssetPlugin, prelude::*};

use galaxy_view::GalaxyViewPlugin;
use game_data_plugin::GameDataPlugin;
use game_options::GameOptionsPlugin;
use game_summary::GameSummaryPlugin;
use main_menu::{GameState, MainMenuPlugin};
use planet_view::PlanetViewPlugin;
use preview::LocalizationSettings;
use star_data::StarPlugin;

// Re-export commonly used types for other modules
pub use preview::GalaxyPreview;

fn main() {
    App::new()
        .init_resource::<LocalizationSettings>()
        .add_plugins((
            DefaultPlugins.set(AssetPlugin {
                watch_for_changes_override: Some(true),
                ..default()
            }),
            GameDataPlugin::default(),
            MainMenuPlugin,
            GameOptionsPlugin,
            GameSummaryPlugin,
            GalaxyViewPlugin,
            StarPlugin,
            PlanetViewPlugin,
        ))
        .add_systems(
            Update,
            return_to_menu_input.run_if(in_state(GameState::GalaxyView)),
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
