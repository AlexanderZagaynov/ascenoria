//! Game data loading plugin.
//!
//! Handles loading RON data files, hot reloading, and initializing game resources.
//!
//! # Module Structure
//! - [`initialization`] - Creates `GameData` and `GameRegistry` resources
//! - [`hot_reload`] - File watching for development-time data updates
//! - [`loader`] - Bevy asset loader for RON files
//!
//! # Usage
//! Add `GameDataPlugin` to your Bevy app to automatically load
//! all game data from `assets/data/` at startup.

pub mod hot_reload;
pub mod initialization;
mod loader;

use bevy::prelude::*;

use crate::data_types::load_game_data;

use self::hot_reload::{DataHotReload, hot_reload_game_data};
use self::initialization::initialize_game_resources;
use self::loader::{RonAsset, RonLoader};

/// Plugin that loads game data from RON files and registers it as a resource.
///
/// # Startup Behavior
/// 1. Calls `load_game_data()` to parse all RON files
/// 2. Creates `GameData` and `GameRegistry` resources
/// 3. Sets up hot-reload file watching (if enabled)
///
/// # Panics
/// Panics at startup if game data cannot be loaded (invalid RON, missing files, etc.).
pub struct GameDataPlugin {
    /// Path to the directory containing the RON data files.
    pub data_path: String,
}

impl Default for GameDataPlugin {
    fn default() -> Self {
        Self {
            data_path: "assets/data".to_string(),
        }
    }
}

/// Resource storing the path to game data files.
///
/// Used by the hot-reload system to know which directory to watch.
#[derive(Resource, Clone)]
pub struct GameDataSource {
    /// Path to the data directory (e.g., "assets/data").
    pub data_path: String,
}

impl Plugin for GameDataPlugin {
    fn build(&self, app: &mut App) {
        // Register RON asset type and loader
        app.init_asset::<RonAsset>()
            .init_asset_loader::<RonLoader>();

        // Store data path for hot-reload system
        app.insert_resource(GameDataSource {
            data_path: self.data_path.clone(),
        });
        app.insert_resource(DataHotReload::default());

        // Load game data synchronously at startup
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
