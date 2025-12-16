use crate::data_types::errors::DataLoadError;
use crate::data_types::entities::TileDistribution;

pub(crate) fn validate_non_negative(
    kind: &'static str,
    id: &str,
    field: &'static str,
    value: f64,
) -> Result<(), DataLoadError> {
    if value < 0.0 {
        return Err(DataLoadError::Validation {
            kind,
            id: id.to_string(),
            message: format!("{field} must be non-negative (got {value})"),
        });
    }
    Ok(())
}

pub(crate) fn validate_positive(
    kind: &'static str,
    id: &str,
    field: &'static str,
    value: f64,
) -> Result<(), DataLoadError> {
    if value <= 0.0 {
        return Err(DataLoadError::Validation {
            kind,
            id: id.to_string(),
            message: format!("{field} must be positive (got {value})"),
        });
    }
    Ok(())
}

pub(crate) fn validate_non_negative_fields(
    kind: &'static str,
    id: &str,
    fields: &[(&'static str, f64)],
) -> Result<(), DataLoadError> {
    for (field, value) in fields {
        validate_non_negative(kind, id, field, *value)?;
    }
    Ok(())
}

pub(crate) fn validate_tile_distribution(
    kind: &'static str,
    id: &str,
    distribution: &TileDistribution,
) -> Result<(), DataLoadError> {
    let total = distribution.black
        + distribution.white
        + distribution.red
        + distribution.green
        + distribution.blue;
    if total != 100 {
        return Err(DataLoadError::Validation {
            kind,
            id: id.to_string(),
            message: format!("tile_distribution must sum to 100 (got {total})"),
        });
    }
    Ok(())
}
