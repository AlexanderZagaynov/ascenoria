mod entities;
mod errors;
mod game_data;
mod ids;
mod loaders;
mod registry;

#[cfg(test)]
#[path = "tests/mod.rs"]
mod tests;

pub use entities::{
    BuildableOn, GenerationMode, Scenario, SpecialBehavior, SurfaceBuilding, SurfaceCellType,
    Technology, VictoryCondition, VictoryType,
};
pub use errors::DataLoadError;
pub use game_data::GameData;
pub use ids::{ScenarioId, SurfaceBuildingId, SurfaceCellTypeId, TechnologyId, VictoryConditionId};
pub use loaders::load_game_data;
pub use registry::GameRegistry;
