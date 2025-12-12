use crate::data::entities::{
    Engine, HullClass, PlanetSize, PlanetSurfaceType, PlanetaryItem, PlanetaryProject, Scanner,
    Shield, SpecialModule, Species, Tech, TileDistribution, VictoryCondition, Weapon,
};
use crate::data::registry::GameRegistry;

use super::helpers::{base_game_data, localized};

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
        crate::data::errors::DataLoadError::DuplicateId { kind, id } => {
            assert_eq!(kind, "species");
            assert_eq!(id, "duplicate");
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
        tech_index: crate::data::NO_TECH_REQUIREMENT,
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
        tech_index: crate::data::NO_TECH_REQUIREMENT,
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
        tech_index: crate::data::NO_TECH_REQUIREMENT,
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
