use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use bevy::prelude::Resource;
use serde::Deserialize;
use thiserror::Error;

/// Supported UI languages for localized strings.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Language {
    /// English localization.
    En,
    /// Russian localization.
    Ru,
}

impl Default for Language {
    fn default() -> Self {
        Language::En
    }
}

/// Localized text wrapper with English and Russian values.
#[derive(Debug, Clone, Deserialize)]
pub struct LocalizedText {
    /// English string.
    pub en: String,
    /// Russian string.
    pub ru: String,
}

impl LocalizedText {
    /// Resolve the text in the requested language.
    pub fn get(&self, language: Language) -> &str {
        match language {
            Language::En => &self.en,
            Language::Ru => &self.ru,
        }
    }
}

/// Trait for entities that expose localized name and description fields.
pub trait LocalizedEntity {
    /// Return the raw localized name fields.
    fn name_text(&self) -> &LocalizedText;
    /// Return the raw localized description fields.
    fn description_text(&self) -> &LocalizedText;

    /// Resolve the localized name.
    fn name(&self, language: Language) -> &str {
        self.name_text().get(language)
    }

    /// Resolve the localized description.
    fn description(&self, language: Language) -> &str {
        self.description_text().get(language)
    }
}

macro_rules! impl_localized_entity {
    ($type:ty) => {
        impl LocalizedEntity for $type {
            fn name_text(&self) -> &LocalizedText {
                &self.name
            }

            fn description_text(&self) -> &LocalizedText {
                &self.description
            }
        }
    };
}

/// Species definition used for selection UI and AI templates.
#[derive(Debug, Clone, Deserialize)]
pub struct Species {
    /// Stable identifier used by references and save games.
    pub id: String,
    /// Localized name for UI presentation.
    pub name: LocalizedText,
    /// Localized description text.
    pub description: LocalizedText,
}

#[derive(Debug, Deserialize)]
struct SpeciesData {
    species: Vec<Species>,
}

/// Planet size definition describing available slots.
#[derive(Debug, Clone, Deserialize)]
pub struct PlanetSize {
    /// Stable identifier.
    pub id: String,
    /// Localized name.
    pub name: LocalizedText,
    /// Localized description.
    pub description: LocalizedText,
    /// Available surface building slots.
    pub surface_slots: u32,
    /// Available orbital building slots.
    pub orbital_slots: u32,
}

/// Distribution of surface tiles by color.
#[derive(Debug, Clone, Deserialize)]
pub struct TileDistribution {
    /// Hostile tiles (black).
    pub black: u32,
    /// Neutral tiles (white).
    pub white: u32,
    /// Red tiles.
    pub red: u32,
    /// Green tiles.
    pub green: u32,
    /// Blue tiles.
    pub blue: u32,
}

/// Surface composition archetype for a planet.
#[derive(Debug, Clone, Deserialize)]
pub struct PlanetSurfaceType {
    /// Stable identifier.
    pub id: String,
    /// Localized name.
    pub name: LocalizedText,
    /// Localized description.
    pub description: LocalizedText,
    /// Percent distribution of tile colors.
    pub tile_distribution: TileDistribution,
}

#[derive(Debug, Deserialize)]
struct PlanetSizesData {
    planet_size: Vec<PlanetSize>,
}

#[derive(Debug, Deserialize)]
struct PlanetSurfaceTypesData {
    planet_surface_type: Vec<PlanetSurfaceType>,
}

/// Shared structure for both surface and orbital installations.
#[derive(Debug, Clone, Deserialize)]
pub struct PlanetaryItem {
    /// Stable identifier.
    pub id: String,
    /// Localized name.
    pub name: LocalizedText,
    /// Localized description.
    pub description: LocalizedText,
    /// Industry output bonus.
    pub industry_bonus: i32,
    /// Research output bonus.
    pub research_bonus: i32,
    /// Prosperity output bonus.
    pub prosperity_bonus: i32,
    /// Maximum population bonus.
    pub max_population_bonus: i32,
    /// Slot size requirement for surface tiles.
    pub slot_size: i32,
    /// Industry cost to build.
    pub industry_cost: i32,
    /// Tech index required to unlock.
    pub tech_index: i32,
}

#[derive(Debug, Deserialize)]
struct PlanetarySurfaceData {
    surface_item: Vec<PlanetaryItem>,
}

#[derive(Debug, Deserialize)]
struct PlanetaryOrbitalData {
    orbital_item: Vec<PlanetaryItem>,
}

