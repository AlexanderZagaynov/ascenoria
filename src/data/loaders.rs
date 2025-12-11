//! TOML loading functions for game data.
//!
//! This module contains the `load_game_data` entry point that reads all TOML files
//! from the data directory and mods, merges them, and returns a validated `GameData`
//! with a corresponding `GameRegistry`.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use super::entities::{
    Engine, HullClass, PlanetSize, PlanetSurfaceType, PlanetaryItem, PlanetaryProject,
    ResearchGraph, Scanner, Shield, SpecialModule, Species, Tech, TechEdge, VictoryCondition,
    VictoryRules, Weapon,
};
use super::errors::DataLoadError;
use super::game_data::GameData;
use super::registry::GameRegistry;
use super::validation::{validate_game_data, validate_tech_edges};

/// Current schema version supported by the loader.
pub const DATA_SCHEMA_VERSION: u32 = 1;

// ---------------------------------------------------------------------------
// TOML wrapper types for deserializing arrays.
// ---------------------------------------------------------------------------

#[derive(Deserialize)]
pub(crate) struct SpeciesData {
    pub species: Vec<Species>,
}

#[derive(Deserialize)]
pub(crate) struct PlanetSizesData {
    pub planet_size: Vec<PlanetSize>,
}

#[derive(Deserialize)]
pub(crate) struct PlanetSurfaceTypesData {
    pub planet_surface_type: Vec<PlanetSurfaceType>,
}

#[derive(Deserialize)]
pub(crate) struct PlanetarySurfaceData {
    pub surface_item: Vec<PlanetaryItem>,
}

#[derive(Deserialize)]
pub(crate) struct PlanetaryOrbitalData {
    pub orbital_item: Vec<PlanetaryItem>,
}

#[derive(Deserialize)]
pub(crate) struct PlanetaryProjectsData {
    pub planetary_project: Vec<PlanetaryProject>,
}

#[derive(Deserialize)]
pub(crate) struct HullClassesData {
    pub hull_class: Vec<HullClass>,
}

#[derive(Deserialize)]
pub(crate) struct EnginesData {
    pub engine: Vec<Engine>,
}

#[derive(Deserialize)]
pub(crate) struct WeaponsData {
    pub weapon: Vec<Weapon>,
}

#[derive(Deserialize)]
pub(crate) struct ShieldsData {
    pub shield: Vec<Shield>,
}

#[derive(Deserialize)]
pub(crate) struct ScannersData {
    pub scanner: Vec<Scanner>,
}

#[derive(Deserialize)]
pub(crate) struct SpecialModulesData {
    pub special_module: Vec<SpecialModule>,
}

#[derive(Deserialize)]
pub(crate) struct TechsData {
    pub tech: Vec<Tech>,
}

#[derive(Deserialize)]
pub(crate) struct TechEdgesData {
    pub tech_edge: Vec<TechEdge>,
}

#[derive(Deserialize)]
pub(crate) struct VictoryConditionsData {
    pub victory_condition: Vec<VictoryCondition>,
}

#[derive(Deserialize)]
struct DataManifest {
    data_schema_version: Option<u32>,
}

#[derive(Default, Deserialize)]
struct ModManifest {
    #[serde(default)]
    priority: i32,
    data_schema_version: Option<u32>,
}

// ---------------------------------------------------------------------------
// Mod loading helpers.
// ---------------------------------------------------------------------------

struct ModDatasets {
    species: Vec<Species>,
    planet_sizes: Vec<PlanetSize>,
    planet_surface_types: Vec<PlanetSurfaceType>,
    surface_items: Vec<PlanetaryItem>,
    orbital_items: Vec<PlanetaryItem>,
    planetary_projects: Vec<PlanetaryProject>,
    hull_classes: Vec<HullClass>,
    engines: Vec<Engine>,
    weapons: Vec<Weapon>,
    shields: Vec<Shield>,
    scanners: Vec<Scanner>,
    special_modules: Vec<SpecialModule>,
    techs: Vec<Tech>,
    tech_edges: Vec<TechEdge>,
    victories: Vec<VictoryCondition>,
    victory_rules: Option<VictoryRules>,
    min_schema_version: u32,
}

