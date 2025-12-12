use bevy::prelude::*;
use crate::planet_data::TileColor;
use crate::planet_view::types::{PlanetView3D, TileCube};

/// Spawn building cubes on special tiles.
pub fn spawn_surface_buildings(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    tiles: &[TileColor],
    row_width: usize,
) {
    // Create a small cube mesh for buildings
    let cube_mesh = meshes.add(Cuboid::new(0.6, 0.6, 0.6));

    // Create materials for different building types
    let industry_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.7, 0.5, 0.2),
        metallic: 0.3,
        ..default()
    });
    let research_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.3, 0.5, 0.8),
        emissive: bevy::color::LinearRgba::rgb(0.1, 0.2, 0.4),
        ..default()
    });
    let prosperity_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.3, 0.7, 0.35),
        ..default()
    });

    let tile_size = 1.0;
    let rows = (tiles.len() + row_width - 1) / row_width;
    let offset_x = -((row_width as f32) * tile_size) / 2.0;
    let offset_z = -((rows as f32) * tile_size) / 2.0;

    // Place buildings on colored (non-white, non-black) tiles
    for (i, tile) in tiles.iter().enumerate() {
        let mat = match tile {
            TileColor::Red => Some(industry_mat.clone()),
            TileColor::Blue => Some(research_mat.clone()),
            TileColor::Green => Some(prosperity_mat.clone()),
            _ => None,
        };

        if let Some(material) = mat {
            let x_idx = i % row_width;
            let z_idx = i / row_width;

            let cx = offset_x + (x_idx as f32 * tile_size) + tile_size / 2.0;
            let cz = offset_z + (z_idx as f32 * tile_size) + tile_size / 2.0;

            let pos = Vec3::new(cx, 0.3, cz); // Center of 0.6 height cube is at 0.3

            commands.spawn((
                Mesh3d(cube_mesh.clone()),
                MeshMaterial3d(material),
                Transform::from_translation(pos),
                TileCube { tile_index: i },
                PlanetView3D,
            ));
        }
    }
}
