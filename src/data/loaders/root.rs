use std::path::{Path, PathBuf};

use crate::data::entities::{ResearchGraph, TechEdge, VictoryRules};
use crate::data::errors::DataLoadError;
use crate::data::game_data::GameData;
use crate::data::registry::GameRegistry;
use crate::data::validation::{validate_game_data, validate_tech_edges};

use super::merge::{merge_by_id, merge_tech_edges};
use super::migration::migrate_game_data;
use super::mod_loading::{ModDatasets, load_mod_datasets};
use super::research_graph::build_research_graph;
use super::toml::{load_toml_file, load_toml_file_optional};
use super::wrappers::{
    DataManifest, EnginesData, HullClassesData, PlanetSizesData, PlanetSurfaceTypesData,
    PlanetaryOrbitalData, PlanetaryProjectsData, PlanetarySurfaceData, ScannersData, ShieldsData,
    SpecialModulesData, SpeciesData, TechEdgesData, TechsData, VictoryConditionsData, WeaponsData,
};

fn apply_mods(game_data: &mut GameData, tech_edges: &mut Vec<TechEdge>, mods: ModDatasets) {
    merge_by_id(game_data.species_mut(), mods.species, |s| s.id.as_str());
    merge_by_id(game_data.planet_sizes_mut(), mods.planet_sizes, |s| {
        s.id.as_str()
    });
    merge_by_id(
        game_data.planet_surface_types_mut(),
        mods.planet_surface_types,
        |s| s.id.as_str(),
    );
    merge_by_id(game_data.surface_items_mut(), mods.surface_items, |s| {
        s.id.as_str()
    });
    merge_by_id(game_data.orbital_items_mut(), mods.orbital_items, |s| {
        s.id.as_str()
    });
    merge_by_id(
        game_data.planetary_projects_mut(),
        mods.planetary_projects,
        |p| p.id.as_str(),
    );
    merge_by_id(game_data.hull_classes_mut(), mods.hull_classes, |h| {
        h.id.as_str()
    });
    merge_by_id(game_data.engines_mut(), mods.engines, |e| e.id.as_str());
    merge_by_id(game_data.weapons_mut(), mods.weapons, |w| w.id.as_str());
    merge_by_id(game_data.shields_mut(), mods.shields, |s| s.id.as_str());
    merge_by_id(game_data.scanners_mut(), mods.scanners, |s| s.id.as_str());
    merge_by_id(game_data.special_modules_mut(), mods.special_modules, |s| {
        s.id.as_str()
    });
    merge_by_id(game_data.techs_mut(), mods.techs, |t| t.id.as_str());
    merge_by_id(game_data.victory_conditions_mut(), mods.victories, |v| {
        v.id.as_str()
    });
    merge_tech_edges(tech_edges, mods.tech_edges);
    if let Some(rules) = mods.victory_rules {
        game_data.set_victory_rules(rules);
    }
}

fn build_research_graph_with_fallback(tech_edges: &[TechEdge]) -> ResearchGraph {
    build_research_graph(tech_edges)
}

/// Load the full set of game data from the provided directory.
pub fn load_game_data<P: AsRef<Path>>(
    data_dir: P,
) -> Result<(GameData, GameRegistry), DataLoadError> {
    let base = data_dir.as_ref();

    let species_path = base.join("species.toml");
    let planet_sizes_path = base.join("planet_sizes.toml");
    let planet_surfaces_path = base.join("planet_surfaces.toml");
    let planetary_surface_path = base.join("planetary_buildings.toml");
    let planetary_orbital_path = base.join("planetary_satellites.toml");
    let planetary_projects_path = base.join("planetary_projects.toml");
    let hulls_path = base.join("ship_hulls.toml");
    let engines_path = base.join("ships_engines.toml");
    let weapons_path = base.join("ships_weapons.toml");
    let shields_path = base.join("ships_shields.toml");
    let scanners_path = base.join("ships_scanners.toml");
    let specials_path = base.join("ships_special.toml");
    let techs_path = base.join("research.toml");
    let tech_prereqs_path = base.join("research_prereqs.toml");
    let victories_path = base.join("victory_conditions.toml");
    let victory_rules_path = base.join("victory_rules.toml");
    let manifest_path = base.join("manifest.toml");
    let mods_dir = base
        .parent()
        .map(|p| p.join("mods"))
        .unwrap_or_else(|| PathBuf::from("assets/mods"));

    let species_data: SpeciesData = load_toml_file(&species_path)?;
    let planet_sizes: PlanetSizesData = load_toml_file(&planet_sizes_path)?;
    let planet_surfaces: PlanetSurfaceTypesData = load_toml_file(&planet_surfaces_path)?;
    let surface_data: PlanetarySurfaceData = load_toml_file(&planetary_surface_path)?;
    let orbital_data: PlanetaryOrbitalData = load_toml_file(&planetary_orbital_path)?;
    let projects_data: PlanetaryProjectsData = load_toml_file(&planetary_projects_path)?;
    let hull_data: HullClassesData = load_toml_file(&hulls_path)?;
    let engine_data: EnginesData = load_toml_file(&engines_path)?;
    let weapon_data: WeaponsData = load_toml_file(&weapons_path)?;
    let shield_data: ShieldsData = load_toml_file(&shields_path)?;
    let scanner_data: ScannersData = load_toml_file(&scanners_path)?;
    let specials_data: SpecialModulesData = load_toml_file(&specials_path)?;
    let techs_data: TechsData = load_toml_file(&techs_path)?;
    let tech_prereqs: TechEdgesData = load_toml_file(&tech_prereqs_path).unwrap_or(TechEdgesData {
        tech_edge: Vec::new(),
    });
    let victory_data: VictoryConditionsData = load_toml_file(&victories_path)?;
    let victory_rules: VictoryRules = load_toml_file(&victory_rules_path)?;
    let manifest = load_toml_file_optional::<DataManifest>(&manifest_path)?;

    let mod_datasets = load_mod_datasets(&mods_dir)?;

    let base_schema_version = manifest
        .as_ref()
        .and_then(|m| m.data_schema_version)
        .unwrap_or(super::DATA_SCHEMA_VERSION);
    if base_schema_version > super::DATA_SCHEMA_VERSION {
        return Err(DataLoadError::UnsupportedSchemaVersion {
            found: base_schema_version,
            current: super::DATA_SCHEMA_VERSION,
            path: manifest_path.display().to_string(),
        });
    }
    let effective_schema_version = base_schema_version.min(mod_datasets.min_schema_version);

    validate_tech_edges(&tech_prereqs.tech_edge, &techs_data.tech)?;
    let mut tech_edges = tech_prereqs.tech_edge;

    let mut game_data = GameData::new(
        species_data.species,
        planet_sizes.planet_size,
        planet_surfaces.planet_surface_type,
        surface_data.surface_item,
        orbital_data.orbital_item,
        projects_data.planetary_project,
        hull_data.hull_class,
        engine_data.engine,
        weapon_data.weapon,
        shield_data.shield,
        scanner_data.scanner,
        specials_data.special_module,
        techs_data.tech,
        build_research_graph_with_fallback(&tech_edges),
        victory_data.victory_condition,
        victory_rules,
    );

    apply_mods(&mut game_data, &mut tech_edges, mod_datasets);
    migrate_game_data(&mut game_data, &mut tech_edges, effective_schema_version)?;
    validate_tech_edges(&tech_edges, game_data.techs())?;
    game_data.set_research_graph(build_research_graph_with_fallback(&tech_edges));

    validate_game_data(&game_data)?;
    let registry = GameRegistry::from_game_data(&game_data)?;
    Ok((game_data, registry))
}
