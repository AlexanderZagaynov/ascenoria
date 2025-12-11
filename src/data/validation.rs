//! Validation helpers for game data.
//!
//! Contains functions that check data integrity: non-negative values,
//! valid tech references, proper tile distributions, etc.

use super::entities::{Tech, TechEdge, TileDistribution};
use super::errors::DataLoadError;
use super::game_data::GameData;

/// Sentinel value indicating no technology requirement.
pub const NO_TECH_REQUIREMENT: i32 = 255;

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

/// Validate all game data integrity constraints.
pub(crate) fn validate_game_data(data: &GameData) -> Result<(), DataLoadError> {
    for size in data.planet_sizes() {
        validate_non_negative_fields(
            "planet_size",
            &size.id,
            &[
                ("surface_slots", size.surface_slots as f64),
                ("orbital_slots", size.orbital_slots as f64),
            ],
        )?;
    }

    for surface_type in data.planet_surface_types() {
        validate_tile_distribution(
            "planet_surface_type",
            &surface_type.id,
            &surface_type.tile_distribution,
        )?;
    }

    for item in data.surface_items() {
        validate_non_negative_fields(
            "surface_item",
            &item.id,
            &[
                ("industry_bonus", item.industry_bonus as f64),
                ("research_bonus", item.research_bonus as f64),
                ("prosperity_bonus", item.prosperity_bonus as f64),
                ("max_population_bonus", item.max_population_bonus as f64),
                ("slot_size", item.slot_size as f64),
                ("industry_cost", item.industry_cost as f64),
                ("tech_index", item.tech_index as f64),
            ],
        )?;
        validate_positive("surface_item", &item.id, "slot_size", item.slot_size as f64)?;
        validate_tech_reference(
            "surface_item",
            &item.id,
            item.tech_index,
            data.techs().len(),
        )?;
    }

    for item in data.orbital_items() {
        validate_non_negative_fields(
            "orbital_item",
            &item.id,
            &[
                ("industry_bonus", item.industry_bonus as f64),
                ("research_bonus", item.research_bonus as f64),
                ("prosperity_bonus", item.prosperity_bonus as f64),
                ("max_population_bonus", item.max_population_bonus as f64),
                ("slot_size", item.slot_size as f64),
                ("industry_cost", item.industry_cost as f64),
                ("tech_index", item.tech_index as f64),
            ],
        )?;
        validate_positive("orbital_item", &item.id, "slot_size", item.slot_size as f64)?;
        validate_tech_reference(
            "orbital_item",
            &item.id,
            item.tech_index,
            data.techs().len(),
        )?;
    }

    for project in data.planetary_projects() {
        validate_non_negative(
            "planetary_project",
            &project.id,
            "industry_cost",
            project.industry_cost as f64,
        )?;
    }

    for hull in data.hull_classes() {
        validate_positive("hull_class", &hull.id, "size_index", hull.size_index as f64)?;
        validate_positive("hull_class", &hull.id, "max_items", hull.max_items as f64)?;
    }

    for engine in data.engines() {
        validate_non_negative_fields(
            "engine",
            &engine.id,
            &[
                ("power_use", engine.power_use as f64),
                ("thrust_rating", engine.thrust_rating as f64),
                ("industry_cost", engine.industry_cost as f64),
            ],
        )?;
        validate_positive(
            "engine",
            &engine.id,
            "thrust_rating",
            engine.thrust_rating as f64,
        )?;
    }

    for weapon in data.weapons() {
        validate_non_negative_fields(
            "weapon",
            &weapon.id,
            &[
                ("power_use", weapon.power_use as f64),
                ("range", weapon.range as f64),
                ("strength", weapon.strength as f64),
                ("uses_per_turn", weapon.uses_per_turn as f64),
                ("industry_cost", weapon.industry_cost as f64),
                ("tech_index", weapon.tech_index as f64),
            ],
        )?;
        validate_positive("weapon", &weapon.id, "range", weapon.range as f64)?;
        validate_positive(
            "weapon",
            &weapon.id,
            "uses_per_turn",
            weapon.uses_per_turn as f64,
        )?;
        validate_tech_reference("weapon", &weapon.id, weapon.tech_index, data.techs().len())?;
    }

    for shield in data.shields() {
        validate_non_negative_fields(
            "shield",
            &shield.id,
            &[
                ("strength", shield.strength as f64),
                ("industry_cost", shield.industry_cost as f64),
            ],
        )?;
        validate_positive("shield", &shield.id, "strength", shield.strength as f64)?;
    }

    for scanner in data.scanners() {
        validate_non_negative_fields(
            "scanner",
            &scanner.id,
            &[
                ("range", scanner.range as f64),
                ("strength", scanner.strength as f64),
                ("industry_cost", scanner.industry_cost as f64),
            ],
        )?;
        validate_positive("scanner", &scanner.id, "range", scanner.range as f64)?;
        validate_positive("scanner", &scanner.id, "strength", scanner.strength as f64)?;
    }

    for module in data.special_modules() {
        validate_non_negative_fields(
            "special_module",
            &module.id,
            &[
                ("power_use", module.power_use as f64),
                ("range", module.range as f64),
                ("industry_cost", module.industry_cost as f64),
            ],
        )?;
    }

    for tech in data.techs() {
        validate_non_negative("tech", &tech.id, "research_cost", tech.research_cost as f64)?;
    }

    if !(0.0..=1.0).contains(&data.victory_rules().domination_threshold) {
        return Err(DataLoadError::Validation {
            kind: "victory_rules",
            id: "domination_threshold".to_string(),
            message: "domination_threshold must be between 0.0 and 1.0".to_string(),
        });
    }

    Ok(())
}
