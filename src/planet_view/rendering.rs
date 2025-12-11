//! 3D rendering helpers for the planet view.
//!
//! Contains mesh creation and material setup for planet surface visualization.

use bevy::prelude::*;
use bevy::render::render_resource::PrimitiveTopology;

use crate::planet::TileColor;

use super::types::{PlanetView3D, TileCube};

/// Create a planet grid mesh with vertex colors based on tiles.
pub fn create_planet_grid_mesh(
    meshes: &mut ResMut<Assets<Mesh>>,
    tiles: &[TileColor],
    row_width: usize,
) -> Handle<Mesh> {
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        bevy_asset::RenderAssetUsages::default(),
    );

    let mut positions: Vec<[f32; 3]> = Vec::new();
    let mut normals: Vec<[f32; 3]> = Vec::new();
    let mut colors: Vec<[f32; 4]> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    let tile_size = 1.0;
    let gap = 0.05; // Small gap between tiles
    let half_size = (tile_size - gap) / 2.0;

    let rows = (tiles.len() + row_width - 1) / row_width;
    let offset_x = -((row_width as f32) * tile_size) / 2.0;
    let offset_z = -((rows as f32) * tile_size) / 2.0;

    for (i, tile) in tiles.iter().enumerate() {
        // Skip black tiles (void)
        if *tile == TileColor::Black {
            continue;
        }

        let x_idx = i % row_width;
        let z_idx = i / row_width;

        let cx = offset_x + (x_idx as f32 * tile_size) + tile_size / 2.0;
        let cz = offset_z + (z_idx as f32 * tile_size) + tile_size / 2.0;
        let cy = 0.0;

        let base_index = positions.len() as u32;

        // 4 vertices for the quad
        positions.push([cx - half_size, cy, cz - half_size]);
        positions.push([cx + half_size, cy, cz - half_size]);
        positions.push([cx + half_size, cy, cz + half_size]);
        positions.push([cx - half_size, cy, cz + half_size]);

        normals.push([0.0, 1.0, 0.0]);
        normals.push([0.0, 1.0, 0.0]);
        normals.push([0.0, 1.0, 0.0]);
        normals.push([0.0, 1.0, 0.0]);

        let color = tile_color_to_linear(*tile);
        colors.push(color);
        colors.push(color);
        colors.push(color);
        colors.push(color);

        // 2 triangles
        indices.push(base_index);
        indices.push(base_index + 2);
        indices.push(base_index + 1);

        indices.push(base_index);
        indices.push(base_index + 3);
        indices.push(base_index + 2);
    }

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    mesh.insert_indices(bevy_mesh::Indices::U32(indices));

    meshes.add(mesh)
}

/// Create a material for the planet.
pub fn create_planet_material(
    materials: &mut ResMut<Assets<StandardMaterial>>,
    surface_type: &str,
) -> Handle<StandardMaterial> {
    let base_color = get_planet_base_color(surface_type);

    materials.add(StandardMaterial {
        base_color,
        // Use vertex colors
        base_color_texture: None,
        perceptual_roughness: 0.7,
        metallic: 0.0,
        reflectance: 0.3,
        ..default()
    })
}

/// Get the base color for a planet type.
pub fn get_planet_base_color(surface_type: &str) -> Color {
    match surface_type {
        "husk" => Color::srgb(0.2, 0.15, 0.15),
        "primordial" => Color::srgb(0.5, 0.45, 0.4),
        "congenial" => Color::srgb(0.6, 0.65, 0.6),
        "eden" => Color::srgb(0.3, 0.6, 0.35),
        "mineral" => Color::srgb(0.6, 0.4, 0.35),
        "supermineral" => Color::srgb(0.7, 0.35, 0.3),
        "chapel" | "cathedral" => Color::srgb(0.4, 0.5, 0.7),
        "special" => Color::srgb(0.6, 0.5, 0.65),
        "tycoon" => Color::srgb(0.75, 0.65, 0.4),
        "cornucopia" => Color::srgb(0.8, 0.7, 0.5),
        _ => Color::srgb(0.6, 0.6, 0.55),
    }
}

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

/// Convert TileColor to linear RGBA for vertex colors.
pub fn tile_color_to_linear(tile: TileColor) -> [f32; 4] {
    match tile {
        TileColor::Black => [0.1, 0.1, 0.1, 1.0],
        TileColor::White => [0.75, 0.75, 0.7, 1.0],
        TileColor::Red => [0.8, 0.3, 0.2, 1.0],
        TileColor::Green => [0.3, 0.7, 0.3, 1.0],
        TileColor::Blue => [0.3, 0.5, 0.8, 1.0],
    }
}

/// Get a representative color for planet thumbnails based on surface type.
pub fn get_planet_thumbnail_color(surface_type: &str) -> Color {
    use super::types::colors;
    match surface_type {
        "husk" => colors::TILE_BLACK,
        "primordial" => Color::srgb(0.4, 0.35, 0.3),
        "congenial" => colors::TILE_WHITE,
        "eden" => colors::TILE_GREEN,
        "mineral" | "supermineral" => colors::TILE_RED,
        "chapel" | "cathedral" => colors::TILE_BLUE,
        "special" => Color::srgb(0.6, 0.5, 0.6),
        "tycoon" => Color::srgb(0.7, 0.6, 0.3),
        "cornucopia" => Color::srgb(0.8, 0.7, 0.5),
        _ => colors::TILE_WHITE,
    }
}
