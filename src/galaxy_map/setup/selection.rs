use bevy::prelude::*;

use crate::galaxy_map::colors;
use crate::galaxy_map::types::{GalaxyMapRoot, GalaxyView3D, SelectionIndicator};

pub fn spawn_selection_indicator(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let selection_mesh = meshes.add(Torus::new(0.3, 0.35));
    let selection_material = materials.add(StandardMaterial {
        base_color: colors::RING_GREEN.with_alpha(0.8),
        emissive: bevy::color::LinearRgba::rgb(0.2, 0.8, 0.3),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        double_sided: true,
        cull_mode: None,
        ..default()
    });

    commands.spawn((
        Mesh3d(selection_mesh),
        MeshMaterial3d(selection_material),
        Transform::from_translation(Vec3::new(0.0, 0.0, -1000.0)) // Hidden initially
            .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
        SelectionIndicator,
        GalaxyMapRoot,
        GalaxyView3D,
    ));
}
