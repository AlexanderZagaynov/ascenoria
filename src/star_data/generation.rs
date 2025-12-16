//! Planet position generation for the star system view.
//!
//! Generates isometric positions for planets based on galaxy data.

use bevy::prelude::*;
use rand::{Rng, SeedableRng, rngs::StdRng};

use crate::galaxy_data::Galaxy;

use super::types::{PlanetPosition, PlanetVisual};

/// Generate planet positions for isometric view from galaxy data.
pub fn generate_planet_positions(
    galaxy: &Galaxy,
    star_index: usize,
    seed: u64,
) -> Vec<PlanetPosition> {
    let Some(system) = galaxy.systems.get(star_index) else {
        return Vec::new();
    };

    let mut rng = StdRng::seed_from_u64(seed.wrapping_add(star_index as u64));
    let mut positions = Vec::with_capacity(system.planets.len());

    let base_radius = 120.0;

    for (i, planet) in system.planets.iter().enumerate() {
        // Distribute planets in a spiral pattern
        let angle = (i as f32 / system.planets.len().max(1) as f32) * std::f32::consts::TAU
            + Rng::gen_range(&mut rng, -0.3..0.3);
        let distance = base_radius + (i as f32 * 60.0) + Rng::gen_range(&mut rng, -20.0..20.0);

        // Convert to isometric 2D coordinates
        let grid_x = angle.cos() * distance;
        let grid_z = angle.sin() * distance * 0.5; // Compress Z for isometric effect

        // Height varies by planet position
        let height = 80.0 + Rng::gen_range(&mut rng, -40.0..60.0);

        // Size based on planet surface slots
        let size = match planet.surface_slots {
            0..=10 => 20.0,
            11..=30 => 30.0,
            31..=50 => 40.0,
            _ => 50.0,
        };

        let visual = PlanetVisual::from_surface_type(&planet.surface_type_id);

        positions.push(PlanetPosition {
            grid_pos: Vec2::new(grid_x, grid_z),
            height,
            size,
            visual,
        });
    }

    positions
}
