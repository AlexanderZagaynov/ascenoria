use crate::data_types::entities::{Tech, TechEdge};
use crate::data_types::errors::DataLoadError;

/// Sentinel value indicating no technology requirement.
pub const NO_TECH_REQUIREMENT: i32 = 255;

pub(crate) fn validate_tech_reference(
    kind: &'static str,
    id: &str,
    tech_index: i32,
    tech_count: usize,
) -> Result<(), DataLoadError> {
    if tech_index == NO_TECH_REQUIREMENT {
        return Ok(());
    }

    if tech_index < 0 || tech_index as usize >= tech_count {
        return Err(DataLoadError::Validation {
            kind,
            id: id.to_string(),
            message: format!(
                "tech_index {tech_index} is out of range for {tech_count} available tech entries"
            ),
        });
    }

    Ok(())
}

pub(crate) fn validate_tech_edges(edges: &[TechEdge], techs: &[Tech]) -> Result<(), DataLoadError> {
    let ids: std::collections::HashSet<String> = techs.iter().map(|t| t.id.clone()).collect();
    for edge in edges {
        if !ids.contains(edge.from.as_str()) {
            return Err(DataLoadError::Validation {
                kind: "tech_edge",
                id: edge.from.clone(),
                message: "prerequisite tech id not found".to_string(),
            });
        }
        if !ids.contains(edge.to.as_str()) {
            return Err(DataLoadError::Validation {
                kind: "tech_edge",
                id: edge.to.clone(),
                message: "target tech id not found".to_string(),
            });
        }
    }
    Ok(())
}
