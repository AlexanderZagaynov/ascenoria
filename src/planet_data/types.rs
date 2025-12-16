//! Data structures for planet generation and placement.

use bevy::prelude::*;

/// Tile color used for generated planet surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileColor {
    Black,
    White,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildingType {
    Base,
    Farm,
    Habitat,
    Factory,
    Laboratory,
    Passage,
    Terraformer,
}

/// Snapshot of a surface tile that may host a building.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SurfaceTile {
    /// Base tile color derived from the surface type distribution.
    pub color: TileColor,
    /// Optional placed building.
    pub building: Option<BuildingType>,
}

/// Surface grid along with placement helpers.
#[derive(Debug, Clone, PartialEq, Eq, Resource)]
pub struct PlanetSurface {
    pub tiles: Vec<SurfaceTile>,
    pub row_width: usize,
}

impl PlanetSurface {
    pub fn new(width: usize, height: usize) -> Self {
        let tiles = vec![
            SurfaceTile {
                color: TileColor::Black,
                building: None,
            };
            width * height
        ];
        Self {
            tiles,
            row_width: width,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&SurfaceTile> {
        if x >= self.row_width {
            return None;
        }
        let idx = y * self.row_width + x;
        self.tiles.get(idx)
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut SurfaceTile> {
        if x >= self.row_width {
            return None;
        }
        let idx = y * self.row_width + x;
        self.tiles.get_mut(idx)
    }

    pub fn height(&self) -> usize {
        if self.row_width == 0 {
            0
        } else {
            self.tiles.len() / self.row_width
        }
    }
}
