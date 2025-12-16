//! Galaxy generation utilities.
//!
//! Functions for generating star positions, star lanes (connections),
//! and other procedural content for the galaxy map.

use bevy::prelude::*;
use rand::{Rng, SeedableRng, rngs::StdRng};

use crate::galaxy_data::Galaxy;

use super::types::StarType;

/// Generate star positions from galaxy data in 3D space.
pub fn generate_star_positions(galaxy: &Galaxy, seed: u64) -> Vec<(Vec3, StarType, String)> {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut positions = Vec::with_capacity(galaxy.systems.len());

    // Generate positions in a spherical volume
    let galaxy_radius = 8.0;

    for (i, system) in galaxy.systems.iter().enumerate() {
        // Spherical coordinates for more natural galaxy distribution
        let theta = Rng::gen_range(&mut rng, 0.0..std::f32::consts::TAU);
        let phi = Rng::gen_range(&mut rng, 0.0..std::f32::consts::PI);
        let r: f32 = Rng::gen_range(&mut rng, 0.3..1.0);
        let r = r.powf(0.5) * galaxy_radius;

        // Flatten the sphere into a disk shape (galaxy-like)
        let disk_factor = 0.3;

        let x = r * phi.sin() * theta.cos();
        let y = r * phi.cos() * disk_factor;
        let z = r * phi.sin() * theta.sin();

        let star_type = StarType::from_seed(seed.wrapping_add(i as u64));
        positions.push((Vec3::new(x, y, z), star_type, system.name.clone()));
    }

    positions
}

/// Generate star lane connections based on distance (connect nearby stars).
pub fn generate_star_lanes(
    positions: &[(Vec3, StarType, String)],
    max_distance: f32,
) -> Vec<(usize, usize)> {
    let mut lanes = Vec::new();

    for i in 0..positions.len() {
        for j in (i + 1)..positions.len() {
            let dist = positions[i].0.distance(positions[j].0);
            if dist < max_distance {
                lanes.push((i, j));
            }
        }
    }

    // Also ensure connectivity: if a star has no lanes, connect to nearest
    for i in 0..positions.len() {
        let has_lane = lanes.iter().any(|(a, b)| *a == i || *b == i);
        if !has_lane && positions.len() > 1 {
            // Find nearest star
            let mut nearest = 0;
            let mut nearest_dist = f32::MAX;
            for j in 0..positions.len() {
                if i != j {
                    let dist = positions[i].0.distance(positions[j].0);
                    if dist < nearest_dist {
                        nearest_dist = dist;
                        nearest = j;
                    }
                }
            }
            lanes.push((i.min(nearest), i.max(nearest)));
        }
    }

    lanes
}

/// Create a cylinder mesh for a star lane between two points.
pub fn create_lane_mesh(start: Vec3, end: Vec3) -> (Mesh, Transform) {
    let direction = end - start;
    let length = direction.length();
    let midpoint = (start + end) / 2.0;

    // Create a thin cylinder
    let mesh = Cylinder::new(0.02, length);

    // Calculate rotation to point from start to end
    let up = Vec3::Y;
    let rotation = if direction.normalize().abs_diff_eq(up, 0.001)
        || direction.normalize().abs_diff_eq(-up, 0.001)
    {
        Quat::IDENTITY
    } else {
        Quat::from_rotation_arc(up, direction.normalize())
    };

    let transform = Transform::from_translation(midpoint).with_rotation(rotation);

    (mesh.into(), transform)
}
