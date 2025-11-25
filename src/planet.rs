use rand::seq::SliceRandom;
use rand::{SeedableRng, rngs::StdRng};

use crate::data::{GameData, TileDistribution};

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

/// Generate a random planet from loaded game data using the provided seed.
pub fn generate_planet(seed: u64, data: &GameData) -> Option<GeneratedPlanet> {
    let mut rng = StdRng::seed_from_u64(seed);

    let size = data.planet_sizes().choose(&mut rng)?;
    let surface_type = data.planet_surface_types().choose(&mut rng)?;

    let surface_slots = size.surface_slots.max(1);
    let orbital_slots = size.orbital_slots;

    let mut tiles = build_tiles(surface_slots, &surface_type.tile_distribution);
    tiles.shuffle(&mut rng);

    let row_width = (surface_slots as f32).sqrt().ceil().max(1.0) as usize;

    Some(GeneratedPlanet {
        size_id: size.id.clone(),
        surface_slots,
        orbital_slots,
        orbital_items: vec![None; orbital_slots as usize],
        surface_type_id: surface_type.id.clone(),
        tiles,
        row_width,
    })
}

fn build_tiles(surface_slots: u32, distribution: &TileDistribution) -> Vec<TileColor> {
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

/// Error returned when placing orbital items.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum OrbitalPlacementError {
    /// All orbital slots are already filled.
    #[error("no free orbital slots available")]
    NoFreeSlots,
}

impl GeneratedPlanet {
    /// Place an orbital item into the first free slot.
    pub fn place_orbital(
        &mut self,
        item_id: impl Into<String>,
    ) -> Result<usize, OrbitalPlacementError> {
        if let Some((idx, slot)) = self
            .orbital_items
            .iter_mut()
            .enumerate()
            .find(|(_, slot)| slot.is_none())
        {
            *slot = Some(item_id.into());
            Ok(idx)
        } else {
            Err(OrbitalPlacementError::NoFreeSlots)
        }
    }
}

/// Render a generated planet as a debug string grid.
pub fn format_planet(planet: &GeneratedPlanet) -> String {
    let mut lines = Vec::new();
    lines.push(format!(
        "Planet size: {} (surface {}, orbital {})",
        planet.size_id, planet.surface_slots, planet.orbital_slots
    ));
    lines.push(format!("Surface type: {}", planet.surface_type_id));
    if planet.orbital_slots > 0 {
        for (idx, slot) in planet.orbital_items.iter().enumerate() {
            let value = slot.as_ref().map(|id| id.as_str()).unwrap_or("<empty>");
            lines.push(format!("Orbital slot {idx}: {value}"));
        }
    } else {
        lines.push("Orbital slots: none".to_string());
    }

    let mut grid = String::new();
    for (i, tile) in planet.tiles.iter().enumerate() {
        grid.push(tile.symbol());
        if (i + 1) % planet.row_width == 0 {
            grid.push('\n');
        }
    }
    if !grid.ends_with('\n') {
        grid.push('\n');
    }

    lines.push("Surface tiles:".to_string());
    lines.push(grid);
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::load_game_data;
    use std::path::PathBuf;

    #[test]
    fn builds_tiles_matching_slot_count() {
        let distribution = TileDistribution {
            black: 10,
            white: 20,
            red: 30,
            green: 20,
            blue: 20,
        };

        let tiles = build_tiles(50, &distribution);
        assert_eq!(tiles.len(), 50);
    }

    #[test]
    fn generates_planet_from_assets() {
        let (data, _) =
            load_game_data(PathBuf::from("assets/data")).expect("Game data should load");

        let planet = generate_planet(42, &data).expect("Planet should generate");
        assert_eq!(planet.surface_slots as usize, planet.tiles.len());
        assert!(!planet.surface_type_id.is_empty());
        assert!(!planet.size_id.is_empty());
        assert!(planet.row_width > 0);
        assert_eq!(planet.orbital_items.len(), planet.orbital_slots as usize);
    }

    #[test]
    fn enforces_orbital_capacity() {
        let mut planet = GeneratedPlanet {
            size_id: "s".into(),
            surface_slots: 4,
            orbital_slots: 2,
            orbital_items: vec![None, None],
            surface_type_id: "t".into(),
            tiles: vec![TileColor::Black; 4],
            row_width: 2,
        };

        assert_eq!(planet.place_orbital("alpha"), Ok(0));
        assert_eq!(planet.place_orbital("beta"), Ok(1));
        assert_eq!(
            planet.place_orbital("gamma"),
            Err(OrbitalPlacementError::NoFreeSlots)
        );
    }
}
