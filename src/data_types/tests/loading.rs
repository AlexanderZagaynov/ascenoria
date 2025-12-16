use crate::data_types::load_game_data;
use std::path::PathBuf;

#[test]
fn loads_full_dataset() {
    let (data, registry) = load_game_data(PathBuf::from("assets/data"))
        .expect("Game data should load from assets/data");

    // Surface Cell Types
    assert!(
        !data.surface_cell_types.is_empty(),
        "Surface cell types should not be empty"
    );
    assert!(
        registry.surface_cell_type(&data, "cell_white").is_some(),
        "Should find cell_white"
    );

    // Surface Buildings
    assert!(
        !data.surface_buildings.is_empty(),
        "Surface buildings should not be empty"
    );
    assert!(
        registry.surface_building(&data, "building_base").is_some(),
        "Should find building_base"
    );

    // Technologies
    assert!(
        !data.technologies.is_empty(),
        "Technologies should not be empty"
    );
    assert!(
        registry.technology(&data, "tech_terraforming").is_some(),
        "Should find tech_terraforming"
    );

    // Victory Conditions
    assert!(
        !data.victory_conditions.is_empty(),
        "Victory conditions should not be empty"
    );
    assert!(
        registry
            .victory_condition(&data, "victory_cover_planet")
            .is_some(),
        "Should find victory_cover_planet"
    );

    // Scenarios
    assert!(!data.scenarios.is_empty(), "Scenarios should not be empty");
    assert!(
        registry.scenario(&data, "scenario_mvp").is_some(),
        "Should find scenario_mvp"
    );
}
