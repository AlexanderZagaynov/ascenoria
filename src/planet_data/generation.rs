//! Planet generation helpers.
//!
//! This module contains functions for procedurally generating planet surfaces.
//! The generation is deterministic based on a seed value, allowing reproducible
//! results for testing and saved games.

use super::types::{BuildingType, PlanetSurface, TileColor};
use rand::prelude::*;

/// Generate a random planet surface for the MVP.
///
/// Creates a 10x10 grid with randomly distributed white and black tiles.
/// A Base building is placed on a random white tile to start the game.
///
/// # Algorithm
///
/// 1. Create empty 10x10 grid
/// 2. Randomly assign each tile as White or Black (50/50 chance)
/// 3. Ensure at least one White tile exists
/// 4. Place Base building on a random White tile
///
/// # Arguments
///
/// * `seed` - Random seed for deterministic generation
///
/// # Returns
///
/// A fully initialized `PlanetSurface` ready for gameplay.
pub fn generate_planet(seed: u64) -> PlanetSurface {
    let mut rng = StdRng::seed_from_u64(seed);
    let width = 10;
    let height = 10;

    let mut surface = PlanetSurface::new(width, height);

    // Randomly assign tile colors (50% white, 50% black)
    for tile in surface.tiles.iter_mut() {
        tile.color = if rng.gen_bool(0.5) {
            TileColor::White
        } else {
            TileColor::Black
        };
    }

    // Ensure at least one White tile exists for the Base
    if !surface.tiles.iter().any(|t| t.color == TileColor::White) {
        surface.tiles[0].color = TileColor::White;
    }

    // Place Base on a random White tile
    let white_indices: Vec<usize> = surface
        .tiles
        .iter()
        .enumerate()
        .filter(|(_, t)| t.color == TileColor::White)
        .map(|(i, _)| i)
        .collect();

    if let Some(&idx) = white_indices.choose(&mut rng) {
        surface.tiles[idx].building = Some(BuildingType::Base);
    }

    surface
}
