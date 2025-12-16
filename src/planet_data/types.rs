//! Data structures for planet generation and placement.
//!
//! This module defines the core data types used to represent a planet's surface,
//! including tiles, buildings, and the surface grid itself.
//!
//! # Design Philosophy
//!
//! The planet surface uses a simple 2D grid where each tile can be:
//! - White (buildable) or Black (requires terraforming)
//! - Empty or containing a building
//! - Connected or disconnected from the power grid
//!
//! This is inspired by Ascendancy's planet management system.

use bevy::prelude::*;

/// Tile color determines what can be built on it.
///
/// In Ascendancy-style gameplay:
/// - **White tiles**: Can have buildings placed directly
/// - **Black tiles**: Require terraforming before building
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileColor {
    /// Unbuildable tile (requires terraforming).
    Black,
    /// Buildable tile (ready for construction).
    White,
}

/// The different types of buildings that can be placed on a planet.
///
/// Each building type has a corresponding ID string used to look up
/// its full definition (yields, cost, color) in the game data files.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildingType {
    /// Starting building, provides all resource types.
    Base,
    /// Produces food for population growth.
    Farm,
    /// Provides housing capacity for population.
    Habitat,
    /// Generates production points for construction.
    Factory,
    /// Generates science points for research.
    Laboratory,
    /// Extends the power grid without providing resources.
    Passage,
    /// Converts black tiles to white tiles.
    Terraformer,
}

impl BuildingType {
    /// Get the string ID used to look up this building in game data.
    ///
    /// These IDs correspond to entries in `surface_buildings.ron`.
    pub fn id(&self) -> &'static str {
        match self {
            BuildingType::Base => "building_base",
            BuildingType::Farm => "building_farm_1",
            BuildingType::Habitat => "building_habitat_1",
            BuildingType::Factory => "building_factory_1",
            BuildingType::Laboratory => "building_laboratory_1",
            BuildingType::Passage => "building_passage",
            BuildingType::Terraformer => "building_terraformer",
        }
    }
}

/// A single tile on the planet surface.
///
/// Tiles are the fundamental unit of the planet grid. Each tile has:
/// - A base color determining buildability
/// - An optional building
/// - A connectivity flag for the power grid
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SurfaceTile {
    /// Base tile color derived from the surface type distribution.
    pub color: TileColor,
    /// Optional placed building (None if tile is empty).
    pub building: Option<BuildingType>,
    /// Whether this tile is connected to the base (power/logistics).
    /// Only connected tiles can have new buildings placed on them.
    pub connected: bool,
}

/// The complete surface grid of a planet.
///
/// Stores tiles in a flat vector with row-major ordering.
/// Use [`get`] and [`get_mut`] methods for coordinate-based access.
///
/// # Example
///
/// ```ignore
/// let surface = PlanetSurface::new(10, 10); // 10x10 grid
/// let tile = surface.get(5, 3); // Get tile at column 5, row 3
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Resource)]
pub struct PlanetSurface {
    /// Flat vector of tiles in row-major order.
    /// Index = y * row_width + x
    pub tiles: Vec<SurfaceTile>,
    /// Number of tiles per row (width of the grid).
    pub row_width: usize,
}

impl PlanetSurface {
    /// Create a new empty surface grid.
    ///
    /// All tiles are initialized as black (unbuildable) with no buildings
    /// and disconnected from the power grid.
    ///
    /// # Arguments
    ///
    /// * `width` - Number of columns in the grid
    /// * `height` - Number of rows in the grid
    pub fn new(width: usize, height: usize) -> Self {
        let tiles = vec![
            SurfaceTile {
                color: TileColor::Black,
                building: None,
                connected: false,
            };
            width * height
        ];
        Self {
            tiles,
            row_width: width,
        }
    }

    /// Get an immutable reference to a tile at the given coordinates.
    ///
    /// Returns `None` if coordinates are out of bounds.
    pub fn get(&self, x: usize, y: usize) -> Option<&SurfaceTile> {
        if x >= self.row_width {
            return None;
        }
        let idx = y * self.row_width + x;
        self.tiles.get(idx)
    }

    /// Get a mutable reference to a tile at the given coordinates.
    ///
    /// Returns `None` if coordinates are out of bounds.
    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut SurfaceTile> {
        if x >= self.row_width {
            return None;
        }
        let idx = y * self.row_width + x;
        self.tiles.get_mut(idx)
    }

    /// Calculate the height (number of rows) of the grid.
    pub fn height(&self) -> usize {
        if self.row_width == 0 {
            0
        } else {
            self.tiles.len() / self.row_width
        }
    }
}
