use bevy::prelude::*;
use std::path::Path;

use crate::data_types::{GameData, GameRegistry};
use crate::galaxy_data::{GalaxyPreview, generate_galaxy};

use super::hot_reload::DataHotReload;

fn asset_relative_path(path: impl AsRef<Path>) -> Option<String> {
    let path = path.as_ref();
    if path.is_absolute() {
        return None;
    }

    let trimmed = path
        .strip_prefix("assets")
        .unwrap_or(path)
        .to_str()?
        .trim_start_matches(['/', '\\'])
        .replace('\\', "/");

    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed)
    }
}

/// Initialize all game resources from loaded data.
pub fn initialize_game_resources(
    app: &mut App,
    game_data: GameData,
    registry: GameRegistry,
    data_path: &str,
) {
    let computed = game_data.compute();
    let generated_galaxy = generate_galaxy(1337, &game_data, 2..=3, 1..=3);

    info!(
        "Generated debug galaxy\n{}",
        crate::galaxy_data::format_galaxy(&generated_galaxy)
    );

    app.insert_resource(registry);
    app.insert_resource(computed);
    app.insert_resource(GalaxyPreview {
        galaxy: generated_galaxy,
    });
    app.insert_resource(game_data);

    // Set up file watchers for hot reload
    if let Some(asset_server) = app.world().get_resource::<AssetServer>().cloned() {
        let mut watchers = app.world_mut().resource_mut::<DataHotReload>();
        let base_path = asset_relative_path(data_path);
        let mods_path = Path::new(data_path)
            .parent()
            .unwrap_or_else(|| Path::new("assets"))
            .join("mods");
        watchers.base_handle = base_path.map(|path| asset_server.load_folder(path));
        watchers.mods_handle =
            asset_relative_path(&mods_path).map(|path| asset_server.load_folder(path));
    }
}
