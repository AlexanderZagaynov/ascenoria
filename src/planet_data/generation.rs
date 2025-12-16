//! Planet generation helpers.

use rand::SeedableRng;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;

use crate::data_types::GameData;

use super::types::{GeneratedPlanet, build_tiles};

/// Generate a random planet from loaded game data using the provided seed.
pub fn generate_planet(seed: u64, data: &GameData) -> Option<GeneratedPlanet> {
    let mut rng = StdRng::seed_from_u64(seed);

    let size = data.planet_sizes().choose(&mut rng)?;
    let surface_type = data.planet_surface_types().choose(&mut rng)?;

    let surface_slots = size.surface_slots.max(1);
    let orbital_slots = size.orbital_slots;

    let mut tiles = build_tiles(surface_slots, &surface_type.tile_distribution);
    tiles.shuffle(&mut rng);

    let row_width = (surface_slots as f32).sqrt().ceil().max(1.0) as usize;

    Some(GeneratedPlanet {
        size_id: size.id.clone(),
        surface_slots,
        orbital_slots,
        orbital_items: vec![None; orbital_slots as usize],
        surface_type_id: surface_type.id.clone(),
        tiles,
        row_width,
    })
}
