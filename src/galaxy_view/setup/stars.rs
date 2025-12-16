use bevy::prelude::*;
use rand::{Rng, SeedableRng, rngs::StdRng};

use crate::galaxy_view::types::{GalaxyViewRoot, GalaxyView3D, StarMarker};

pub fn spawn_stars(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    star_positions: &[(Vec3, crate::galaxy_view::types::StarType, String)],
) {
    // Create star mesh (small sphere)
    let star_mesh = meshes.add(Sphere::new(0.2));
    let glow_mesh = meshes.add(Sphere::new(0.35));

    // Spawn stars
    for (i, (pos, star_type, _name)) in star_positions.iter().enumerate() {
        // Main star material with emission
        let star_material = materials.add(StandardMaterial {
            base_color: star_type.color(),
            emissive: star_type.color().to_linear() * 5.0,
            ..default()
        });

        // Glow material
        let glow_material = materials.add(StandardMaterial {
            base_color: star_type.color().with_alpha(0.3),
            emissive: star_type.color().to_linear() * 2.0,
            alpha_mode: AlphaMode::Blend,
            ..default()
        });

        // Main star
        commands.spawn((
            Mesh3d(star_mesh.clone()),
            MeshMaterial3d(star_material),
            Transform::from_translation(*pos),
            StarMarker { star_index: i },
            GalaxyViewRoot,
            GalaxyView3D,
        ));

        // Glow effect
        commands.spawn((
            Mesh3d(glow_mesh.clone()),
            MeshMaterial3d(glow_material),
            Transform::from_translation(*pos),
            GalaxyViewRoot,
            GalaxyView3D,
        ));
    }
}

pub fn spawn_background_stars(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let bg_star_mesh = meshes.add(Sphere::new(0.03));
    let bg_star_material = materials.add(StandardMaterial {
        base_color: Color::srgba(0.7, 0.7, 0.8, 0.5),
        emissive: bevy::color::LinearRgba::rgb(0.2, 0.2, 0.3),
        alpha_mode: AlphaMode::Blend,
        ..default()
    });

    let mut bg_rng = StdRng::seed_from_u64(42);
    for _ in 0..200 {
        let x = Rng::gen_range(&mut bg_rng, -15.0..15.0);
        let y = Rng::gen_range(&mut bg_rng, -8.0..8.0);
        let z = Rng::gen_range(&mut bg_rng, -15.0..15.0);

        commands.spawn((
            Mesh3d(bg_star_mesh.clone()),
            MeshMaterial3d(bg_star_material.clone()),
            Transform::from_xyz(x, y, z),
            GalaxyViewRoot,
            GalaxyView3D,
        ));
    }
}
