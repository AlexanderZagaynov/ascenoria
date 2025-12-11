use std::fs;
use std::path::{Path, PathBuf};

use crate::data::entities::{
    Engine, HullClass, PlanetSize, PlanetSurfaceType, PlanetaryItem, PlanetaryProject, Scanner,
    Shield, SpecialModule, Species, Tech, TechEdge, VictoryCondition, VictoryRules, Weapon,
};
use crate::data::errors::DataLoadError;

use super::toml::load_toml_file_optional;
use super::wrappers::{
    EnginesData, HullClassesData, ModManifest, PlanetSizesData, PlanetSurfaceTypesData,
    PlanetaryOrbitalData, PlanetaryProjectsData, PlanetarySurfaceData, ScannersData, ShieldsData,
    SpecialModulesData, SpeciesData, TechEdgesData, TechsData, VictoryConditionsData, WeaponsData,
};

pub(crate) struct ModDatasets {
    pub species: Vec<Species>,
    pub planet_sizes: Vec<PlanetSize>,
    pub planet_surface_types: Vec<PlanetSurfaceType>,
    pub surface_items: Vec<PlanetaryItem>,
    pub orbital_items: Vec<PlanetaryItem>,
    pub planetary_projects: Vec<PlanetaryProject>,
    pub hull_classes: Vec<HullClass>,
    pub engines: Vec<Engine>,
    pub weapons: Vec<Weapon>,
    pub shields: Vec<Shield>,
    pub scanners: Vec<Scanner>,
    pub special_modules: Vec<SpecialModule>,
    pub techs: Vec<Tech>,
    pub tech_edges: Vec<TechEdge>,
    pub victories: Vec<VictoryCondition>,
    pub victory_rules: Option<VictoryRules>,
    pub min_schema_version: u32,
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
            min_schema_version: super::DATA_SCHEMA_VERSION,
        }
    }
}

pub(crate) fn load_mod_datasets(mods_dir: &Path) -> Result<ModDatasets, DataLoadError> {
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
            .unwrap_or(super::DATA_SCHEMA_VERSION);
        if schema_version > super::DATA_SCHEMA_VERSION {
            return Err(DataLoadError::UnsupportedSchemaVersion {
                found: schema_version,
                current: super::DATA_SCHEMA_VERSION,
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
