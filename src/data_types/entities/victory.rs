use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum VictoryType {
    CoverAllTiles,
}

#[derive(Debug, Clone, Deserialize)]
pub struct VictoryCondition {
    pub id: String,
    pub name_en: String,
    #[serde(rename = "type")]
    pub condition_type: VictoryType,
}
