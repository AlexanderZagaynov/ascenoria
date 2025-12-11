//! Unit tests for the data module.

use std::path::PathBuf;

use super::compute::GameDataComputed;
use super::entities::*;
use super::game_data::GameData;
use super::loaders::load_game_data;
use super::localization::{Language, LocalizedText, NamedEntity};
use super::registry::GameRegistry;
use super::validation::NO_TECH_REQUIREMENT;

fn localized(text: &str) -> LocalizedText {
    LocalizedText {
        en: text.to_string(),
        ru: text.to_string(),
    }
}

fn base_game_data() -> GameData {
    GameData::new(
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        ResearchGraph::default(),
        Vec::new(),
        VictoryRules::default(),
    )
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
    *data.species_mut() = vec![
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

    let error = GameRegistry::from_game_data(&data).expect_err("Duplicate ids should be reported");

    match error {
        super::errors::DataLoadError::DuplicateId { kind, id } => {
            assert_eq!(kind, "species");
            assert_eq!(id, "duplicate");
        }
        other => panic!("Unexpected error: {other:?}"),
    }
}

#[test]
fn rejects_negative_values() {
    let mut data = base_game_data();
    data.hull_classes_mut().push(HullClass {
        id: "bad_hull".to_string(),
        name: localized("Bad"),
        description: localized("Bad"),
        size_index: -1,
        max_items: 2,
    });

    let error = super::validation::validate_game_data(&data)
        .expect_err("Negative values should fail validation");
    match error {
        super::errors::DataLoadError::Validation { kind, id, .. } => {
            assert_eq!(kind, "hull_class");
            assert_eq!(id, "bad_hull");
        }
        other => panic!("Unexpected error: {other:?}"),
    }
}

#[test]
fn rejects_out_of_range_tech_reference() {
    let mut data = base_game_data();

    data.techs_mut().push(Tech {
        id: "starter".to_string(),
        name: localized("Starter"),
        description: localized("Starter tech"),
        research_cost: 1,
    });

    data.weapons_mut().push(Weapon {
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

    let error = super::validation::validate_game_data(&data)
        .expect_err("Validation should fail when tech_index is out of range");

    match error {
        super::errors::DataLoadError::Validation { kind, id, message } => {
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
    data.techs_mut().push(Tech {
        id: "starter".to_string(),
        name: localized("Starter"),
        description: localized("Starter tech"),
        research_cost: 1,
    });

    data.weapons_mut().push(Weapon {
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

    let error = super::validation::validate_game_data(&data)
        .expect_err("Negative strength should fail validation");

    match error {
        super::errors::DataLoadError::Validation { kind, id, message } => {
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
    assert_eq!(
        super::localization::HasDescription::description(&hull, Language::En),
        "Light hull"
    );
    assert_eq!(
        super::localization::HasDescription::description(&hull, Language::Ru),
        "Легкий корпус"
    );
}

#[test]
fn computes_derived_stats_for_sample_dataset() {
    let mut data = base_game_data();

    data.techs_mut().push(Tech {
        id: "start".to_string(),
        name: localized("Starter"),
        description: localized("Starter tech"),
        research_cost: 1,
    });

    data.weapons_mut().push(Weapon {
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

    data.engines_mut().push(Engine {
        id: "impulse".to_string(),
        name: localized("Impulse"),
        description: localized("Standard engine"),
        power_use: 2,
        thrust_rating: 8.0,
        industry_cost: 12,
    });

    data.surface_items_mut().push(PlanetaryItem {
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

    data.orbital_items_mut().push(PlanetaryItem {
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

    super::validation::validate_game_data(&data)
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
    data.techs_mut().push(Tech {
        id: "starter".to_string(),
        name: localized("Starter Tech"),
        description: localized("Allows basic modules"),
        research_cost: 10,
    });

    data.surface_items_mut().push(PlanetaryItem {
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

    let error = super::validation::validate_game_data(&data)
        .expect_err("Invalid tech reference should fail validation");

    match error {
        super::errors::DataLoadError::Validation { kind, id, .. } => {
            assert_eq!(kind, "surface_item");
            assert_eq!(id, "basic_factory");
        }
        other => panic!("Unexpected error: {other:?}"),
    }
}

#[test]
fn rejects_zero_slot_size() {
    let mut data = base_game_data();
    data.techs_mut().push(Tech {
        id: "starter".to_string(),
        name: localized("Starter Tech"),
        description: localized("Allows basic modules"),
        research_cost: 10,
    });

    data.surface_items_mut().push(PlanetaryItem {
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

    let error =
        super::validation::validate_game_data(&data).expect_err("Slot size must be positive");

    match error {
        super::errors::DataLoadError::Validation { kind, id, .. } => {
            assert_eq!(kind, "surface_item");
            assert_eq!(id, "bad_slot");
        }
        other => panic!("Unexpected error: {other:?}"),
    }
}

#[test]
fn lookup_tables_cover_all_entities() {
    let mut data = base_game_data();
    data.species_mut().push(Species {
        id: "orfa".to_string(),
        name: localized("Orfa"),
        description: localized("Species"),
    });
    data.planet_sizes_mut().push(PlanetSize {
        id: "small".to_string(),
        name: localized("Small"),
        description: localized("Small planet"),
        surface_slots: 3,
        orbital_slots: 1,
    });
    data.planet_surface_types_mut().push(PlanetSurfaceType {
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
    data.surface_items_mut().push(PlanetaryItem {
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
    data.orbital_items_mut().push(PlanetaryItem {
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
    data.planetary_projects_mut().push(PlanetaryProject {
        id: "cleanup".to_string(),
        name: localized("Cleanup"),
        description: localized("Project"),
        industry_cost: 5,
    });
    data.hull_classes_mut().push(HullClass {
        id: "frigate".to_string(),
        name: localized("Frigate"),
        description: localized("Hull"),
        size_index: 1,
        max_items: 4,
    });
    data.engines_mut().push(Engine {
        id: "thruster".to_string(),
        name: localized("Thruster"),
        description: localized("Engine"),
        power_use: 1,
        thrust_rating: 1.0,
        industry_cost: 1,
    });
    data.weapons_mut().push(Weapon {
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
    data.shields_mut().push(Shield {
        id: "bubble".to_string(),
        name: localized("Bubble"),
        description: localized("Shield"),
        strength: 1.0,
        industry_cost: 1,
    });
    data.scanners_mut().push(Scanner {
        id: "ocular".to_string(),
        name: localized("Ocular"),
        description: localized("Scanner"),
        range: 1,
        strength: 1.0,
        industry_cost: 1,
    });
    data.special_modules_mut().push(SpecialModule {
        id: "cloak".to_string(),
        name: localized("Cloak"),
        description: localized("Hide"),
        power_use: 1,
        range: 1,
        industry_cost: 1,
    });
    data.techs_mut().push(Tech {
        id: "starter".to_string(),
        name: localized("Starter"),
        description: localized("Tech"),
        research_cost: 1,
    });
    data.victory_conditions_mut().push(VictoryCondition {
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
    data.weapons_mut().push(Weapon {
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
    data.engines_mut().push(Engine {
        id: "thruster".to_string(),
        name: localized("Thruster"),
        description: localized("Engine"),
        power_use: 2,
        thrust_rating: 4.0,
        industry_cost: 1,
    });
    data.surface_items_mut().push(PlanetaryItem {
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
    data.orbital_items_mut().push(PlanetaryItem {
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
