use crate::data::{GameData, GameRegistry};
use thiserror::Error;

/// Supported module categories for ship designs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModuleCategory {
    Engine,
    Weapon,
    Shield,
    Scanner,
    Special,
}

/// Installed module entry with category and identifier.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstalledModule {
    pub category: ModuleCategory,
    pub id: String,
}

/// Ship design layout containing hull and installed modules.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShipDesign {
    pub hull_id: String,
    pub modules: Vec<InstalledModule>,
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

impl ShipDesign {
    /// Create a new design for the given hull identifier.
    pub fn new(hull_id: impl Into<String>) -> Self {
        Self {
            hull_id: hull_id.into(),
            modules: Vec::new(),
        }
    }

    /// Add a module to the design without validating limits.
    pub fn add_module(&mut self, category: ModuleCategory, id: impl Into<String>) {
        self.modules.push(InstalledModule {
            category,
            id: id.into(),
        });
    }

    /// Validate the design against available data and placement rules.
    pub fn validate(&self, data: &GameData, registry: &GameRegistry) -> Result<(), DesignError> {
        let hull = registry
            .hull_class(data, self.hull_id.as_str())
            .ok_or_else(|| DesignError::HullNotFound(self.hull_id.clone()))?;

        if self.modules.len() as i32 > hull.max_items {
            return Err(DesignError::TooManyModules {
                max: hull.max_items,
                count: self.modules.len() as i32,
            });
        }

        let has_engine = self
            .modules
            .iter()
            .any(|m| m.category == ModuleCategory::Engine);
        if !has_engine {
            return Err(DesignError::MissingEngine);
        }

        for m in &self.modules {
            let exists = match m.category {
                ModuleCategory::Engine => registry.engine(data, m.id.as_str()).is_some(),
                ModuleCategory::Weapon => registry.weapon(data, m.id.as_str()).is_some(),
                ModuleCategory::Shield => registry.shield(data, m.id.as_str()).is_some(),
                ModuleCategory::Scanner => registry.scanner(data, m.id.as_str()).is_some(),
                ModuleCategory::Special => registry.special_module(data, m.id.as_str()).is_some(),
            };

            if !exists {
                return Err(DesignError::ModuleNotFound {
                    category: m.category,
                    id: m.id.clone(),
                });
            }
        }

        Ok(())
    }

    /// Compute aggregate stats for this design.
    pub fn compute_stats(
        &self,
        data: &GameData,
        registry: &GameRegistry,
    ) -> Result<ShipStats, DesignError> {
        self.validate(data, registry)?;

        let mut stats = ShipStats::default();
        for m in &self.modules {
            match m.category {
                ModuleCategory::Engine => {
                    if let Some(engine) = registry.engine(data, m.id.as_str()) {
                        stats.total_power_use += engine.power_use;
                    } else {
                        return Err(DesignError::ModuleNotFound {
                            category: m.category,
                            id: m.id.clone(),
                        });
                    }
                }
                ModuleCategory::Weapon => {
                    if let Some(weapon) = registry.weapon(data, m.id.as_str()) {
                        stats.total_power_use += weapon.power_use;
                        stats.total_firepower += weapon.strength;
                    } else {
                        return Err(DesignError::ModuleNotFound {
                            category: m.category,
                            id: m.id.clone(),
                        });
                    }
                }
                ModuleCategory::Shield => {
                    if let Some(shield) = registry.shield(data, m.id.as_str()) {
                        stats.total_defense += shield.strength;
                    } else {
                        return Err(DesignError::ModuleNotFound {
                            category: m.category,
                            id: m.id.clone(),
                        });
                    }
                }
                ModuleCategory::Scanner => {
                    if let Some(scanner) = registry.scanner(data, m.id.as_str()) {
                        stats.sensor_range = stats.sensor_range.max(scanner.range);
                    } else {
                        return Err(DesignError::ModuleNotFound {
                            category: m.category,
                            id: m.id.clone(),
                        });
                    }
                }
                ModuleCategory::Special => {
                    if let Some(module) = registry.special_module(data, m.id.as_str()) {
                        stats.total_power_use += module.power_use;
                    } else {
                        return Err(DesignError::ModuleNotFound {
                            category: m.category,
                            id: m.id.clone(),
                        });
                    }
                }
            }
        }

        Ok(stats)
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::load_game_data;
    use std::path::PathBuf;

    fn load_assets() -> (GameData, GameRegistry) {
        load_game_data(PathBuf::from("assets/data")).expect("assets should load")
    }

    fn engine_id(data: &GameData) -> &str {
        data.engines()
            .first()
            .map(|e| e.id.as_str())
            .expect("engine in assets")
    }

    #[test]
    fn validates_design_within_limits() {
        let (data, registry) = load_assets();
        let hull_id = "small";
        let engine = engine_id(&data);

        let mut design = ShipDesign::new(hull_id);
        design.add_module(ModuleCategory::Engine, engine);

        assert!(design.validate(&data, &registry).is_ok());
    }

    #[test]
    fn rejects_design_without_engine() {
        let (data, registry) = load_assets();
        let hull_id = "small";

        let mut design = ShipDesign::new(hull_id);
        design.add_module(ModuleCategory::Weapon, "laser_beam");

        let err = design
            .validate(&data, &registry)
            .expect_err("missing engine should fail");
        assert!(matches!(err, DesignError::MissingEngine));
    }

    #[test]
    fn rejects_over_capacity_design() {
        let (data, registry) = load_assets();
        let hull_id = "small";
        let engine = engine_id(&data);

        let mut design = ShipDesign::new(hull_id);
        for _ in 0..6 {
            design.add_module(ModuleCategory::Engine, engine);
        }

        let err = design
            .validate(&data, &registry)
            .expect_err("over capacity should fail");
        assert!(matches!(err, DesignError::TooManyModules { .. }));
    }

    #[test]
    fn rejects_unknown_module_id() {
        let (data, registry) = load_assets();
        let hull_id = "small";

        let mut design = ShipDesign::new(hull_id);
        design.add_module(ModuleCategory::Engine, "unknown_engine");

        let err = design
            .validate(&data, &registry)
            .expect_err("unknown module should fail");
        assert!(matches!(
            err,
            DesignError::ModuleNotFound {
                category: ModuleCategory::Engine,
                ..
            }
        ));
    }
}