/// Long-running planetary project definition.
#[derive(Debug, Clone, Deserialize)]
pub struct PlanetaryProject {
    /// Stable identifier.
    pub id: String,
    /// Localized name.
    pub name: LocalizedText,
    /// Localized description.
    pub description: LocalizedText,
    /// Industry cost to complete.
    pub industry_cost: i32,
}

#[derive(Debug, Deserialize)]
struct PlanetaryProjectsData {
    planetary_project: Vec<PlanetaryProject>,
}

/// Hull class template used by the ship designer.
#[derive(Debug, Clone, Deserialize)]
pub struct HullClass {
    /// Stable identifier.
    pub id: String,
    /// Localized name.
    pub name: LocalizedText,
    /// Localized description.
    pub description: LocalizedText,
    /// Size index used for balancing.
    pub size_index: i32,
    /// Maximum module count supported by the hull.
    pub max_items: i32,
}

#[derive(Debug, Deserialize)]
struct HullClassesData {
    hull_class: Vec<HullClass>,
}

/// Engine module definition.
#[derive(Debug, Clone, Deserialize)]
pub struct Engine {
    /// Stable identifier.
    pub id: String,
    /// Localized name.
    pub name: LocalizedText,
    /// Localized description.
    pub description: LocalizedText,
    /// Power draw of the engine.
    pub power_use: i32,
    /// Thrust rating used for movement calculations.
    pub thrust_rating: f32,
    /// Industry cost to build.
    pub industry_cost: i32,
}

#[derive(Debug, Deserialize)]
struct EnginesData {
    engine: Vec<Engine>,
}

/// Weapon module definition.
#[derive(Debug, Clone, Deserialize)]
pub struct Weapon {
    /// Stable identifier.
    pub id: String,
    /// Localized name.
    pub name: LocalizedText,
    /// Localized description.
    pub description: LocalizedText,
    /// Power draw.
    pub power_use: i32,
    /// Weapon range.
    pub range: i32,
    /// Damage strength.
    pub strength: f32,
    /// Uses per turn.
    pub uses_per_turn: i32,
    /// Industry cost to build.
    pub industry_cost: i32,
    /// Required tech index to unlock.
    pub tech_index: i32,
}

#[derive(Debug, Deserialize)]
struct WeaponsData {
    weapon: Vec<Weapon>,
}

/// Shield module definition.
#[derive(Debug, Clone, Deserialize)]
pub struct Shield {
    /// Stable identifier.
    pub id: String,
    /// Localized name.
    pub name: LocalizedText,
    /// Localized description.
    pub description: LocalizedText,
    /// Shield strength.
    pub strength: f32,
    /// Industry cost to build.
    pub industry_cost: i32,
}

#[derive(Debug, Deserialize)]
struct ShieldsData {
    shield: Vec<Shield>,
}

/// Scanner module definition.
#[derive(Debug, Clone, Deserialize)]
pub struct Scanner {
    /// Stable identifier.
    pub id: String,
    /// Localized name.
    pub name: LocalizedText,
    /// Localized description.
    pub description: LocalizedText,
    /// Scanner range.
    pub range: i32,
    /// Scanner strength.
    pub strength: f32,
    /// Industry cost to build.
    pub industry_cost: i32,
}

#[derive(Debug, Deserialize)]
struct ScannersData {
    scanner: Vec<Scanner>,
}

/// Special module definition with bespoke effects.
#[derive(Debug, Clone, Deserialize)]
pub struct SpecialModule {
    /// Stable identifier.
    pub id: String,
    /// Localized name.
    pub name: LocalizedText,
    /// Localized description.
    pub description: LocalizedText,
    /// Power draw of the special module.
    pub power_use: i32,
    /// Effective range.
    pub range: i32,
    /// Industry cost to build.
    pub industry_cost: i32,
}

#[derive(Debug, Deserialize)]
struct SpecialModulesData {
    special_module: Vec<SpecialModule>,
}

/// Technology entry with cost and localization.
#[derive(Debug, Clone, Deserialize)]
pub struct Tech {
    /// Stable identifier.
    pub id: String,
    /// Localized name.
    pub name: LocalizedText,
    /// Localized description.
    pub description: LocalizedText,
    /// Research cost to unlock the technology.
    pub research_cost: i32,
}

#[derive(Debug, Deserialize)]
struct TechsData {
    tech: Vec<Tech>,
}

/// Victory condition archetype.
#[derive(Debug, Clone, Deserialize)]
pub struct VictoryCondition {
    /// Stable identifier.
    pub id: String,
    /// Localized name.
    pub name: LocalizedText,
    /// Localized description.
    pub description: LocalizedText,
}

