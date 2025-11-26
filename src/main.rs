mod combat;
mod data;
mod galaxy;
mod industry;
mod planet;
mod research;
mod ship_blueprints;
mod ship_design;
mod ship_ui;

use bevy::{
    prelude::*,
    text::{TextColor, TextFont},
};

use data::{
    GameData, GameRegistry, Language, LocalizedEntity, NO_TECH_REQUIREMENT, load_game_data,
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
                let victory_preview = VictoryPreview {
                    state: VictoryState::new(
                        generated_galaxy.systems.len() as i32,
                        DominationConfig::default(),
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
            }
            Err(err) => {
                error!("Failed to load game data from {}: {}", self.data_path, err);
                panic!("Failed to load game data; see error log for details");
            }
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
struct GalaxyPreview {
    galaxy: Galaxy,
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
        if let Some(cost) = industry_cost(data, registry, &BuildKind::Surface, item.id.as_str()) {
            industry.enqueue(BuildKind::Surface, item.id.clone(), cost);
        }
    }
    if let Some(item) = data.orbital_items().first() {
        if let Some(cost) = industry_cost(data, registry, &BuildKind::Orbital, item.id.as_str()) {
            industry.enqueue(BuildKind::Orbital, item.id.clone(), cost);
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
            &victory,
            &research,
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
            &victory,
            &research,
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
    victory: Res<VictoryPreview>,
    mut research: ResMut<ResearchPreview>,
    mut tech_state: ResMut<TechState>,
    localization: Res<LocalizationSettings>,
    mut text_query: Query<&mut Text, With<LocalizedPreviewText>>,
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
            &victory,
            research.as_ref(),
            text_query,
        );
    }
}

fn main() {
    App::new()
        .init_resource::<LocalizationSettings>()
        .add_plugins((DefaultPlugins, GameDataPlugin::default()))
        .add_systems(Startup, setup_ui)
        .add_systems(
            Update,
            (
                toggle_language,
                hull_selection_input,
                surface_building_input,
                orbital_building_input,
                research_input,
            ),
        )
        .run();
}
