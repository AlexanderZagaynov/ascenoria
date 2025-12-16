use bevy::prelude::*;

use crate::galaxy_view::generation::{create_lane_mesh, generate_star_lanes};
use crate::galaxy_view::types::{GalaxyViewRoot, GalaxyView3D, StarLane};

pub fn spawn_star_lanes(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    star_positions: &[(Vec3, crate::galaxy_view::types::StarType, String)],
) {
    let lanes = generate_star_lanes(star_positions, 5.0);
    let lane_material = materials.add(StandardMaterial {
        base_color: Color::srgba(0.2, 0.5, 0.9, 0.6),
        emissive: bevy::color::LinearRgba::rgb(0.1, 0.3, 0.6),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });

    for (i, j) in &lanes {
        let (lane_mesh, lane_transform) =
            create_lane_mesh(star_positions[*i].0, star_positions[*j].0);
        commands.spawn((
            Mesh3d(meshes.add(lane_mesh)),
            MeshMaterial3d(lane_material.clone()),
            lane_transform,
            StarLane,
            GalaxyViewRoot,
            GalaxyView3D,
        ));
    }
}