#[derive(Debug, Deserialize)]
struct VictoryConditionsData {
    victory_condition: Vec<VictoryCondition>,
}

impl_localized_entity!(Species);
impl_localized_entity!(PlanetSize);
impl_localized_entity!(PlanetSurfaceType);
impl_localized_entity!(PlanetaryItem);
impl_localized_entity!(PlanetaryProject);
impl_localized_entity!(HullClass);
impl_localized_entity!(Engine);
impl_localized_entity!(Weapon);
impl_localized_entity!(Shield);
impl_localized_entity!(Scanner);
impl_localized_entity!(SpecialModule);
impl_localized_entity!(Tech);
impl_localized_entity!(VictoryCondition);

/// Aggregated game data loaded from TOML assets.
#[derive(Debug, Resource)]
pub struct GameData {
    /// Playable and AI species.
    pub species: Vec<Species>,
    /// Planet size definitions.
    pub planet_sizes: Vec<PlanetSize>,
    /// Surface type distributions.
    pub planet_surface_types: Vec<PlanetSurfaceType>,
    /// Surface installation templates.
    pub surface_items: Vec<PlanetaryItem>,
    /// Orbital installation templates.
    pub orbital_items: Vec<PlanetaryItem>,
    /// Planetary projects definitions.
    pub planetary_projects: Vec<PlanetaryProject>,
    /// Hull classes available to the ship designer.
    pub hull_classes: Vec<HullClass>,
    /// Engine modules.
    pub engines: Vec<Engine>,
    /// Weapon modules.
    pub weapons: Vec<Weapon>,
    /// Shield modules.
    pub shields: Vec<Shield>,
    /// Scanner modules.
    pub scanners: Vec<Scanner>,
    /// Special modules.
    pub special_modules: Vec<SpecialModule>,
    /// Technologies.
    pub techs: Vec<Tech>,
    /// Victory condition archetypes.
    pub victory_conditions: Vec<VictoryCondition>,

    species_index: HashMap<String, usize>,
    hull_index: HashMap<String, usize>,
    tech_index: HashMap<String, usize>,
}

impl GameData {
    /// Resolve a species by identifier.
    pub fn species_by_id(&self, id: &str) -> Option<&Species> {
        self.species_index
            .get(id)
            .and_then(|index| self.species.get(*index))
    }

    /// Resolve a hull class by identifier.
    pub fn hull_class_by_id(&self, id: &str) -> Option<&HullClass> {
        self.hull_index
            .get(id)
            .and_then(|index| self.hull_classes.get(*index))
    }

    /// Resolve a tech by identifier.
    pub fn tech_by_id(&self, id: &str) -> Option<&Tech> {
        self.tech_index.get(id).and_then(|index| self.techs.get(*index))
    }

    fn build_indexes(&mut self) -> Result<(), DataLoadError> {
        self.species_index = build_index("species", &self.species, |s| &s.id)?;
        self.hull_index = build_index("hull_class", &self.hull_classes, |h| &h.id)?;
        self.tech_index = build_index("tech", &self.techs, |t| &t.id)?;
        Ok(())
    }

