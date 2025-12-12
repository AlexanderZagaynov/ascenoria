use crate::data::validation::validate_game_data;
use crate::data::{
    DataLoadError, GameData, HullClass, NO_TECH_REQUIREMENT, PlanetaryItem, Tech, Weapon,
};

use super::helpers::{base_game_data, localized};

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

    let error = validate_game_data(&data).expect_err("Negative values should fail validation");
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

    let error = validate_game_data(&data)
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

    let error = validate_game_data(&data).expect_err("Negative strength should fail validation");

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

    let error =
        validate_game_data(&data).expect_err("Invalid tech reference should fail validation");

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

    let error = validate_game_data(&data).expect_err("Slot size must be positive");

    match error {
        DataLoadError::Validation { kind, id, .. } => {
            assert_eq!(kind, "surface_item");
            assert_eq!(id, "bad_slot");
        }
        other => panic!("Unexpected error: {other:?}"),
    }
}

#[test]
fn base_game_data_smoke() {
    let data: GameData = base_game_data();
    assert!(data.species().is_empty());
}
