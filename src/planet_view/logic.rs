//! Game logic for the planet view.
//!
//! This module contains pure logic functions that operate on the game state
//! without directly interacting with Bevy's ECS. This separation allows for
//! easier testing and cleaner code organization.
//!
//! # Connectivity System
//!
//! The main feature here is the tile connectivity algorithm, which determines
//! which tiles are "powered" by being connected to the base through a chain
//! of buildings. This is inspired by Ascendancy's adjacency mechanics.

use crate::data_types::GameData;
use crate::data_types::GameRegistry;
use crate::planet_data::{BuildingType, PlanetSurface};
use std::collections::{HashSet, VecDeque};

/// Update the connectivity status of all tiles on the planet surface.
///
/// This function implements a Breadth-First Search (BFS) algorithm starting
/// from the Base building. Tiles are marked as "connected" if they are:
/// 1. A building that counts for adjacency (grid node), OR
/// 2. Adjacent to a grid node
///
/// # Algorithm
///
/// 1. Reset all tiles to disconnected
/// 2. Find the Base building (starting point)
/// 3. BFS to find all "grid nodes" (buildings with `counts_for_adjacency: true`)
/// 4. Mark grid nodes and their orthogonal neighbors as connected
///
/// # Arguments
///
/// * `surface` - The planet surface to update (mutated in place)
/// * `game_data` - Game data containing building definitions
/// * `registry` - Registry for looking up building IDs
pub fn update_connectivity(
    surface: &mut PlanetSurface,
    _game_data: &GameData,
    _registry: &GameRegistry,
) {
    let width = surface.row_width;
    let height = surface.tiles.len() / width;

    // Step 1: Reset all tiles to disconnected
    for tile in &mut surface.tiles {
        tile.connected = false;
    }

    // Step 2: Find the Base building (our starting node)
    let mut base_index = None;
    for (i, tile) in surface.tiles.iter().enumerate() {
        if let Some(BuildingType::Base) = tile.building {
            base_index = Some(i);
            break;
        }
    }

    // If no base exists, nothing can be connected
    let Some(start_node) = base_index else { return };

    // Step 3: BFS to find all "Grid Nodes" (buildings that extend the power grid)
    let mut grid_nodes = HashSet::new();
    let mut queue = VecDeque::new();

    // Base always counts as a grid node
    grid_nodes.insert(start_node);
    queue.push_back(start_node);

    // BFS: Explore neighbors of each grid node
    while let Some(idx) = queue.pop_front() {
        let x = idx % width;
        let y = idx / width;

        // Collect orthogonal neighbors (up, down, left, right)
        let mut neighbors = Vec::new();
        if x > 0 {
            neighbors.push(idx - 1); // Left
        }
        if x < width - 1 {
            neighbors.push(idx + 1); // Right
        }
        if y > 0 {
            neighbors.push(idx - width); // Up
        }
        if y < height - 1 {
            neighbors.push(idx + width); // Down
        }

        // Check each neighbor for buildings that extend the grid
        for n_idx in neighbors {
            if grid_nodes.contains(&n_idx) {
                continue; // Already processed
            }

            if surface.tiles[n_idx].building.is_some() {
                // Any completed building extends the grid.
                grid_nodes.insert(n_idx);
                queue.push_back(n_idx);
            }
        }
    }

    // Step 4: Mark grid nodes AND their neighbors as connected
    // This allows building on empty tiles adjacent to grid nodes
    for &idx in &grid_nodes {
        surface.tiles[idx].connected = true;

        let x = idx % width;
        let y = idx / width;

        // Mark orthogonal neighbors as connected (even if empty)
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