    fn validate(&self) -> Result<(), DataLoadError> {
        for size in &self.planet_sizes {
            validate_non_negative_fields(
                "planet_size",
                &size.id,
                &[
                    ("surface_slots", size.surface_slots as f64),
                    ("orbital_slots", size.orbital_slots as f64),
                ],
            )?;
        }

        for surface_type in &self.planet_surface_types {
            validate_tile_distribution("planet_surface_type", &surface_type.id, &surface_type.tile_distribution)?;
        }

        for item in &self.surface_items {
            validate_non_negative_fields(
                "surface_item",
                &item.id,
                &[
                    ("industry_bonus", item.industry_bonus as f64),
                    ("research_bonus", item.research_bonus as f64),
                    ("prosperity_bonus", item.prosperity_bonus as f64),
                    ("max_population_bonus", item.max_population_bonus as f64),
                    ("slot_size", item.slot_size as f64),
                    ("industry_cost", item.industry_cost as f64),
                    ("tech_index", item.tech_index as f64),
                ],
            )?;
        }

        for item in &self.orbital_items {
            validate_non_negative_fields(
                "orbital_item",
                &item.id,
                &[
                    ("industry_bonus", item.industry_bonus as f64),
                    ("research_bonus", item.research_bonus as f64),
                    ("prosperity_bonus", item.prosperity_bonus as f64),
                    ("max_population_bonus", item.max_population_bonus as f64),
                    ("slot_size", item.slot_size as f64),
                    ("industry_cost", item.industry_cost as f64),
                    ("tech_index", item.tech_index as f64),
                ],
            )?;
        }

        for project in &self.planetary_projects {
            validate_non_negative(
                "planetary_project",
                &project.id,
                "industry_cost",
                project.industry_cost as f64,
            )?;
        }

        for hull in &self.hull_classes {
            validate_non_negative_fields(
                "hull_class",
                &hull.id,
                &[("size_index", hull.size_index as f64), ("max_items", hull.max_items as f64)],
            )?;
        }

        for engine in &self.engines {
            validate_non_negative_fields(
                "engine",
                &engine.id,
                &[
                    ("power_use", engine.power_use as f64),
                    ("thrust_rating", engine.thrust_rating as f64),
                    ("industry_cost", engine.industry_cost as f64),
                ],
            )?;
        }

        for weapon in &self.weapons {
            validate_non_negative_fields(
                "weapon",
                &weapon.id,
                &[
                    ("power_use", weapon.power_use as f64),
                    ("range", weapon.range as f64),
                    ("strength", weapon.strength as f64),
                    ("uses_per_turn", weapon.uses_per_turn as f64),
                    ("industry_cost", weapon.industry_cost as f64),
                    ("tech_index", weapon.tech_index as f64),
                ],
            )?;
        }

        for shield in &self.shields {
            validate_non_negative_fields(
                "shield",
                &shield.id,
                &[("strength", shield.strength as f64), ("industry_cost", shield.industry_cost as f64)],
            )?;
        }

        for scanner in &self.scanners {
            validate_non_negative_fields(
                "scanner",
                &scanner.id,
                &[
                    ("range", scanner.range as f64),
                    ("strength", scanner.strength as f64),
                    ("industry_cost", scanner.industry_cost as f64),
                ],
            )?;
        }

        for module in &self.special_modules {
            validate_non_negative_fields(
                "special_module",
                &module.id,
                &[
                    ("power_use", module.power_use as f64),
                    ("range", module.range as f64),
                    ("industry_cost", module.industry_cost as f64),
                ],
            )?;
        }

        for tech in &self.techs {
            validate_non_negative("tech", &tech.id, "research_cost", tech.research_cost as f64)?;
        }

        Ok(())
    }
}

/// Errors that can occur while loading TOML game data.
#[derive(Debug, Error)]
pub enum DataLoadError {
    /// File read failure.
    #[error("Failed to read {path}: {source}")]
    Io {
        /// Source I/O error.
        source: std::io::Error,
        /// Path that failed.
        path: String,
    },
    /// TOML parse failure.
    #[error("Failed to parse {path}: {source}")]
    Parse {
        /// TOML parse error.
        source: toml::de::Error,
        /// Path that failed.
        path: String,
    },
    /// Duplicate identifier encountered.
    #[error("Duplicate {kind} id encountered: {id}")]
    DuplicateId {
        /// Entity type string.
        kind: &'static str,
        /// Duplicated identifier.
        id: String,
    },
    /// Validation failure.
    #[error("Validation failed for {kind} '{id}': {message}")]
    Validation {
        /// Entity type string.
        kind: &'static str,
        /// Identifier that failed validation.
        id: String,
        /// Validation error details.
        message: String,
    },
}

