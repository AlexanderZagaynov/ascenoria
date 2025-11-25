use bevy::prelude::*;

mod data;

use data::load_game_data;

/// Plugin that loads game data from TOML files and registers it as a resource.
pub struct GameDataPlugin {
    /// Path to the directory containing the canonical TOML files.
    pub data_path: String,
}

impl Default for GameDataPlugin {
    fn default() -> Self {
        Self {
            data_path: "assets/data".to_string(),
        }
    }
}

impl Plugin for GameDataPlugin {
    fn build(&self, app: &mut App) {
        match load_game_data(&self.data_path) {
            Ok(game_data) => {
                info!("Loaded game data from {}", self.data_path);
                app.insert_resource(game_data);
            }
            Err(err) => {
                panic!("Failed to load game data from {}: {}", self.data_path, err);
            }
        }
    }
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, GameDataPlugin::default()))
        .run();
}
