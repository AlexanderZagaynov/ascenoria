use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum GenerationMode {
    RandomWhiteBlack,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Scenario {
    pub id: String,
    pub name_en: String,
    pub grid_width: u32,
    pub grid_height: u32,
    pub start_building_id: String,
    pub generation_mode: GenerationMode,
    pub black_ratio: f32,
    pub victory_condition_id: String,
}
