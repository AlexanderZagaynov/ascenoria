use std::path::Path;

use crate::data_types::errors::DataLoadError;
use crate::data_types::game_data::GameData;
use crate::data_types::registry::GameRegistry;
// use crate::data_types::validation::validate_game_data;

use super::toml::load_toml_file;
use super::wrappers::{
    ScenariosData, SurfaceBuildingsData, SurfaceCellTypesData, TechnologiesData,
    VictoryConditionsData,
};

/// Load the full set of game data from the provided directory.
pub fn load_game_data<P: AsRef<Path>>(
    data_dir: P,
) -> Result<(GameData, GameRegistry), DataLoadError> {
    let base = data_dir.as_ref();

    let surface_cell_types_path = base.join("surface_cell_types.toml");
    let surface_buildings_path = base.join("surface_buildings.toml");
    let technologies_path = base.join("technologies.toml");
    let victory_conditions_path = base.join("victory_conditions.toml");
    let scenarios_path = base.join("scenarios.toml");

    let surface_cell_types_data: SurfaceCellTypesData = load_toml_file(&surface_cell_types_path)?;
    let surface_buildings_data: SurfaceBuildingsData = load_toml_file(&surface_buildings_path)?;
    let technologies_data: TechnologiesData = load_toml_file(&technologies_path)?;
    let victory_conditions_data: VictoryConditionsData = load_toml_file(&victory_conditions_path)?;
    let scenarios_data: ScenariosData = load_toml_file(&scenarios_path)?;

    let game_data = GameData {
        surface_cell_types: surface_cell_types_data.surface_cell_type,
        surface_buildings: surface_buildings_data.surface_building,
        technologies: technologies_data.technology,
        victory_conditions: victory_conditions_data.victory_condition,
        scenarios: scenarios_data.scenario,
    };

    // validate_game_data(&game_data)?;

    let registry = GameRegistry::from_game_data(&game_data)?;

    Ok((game_data, registry))
}
