use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::path::{Path, PathBuf};

use bevy::prelude::Resource;
use serde::Deserialize;
use thiserror::Error;

/// Sentinel value indicating no technology requirement.
pub const NO_TECH_REQUIREMENT: i32 = 255;
/// Current schema version for TOML game data files.
pub const DATA_SCHEMA_VERSION: u32 = 1;

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
}

/// Trait for entities with a stable identifier.
pub trait HasId {
    /// Borrow the identifier as a string slice.
    fn id(&self) -> &str;
}

/// Trait for entities that expose a localized name.
pub trait NamedEntity: LocalizedEntity {
    /// Resolve the localized name.
    fn name(&self, language: Language) -> &str {
        self.name_text().get(language)
    }
}

/// Trait for entities that expose a localized description.
pub trait HasDescription: LocalizedEntity {
    /// Resolve the localized description.
    fn description(&self, language: Language) -> &str {
        self.description_text().get(language)
    }
}

impl<T: LocalizedEntity> NamedEntity for T {}
impl<T: LocalizedEntity> HasDescription for T {}

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

macro_rules! impl_has_id {
    ($type:ty) => {
        impl HasId for $type {
            fn id(&self) -> &str {
                &self.id
            }
        }
    };
}

macro_rules! define_id_type {
    ($name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct $name(pub String);

        impl From<&str> for $name {
            fn from(value: &str) -> Self {
                Self(value.to_string())
            }
        }

        impl From<String> for $name {
            fn from(value: String) -> Self {
                Self(value)
            }
        }

        impl From<$name> for String {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl $name {
            /// Borrow the underlying identifier as a string slice.
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }
    };
}

define_id_type!(SpeciesId);
define_id_type!(PlanetSizeId);
define_id_type!(PlanetSurfaceTypeId);
define_id_type!(PlanetaryItemId);
define_id_type!(PlanetaryProjectId);
define_id_type!(HullClassId);
define_id_type!(EngineId);
define_id_type!(WeaponId);
define_id_type!(ShieldId);
define_id_type!(ScannerId);
define_id_type!(SpecialModuleId);
define_id_type!(TechId);
define_id_type!(VictoryConditionId);

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

/// Edge between technologies (from prerequisite to unlock).
#[derive(Debug, Clone, Deserialize)]
pub struct TechEdge {
    /// Prerequisite tech id.
    pub from: String,
    /// Dependent tech id.
    pub to: String,
}

#[derive(Debug, Deserialize)]
struct TechEdgesData {
    tech_edge: Vec<TechEdge>,
}

/// Research graph mapping prerequisites and unlocks.
#[derive(Debug, Default)]
pub struct ResearchGraph {
    prereqs: HashMap<String, Vec<String>>,
    unlocks: HashMap<String, Vec<String>>,
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

/// Tunable parameters for victory checks.
#[derive(Debug, Clone, Deserialize)]
pub struct VictoryRules {
    /// Fraction of systems required to claim domination.
    pub domination_threshold: f32,
}

impl Default for VictoryRules {
    fn default() -> Self {
        Self {
            domination_threshold: 0.5,
        }
    }
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

impl_has_id!(Species);
impl_has_id!(PlanetSize);
impl_has_id!(PlanetSurfaceType);
impl_has_id!(PlanetaryItem);
impl_has_id!(PlanetaryProject);
impl_has_id!(HullClass);
impl_has_id!(Engine);
impl_has_id!(Weapon);
impl_has_id!(Shield);
impl_has_id!(Scanner);
impl_has_id!(SpecialModule);
impl_has_id!(Tech);
impl_has_id!(VictoryCondition);

/// Derived stats for weapon modules.
#[derive(Debug, Clone, Copy)]
pub struct WeaponComputed {
    /// Approximate damage per turn for the weapon.
    pub dps: f32,
}

/// Derived stats for engines.
#[derive(Debug, Clone, Copy)]
pub struct EngineComputed {
    /// Optional thrust efficiency (thrust per power unit).
    pub efficiency: Option<f32>,
}

/// Aggregated bonuses for planetary items.
#[derive(Debug, Clone, Copy)]
pub struct PlanetaryItemComputed {
    /// Sum of all additive bonuses the building provides.
    pub total_bonus: i32,
}

/// Cache of derived stats to keep calculations centralized.
#[derive(Debug, Resource, Default)]
pub struct GameDataComputed {
    /// Derived stats for weapons, keyed by identifier.
    pub weapon_stats: HashMap<String, WeaponComputed>,
    /// Derived stats for engines, keyed by identifier.
    pub engine_stats: HashMap<String, EngineComputed>,
    /// Derived bonuses for surface items, keyed by identifier.
    pub surface_item_stats: HashMap<String, PlanetaryItemComputed>,
    /// Derived bonuses for orbital items, keyed by identifier.
    pub orbital_item_stats: HashMap<String, PlanetaryItemComputed>,
}

/// Aggregated game data loaded from TOML assets.
#[derive(Debug, Resource)]
pub struct GameData {
    /// Playable and AI species.
    species: Vec<Species>,
    /// Planet size definitions.
    planet_sizes: Vec<PlanetSize>,
    /// Surface type distributions.
    planet_surface_types: Vec<PlanetSurfaceType>,
    /// Surface installation templates.
    surface_items: Vec<PlanetaryItem>,
    /// Orbital installation templates.
    orbital_items: Vec<PlanetaryItem>,
    /// Planetary projects definitions.
    planetary_projects: Vec<PlanetaryProject>,
    /// Hull classes available to the ship designer.
    hull_classes: Vec<HullClass>,
    /// Engine modules.
    engines: Vec<Engine>,
    /// Weapon modules.
    weapons: Vec<Weapon>,
    /// Shield modules.
    shields: Vec<Shield>,
    /// Scanner modules.
    scanners: Vec<Scanner>,
    /// Special modules.
    special_modules: Vec<SpecialModule>,
    /// Technologies.
    techs: Vec<Tech>,
    /// Research graph edges.
    research_graph: ResearchGraph,
    /// Victory condition archetypes.
    victory_conditions: Vec<VictoryCondition>,
    /// Tunable parameters for victory checks.
    victory_rules: VictoryRules,
}

/// Read-only lookup tables for game data identifiers.
#[derive(Debug, Resource, Default)]
pub struct GameRegistry {
    species_by_id: HashMap<SpeciesId, usize>,
    planet_size_by_id: HashMap<PlanetSizeId, usize>,
    planet_surface_type_by_id: HashMap<PlanetSurfaceTypeId, usize>,
    surface_item_by_id: HashMap<PlanetaryItemId, usize>,
    orbital_item_by_id: HashMap<PlanetaryItemId, usize>,
    planetary_project_by_id: HashMap<PlanetaryProjectId, usize>,
    hull_class_by_id: HashMap<HullClassId, usize>,
    engine_by_id: HashMap<EngineId, usize>,
    weapon_by_id: HashMap<WeaponId, usize>,
    shield_by_id: HashMap<ShieldId, usize>,
    scanner_by_id: HashMap<ScannerId, usize>,
    special_module_by_id: HashMap<SpecialModuleId, usize>,
    tech_by_id: HashMap<TechId, usize>,
    victory_condition_by_id: HashMap<VictoryConditionId, usize>,
}

impl GameData {
    /// Get all species definitions.
    pub fn species(&self) -> &[Species] {
        &self.species
    }

    /// Get all planet size archetypes.
    pub fn planet_sizes(&self) -> &[PlanetSize] {
        &self.planet_sizes
    }

    /// Get surface composition distributions.
    pub fn planet_surface_types(&self) -> &[PlanetSurfaceType] {
        &self.planet_surface_types
    }

    /// Get all surface building templates.
    pub fn surface_items(&self) -> &[PlanetaryItem] {
        &self.surface_items
    }

    /// Get all orbital building templates.
    pub fn orbital_items(&self) -> &[PlanetaryItem] {
        &self.orbital_items
    }

    /// Get planetary project definitions.
    pub fn planetary_projects(&self) -> &[PlanetaryProject] {
        &self.planetary_projects
    }

    /// Get ship hull templates.
    pub fn hull_classes(&self) -> &[HullClass] {
        &self.hull_classes
    }

    /// Get engine module definitions.
    pub fn engines(&self) -> &[Engine] {
        &self.engines
    }

    /// Get weapon module definitions.
    pub fn weapons(&self) -> &[Weapon] {
        &self.weapons
    }

    /// Get shield module definitions.
    pub fn shields(&self) -> &[Shield] {
        &self.shields
    }

    /// Get scanner module definitions.
    pub fn scanners(&self) -> &[Scanner] {
        &self.scanners
    }

    /// Get special module definitions.
    pub fn special_modules(&self) -> &[SpecialModule] {
        &self.special_modules
    }

    /// Get all technologies.
    pub fn techs(&self) -> &[Tech] {
        &self.techs
    }

    /// Get prerequisites for a technology id.
    pub fn tech_prereqs(&self, id: &str) -> &[String] {
        self.research_graph
            .prereqs
            .get(id)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    /// Get unlocks for a technology id.
    pub fn tech_unlocks(&self, id: &str) -> &[String] {
        self.research_graph
            .unlocks
            .get(id)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    /// Get victory condition archetypes.
    pub fn victory_conditions(&self) -> &[VictoryCondition] {
        &self.victory_conditions
    }

    /// Get victory rules configuration.
    pub fn victory_rules(&self) -> &VictoryRules {
        &self.victory_rules
    }

    /// Compute derived stats for frequently used entities.
    pub fn compute(&self) -> GameDataComputed {
        let mut weapon_stats = HashMap::new();
        for weapon in &self.weapons {
            weapon_stats.insert(
                weapon.id.clone(),
                WeaponComputed {
                    dps: weapon.strength * weapon.uses_per_turn as f32,
                },
            );
        }

        let mut engine_stats = HashMap::new();
        for engine in &self.engines {
            let efficiency = if engine.power_use > 0 {
                Some(engine.thrust_rating / engine.power_use as f32)
            } else {
                None
            };

            engine_stats.insert(engine.id.clone(), EngineComputed { efficiency });
        }

        let compute_item = |item: &PlanetaryItem| PlanetaryItemComputed {
            total_bonus: item.industry_bonus
                + item.research_bonus
                + item.prosperity_bonus
                + item.max_population_bonus,
        };

        let surface_item_stats = self
            .surface_items
            .iter()
            .map(|item| (item.id.clone(), compute_item(item)))
            .collect();

        let orbital_item_stats = self
            .orbital_items
            .iter()
            .map(|item| (item.id.clone(), compute_item(item)))
            .collect();

        GameDataComputed {
            weapon_stats,
            engine_stats,
            surface_item_stats,
            orbital_item_stats,
        }
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
            validate_tile_distribution(
                "planet_surface_type",
                &surface_type.id,
                &surface_type.tile_distribution,
            )?;
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
            validate_positive("surface_item", &item.id, "slot_size", item.slot_size as f64)?;
            validate_tech_reference("surface_item", &item.id, item.tech_index, self.techs.len())?;
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
            validate_positive("orbital_item", &item.id, "slot_size", item.slot_size as f64)?;
            validate_tech_reference("orbital_item", &item.id, item.tech_index, self.techs.len())?;
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
            validate_positive("hull_class", &hull.id, "size_index", hull.size_index as f64)?;
            validate_positive("hull_class", &hull.id, "max_items", hull.max_items as f64)?;
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
            validate_positive(
                "engine",
                &engine.id,
                "thrust_rating",
                engine.thrust_rating as f64,
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
            validate_positive("weapon", &weapon.id, "range", weapon.range as f64)?;
            validate_positive(
                "weapon",
                &weapon.id,
                "uses_per_turn",
                weapon.uses_per_turn as f64,
            )?;
            validate_tech_reference("weapon", &weapon.id, weapon.tech_index, self.techs.len())?;
        }

        for shield in &self.shields {
            validate_non_negative_fields(
                "shield",
                &shield.id,
                &[
                    ("strength", shield.strength as f64),
                    ("industry_cost", shield.industry_cost as f64),
                ],
            )?;
            validate_positive("shield", &shield.id, "strength", shield.strength as f64)?;
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
            validate_positive("scanner", &scanner.id, "range", scanner.range as f64)?;
            validate_positive("scanner", &scanner.id, "strength", scanner.strength as f64)?;
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

        if !(0.0..=1.0).contains(&self.victory_rules.domination_threshold) {
            return Err(DataLoadError::Validation {
                kind: "victory_rules",
                id: "domination_threshold".to_string(),
                message: "domination_threshold must be between 0.0 and 1.0".to_string(),
            });
        }

        Ok(())
    }
}

impl GameRegistry {
    fn resolve<'a, I: Eq + std::hash::Hash, T>(
        items: &'a [T],
        index: &HashMap<I, usize>,
        id: impl Into<I>,
    ) -> Option<&'a T> {
        index.get(&id.into()).and_then(|idx| items.get(*idx))
    }

    /// Build lookup tables from loaded game data.
    pub fn from_game_data(data: &GameData) -> Result<Self, DataLoadError> {
        Ok(Self {
            species_by_id: build_typed_index("species", &data.species, |s| {
                SpeciesId::from(s.id.clone())
            })?,
            planet_size_by_id: build_typed_index("planet_size", &data.planet_sizes, |p| {
                PlanetSizeId::from(p.id.clone())
            })?,
            planet_surface_type_by_id: build_typed_index(
                "planet_surface_type",
                &data.planet_surface_types,
                |p| PlanetSurfaceTypeId::from(p.id.clone()),
            )?,
            surface_item_by_id: build_typed_index("surface_item", &data.surface_items, |i| {
                PlanetaryItemId::from(i.id.clone())
            })?,
            orbital_item_by_id: build_typed_index("orbital_item", &data.orbital_items, |i| {
                PlanetaryItemId::from(i.id.clone())
            })?,
            planetary_project_by_id: build_typed_index(
                "planetary_project",
                &data.planetary_projects,
                |p| PlanetaryProjectId::from(p.id.clone()),
            )?,
            hull_class_by_id: build_typed_index("hull_class", &data.hull_classes, |h| {
                HullClassId::from(h.id.clone())
            })?,
            engine_by_id: build_typed_index("engine", &data.engines, |e| {
                EngineId::from(e.id.clone())
            })?,
            weapon_by_id: build_typed_index("weapon", &data.weapons, |w| {
                WeaponId::from(w.id.clone())
            })?,
            shield_by_id: build_typed_index("shield", &data.shields, |s| {
                ShieldId::from(s.id.clone())
            })?,
            scanner_by_id: build_typed_index("scanner", &data.scanners, |s| {
                ScannerId::from(s.id.clone())
            })?,
            special_module_by_id: build_typed_index(
                "special_module",
                &data.special_modules,
                |s| SpecialModuleId::from(s.id.clone()),
            )?,
            tech_by_id: build_typed_index("tech", &data.techs, |t| TechId::from(t.id.clone()))?,
            victory_condition_by_id: build_typed_index(
                "victory_condition",
                &data.victory_conditions,
                |v| VictoryConditionId::from(v.id.clone()),
            )?,
        })
    }

    /// Resolve a species by identifier.
    pub fn species<'a>(&self, data: &'a GameData, id: impl Into<SpeciesId>) -> Option<&'a Species> {
        Self::resolve(&data.species, &self.species_by_id, id)
    }

    /// Resolve a planet size by identifier.
    pub fn planet_size<'a>(
        &self,
        data: &'a GameData,
        id: impl Into<PlanetSizeId>,
    ) -> Option<&'a PlanetSize> {
        Self::resolve(&data.planet_sizes, &self.planet_size_by_id, id)
    }

    /// Resolve a planet surface type by identifier.
    pub fn planet_surface_type<'a>(
        &self,
        data: &'a GameData,
        id: impl Into<PlanetSurfaceTypeId>,
    ) -> Option<&'a PlanetSurfaceType> {
        Self::resolve(
            &data.planet_surface_types,
            &self.planet_surface_type_by_id,
            id,
        )
    }

    /// Resolve a surface building by identifier.
    pub fn surface_item<'a>(
        &self,
        data: &'a GameData,
        id: impl Into<PlanetaryItemId>,
    ) -> Option<&'a PlanetaryItem> {
        Self::resolve(&data.surface_items, &self.surface_item_by_id, id)
    }

    /// Resolve an orbital building by identifier.
    pub fn orbital_item<'a>(
        &self,
        data: &'a GameData,
        id: impl Into<PlanetaryItemId>,
    ) -> Option<&'a PlanetaryItem> {
        Self::resolve(&data.orbital_items, &self.orbital_item_by_id, id)
    }

    /// Resolve a planetary project by identifier.
    pub fn planetary_project<'a>(
        &self,
        data: &'a GameData,
        id: impl Into<PlanetaryProjectId>,
    ) -> Option<&'a PlanetaryProject> {
        Self::resolve(&data.planetary_projects, &self.planetary_project_by_id, id)
    }

    /// Resolve a hull class by identifier.
    pub fn hull_class<'a>(
        &self,
        data: &'a GameData,
        id: impl Into<HullClassId>,
    ) -> Option<&'a HullClass> {
        Self::resolve(&data.hull_classes, &self.hull_class_by_id, id)
    }

    /// Resolve an engine by identifier.
    pub fn engine<'a>(&self, data: &'a GameData, id: impl Into<EngineId>) -> Option<&'a Engine> {
        Self::resolve(&data.engines, &self.engine_by_id, id)
    }

    /// Resolve a weapon by identifier.
    pub fn weapon<'a>(&self, data: &'a GameData, id: impl Into<WeaponId>) -> Option<&'a Weapon> {
        Self::resolve(&data.weapons, &self.weapon_by_id, id)
    }

    /// Resolve a shield by identifier.
    pub fn shield<'a>(&self, data: &'a GameData, id: impl Into<ShieldId>) -> Option<&'a Shield> {
        Self::resolve(&data.shields, &self.shield_by_id, id)
    }

    /// Resolve a scanner by identifier.
    pub fn scanner<'a>(&self, data: &'a GameData, id: impl Into<ScannerId>) -> Option<&'a Scanner> {
        Self::resolve(&data.scanners, &self.scanner_by_id, id)
    }

    /// Resolve a special module by identifier.
    pub fn special_module<'a>(
        &self,
        data: &'a GameData,
        id: impl Into<SpecialModuleId>,
    ) -> Option<&'a SpecialModule> {
        Self::resolve(&data.special_modules, &self.special_module_by_id, id)
    }

    /// Resolve a technology by identifier.
    pub fn tech<'a>(&self, data: &'a GameData, id: impl Into<TechId>) -> Option<&'a Tech> {
        Self::resolve(&data.techs, &self.tech_by_id, id)
    }

    /// Resolve a victory condition by identifier.
    pub fn victory_condition<'a>(
        &self,
        data: &'a GameData,
        id: impl Into<VictoryConditionId>,
    ) -> Option<&'a VictoryCondition> {
        Self::resolve(&data.victory_conditions, &self.victory_condition_by_id, id)
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
    /// Schema version is newer than the loader understands.
    #[error("Unsupported schema version {found} in {path}; current version is {current}")]
    UnsupportedSchemaVersion {
        /// Version found in manifest.
        found: u32,
        /// Latest version supported by the loader.
        current: u32,
        /// File path that declared the version.
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

fn build_typed_index<T, F, I>(
    kind: &'static str,
    items: &[T],
    id_fn: F,
) -> Result<HashMap<I, usize>, DataLoadError>
where
    F: Fn(&T) -> I,
    I: Eq + std::hash::Hash + Clone + Into<String>,
{
    let mut index = HashMap::new();
    for (i, item) in items.iter().enumerate() {
        let id = id_fn(item);
        if index.insert(id.clone(), i).is_some() {
            return Err(DataLoadError::DuplicateId {
                kind,
                id: id.into(),
            });
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

fn validate_positive(
    kind: &'static str,
    id: &str,
    field: &'static str,
    value: f64,
) -> Result<(), DataLoadError> {
    if value <= 0.0 {
        return Err(DataLoadError::Validation {
            kind,
            id: id.to_string(),
            message: format!("{field} must be positive (got {value})"),
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
    let total = distribution.black
        + distribution.white
        + distribution.red
        + distribution.green
        + distribution.blue;
    if total != 100 {
        return Err(DataLoadError::Validation {
            kind,
            id: id.to_string(),
            message: format!("tile_distribution must sum to 100 (got {total})"),
        });
    }
    Ok(())
}

fn validate_tech_reference(
    kind: &'static str,
    id: &str,
    tech_index: i32,
    tech_count: usize,
) -> Result<(), DataLoadError> {
    if tech_index == NO_TECH_REQUIREMENT {
        return Ok(());
    }

    if tech_index < 0 || tech_index as usize >= tech_count {
        return Err(DataLoadError::Validation {
            kind,
            id: id.to_string(),
            message: format!(
                "tech_index {tech_index} is out of range for {tech_count} available tech entries"
            ),
        });
    }

    Ok(())
}

fn validate_tech_edges(edges: &[TechEdge], techs: &[Tech]) -> Result<(), DataLoadError> {
    let ids: std::collections::HashSet<String> = techs.iter().map(|t| t.id.clone()).collect();
    for edge in edges {
        if !ids.contains(edge.from.as_str()) {
            return Err(DataLoadError::Validation {
                kind: "tech_edge",
                id: edge.from.clone(),
                message: "prerequisite tech id not found".to_string(),
            });
        }
        if !ids.contains(edge.to.as_str()) {
            return Err(DataLoadError::Validation {
                kind: "tech_edge",
                id: edge.to.clone(),
                message: "target tech id not found".to_string(),
            });
        }
    }
    Ok(())
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
    merge_by_id(&mut game_data.species, mods.species, |s| s.id.as_str());
    merge_by_id(&mut game_data.planet_sizes, mods.planet_sizes, |s| {
        s.id.as_str()
    });
    merge_by_id(
        &mut game_data.planet_surface_types,
        mods.planet_surface_types,
        |s| s.id.as_str(),
    );
    merge_by_id(&mut game_data.surface_items, mods.surface_items, |s| {
        s.id.as_str()
    });
    merge_by_id(&mut game_data.orbital_items, mods.orbital_items, |s| {
        s.id.as_str()
    });
    merge_by_id(
        &mut game_data.planetary_projects,
        mods.planetary_projects,
        |p| p.id.as_str(),
    );
    merge_by_id(&mut game_data.hull_classes, mods.hull_classes, |h| {
        h.id.as_str()
    });
    merge_by_id(&mut game_data.engines, mods.engines, |e| e.id.as_str());
    merge_by_id(&mut game_data.weapons, mods.weapons, |w| w.id.as_str());
    merge_by_id(&mut game_data.shields, mods.shields, |s| s.id.as_str());
    merge_by_id(&mut game_data.scanners, mods.scanners, |s| s.id.as_str());
    merge_by_id(&mut game_data.special_modules, mods.special_modules, |s| {
        s.id.as_str()
    });
    merge_by_id(&mut game_data.techs, mods.techs, |t| t.id.as_str());
    merge_by_id(&mut game_data.victory_conditions, mods.victories, |v| {
        v.id.as_str()
    });
    merge_tech_edges(tech_edges, mods.tech_edges);
    if let Some(rules) = mods.victory_rules {
        game_data.victory_rules = rules;
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
        research_graph: build_research_graph(&tech_edges),
        victory_conditions: victory_data.victory_condition,
        victory_rules,
    };

    apply_mods(&mut game_data, &mut tech_edges, mod_datasets);
    migrate_game_data(&mut game_data, &mut tech_edges, effective_schema_version)?;
    validate_tech_edges(&tech_edges, &game_data.techs)?;
    game_data.research_graph = build_research_graph(&tech_edges);

    game_data.validate()?;
    let registry = GameRegistry::from_game_data(&game_data)?;
    Ok((game_data, registry))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

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
            research_graph: ResearchGraph::default(),
            victory_conditions: Vec::new(),
            victory_rules: VictoryRules::default(),
        }
    }

    #[test]
    fn loads_full_dataset() {
        let (data, registry) = load_game_data(PathBuf::from("assets/data"))
            .expect("Game data should load from assets/data");

        assert!(
            !data.species().is_empty(),
            "Species list should not be empty"
        );
        assert!(
            registry.species(&data, "orfa").is_some(),
            "Species lookup should work"
        );
        assert!(
            !data.techs().is_empty(),
            "Tech list should be populated from research.toml"
        );
        assert!(
            registry.hull_class(&data, "enormous").is_some(),
            "Hull class lookup should succeed for known ids"
        );
        assert!(
            !data.victory_conditions().is_empty(),
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

        let error =
            GameRegistry::from_game_data(&data).expect_err("Duplicate ids should be reported");

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

        let error = data
            .validate()
            .expect_err("Negative values should fail validation");
        match error {
            DataLoadError::Validation { kind, id, .. } => {
                assert_eq!(kind, "hull_class");
                assert_eq!(id, "bad_hull");
            }
            other => panic!("Unexpected error: {other:?}"),
        }
    }

    #[test]
    fn rejects_out_of_range_tech_reference() {
        let mut data = base_game_data();

        data.techs.push(Tech {
            id: "starter".to_string(),
            name: localized("Starter"),
            description: localized("Starter tech"),
            research_cost: 1,
        });

        data.weapons.push(Weapon {
            id: "laser".to_string(),
            name: localized("Laser"),
            description: localized("Basic laser"),
            power_use: 1,
            range: 5,
            strength: 1.0,
            uses_per_turn: 1,
            industry_cost: 1,
            tech_index: 5,
        });

        let error = data
            .validate()
            .expect_err("Validation should fail when tech_index is out of range");

        match error {
            DataLoadError::Validation { kind, id, message } => {
                assert_eq!(kind, "weapon");
                assert_eq!(id, "laser");
                assert!(message.contains("tech_index 5 is out of range"));
            }
            other => panic!("Unexpected error: {other:?}"),
        }
    }

    #[test]
    fn rejects_negative_weapon_strength() {
        let mut data = base_game_data();
        data.techs.push(Tech {
            id: "starter".to_string(),
            name: localized("Starter"),
            description: localized("Starter tech"),
            research_cost: 1,
        });

        data.weapons.push(Weapon {
            id: "laser".to_string(),
            name: localized("Laser"),
            description: localized("Zero strength"),
            power_use: 1,
            range: 5,
            strength: -0.5,
            uses_per_turn: 1,
            industry_cost: 1,
            tech_index: NO_TECH_REQUIREMENT,
        });

        let error = data
            .validate()
            .expect_err("Negative strength should fail validation");

        match error {
            DataLoadError::Validation { kind, id, message } => {
                assert_eq!(kind, "weapon");
                assert_eq!(id, "laser");
                assert!(message.contains("strength"));
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

    #[test]
    fn computes_derived_stats_for_sample_dataset() {
        let mut data = base_game_data();

        data.techs.push(Tech {
            id: "start".to_string(),
            name: localized("Starter"),
            description: localized("Starter tech"),
            research_cost: 1,
        });

        data.weapons.push(Weapon {
            id: "laser".to_string(),
            name: localized("Laser"),
            description: localized("Basic laser"),
            power_use: 2,
            range: 3,
            strength: 5.0,
            uses_per_turn: 2,
            industry_cost: 10,
            tech_index: 0,
        });

        data.engines.push(Engine {
            id: "impulse".to_string(),
            name: localized("Impulse"),
            description: localized("Standard engine"),
            power_use: 2,
            thrust_rating: 8.0,
            industry_cost: 12,
        });

        data.surface_items.push(PlanetaryItem {
            id: "hab".to_string(),
            name: localized("Hab"),
            description: localized("Habitation"),
            industry_bonus: 1,
            research_bonus: 2,
            prosperity_bonus: 3,
            max_population_bonus: 4,
            slot_size: 1,
            industry_cost: 5,
            tech_index: 0,
        });

        data.orbital_items.push(PlanetaryItem {
            id: "orb".to_string(),
            name: localized("Orb"),
            description: localized("Orbital"),
            industry_bonus: 2,
            research_bonus: 0,
            prosperity_bonus: 1,
            max_population_bonus: 0,
            slot_size: 1,
            industry_cost: 3,
            tech_index: 0,
        });

        data.validate()
            .expect("All numeric fields should pass validation");

        let computed = data.compute();

        assert_eq!(computed.weapon_stats["laser"].dps, 10.0);
        assert_eq!(computed.engine_stats["impulse"].efficiency, Some(4.0));
        assert_eq!(computed.surface_item_stats["hab"].total_bonus, 10);
        assert_eq!(computed.orbital_item_stats["orb"].total_bonus, 3);
    }

    #[test]
    fn rejects_invalid_tech_reference() {
        let mut data = base_game_data();
        data.techs.push(Tech {
            id: "starter".to_string(),
            name: localized("Starter Tech"),
            description: localized("Allows basic modules"),
            research_cost: 10,
        });

        data.surface_items.push(PlanetaryItem {
            id: "basic_factory".to_string(),
            name: localized("Factory"),
            description: localized("Produces industry"),
            industry_bonus: 1,
            research_bonus: 0,
            prosperity_bonus: 0,
            max_population_bonus: 0,
            slot_size: 1,
            industry_cost: 5,
            tech_index: 5,
        });

        let error = data
            .validate()
            .expect_err("Invalid tech reference should fail validation");

        match error {
            DataLoadError::Validation { kind, id, .. } => {
                assert_eq!(kind, "surface_item");
                assert_eq!(id, "basic_factory");
            }
            other => panic!("Unexpected error: {other:?}"),
        }
    }

    #[test]
    fn rejects_zero_slot_size() {
        let mut data = base_game_data();
        data.techs.push(Tech {
            id: "starter".to_string(),
            name: localized("Starter Tech"),
            description: localized("Allows basic modules"),
            research_cost: 10,
        });

        data.surface_items.push(PlanetaryItem {
            id: "bad_slot".to_string(),
            name: localized("Bad"),
            description: localized("Bad"),
            industry_bonus: 0,
            research_bonus: 0,
            prosperity_bonus: 0,
            max_population_bonus: 0,
            slot_size: 0,
            industry_cost: 0,
            tech_index: NO_TECH_REQUIREMENT,
        });

        let error = data.validate().expect_err("Slot size must be positive");

        match error {
            DataLoadError::Validation { kind, id, .. } => {
                assert_eq!(kind, "surface_item");
                assert_eq!(id, "bad_slot");
            }
            other => panic!("Unexpected error: {other:?}"),
        }
    }

    #[test]
    fn lookup_tables_cover_all_entities() {
        let mut data = base_game_data();
        data.species.push(Species {
            id: "orfa".to_string(),
            name: localized("Orfa"),
            description: localized("Species"),
        });
        data.planet_sizes.push(PlanetSize {
            id: "small".to_string(),
            name: localized("Small"),
            description: localized("Small planet"),
            surface_slots: 3,
            orbital_slots: 1,
        });
        data.planet_surface_types.push(PlanetSurfaceType {
            id: "lush".to_string(),
            name: localized("Lush"),
            description: localized("Green"),
            tile_distribution: TileDistribution {
                black: 0,
                white: 0,
                red: 0,
                green: 0,
                blue: 100,
            },
        });
        data.surface_items.push(PlanetaryItem {
            id: "factory".to_string(),
            name: localized("Factory"),
            description: localized("Makes stuff"),
            industry_bonus: 1,
            research_bonus: 0,
            prosperity_bonus: 0,
            max_population_bonus: 0,
            slot_size: 1,
            industry_cost: 1,
            tech_index: NO_TECH_REQUIREMENT,
        });
        data.orbital_items.push(PlanetaryItem {
            id: "scanner".to_string(),
            name: localized("Scanner"),
            description: localized("Watches"),
            industry_bonus: 0,
            research_bonus: 1,
            prosperity_bonus: 0,
            max_population_bonus: 0,
            slot_size: 1,
            industry_cost: 1,
            tech_index: NO_TECH_REQUIREMENT,
        });
        data.planetary_projects.push(PlanetaryProject {
            id: "cleanup".to_string(),
            name: localized("Cleanup"),
            description: localized("Project"),
            industry_cost: 5,
        });
        data.hull_classes.push(HullClass {
            id: "frigate".to_string(),
            name: localized("Frigate"),
            description: localized("Hull"),
            size_index: 1,
            max_items: 4,
        });
        data.engines.push(Engine {
            id: "thruster".to_string(),
            name: localized("Thruster"),
            description: localized("Engine"),
            power_use: 1,
            thrust_rating: 1.0,
            industry_cost: 1,
        });
        data.weapons.push(Weapon {
            id: "laser".to_string(),
            name: localized("Laser"),
            description: localized("Pew"),
            power_use: 1,
            range: 10,
            strength: 1.0,
            uses_per_turn: 1,
            industry_cost: 1,
            tech_index: NO_TECH_REQUIREMENT,
        });
        data.shields.push(Shield {
            id: "bubble".to_string(),
            name: localized("Bubble"),
            description: localized("Shield"),
            strength: 1.0,
            industry_cost: 1,
        });
        data.scanners.push(Scanner {
            id: "ocular".to_string(),
            name: localized("Ocular"),
            description: localized("Scanner"),
            range: 1,
            strength: 1.0,
            industry_cost: 1,
        });
        data.special_modules.push(SpecialModule {
            id: "cloak".to_string(),
            name: localized("Cloak"),
            description: localized("Hide"),
            power_use: 1,
            range: 1,
            industry_cost: 1,
        });
        data.techs.push(Tech {
            id: "starter".to_string(),
            name: localized("Starter"),
            description: localized("Tech"),
            research_cost: 1,
        });
        data.victory_conditions.push(VictoryCondition {
            id: "domination".to_string(),
            name: localized("Domination"),
            description: localized("Win"),
        });

        let registry =
            GameRegistry::from_game_data(&data).expect("Indexes should build for populated data");

        assert!(registry.species(&data, "orfa").is_some());
        assert!(registry.planet_size(&data, "small").is_some());
        assert!(registry.planet_surface_type(&data, "lush").is_some());
        assert!(registry.surface_item(&data, "factory").is_some());
        assert!(registry.orbital_item(&data, "scanner").is_some());
        assert!(registry.planetary_project(&data, "cleanup").is_some());
        assert!(registry.hull_class(&data, "frigate").is_some());
        assert!(registry.engine(&data, "thruster").is_some());
        assert!(registry.weapon(&data, "laser").is_some());
        assert!(registry.shield(&data, "bubble").is_some());
        assert!(registry.scanner(&data, "ocular").is_some());
        assert!(registry.special_module(&data, "cloak").is_some());
        assert!(registry.tech(&data, "starter").is_some());
        assert!(registry.victory_condition(&data, "domination").is_some());
    }

    #[test]
    fn computes_derived_stats() {
        let mut data = base_game_data();
        data.weapons.push(Weapon {
            id: "laser".to_string(),
            name: localized("Laser"),
            description: localized("Test weapon"),
            power_use: 1,
            range: 10,
            strength: 2.5,
            uses_per_turn: 3,
            industry_cost: 1,
            tech_index: NO_TECH_REQUIREMENT,
        });
        data.engines.push(Engine {
            id: "thruster".to_string(),
            name: localized("Thruster"),
            description: localized("Engine"),
            power_use: 2,
            thrust_rating: 4.0,
            industry_cost: 1,
        });
        data.surface_items.push(PlanetaryItem {
            id: "factory".to_string(),
            name: localized("Factory"),
            description: localized("Bonus"),
            industry_bonus: 1,
            research_bonus: 1,
            prosperity_bonus: 0,
            max_population_bonus: 0,
            slot_size: 1,
            industry_cost: 1,
            tech_index: NO_TECH_REQUIREMENT,
        });
        data.orbital_items.push(PlanetaryItem {
            id: "sat".to_string(),
            name: localized("Sat"),
            description: localized("Bonus"),
            industry_bonus: 0,
            research_bonus: 2,
            prosperity_bonus: 1,
            max_population_bonus: 0,
            slot_size: 1,
            industry_cost: 1,
            tech_index: NO_TECH_REQUIREMENT,
        });

        let computed = data.compute();

        let weapon = computed
            .weapon_stats
            .get("laser")
            .expect("Weapon stats computed");
        assert!((weapon.dps - 7.5).abs() < f32::EPSILON);

        let engine = computed
            .engine_stats
            .get("thruster")
            .expect("Engine stats computed");
        assert_eq!(engine.efficiency, Some(2.0));

        let surface_bonus = computed
            .surface_item_stats
            .get("factory")
            .expect("Surface bonuses computed");
        assert_eq!(surface_bonus.total_bonus, 2);

        let orbital_bonus = computed
            .orbital_item_stats
            .get("sat")
            .expect("Orbital bonuses computed");
        assert_eq!(orbital_bonus.total_bonus, 3);
    }
}
