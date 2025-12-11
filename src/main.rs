mod combat;
mod data;
mod galaxy;
mod galaxy_map;
mod game_options;
mod game_summary;
mod industry;
mod main_menu;
mod planet;
mod planet_view;
mod research;
mod ship_blueprints;
mod ship_design;
mod ship_ui;
mod star_system;
mod victory;

use bevy::{
    asset::{AssetEvent, AssetPlugin, LoadedFolder},
    ecs::message::MessageReader,
    ecs::system::SystemParam,
    prelude::*,
    text::{TextColor, TextFont},
};
use std::path::Path;

use galaxy_map::GalaxyMapPlugin;
use game_options::GameOptionsPlugin;
use game_summary::GameSummaryPlugin;
use main_menu::{GameState, MainMenuPlugin};
use planet_view::PlanetViewPlugin;
use star_system::StarSystemPlugin;

use data::{
    GameData, GameDataComputed, GameRegistry, HasDescription, HasId, Language, NO_TECH_REQUIREMENT,
    NamedEntity, load_game_data,
};
use galaxy::{Galaxy, format_galaxy, generate_galaxy};
use industry::{BuildKind, PlanetIndustry, industry_cost};
use planet::{
    GeneratedPlanet, OrbitalPreview, PlanetOrbitals, PlanetSurface, SurfacePreview, format_planet,
    generate_planet,
};
use research::ResearchState;
use ship_design::{ModuleCategory, ShipDesign};
use ship_ui::HullSelection;
use victory::{DominationConfig, VictoryState};

/// Plugin that loads game data from TOML files and registers it as a resource.
pub struct GameDataPlugin {
    /// Path to the directory containing the canonical TOML files.
    pub data_path: String,
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

impl Default for GameDataPlugin {
    fn default() -> Self {
        Self {
            data_path: "assets/data".to_string(),
        }
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
                let computed = game_data.compute();
                let generated_planet = generate_planet(42, &game_data);
                let generated_galaxy = generate_galaxy(1337, &game_data, 2..=3, 1..=3);
                if let Some(ref planet) = generated_planet {
                    info!("Generated debug planet\n{}", format_planet(planet));
                } else {
                    warn!("No planet generated; check planet size and surface data.");
                }
                info!(
                    "Generated debug galaxy\n{}",
                    format_galaxy(&generated_galaxy)
                );
                let mut tech_state = TechState::default();
                if let Some(first) = game_data.techs().first() {
                    tech_state.completed.insert(first.id.clone());
                }
                let mut surface_construction =
                    SurfaceConstruction::with_planet(generated_planet.clone());
                let mut orbital_construction =
                    OrbitalConstruction::with_planet(generated_planet.clone());
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
                    state: VictoryState::new(
                        generated_galaxy.systems.len() as i32,
                        domination_config,
                    ),
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

                if let Some(asset_server) = app.world().get_resource::<AssetServer>().cloned() {
                    let mut watchers = app.world_mut().resource_mut::<DataHotReload>();
                    let base_path = asset_relative_path(&self.data_path);
                    let mods_path = Path::new(&self.data_path)
                        .parent()
                        .unwrap_or_else(|| Path::new("assets"))
                        .join("mods");
                    watchers.base_handle = base_path.map(|path| asset_server.load_folder(path));
                    watchers.mods_handle =
                        asset_relative_path(&mods_path).map(|path| asset_server.load_folder(path));
                }
                app.add_systems(Update, hot_reload_game_data);
            }
            Err(err) => {
                error!("Failed to load game data from {}: {}", self.data_path, err);
                panic!("Failed to load game data; see error log for details");
            }
        }
    }
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
    localization: Res<LocalizationSettings>,
    text_query: Query<&mut Text, With<LocalizedPreviewText>>,
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
            rebuild_preview_text(
                &game_data,
                &localization,
                &planet_preview,
                &galaxy_preview,
                &hull_selection,
                &tech_state,
                &surface_construction,
                &orbital_construction,
                &industry,
                &research,
                &victory,
                text_query,
            );
            info!("Hot reloaded game data from {}", source.data_path);
        }
        Err(err) => {
            warn!("Failed to hot reload game data: {err}");
        }
    }
}

/// Current language selection for UI rendering.
#[derive(Resource, Default)]
struct LocalizationSettings {
    language: Language,
}

impl LocalizationSettings {
    fn toggle(&mut self) {
        self.language = match self.language {
            Language::En => Language::Ru,
            Language::Ru => Language::En,
        };
    }
}

