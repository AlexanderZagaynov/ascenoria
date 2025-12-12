use crate::data::entities::{Engine, PlanetaryItem, Tech, Weapon};

use super::helpers::{base_game_data, localized};

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

    crate::data::validation::validate_game_data(&data)
        .expect("All numeric fields should pass validation");

    let computed = data.compute();

    assert_eq!(computed.weapon_stats["laser"].dps, 10.0);
    assert_eq!(computed.engine_stats["impulse"].efficiency, Some(4.0));
    assert_eq!(computed.surface_item_stats["hab"].total_bonus, 10);
    assert_eq!(computed.orbital_item_stats["orb"].total_bonus, 3);
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
        tech_index: crate::data::NO_TECH_REQUIREMENT,
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
        tech_index: crate::data::NO_TECH_REQUIREMENT,
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
        tech_index: crate::data::NO_TECH_REQUIREMENT,
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