fn build_index<T, F>(kind: &'static str, items: &[T], id_fn: F) -> Result<HashMap<String, usize>, DataLoadError>
where
    F: Fn(&T) -> &str,
{
    let mut index = HashMap::new();
    for (i, item) in items.iter().enumerate() {
        let id = id_fn(item).to_string();
        if index.insert(id.clone(), i).is_some() {
            return Err(DataLoadError::DuplicateId { kind, id });
        }
    }
    Ok(index)
}

fn validate_non_negative(
    kind: &'static str,
    id: &str,
    field: &'static str,
    value: f64,
) -> Result<(), DataLoadError> {
    if value < 0.0 {
        return Err(DataLoadError::Validation {
            kind,
            id: id.to_string(),
            message: format!("{field} must be non-negative (got {value})"),
        });
    }

    Ok(())
}

fn validate_non_negative_fields(
    kind: &'static str,
    id: &str,
    fields: &[(&'static str, f64)],
) -> Result<(), DataLoadError> {
    for (field, value) in fields {
        validate_non_negative(kind, id, field, *value)?;
    }
    Ok(())
}

fn validate_tile_distribution(
    kind: &'static str,
    id: &str,
    distribution: &TileDistribution,
) -> Result<(), DataLoadError> {
    let total = distribution.black + distribution.white + distribution.red + distribution.green + distribution.blue;
    if total != 100 {
        return Err(DataLoadError::Validation {
            kind,
            id: id.to_string(),
            message: format!("tile_distribution must sum to 100 (got {total})"),
        });
    }
    Ok(())
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

/// Load the full set of game data from the provided directory.
pub fn load_game_data<P: AsRef<Path>>(data_dir: P) -> Result<GameData, DataLoadError> {
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
    let victories_path = base.join("victory_conditions.toml");

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
    let victory_data: VictoryConditionsData = load_toml_file(&victories_path)?;

    let mut game_data = GameData {
        species: species_data.species,
        planet_sizes: planet_sizes.planet_size,
        planet_surface_types: planet_surfaces.planet_surface_type,
        surface_items: surface_data.surface_item,
        orbital_items: orbital_data.orbital_item,
        planetary_projects: projects_data.planetary_project,
        hull_classes: hull_data.hull_class,
        engines: engine_data.engine,
        weapons: weapon_data.weapon,
        shields: shield_data.shield,
        scanners: scanner_data.scanner,
        special_modules: specials_data.special_module,
        techs: techs_data.tech,
        victory_conditions: victory_data.victory_condition,
        species_index: HashMap::new(),
        hull_index: HashMap::new(),
        tech_index: HashMap::new(),
    };

    game_data.validate()?;
    game_data.build_indexes()?;
    Ok(game_data)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn localized(text: &str) -> LocalizedText {
        LocalizedText {
            en: text.to_string(),
            ru: text.to_string(),
        }
    }

    fn base_game_data() -> GameData {
        GameData {
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
            victory_conditions: Vec::new(),
            species_index: HashMap::new(),
            hull_index: HashMap::new(),
            tech_index: HashMap::new(),
        }
    }

    #[test]
    fn loads_full_dataset() {
        let data = load_game_data(PathBuf::from("assets/data"))
            .expect("Game data should load from assets/data");

        assert!(!data.species.is_empty(), "Species list should not be empty");
        assert!(
            data.species_by_id("orfa").is_some(),
            "Species lookup should work"
        );
        assert!(
            !data.techs.is_empty(),
            "Tech list should be populated from research.toml"
        );
        assert!(
            data.hull_class_by_id("enormous").is_some(),
            "Hull class lookup should succeed for known ids"
        );
        assert!(
            !data.victory_conditions.is_empty(),
            "Victory conditions should load"
        );
    }

    #[test]
    fn rejects_duplicate_ids() {
        let mut data = base_game_data();
        data.species = vec![
            Species {
                id: "duplicate".to_string(),
                name: localized("Duplicate"),
                description: localized("Duplicate species"),
            },
            Species {
                id: "duplicate".to_string(),
                name: localized("Duplicate Two"),
                description: localized("Duplicate species"),
            },
        ];

        let error = data
            .build_indexes()
            .expect_err("Duplicate ids should be reported");

        match error {
            DataLoadError::DuplicateId { kind, id } => {
                assert_eq!(kind, "species");
                assert_eq!(id, "duplicate");
            }
            other => panic!("Unexpected error: {other:?}"),
        }
    }

    #[test]
    fn rejects_negative_values() {
        let mut data = base_game_data();
        data.hull_classes.push(HullClass {
            id: "bad_hull".to_string(),
            name: localized("Bad"),
            description: localized("Bad"),
            size_index: -1,
            max_items: 2,
        });

        let error = data.validate().expect_err("Negative values should fail validation");
        match error {
            DataLoadError::Validation { kind, id, .. } => {
                assert_eq!(kind, "hull_class");
                assert_eq!(id, "bad_hull");
            }
            other => panic!("Unexpected error: {other:?}"),
        }
    }

    #[test]
    fn localized_entity_helpers_resolve_language() {
        let hull = HullClass {
            id: "frigate".to_string(),
            name: LocalizedText {
                en: "Frigate".to_string(),
                ru: "Фрегат".to_string(),
            },
            description: LocalizedText {
                en: "Light hull".to_string(),
                ru: "Легкий корпус".to_string(),
            },
            size_index: 1,
            max_items: 4,
        };

        assert_eq!(hull.name(Language::En), "Frigate");
        assert_eq!(hull.name(Language::Ru), "Фрегат");
        assert_eq!(hull.description(Language::En), "Light hull");
        assert_eq!(hull.description(Language::Ru), "Легкий корпус");
    }
}
