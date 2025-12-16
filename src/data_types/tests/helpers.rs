use crate::data_types::GameData;

pub fn base_game_data() -> GameData {
    GameData {
        surface_cell_types: Vec::new(),
        surface_buildings: Vec::new(),
        technologies: Vec::new(),
        victory_conditions: Vec::new(),
        scenarios: Vec::new(),
    }
}
