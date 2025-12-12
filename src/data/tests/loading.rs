use std::path::PathBuf;

use crate::data::load_game_data;

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
