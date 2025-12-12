//! Game data loading plugin.
//!
//! Handles loading TOML data files, hot reloading, and initializing game resources.

pub mod hot_reload;
pub mod initialization;

use bevy::prelude::*;

use crate::data::load_game_data;

use self::hot_reload::{DataHotReload, hot_reload_game_data};
use self::initialization::initialize_game_resources;

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

#[derive(Resource, Clone)]
pub struct GameDataSource {
    pub data_path: String,
}

impl Plugin for GameDataPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameDataSource {
            data_path: self.data_path.clone(),
        });
        app.insert_resource(DataHotReload::default());

        match load_game_data(&self.data_path) {
            Ok((game_data, registry)) => {
                info!("Loaded game data from {}", self.data_path);
                initialize_game_resources(app, game_data, registry, &self.data_path);
                app.add_systems(Update, hot_reload_game_data);
            }
            Err(err) => {
                error!("Failed to load game data from {}: {}", self.data_path, err);
                panic!("Failed to load game data; see error log for details");
            }
        }
    }
}