impl Default for ModDatasets {
    fn default() -> Self {
        Self {
            species: Vec::new(),
            planet_sizes: Vec::new(),
            planet_surface_types: Vec::new(),
            surface_items: Vec::new(),
            orbital_items: Vec::new(),
            planetary_projects: Vec::new(),
            hull_classes: Vec::new(),
            engines: Vec::new(),
            weapons: Vec::new(),
            shields: Vec::new(),
            scanners: Vec::new(),
            special_modules: Vec::new(),
            techs: Vec::new(),
            tech_edges: Vec::new(),
            victories: Vec::new(),
            victory_rules: None,
            min_schema_version: DATA_SCHEMA_VERSION,
        }
    }
}

fn load_toml_file<T>(path: &Path) -> Result<T, DataLoadError>
where
    T: for<'de> Deserialize<'de>,
{
    let content = fs::read_to_string(path).map_err(|source| DataLoadError::Io {
        source,
        path: path.display().to_string(),
    })?;

    toml::from_str::<T>(&content).map_err(|source| DataLoadError::Parse {
        source,
        path: path.display().to_string(),
    })
}

fn load_toml_file_optional<T>(path: &Path) -> Result<Option<T>, DataLoadError>
where
    T: for<'de> Deserialize<'de>,
{
    match fs::read_to_string(path) {
        Ok(content) => {
            toml::from_str::<T>(&content)
                .map(Some)
                .map_err(|source| DataLoadError::Parse {
                    source,
                    path: path.display().to_string(),
                })
        }
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(source) => Err(DataLoadError::Io {
            source,
            path: path.display().to_string(),
        }),
    }
}

fn merge_by_id<T, F>(base: &mut Vec<T>, mods: Vec<T>, id_fn: F)
where
    F: Fn(&T) -> &str,
{
    for item in mods {
        let id = id_fn(&item);
        if let Some(pos) = base.iter().position(|b| id_fn(b) == id) {
            base[pos] = item;
        } else {
            base.push(item);
        }
    }
}

fn merge_tech_edges(base: &mut Vec<TechEdge>, mods: Vec<TechEdge>) {
    if mods.is_empty() {
        return;
    }

    let mut merged: BTreeMap<(String, String), TechEdge> = BTreeMap::new();
    for edge in base.drain(..) {
        merged.insert((edge.from.clone(), edge.to.clone()), edge);
    }
    for edge in mods {
        merged.insert((edge.from.clone(), edge.to.clone()), edge);
    }
    *base = merged.into_values().collect();
}

