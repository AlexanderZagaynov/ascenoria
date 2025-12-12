//! Computed / derived stats from game data.
//!
//! Contains structures that cache frequently-used calculations
//! so they don't need to be recomputed each frame.

use std::collections::HashMap;

use bevy::prelude::Resource;

use super::entities::{Engine, PlanetaryItem, Weapon};

/// Derived stats for weapon modules.
#[derive(Debug, Clone, Copy)]
pub struct WeaponComputed {
    /// Approximate damage per turn for the weapon.
    pub dps: f32,
}

/// Derived stats for engines.
#[derive(Debug, Clone, Copy)]
pub struct EngineComputed {
    /// Optional thrust efficiency (thrust per power unit).
    pub efficiency: Option<f32>,
}

/// Aggregated bonuses for planetary items.
#[derive(Debug, Clone, Copy)]
pub struct PlanetaryItemComputed {
    /// Sum of all additive bonuses the building provides.
    pub total_bonus: i32,
}

/// Cache of derived stats to keep calculations centralized.
#[derive(Debug, Resource, Default)]
pub struct GameDataComputed {
    /// Derived stats for weapons, keyed by identifier.
    pub weapon_stats: HashMap<String, WeaponComputed>,
    /// Derived stats for engines, keyed by identifier.
    pub engine_stats: HashMap<String, EngineComputed>,
    /// Derived bonuses for surface items, keyed by identifier.
    pub surface_item_stats: HashMap<String, PlanetaryItemComputed>,
    /// Derived bonuses for orbital items, keyed by identifier.
    pub orbital_item_stats: HashMap<String, PlanetaryItemComputed>,
}

impl GameDataComputed {
    /// Compute derived stats from raw game data collections.
    pub fn from_data(
        weapons: &[Weapon],
        engines: &[Engine],
        surface_items: &[PlanetaryItem],
        orbital_items: &[PlanetaryItem],
    ) -> Self {
        let mut weapon_stats = HashMap::new();
        for weapon in weapons {
            weapon_stats.insert(
                weapon.id.clone(),
                WeaponComputed {
                    dps: weapon.strength * weapon.uses_per_turn as f32,
                },
            );
        }

        let mut engine_stats = HashMap::new();
        for engine in engines {
            let efficiency = if engine.power_use > 0 {
                Some(engine.thrust_rating / engine.power_use as f32)
            } else {
                None
            };
            engine_stats.insert(engine.id.clone(), EngineComputed { efficiency });
        }

        let compute_item = |item: &PlanetaryItem| PlanetaryItemComputed {
            total_bonus: item.industry_bonus
                + item.research_bonus
                + item.prosperity_bonus
                + item.max_population_bonus,
        };

        let surface_item_stats = surface_items
            .iter()
            .map(|item| (item.id.clone(), compute_item(item)))
            .collect();

        let orbital_item_stats = orbital_items
            .iter()
            .map(|item| (item.id.clone(), compute_item(item)))
            .collect();

        Self {
            weapon_stats,
            engine_stats,
            surface_item_stats,
            orbital_item_stats,
        }
    }
}
