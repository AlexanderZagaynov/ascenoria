use bevy::prelude::*;
use bevy::render::render_resource::PrimitiveTopology;
use crate::planet_data::TileColor;

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
