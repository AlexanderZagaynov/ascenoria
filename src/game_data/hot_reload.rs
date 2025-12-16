use bevy::{
    asset::{AssetEvent, LoadedFolder},
    ecs::message::MessageReader,
    ecs::system::SystemParam,
    prelude::*,
};

use crate::data_types::{GameData, GameDataComputed, GameRegistry, load_game_data};
use crate::galaxy_data::{GalaxyPreview, generate_galaxy};

use super::GameDataSource;

#[derive(Resource, Default)]
pub struct DataHotReload {
    pub base_handle: Option<Handle<LoadedFolder>>,
    pub mods_handle: Option<Handle<LoadedFolder>>,
}

impl DataHotReload {
    pub fn matches(&self, event: &AssetEvent<LoadedFolder>) -> bool {
        let handles = [self.base_handle.as_ref(), self.mods_handle.as_ref()];
        handles.into_iter().flatten().any(|handle| {
            event.is_added(handle.id())
                || event.is_modified(handle.id())
                || event.is_loaded_with_dependencies(handle.id())
                || event.is_removed(handle.id())
        })
    }
}

#[derive(SystemParam)]
pub struct HotReloadTargets<'w> {
    game_data: ResMut<'w, GameData>,
    registry: ResMut<'w, GameRegistry>,
    computed: ResMut<'w, GameDataComputed>,
    galaxy_preview: ResMut<'w, GalaxyPreview>,
}

pub fn hot_reload_game_data(
    asset_server: Res<AssetServer>,
    source: Res<GameDataSource>,
    watchers: Res<DataHotReload>,
    mut events: MessageReader<AssetEvent<LoadedFolder>>,
    targets: HotReloadTargets,
) {
    if !asset_server.watching_for_changes() {
        return;
    }

    let mut should_reload = false;
    for event in events.read() {
        if watchers.matches(event) {
            should_reload = true;
            break;
        }
    }

    if !should_reload {
        return;
    }

    let HotReloadTargets {
        mut game_data,
        mut registry,
        mut computed,
        mut galaxy_preview,
    } = targets;

    match load_game_data(&source.data_path) {
        Ok((new_data, new_registry)) => {
            let new_computed = new_data.compute();
            let new_galaxy = generate_galaxy(1337, &new_data, 2..=3, 1..=3);

            *game_data = new_data;
            *registry = new_registry;
            *computed = new_computed;
            *galaxy_preview = GalaxyPreview {
                galaxy: new_galaxy,
            };

            info!("Hot reloaded game data from {}", source.data_path);
        }
        Err(err) => {
            warn!("Failed to hot reload game data: {err}");
        }
    }
}