#[derive(Component)]
struct LocalizedPreviewText;

/// Holds a generated planet for debug visualization.
#[derive(Resource, Default)]
struct PlanetPreview {
    planet: Option<GeneratedPlanet>,
}

/// Holds a generated galaxy snapshot.
#[derive(Resource)]
pub struct GalaxyPreview {
    pub galaxy: Galaxy,
}

impl Default for GalaxyPreview {
    fn default() -> Self {
        Self {
            galaxy: Galaxy {
                systems: Vec::new(),
            },
        }
    }
}

/// Placeholder industry/build queue preview.
#[derive(Resource, Default)]
struct IndustryPreview {
    industry: PlanetIndustry,
}

/// Tracks domination victory progress.
#[derive(Resource)]
struct VictoryPreview {
    state: VictoryState,
}

impl Default for VictoryPreview {
    fn default() -> Self {
        Self {
            state: VictoryState::new(0, DominationConfig::default()),
        }
    }
}

/// Research progress and selection.
#[derive(Resource)]
struct ResearchPreview {
    state: ResearchState,
}

impl Default for ResearchPreview {
    fn default() -> Self {
        Self {
            state: ResearchState::new(1),
        }
    }
}

fn build_industry_preview(data: &GameData, registry: &GameRegistry) -> IndustryPreview {
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

/// Tracks unlocked technologies by index for filtering build options.
#[derive(Resource, Default)]
struct TechState {
    completed: std::collections::HashSet<String>,
}

impl TechState {
    fn is_unlocked(&self, tech_index: i32, techs: &[data::Tech]) -> bool {
        if tech_index == NO_TECH_REQUIREMENT {
            return true;
        }
        techs
            .get(tech_index as usize)
            .map(|t| self.completed.contains(&t.id))
            .unwrap_or(false)
    }
}

/// Surface construction preview and selection state.
#[derive(Resource, Default)]
struct SurfaceConstruction {
    surface: Option<PlanetSurface>,
    selected_building: usize,
    preview: Option<SurfacePreview>,
}

impl SurfaceConstruction {
    fn with_planet(planet: Option<GeneratedPlanet>) -> Self {
        Self {
            surface: planet.as_ref().map(PlanetSurface::from),
            selected_building: 0,
            preview: None,
        }
    }
}

/// Orbital construction preview and selection state.
#[derive(Resource, Default)]
struct OrbitalConstruction {
    orbitals: Option<PlanetOrbitals>,
    selected_building: usize,
    preview: Option<OrbitalPreview>,
}

impl OrbitalConstruction {
    fn with_planet(planet: Option<GeneratedPlanet>) -> Self {
        Self {
            orbitals: planet.as_ref().map(PlanetOrbitals::from),
            selected_building: 0,
            preview: None,
        }
    }
}

fn refresh_surface_preview(
    surface_construction: &mut SurfaceConstruction,
    game_data: &GameData,
    tech_state: &TechState,
) {
    let available_buildings: Vec<_> = game_data
        .surface_items()
        .iter()
        .filter(|item| tech_state.is_unlocked(item.tech_index, game_data.techs()))
        .collect();

    if available_buildings.is_empty() {
        surface_construction.preview = None;
        surface_construction.selected_building = 0;
        return;
    }

    if surface_construction.selected_building >= available_buildings.len() {
        surface_construction.selected_building = available_buildings.len() - 1;
    }

    if let Some(surface) = &surface_construction.surface {
        if let Some(item) = available_buildings.get(surface_construction.selected_building) {
            surface_construction.preview = surface.preview_placement(item).ok();
        }
    } else {
        surface_construction.preview = None;
    }
}

fn refresh_orbital_preview(
    orbital_construction: &mut OrbitalConstruction,
    game_data: &GameData,
    tech_state: &TechState,
) {
    let available_buildings: Vec<_> = game_data
        .orbital_items()
        .iter()
        .filter(|item| tech_state.is_unlocked(item.tech_index, game_data.techs()))
        .collect();

    if available_buildings.is_empty() {
        orbital_construction.preview = None;
        orbital_construction.selected_building = 0;
        return;
    }

    if orbital_construction.selected_building >= available_buildings.len() {
        orbital_construction.selected_building = available_buildings.len() - 1;
    }

    if let Some(orbitals) = &orbital_construction.orbitals {
        if let Some(item) = available_buildings.get(orbital_construction.selected_building) {
            orbital_construction.preview = orbitals.preview_placement(item).ok();
        }
    } else {
        orbital_construction.preview = None;
    }
}

fn localized_preview(
    game_data: &GameData,
    language: Language,
    planet_preview: &PlanetPreview,
    galaxy_preview: &GalaxyPreview,
    hull_selection: &HullSelection,
    tech_state: &TechState,
    surface_construction: &SurfaceConstruction,
    orbital_construction: &OrbitalConstruction,
    industry: &IndustryPreview,
    research: &ResearchPreview,
    victory: &VictoryPreview,
) -> String {
    let mut lines = Vec::new();

    if let Some(species) = game_data.species().first() {
        lines.push(species.name(language).to_string());
        lines.push(species.description(language).to_string());
    }

    lines.push(String::new());
    lines.push(hull_selection.render(language));

    if let Some(selected_hull) = hull_selection.selected_id() {
        let mut design = ShipDesign::new(selected_hull.to_string());
        design.add_module(ModuleCategory::Engine, "tonklin_motor");
        lines.push(String::new());
        lines.push("Hull slots:".to_string());
        lines.push(hull_selection.render_slots(&design));
    }

    if let Some(engine) = game_data.engines().first() {
        lines.push(engine.name(language).to_string());
        lines.push(engine.description(language).to_string());
    }

    if let Some(planet) = &planet_preview.planet {
        lines.push(String::new());
        lines.push("Debug planet preview:".to_string());
        lines.push(format_planet(planet));
    }

    lines.push(String::new());
    lines.push("Build queue:".to_string());
    if industry.industry.queue.is_empty() {
        lines.push("  (empty)".to_string());
    } else {
        for (idx, order) in industry.industry.queue.iter().enumerate() {
            lines.push(format!(
                "  {}. {:?} {} — remaining {}",
                idx + 1,
                order.kind,
                order.id,
                order.remaining_cost
            ));
        }
    }

    lines.push(String::new());
    lines.push("Research:".to_string());
    let techs = game_data.techs();
    if techs.is_empty() {
        lines.push("  No techs loaded".to_string());
    } else {
        if let Some(active) = &research.state.active {
            lines.push(format!(
                "  Active: {} (remaining {}, spent {})",
                active.id, active.remaining, active.spent
            ));
        } else {
            lines.push("  Active: None".to_string());
        }
        if let Some(sel) = techs.get(research.state.selected) {
            lines.push(format!(
                "  Selected: {} — {} (cost {})",
                sel.name(language),
                sel.description(language),
                sel.research_cost
            ));
            let prereqs = game_data.tech_prereqs(&sel.id);
            if prereqs.is_empty() {
                lines.push("  Prereqs: none".to_string());
            } else {
                lines.push("  Prereqs:".to_string());
                for p in prereqs {
                    let status = if research.state.completed.contains(p) {
                        "[done]"
                    } else {
                        "[locked]"
                    };
                    lines.push(format!("    {} {}", status, p));
                }
            }
            let unlocks = game_data.tech_unlocks(&sel.id);
            if !unlocks.is_empty() {
                lines.push(format!("  Unlocks: {}", unlocks.join(", ")));
            }
        }
        if research.state.completed.is_empty() {
            lines.push("  Completed: none".to_string());
        } else {
            lines.push(format!(
                "  Completed: {}",
                research.state.completed.join(", ")
            ));
        }
    }

    lines.push(String::new());
    lines.push(format!(
        "Domination: {} / {} systems (threshold {:.0}%)",
        victory.state.controlled_systems,
        victory.state.total_systems,
        victory.state.config.threshold * 100.0
    ));
    if victory.state.domination_achieved {
        lines.push("  Victory achieved!".to_string());
    }
    if victory.state.tech_victory {
        lines.push("Tech victory achieved!".to_string());
    }

    lines.push(String::new());
    lines.push("Orbital construction:".to_string());

    let available_orbitals: Vec<_> = game_data
        .orbital_items()
        .iter()
        .filter(|item| tech_state.is_unlocked(item.tech_index, game_data.techs()))
        .collect();

    if let Some(orbitals) = &orbital_construction.orbitals {
        lines.push(format!(
            "Used orbital slots: {} / {}",
            orbitals.used_slots(),
            orbitals.capacity()
        ));

        if available_orbitals.is_empty() {
            lines.push("No unlocked orbital structures.".to_string());
        } else {
            lines.push("Orbitals (use , and . to cycle, / to confirm):".to_string());
            for (idx, item) in available_orbitals.iter().enumerate() {
                let marker = if idx == orbital_construction.selected_building {
                    '>'
                } else {
                    ' '
                };

                lines.push(format!(
                    "{marker} {} (cost {})",
                    item.name(language),
                    item.industry_cost
                ));
            }

            lines.push(String::new());
            lines.push("Orbital slots:".to_string());
            lines.push(orbitals.render_with_preview(orbital_construction.preview.as_ref()));
        }
    } else {
        lines.push("No orbital capacity available.".to_string());
    }

    lines.push(String::new());
    lines.push("Surface construction:".to_string());

    let available_buildings: Vec<_> = game_data
        .surface_items()
        .iter()
        .filter(|item| tech_state.is_unlocked(item.tech_index, game_data.techs()))
        .collect();

    if let Some(surface) = &surface_construction.surface {
        lines.push(format!(
            "Used slots: {} / {}",
            surface.used_slots(),
            surface.capacity()
        ));

        if available_buildings.is_empty() {
            lines.push("No unlocked surface buildings.".to_string());
        } else {
            lines.push("Available buildings (use [ and ] to cycle, Enter to confirm):".to_string());
            for (idx, item) in available_buildings.iter().enumerate() {
                let marker = if idx == surface_construction.selected_building {
                    '>'
                } else {
                    ' '
                };

                lines.push(format!(
                    "{marker} {} (slots {}, cost {})",
                    item.name(language),
                    item.slot_size,
                    item.industry_cost
                ));
            }

            lines.push(String::new());
            lines.push("Placement preview:".to_string());
            lines.push(surface.render_with_preview(surface_construction.preview.as_ref()));
        }
    } else {
        lines.push("No planet surface generated.".to_string());
    }

    lines.push(String::new());
    lines.push("Debug galaxy preview:".to_string());
    lines.push(format!("Systems: {}", galaxy_preview.galaxy.systems.len()));
    if let Some(first) = galaxy_preview.galaxy.systems.first() {
        lines.push(format!(
            "First system: {} ({} planets)",
            first.name,
            first.planets.len()
        ));
    }

    lines.join("\n")
}

fn rebuild_preview_text(
    game_data: &GameData,
    localization: &LocalizationSettings,
    planet_preview: &PlanetPreview,
    galaxy_preview: &GalaxyPreview,
    hull_selection: &HullSelection,
    tech_state: &TechState,
    surface_construction: &SurfaceConstruction,
    orbital_construction: &OrbitalConstruction,
    industry: &IndustryPreview,
    research: &ResearchPreview,
    victory: &VictoryPreview,
    mut text_query: Query<&mut Text, With<LocalizedPreviewText>>,
) {
    let preview = localized_preview(
        game_data,
        localization.language,
        planet_preview,
        galaxy_preview,
        hull_selection,
        tech_state,
        surface_construction,
        orbital_construction,
        industry,
        research,
        victory,
    );

    for mut text in &mut text_query {
        text.0 = preview.clone();
    }
}

fn setup_ui(
    mut commands: Commands,
    game_data: Res<GameData>,
    localization: Res<LocalizationSettings>,
    planet_preview: Res<PlanetPreview>,
    galaxy_preview: Res<GalaxyPreview>,
    hull_selection: Res<HullSelection>,
    tech_state: Res<TechState>,
    surface_construction: Res<SurfaceConstruction>,
    orbital_construction: Res<OrbitalConstruction>,
    industry: Res<IndustryPreview>,
    research: Res<ResearchPreview>,
    victory: Res<VictoryPreview>,
) {
    // Spawn camera for in-game UI
    commands.spawn((Camera2d::default(), InGameUI));

    let preview = localized_preview(
        &game_data,
        localization.language,
        &planet_preview,
        &galaxy_preview,
        hull_selection.as_ref(),
        &tech_state,
        &surface_construction,
        &orbital_construction,
        &industry,
        &research,
        &victory,
    );
    commands.spawn((
        Text::new(preview),
        TextFont {
            font_size: 18.0,
            ..Default::default()
        },
        TextColor(Color::WHITE),
        LocalizedPreviewText,
        InGameUI,
    ));

    // Instructions text
    commands.spawn((
        Text::new("Press ESC to return to main menu"),
        TextFont {
            font_size: 14.0,
            ..Default::default()
        },
        TextColor(Color::srgba(0.7, 0.7, 0.7, 0.8)),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
        InGameUI,
    ));
}

fn toggle_language(
    input: Res<ButtonInput<KeyCode>>,
    mut localization: ResMut<LocalizationSettings>,
    game_data: Res<GameData>,
    planet_preview: Res<PlanetPreview>,
    galaxy_preview: Res<GalaxyPreview>,
    hull_selection: Res<HullSelection>,
    tech_state: Res<TechState>,
    surface_construction: Res<SurfaceConstruction>,
    orbital_construction: Res<OrbitalConstruction>,
    industry: Res<IndustryPreview>,
    research: Res<ResearchPreview>,
    victory: Res<VictoryPreview>,
    text_query: Query<&mut Text, With<LocalizedPreviewText>>,
) {
    if !input.just_pressed(KeyCode::KeyL) {
        return;
    }

    localization.toggle();
    rebuild_preview_text(
        &game_data,
        &localization,
        &planet_preview,
        &galaxy_preview,
        hull_selection.as_ref(),
        &tech_state,
        &surface_construction,
        &orbital_construction,
        &industry,
        &research,
        &victory,
        text_query,
    );
}

fn hull_selection_input(
    input: Res<ButtonInput<KeyCode>>,
    game_data: Res<GameData>,
    planet_preview: Res<PlanetPreview>,
    galaxy_preview: Res<GalaxyPreview>,
    mut hull_selection: ResMut<HullSelection>,
    localization: Res<LocalizationSettings>,
    tech_state: Res<TechState>,
    surface_construction: Res<SurfaceConstruction>,
    orbital_construction: Res<OrbitalConstruction>,
    industry: Res<IndustryPreview>,
    research: Res<ResearchPreview>,
    victory: Res<VictoryPreview>,
    text_query: Query<&mut Text, With<LocalizedPreviewText>>,
) {
    let mut changed = false;
    if input.just_pressed(KeyCode::ArrowDown) {
        hull_selection.next();
        changed = true;
    } else if input.just_pressed(KeyCode::ArrowUp) {
        hull_selection.prev();
        changed = true;
    }

    if changed {
        rebuild_preview_text(
            &game_data,
            &localization,
            &planet_preview,
            &galaxy_preview,
            hull_selection.as_ref(),
            &tech_state,
            &surface_construction,
            &orbital_construction,
            &industry,
            &research,
            &victory,
            text_query,
        );
    }
}

fn surface_building_input(
    input: Res<ButtonInput<KeyCode>>,
    game_data: Res<GameData>,
    planet_preview: Res<PlanetPreview>,
    galaxy_preview: Res<GalaxyPreview>,
    hull_selection: Res<HullSelection>,
    mut surface_construction: ResMut<SurfaceConstruction>,
    tech_state: Res<TechState>,
    localization: Res<LocalizationSettings>,
    orbital_construction: Res<OrbitalConstruction>,
    industry: Res<IndustryPreview>,
    research: Res<ResearchPreview>,
    victory: Res<VictoryPreview>,
    text_query: Query<&mut Text, With<LocalizedPreviewText>>,
) {
    let available_buildings: Vec<_> = game_data
        .surface_items()
        .iter()
        .filter(|item| tech_state.is_unlocked(item.tech_index, game_data.techs()))
        .collect();

    if available_buildings.is_empty() {
        return;
    }

    let mut changed = false;
    if input.just_pressed(KeyCode::BracketRight) {
        surface_construction.selected_building =
            (surface_construction.selected_building + 1) % available_buildings.len();
        changed = true;
    } else if input.just_pressed(KeyCode::BracketLeft) {
        if surface_construction.selected_building == 0 {
            surface_construction.selected_building = available_buildings.len() - 1;
        } else {
            surface_construction.selected_building -= 1;
        }
        changed = true;
    }

    if input.just_pressed(KeyCode::Enter) {
        if let Some(preview) = surface_construction.preview.clone() {
            if let Some(surface) = surface_construction.surface.as_mut() {
                surface.apply_preview(&preview);
                changed = true;
            }
        }
    }

    if changed {
        refresh_surface_preview(&mut surface_construction, &game_data, &tech_state);

        rebuild_preview_text(
            &game_data,
            &localization,
            &planet_preview,
            &galaxy_preview,
            hull_selection.as_ref(),
            &tech_state,
            &surface_construction,
            &orbital_construction,
            &industry,
            &research,
            &victory,
            text_query,
        );
    }
}

fn orbital_building_input(
    input: Res<ButtonInput<KeyCode>>,
    game_data: Res<GameData>,
    planet_preview: Res<PlanetPreview>,
    galaxy_preview: Res<GalaxyPreview>,
    hull_selection: Res<HullSelection>,
    mut orbital_construction: ResMut<OrbitalConstruction>,
    tech_state: Res<TechState>,
    localization: Res<LocalizationSettings>,
    surface_construction: Res<SurfaceConstruction>,
    industry: Res<IndustryPreview>,
    research: Res<ResearchPreview>,
    victory: Res<VictoryPreview>,
    text_query: Query<&mut Text, With<LocalizedPreviewText>>,
) {
    let available_buildings: Vec<_> = game_data
        .orbital_items()
        .iter()
        .filter(|item| tech_state.is_unlocked(item.tech_index, game_data.techs()))
        .collect();

    if available_buildings.is_empty() {
        return;
    }

    let mut changed = false;
    if input.just_pressed(KeyCode::Comma) {
        if orbital_construction.selected_building == 0 {
            orbital_construction.selected_building = available_buildings.len() - 1;
        } else {
            orbital_construction.selected_building -= 1;
        }
        changed = true;
    } else if input.just_pressed(KeyCode::Period) {
        orbital_construction.selected_building =
            (orbital_construction.selected_building + 1) % available_buildings.len();
        changed = true;
    }

    if input.just_pressed(KeyCode::Slash) {
        if let Some(preview) = orbital_construction.preview.clone() {
            if let Some(orbitals) = orbital_construction.orbitals.as_mut() {
                orbitals.apply_preview(&preview);
                changed = true;
            }
        }
    }

    if changed {
        refresh_orbital_preview(&mut orbital_construction, &game_data, &tech_state);

        rebuild_preview_text(
            &game_data,
            &localization,
            &planet_preview,
            &galaxy_preview,
            hull_selection.as_ref(),
            &tech_state,
            &surface_construction,
            &orbital_construction,
            &industry,
            &research,
            &victory,
            text_query,
        );
    }
}

fn research_input(
    input: Res<ButtonInput<KeyCode>>,
    game_data: Res<GameData>,
    planet_preview: Res<PlanetPreview>,
    galaxy_preview: Res<GalaxyPreview>,
    hull_selection: Res<HullSelection>,
    surface_construction: Res<SurfaceConstruction>,
    orbital_construction: Res<OrbitalConstruction>,
    industry: Res<IndustryPreview>,
    mut victory: ResMut<VictoryPreview>,
    mut research: ResMut<ResearchPreview>,
    mut tech_state: ResMut<TechState>,
    localization: Res<LocalizationSettings>,
    text_query: Query<&mut Text, With<LocalizedPreviewText>>,
) {
    let total = game_data.techs().len();
    let mut changed = false;

    if input.just_pressed(KeyCode::KeyN) {
        research.state.next(total);
        changed = true;
    } else if input.just_pressed(KeyCode::KeyP) {
        research.state.prev(total);
        changed = true;
    }

    if input.just_pressed(KeyCode::KeyR) {
        research.state.start_selected(&game_data);
        changed = true;
    }

    if input.just_pressed(KeyCode::KeyO) {
        if let Some(completed) = research.state.process_turn() {
            for id in &research.state.completed {
                tech_state.completed.insert(id.clone());
            }
            victory
                .state
                .check_tech_victory(game_data.techs().len(), research.state.completed.len());
            info!("Completed research {}", completed);
        }
        changed = true;
    }

    if changed {
        rebuild_preview_text(
            &game_data,
            &localization,
            &planet_preview,
            &galaxy_preview,
            hull_selection.as_ref(),
            &tech_state,
            &surface_construction,
            &orbital_construction,
            &industry,
            research.as_ref(),
            &victory,
            text_query,
        );
    }
}

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
            GalaxyMapPlugin,
            StarSystemPlugin,
            PlanetViewPlugin,
        ))
        .add_systems(
            Update,
            return_to_menu_input.run_if(in_state(GameState::InGame)),
        )
        .run();
}

/// Marker component for in-game UI elements.
#[derive(Component)]
struct InGameUI;

fn cleanup_game_ui(mut commands: Commands, query: Query<Entity, With<InGameUI>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

fn return_to_menu_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::MainMenu);
    }
}
