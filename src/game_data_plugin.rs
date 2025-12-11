//! Game data loading plugin.
//!
//! Handles loading TOML data files, hot reloading, and initializing game resources.

use bevy::{
    asset::{AssetEvent, LoadedFolder},
    ecs::message::MessageReader,
    ecs::system::SystemParam,
    prelude::*,
};
use std::path::Path;

use crate::data::{GameData, GameDataComputed, GameRegistry, HasId, load_game_data};
use crate::galaxy::generate_galaxy;
use crate::industry::{BuildKind, PlanetIndustry, industry_cost};
use crate::planet::generate_planet;
use crate::preview::{
    GalaxyPreview, IndustryPreview, OrbitalConstruction, PlanetPreview, ResearchPreview,
    SurfaceConstruction, TechState, VictoryPreview, refresh_orbital_preview,
    refresh_surface_preview,
};
use crate::research::ResearchState;
use crate::ship_ui::HullSelection;
use crate::victory::{DominationConfig, VictoryState};

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

#[derive(Resource, Clone)]
struct GameDataSource {
    data_path: String,
}

#[derive(Resource, Default)]
struct DataHotReload {
    base_handle: Option<Handle<LoadedFolder>>,
    mods_handle: Option<Handle<LoadedFolder>>,
}

impl DataHotReload {
    fn matches(&self, event: &AssetEvent<LoadedFolder>) -> bool {
        let handles = [self.base_handle.as_ref(), self.mods_handle.as_ref()];
        handles.into_iter().flatten().any(|handle| {
            event.is_added(handle.id())
                || event.is_modified(handle.id())
                || event.is_loaded_with_dependencies(handle.id())
                || event.is_removed(handle.id())
        })
    }
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

/// Initialize all game resources from loaded data.
fn initialize_game_resources(
    app: &mut App,
    game_data: GameData,
    registry: GameRegistry,
    data_path: &str,
) {
    let computed = game_data.compute();
    let generated_planet = generate_planet(42, &game_data);
    let generated_galaxy = generate_galaxy(1337, &game_data, 2..=3, 1..=3);

    if let Some(ref planet) = generated_planet {
        info!(
            "Generated debug planet\n{}",
            crate::planet::format_planet(planet)
        );
    } else {
        warn!("No planet generated; check planet size and surface data.");
    }
    info!(
        "Generated debug galaxy\n{}",
        crate::galaxy::format_galaxy(&generated_galaxy)
    );

    let mut tech_state = TechState::default();
    if let Some(first) = game_data.techs().first() {
        tech_state.completed.insert(first.id.clone());
    }

    let mut surface_construction = SurfaceConstruction::with_planet(generated_planet.clone());
    let mut orbital_construction = OrbitalConstruction::with_planet(generated_planet.clone());
    refresh_surface_preview(&mut surface_construction, &game_data, &tech_state);
    refresh_orbital_preview(&mut orbital_construction, &game_data, &tech_state);

    let industry_preview = build_industry_preview(&game_data, &registry);
    let research_preview = ResearchPreview {
        state: ResearchState::new(1),
    };

    let domination_config = DominationConfig {
        threshold: game_data.victory_rules().domination_threshold,
    };
    let victory_preview = VictoryPreview {
        state: VictoryState::new(generated_galaxy.systems.len() as i32, domination_config),
    };

    app.insert_resource(registry);
    app.insert_resource(computed);
    app.insert_resource(PlanetPreview {
        planet: generated_planet,
    });
    app.insert_resource(HullSelection::from_game_data(&game_data));
    app.insert_resource(GalaxyPreview {
        galaxy: generated_galaxy,
    });
    app.insert_resource(industry_preview);
    app.insert_resource(research_preview);
    app.insert_resource(victory_preview);
    app.insert_resource(surface_construction);
    app.insert_resource(orbital_construction);
    app.insert_resource(tech_state);
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

/// Build industry preview from game data.
pub fn build_industry_preview(data: &GameData, registry: &GameRegistry) -> IndustryPreview {
    let mut industry = PlanetIndustry::new(5);
    if let Some(item) = data.surface_items().first() {
        if let Some(cost) = industry_cost(data, registry, &BuildKind::Surface, item.id()) {
            industry.enqueue(BuildKind::Surface, item.id().to_string(), cost);
        }
    }
    if let Some(item) = data.orbital_items().first() {
        if let Some(cost) = industry_cost(data, registry, &BuildKind::Orbital, item.id()) {
            industry.enqueue(BuildKind::Orbital, item.id().to_string(), cost);
        }
    }
    IndustryPreview { industry }
}

#[derive(SystemParam)]
struct HotReloadTargets<'w> {
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

fn hot_reload_game_data(
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
