use crate::data_types::GameData;
use crate::data_types::GameRegistry;
use crate::data_types::SurfaceBuildingId;
use crate::planet_data::{BuildingType, PlanetSurface};
use std::collections::{HashSet, VecDeque};

pub fn update_connectivity(
    surface: &mut PlanetSurface,
    game_data: &GameData,
    registry: &GameRegistry,
) {
    let width = surface.row_width;
    let height = surface.tiles.len() / width;

    // Reset connectivity
    for tile in &mut surface.tiles {
        tile.connected = false;
    }

    // Find Base
    let mut base_index = None;
    for (i, tile) in surface.tiles.iter().enumerate() {
        if let Some(BuildingType::Base) = tile.building {
            base_index = Some(i);
            break;
        }
    }

    let Some(start_node) = base_index else { return };

    // BFS to find all "Grid Nodes" (connected buildings that extend the grid)
    let mut grid_nodes = HashSet::new();
    let mut queue = VecDeque::new();

    // Helper to check if a building counts for adjacency
    let counts_for_adjacency = |b_type: BuildingType| -> bool {
        let id_str = b_type.id();
        let id = SurfaceBuildingId::from(id_str);
        if let Some(&idx) = registry.surface_building_by_id.get(&id) {
            if let Some(building_data) = game_data.surface_buildings().get(idx) {
                return building_data.counts_for_adjacency;
            }
        }
        false
    };

    // Base always counts
    grid_nodes.insert(start_node);
    queue.push_back(start_node);

    while let Some(idx) = queue.pop_front() {
        let x = idx % width;
        let y = idx / width;

        // Check neighbors
        let mut neighbors = Vec::new();
        if x > 0 {
            neighbors.push(idx - 1);
        }
        if x < width - 1 {
            neighbors.push(idx + 1);
        }
        if y > 0 {
            neighbors.push(idx - width);
        }
        if y < height - 1 {
            neighbors.push(idx + width);
        }

        for n_idx in neighbors {
            if grid_nodes.contains(&n_idx) {
                continue;
            }

            if let Some(b_type) = surface.tiles[n_idx].building {
                if counts_for_adjacency(b_type) {
                    grid_nodes.insert(n_idx);
                    queue.push_back(n_idx);
                }
            }
        }
    }

    // Mark connected tiles (Grid Nodes + Neighbors)
    for &idx in &grid_nodes {
        surface.tiles[idx].connected = true;

        let x = idx % width;
        let y = idx / width;

        if x > 0 {
            surface.tiles[idx - 1].connected = true;
        }
        if x < width - 1 {
            surface.tiles[idx + 1].connected = true;
        }
        if y > 0 {
            surface.tiles[idx - width].connected = true;
        }
        if y < height - 1 {
            surface.tiles[idx + width].connected = true;
        }
    }
}
