use bevy::{
    asset::{AssetEvent, LoadedFolder},
    ecs::message::MessageReader,
    ecs::system::SystemParam,
    prelude::*,
};

use crate::data::{GameData, GameDataComputed, GameRegistry, HasId, load_game_data};
use crate::galaxy::generate_galaxy;
use crate::planet::generate_planet;
use crate::preview::{
    GalaxyPreview, IndustryPreview, OrbitalConstruction, PlanetPreview, ResearchPreview,
    SurfaceConstruction, TechState, VictoryPreview, refresh_orbital_preview,
    refresh_surface_preview,
};
use crate::research::ResearchState;
use crate::ship_ui::HullSelection;
use crate::victory::{DominationConfig, VictoryState};

use super::GameDataSource;
use super::initialization::build_industry_preview;

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
    planet_preview: ResMut<'w, PlanetPreview>,
    galaxy_preview: ResMut<'w, GalaxyPreview>,
    hull_selection: ResMut<'w, HullSelection>,
    industry: ResMut<'w, IndustryPreview>,
    research: ResMut<'w, ResearchPreview>,
    victory: ResMut<'w, VictoryPreview>,
    surface_construction: ResMut<'w, SurfaceConstruction>,
    orbital_construction: ResMut<'w, OrbitalConstruction>,
    tech_state: ResMut<'w, TechState>,
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
        mut planet_preview,
        mut galaxy_preview,
        mut hull_selection,
        mut industry,
        mut research,
        mut victory,
        mut surface_construction,
        mut orbital_construction,
        mut tech_state,
    } = targets;

    match load_game_data(&source.data_path) {
        Ok((new_data, new_registry)) => {
            let new_computed = new_data.compute();
            let new_planet = generate_planet(42, &new_data);
            let new_galaxy = generate_galaxy(1337, &new_data, 2..=3, 1..=3);

            let mut completed: std::collections::HashSet<String> = tech_state
                .completed
                .iter()
                .filter(|id| new_data.techs().iter().any(|tech| &tech.id == *id))
                .cloned()
                .collect();
            if completed.is_empty() {
                if let Some(first) = new_data.techs().first() {
                    completed.insert(first.id.clone());
                }
            }

            *game_data = new_data;
            *registry = new_registry;
            *computed = new_computed;
            *planet_preview = PlanetPreview {
                planet: new_planet.clone(),
            };
            *galaxy_preview = GalaxyPreview {
                galaxy: new_galaxy.clone(),
            };
            *tech_state = TechState { completed };
            *hull_selection = HullSelection::from_game_data(&game_data);
            *industry = build_industry_preview(&game_data, &registry);
            *research = ResearchPreview {
                state: ResearchState::new(1),
            };
            let domination_config = DominationConfig {
                threshold: game_data.victory_rules().domination_threshold,
            };
            *victory = VictoryPreview {
                state: VictoryState::new(
                    galaxy_preview.galaxy.systems.len() as i32,
                    domination_config,
                ),
            };
            *surface_construction = SurfaceConstruction::with_planet(new_planet.clone());
            *orbital_construction = OrbitalConstruction::with_planet(new_planet);

            victory
                .state
                .check_tech_victory(game_data.techs().len(), tech_state.completed.len());

            refresh_surface_preview(&mut surface_construction, &game_data, &tech_state);
            refresh_orbital_preview(&mut orbital_construction, &game_data, &tech_state);

            info!("Hot reloaded game data from {}", source.data_path);
        }
        Err(err) => {
            warn!("Failed to hot reload game data: {err}");
        }
    }
}
