use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Technology {
    pub id: String,
    pub name_en: String,
    pub science_cost: i32,
}