fn load_mod_datasets(mods_dir: &Path) -> Result<ModDatasets, DataLoadError> {
    let mut datasets = ModDatasets::default();
    let entries = match fs::read_dir(mods_dir) {
        Ok(entries) => entries,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(datasets),
        Err(source) => {
            return Err(DataLoadError::Io {
                source,
                path: mods_dir.display().to_string(),
            });
        }
    };

    let mut mods: Vec<(i32, String, PathBuf)> = Vec::new();
    for entry in entries {
        let Ok(entry) = entry else {
            continue;
        };
        let Ok(file_type) = entry.file_type() else {
            continue;
        };
        if !file_type.is_dir() {
            continue;
        }

        let mod_dir = entry.path();
        let manifest = load_toml_file_optional::<ModManifest>(&mod_dir.join("mod.toml"))?;
        let priority = manifest.as_ref().map(|m| m.priority).unwrap_or_default();
        let schema_version = manifest
            .as_ref()
            .and_then(|m| m.data_schema_version)
            .unwrap_or(DATA_SCHEMA_VERSION);
        if schema_version > DATA_SCHEMA_VERSION {
            return Err(DataLoadError::UnsupportedSchemaVersion {
                found: schema_version,
                current: DATA_SCHEMA_VERSION,
                path: mod_dir.display().to_string(),
            });
        }
        datasets.min_schema_version = datasets.min_schema_version.min(schema_version);
        let name = entry.file_name().to_string_lossy().to_string();
        mods.push((priority, name, mod_dir));
    }

    mods.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));

    for (_, _, mod_dir) in mods {
        let data_dir = mod_dir.join("data");
        if !data_dir.is_dir() {
            continue;
        }
        if let Some(data) = load_toml_file_optional::<SpeciesData>(&data_dir.join("species.toml"))?
        {
            datasets.species.extend(data.species);
        }
        if let Some(data) =
            load_toml_file_optional::<PlanetSizesData>(&data_dir.join("planet_sizes.toml"))?
        {
            datasets.planet_sizes.extend(data.planet_size);
        }
        if let Some(data) = load_toml_file_optional::<PlanetSurfaceTypesData>(
            &data_dir.join("planet_surfaces.toml"),
        )? {
            datasets
                .planet_surface_types
                .extend(data.planet_surface_type);
        }
        if let Some(data) = load_toml_file_optional::<PlanetarySurfaceData>(
            &data_dir.join("planetary_buildings.toml"),
        )? {
            datasets.surface_items.extend(data.surface_item);
        }
        if let Some(data) = load_toml_file_optional::<PlanetaryOrbitalData>(
            &data_dir.join("planetary_satellites.toml"),
        )? {
            datasets.orbital_items.extend(data.orbital_item);
        }
        if let Some(data) = load_toml_file_optional::<PlanetaryProjectsData>(
            &data_dir.join("planetary_projects.toml"),
        )? {
            datasets.planetary_projects.extend(data.planetary_project);
        }
        if let Some(data) =
            load_toml_file_optional::<HullClassesData>(&data_dir.join("ship_hulls.toml"))?
        {
            datasets.hull_classes.extend(data.hull_class);
        }
        if let Some(data) =
            load_toml_file_optional::<EnginesData>(&data_dir.join("ships_engines.toml"))?
        {
            datasets.engines.extend(data.engine);
        }
        if let Some(data) =
            load_toml_file_optional::<WeaponsData>(&data_dir.join("ships_weapons.toml"))?
        {
            datasets.weapons.extend(data.weapon);
        }
        if let Some(data) =
            load_toml_file_optional::<ShieldsData>(&data_dir.join("ships_shields.toml"))?
        {
            datasets.shields.extend(data.shield);
        }
        if let Some(data) =
            load_toml_file_optional::<ScannersData>(&data_dir.join("ships_scanners.toml"))?
        {
            datasets.scanners.extend(data.scanner);
        }
        if let Some(data) =
            load_toml_file_optional::<SpecialModulesData>(&data_dir.join("ships_special.toml"))?
        {
            datasets.special_modules.extend(data.special_module);
        }
        if let Some(data) = load_toml_file_optional::<TechsData>(&data_dir.join("research.toml"))? {
            datasets.techs.extend(data.tech);
        }
        if let Some(data) =
            load_toml_file_optional::<TechEdgesData>(&data_dir.join("research_prereqs.toml"))?
        {
            datasets.tech_edges.extend(data.tech_edge);
        }
        if let Some(data) = load_toml_file_optional::<VictoryConditionsData>(
            &data_dir.join("victory_conditions.toml"),
        )? {
            datasets.victories.extend(data.victory_condition);
        }
        if let Some(rules) =
            load_toml_file_optional::<VictoryRules>(&data_dir.join("victory_rules.toml"))?
        {
            datasets.victory_rules = Some(rules);
        }
    }

    Ok(datasets)
}

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

fn migrate_game_data(
    _game_data: &mut GameData,
    _tech_edges: &mut Vec<TechEdge>,
    from_version: u32,
) -> Result<(), DataLoadError> {
    if from_version > DATA_SCHEMA_VERSION {
        return Err(DataLoadError::UnsupportedSchemaVersion {
            found: from_version,
            current: DATA_SCHEMA_VERSION,
            path: "manifest".to_string(),
        });
    }

    // No migrations are required yet. Future schema changes can be added here.
    Ok(())
}

fn build_research_graph(edges: &[TechEdge]) -> ResearchGraph {
    let mut graph = ResearchGraph::default();
    for edge in edges {
        graph
            .prereqs
            .entry(edge.to.clone())
            .or_default()
            .push(edge.from.clone());
        graph
            .unlocks
            .entry(edge.from.clone())
            .or_default()
            .push(edge.to.clone());
    }
    graph
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
        .unwrap_or(DATA_SCHEMA_VERSION);
    if base_schema_version > DATA_SCHEMA_VERSION {
        return Err(DataLoadError::UnsupportedSchemaVersion {
            found: base_schema_version,
            current: DATA_SCHEMA_VERSION,
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
        build_research_graph(&tech_edges),
        victory_data.victory_condition,
        victory_rules,
    );

    apply_mods(&mut game_data, &mut tech_edges, mod_datasets);
    migrate_game_data(&mut game_data, &mut tech_edges, effective_schema_version)?;
    validate_tech_edges(&tech_edges, game_data.techs())?;
    game_data.set_research_graph(build_research_graph(&tech_edges));

    validate_game_data(&game_data)?;
    let registry = GameRegistry::from_game_data(&game_data)?;
    Ok((game_data, registry))
}
