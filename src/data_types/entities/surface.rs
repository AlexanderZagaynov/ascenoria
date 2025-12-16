use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct SurfaceCellType {
    pub id: String,
    pub name_en: String,
    pub is_usable: bool,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BuildableOn {
    White,
    Black,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SpecialBehavior {
    None,
    Terraformer,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SurfaceBuilding {
    pub id: String,
    pub name_en: String,
    pub color: (f32, f32, f32),
    pub buildable_on_cell_type: BuildableOn,
    pub requires_adjacency: bool,
    pub counts_for_adjacency: bool,
    pub y_food: i32,
    pub y_housing: i32,
    pub y_production: i32,
    pub y_science: i32,
    pub unlocked_by_tech_id: Option<String>,
    pub special_behavior: SpecialBehavior,
}
