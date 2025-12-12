use crate::data_types::{GameData, GameRegistry};
use serde::{Deserialize, Serialize};
use super::types::{DesignError, InstalledModule, ModuleCategory, ShipStats};

/// Ship design layout containing hull and installed modules.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShipDesign {
    pub hull_id: String,
    pub modules: Vec<InstalledModule>,
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
