use crate::data::errors::DataLoadError;
use crate::data::game_data::GameData;

use crate::data::entities::TechEdge;

pub(crate) fn migrate_game_data(
    _game_data: &mut GameData,
    _tech_edges: &mut Vec<TechEdge>,
    from_version: u32,
) -> Result<(), DataLoadError> {
    if from_version > super::DATA_SCHEMA_VERSION {
        return Err(DataLoadError::UnsupportedSchemaVersion {
            found: from_version,
            current: super::DATA_SCHEMA_VERSION,
            path: "manifest".to_string(),
        });
    }

    Ok(())
}
