use bevy::prelude::*;
use std::path::Path;

use crate::data::{GameData, GameRegistry, HasId};
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
