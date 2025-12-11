//! Data structures for planet generation and placement.

use crate::data::TileDistribution;

/// Tile color used for generated planet surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileColor {
    Black,
    White,
    Red,
    Green,
    Blue,
}

impl TileColor {
    /// Short symbol for debug display.
    pub fn symbol(self) -> char {
        match self {
            TileColor::Black => 'B',
            TileColor::White => 'W',
            TileColor::Red => 'R',
            TileColor::Green => 'G',
            TileColor::Blue => 'L',
        }
    }
}

/// Generated planet snapshot for debug display and validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratedPlanet {
    /// Selected planet size identifier.
    pub size_id: String,
    /// Total surface slots.
    pub surface_slots: u32,
    /// Total orbital slots.
    pub orbital_slots: u32,
    /// Placed orbital items keyed by slot index.
    pub orbital_items: Vec<Option<String>>,
    /// Selected surface type identifier.
    pub surface_type_id: String,
    /// Tile colors laid out in a simple grid (row-major).
    pub tiles: Vec<TileColor>,
    /// Row width for rendering the grid as text.
    pub row_width: usize,
}

/// Error returned when placing orbital items.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum OrbitalPlacementError {
    /// All orbital slots are already filled.
    #[error("no free orbital slots available")]
    NoFreeSlots,
    /// Not enough slots for the requested orbital size.
    #[error("not enough free orbital slots: required {required}, available {available}")]
    InsufficientSlots { required: usize, available: usize },
}

/// Error returned when placing surface buildings.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum SurfacePlacementError {
    /// Planet has no surface representation (e.g., generation failed).
    #[error("no planet surface available")]
    NoSurface,
    /// Not enough empty tiles to fit the requested building.
    #[error("not enough free surface tiles: required {required}, available {available}")]
    InsufficientSlots { required: usize, available: usize },
}

/// Snapshot of a surface tile that may host a building.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SurfaceTile {
    /// Base tile color derived from the surface type distribution.
    pub color: TileColor,
    /// Optional placed building identifier.
    pub building_id: Option<String>,
}

/// Surface grid along with placement helpers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlanetSurface {
    pub(crate) tiles: Vec<SurfaceTile>,
    pub(crate) row_width: usize,
    pub(crate) capacity: usize,
    pub(crate) used_slots: usize,
}

/// Preview of a potential surface placement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SurfacePreview {
    /// Identifier of the building being placed.
    pub building_id: String,
    /// Tile indices that will be occupied by the building.
    pub tile_indices: Vec<usize>,
}

/// Preview of an orbital placement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrbitalPreview {
    /// Identifier of the orbital item.
    pub building_id: String,
    /// Slot indices that will be occupied (usually a single slot).
    pub slot_indices: Vec<usize>,
}

/// Orbital construction state for a planet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlanetOrbitals {
    pub(crate) slots: Vec<Option<String>>,
}

/// Build tile colors to fill a surface of `surface_slots` size.
pub(crate) fn build_tiles(surface_slots: u32, distribution: &TileDistribution) -> Vec<TileColor> {
    let desired = [
        (TileColor::Black, distribution.black),
        (TileColor::White, distribution.white),
        (TileColor::Red, distribution.red),
        (TileColor::Green, distribution.green),
        (TileColor::Blue, distribution.blue),
    ];

    let mut counts: Vec<(TileColor, u32, f64)> = desired
        .iter()
        .map(|(color, percent)| {
            let exact = surface_slots as f64 * (*percent as f64 / 100.0);
            let base = exact.floor() as u32;
            (*color, base, exact - base as f64)
        })
        .collect();

    let allocated: u32 = counts.iter().map(|(_, base, _)| *base).sum();
    let mut remaining = surface_slots.saturating_sub(allocated);

    while remaining > 0 {
        if let Some((_, base, _)) = counts
            .iter_mut()
            .max_by(|a, b| a.2.partial_cmp(&b.2).unwrap_or(std::cmp::Ordering::Equal))
        {
            *base += 1;
        }
        remaining -= 1;
    }

    let mut tiles = Vec::with_capacity(surface_slots as usize);
    for (color, count, _) in counts {
        tiles.extend(std::iter::repeat(color).take(count as usize));
    }
    tiles
}
