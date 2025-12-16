use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Supported module categories for ship designs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModuleCategory {
    Engine,
    Weapon,
    Shield,
    Scanner,
    Special,
}

/// Installed module entry with category and identifier.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstalledModule {
    pub category: ModuleCategory,
    pub id: String,
}

/// Aggregate stats for a ship design.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ShipStats {
    /// Sum of power draw across all modules.
    pub total_power_use: i32,
    /// Sum of weapon strength.
    pub total_firepower: f32,
    /// Sum of shield strength.
    pub total_defense: f32,
    /// Highest scanner range installed.
    pub sensor_range: i32,
}

/// Errors that can occur while validating a ship design.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum DesignError {
    /// Referenced hull identifier does not exist.
    #[error("hull '{0}' not found")]
    HullNotFound(String),
    /// Too many modules are placed compared to hull allowance.
    #[error("too many modules: {count} > max {max}")]
    TooManyModules { max: i32, count: i32 },
    /// A required engine is missing.
    #[error("at least one engine is required")]
    MissingEngine,
    /// Referenced module id does not exist for the given category.
    #[error("module not found in {category:?}: {id}")]
    ModuleNotFound {
        category: ModuleCategory,
        id: String,
    },
}
