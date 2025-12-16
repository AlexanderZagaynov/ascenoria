//! Planet generation helpers.

use super::types::{BuildingType, PlanetSurface, TileColor};
use rand::prelude::*;

/// Generate a random planet for MVP.
pub fn generate_planet(seed: u64) -> PlanetSurface {
    let mut rng = StdRng::seed_from_u64(seed);
    let width = 10;
    let height = 10;

    let mut surface = PlanetSurface::new(width, height);

    for tile in surface.tiles.iter_mut() {
        // 50% chance of White, 50% Black
        tile.color = if rng.gen_bool(0.5) {
            TileColor::White
        } else {
            TileColor::Black
        };
    }

    // Ensure at least one White tile for the Base
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
