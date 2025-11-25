use rand::seq::SliceRandom;
use rand::{SeedableRng, rngs::StdRng};

use crate::data::{GameData, PlanetaryItem, TileDistribution};

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
    tiles: Vec<SurfaceTile>,
    row_width: usize,
    capacity: usize,
    used_slots: usize,
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
    slots: Vec<Option<String>>,
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

impl From<&GeneratedPlanet> for PlanetSurface {
    fn from(planet: &GeneratedPlanet) -> Self {
        let tiles = planet
            .tiles
            .iter()
            .map(|color| SurfaceTile {
                color: *color,
                building_id: None,
            })
            .collect();

        PlanetSurface {
            tiles,
            row_width: planet.row_width.max(1),
            capacity: planet.surface_slots as usize,
            used_slots: 0,
        }
    }
}

impl From<&GeneratedPlanet> for PlanetOrbitals {
    fn from(planet: &GeneratedPlanet) -> Self {
        Self {
            slots: planet.orbital_items.clone(),
        }
    }
}

impl PlanetSurface {
    /// Number of used slots.
    pub fn used_slots(&self) -> usize {
        self.used_slots
    }

    /// Total capacity of the surface grid.
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Attempt to preview placement of a surface building.
    pub fn preview_placement(
        &self,
        item: &PlanetaryItem,
    ) -> Result<SurfacePreview, SurfacePlacementError> {
        let required = item.slot_size.max(0) as usize;
        if self.tiles.is_empty() {
            return Err(SurfacePlacementError::NoSurface);
        }

        let free_tiles: Vec<usize> = self
            .tiles
            .iter()
            .enumerate()
            .filter_map(|(idx, tile)| {
                if tile.building_id.is_none() {
                    Some(idx)
                } else {
                    None
                }
            })
            .collect();

        if free_tiles.len() < required {
            return Err(SurfacePlacementError::InsufficientSlots {
                required,
                available: free_tiles.len(),
            });
        }

        Ok(SurfacePreview {
            building_id: item.id.clone(),
            tile_indices: free_tiles.into_iter().take(required).collect(),
        })
    }

    /// Apply a previously computed placement preview.
    pub fn apply_preview(&mut self, preview: &SurfacePreview) {
        for idx in &preview.tile_indices {
            if let Some(tile) = self.tiles.get_mut(*idx) {
                tile.building_id = Some(preview.building_id.clone());
            }
        }
        self.used_slots += preview.tile_indices.len();
    }

    /// Render the surface grid using tile symbols and building markers.
    pub fn render(&self) -> String {
        self.render_with_preview(None)
    }

    /// Render the surface grid with an optional placement preview applied.
    pub fn render_with_preview(&self, preview: Option<&SurfacePreview>) -> String {
        let mut clone = self.clone();
        if let Some(preview) = preview {
            clone.apply_preview(preview);
        }

        let mut grid = String::new();
        for (i, tile) in clone.tiles.iter().enumerate() {
            if let Some(building) = &tile.building_id {
                let symbol = building.chars().next().unwrap_or('?').to_ascii_uppercase();
                grid.push(symbol);
            } else {
                grid.push(tile.color.symbol());
            }

            if (i + 1) % clone.row_width == 0 {
                grid.push('\n');
            }
        }

        if !grid.ends_with('\n') {
            grid.push('\n');
        }

        grid
    }
}

impl PlanetOrbitals {
    /// Total orbital capacity.
    pub fn capacity(&self) -> usize {
        self.slots.len()
    }

    /// Number of occupied orbital slots.
    pub fn used_slots(&self) -> usize {
        self.slots.iter().filter(|s| s.is_some()).count()
    }

    /// Preview placing an orbital item into the first available slots.
    pub fn preview_placement(
        &self,
        item: &PlanetaryItem,
    ) -> Result<OrbitalPreview, OrbitalPlacementError> {
        let required = item.slot_size.max(0) as usize;
        let free_slots: Vec<usize> = self
            .slots
            .iter()
            .enumerate()
            .filter_map(|(idx, slot)| if slot.is_none() { Some(idx) } else { None })
            .collect();

        if free_slots.len() < required.max(1) {
            return Err(OrbitalPlacementError::InsufficientSlots {
                required: required.max(1),
                available: free_slots.len(),
            });
        }

        let slots_to_use = free_slots.into_iter().take(required.max(1)).collect();
        Ok(OrbitalPreview {
            building_id: item.id.clone(),
            slot_indices: slots_to_use,
        })
    }

    /// Apply a placement preview to the orbital state.
    pub fn apply_preview(&mut self, preview: &OrbitalPreview) {
        for idx in &preview.slot_indices {
            if let Some(slot) = self.slots.get_mut(*idx) {
                *slot = Some(preview.building_id.clone());
            }
        }
    }

    /// Render orbital slots as a list for debug UI.
    pub fn render(&self) -> String {
        let mut lines = Vec::new();
        for (idx, slot) in self.slots.iter().enumerate() {
            let value = slot.as_deref().unwrap_or("<empty>");
            lines.push(format!("Orbital slot {idx}: {value}"));
        }
        lines.join("\n")
    }

    /// Render orbital slots with an optional preview applied.
    pub fn render_with_preview(&self, preview: Option<&OrbitalPreview>) -> String {
        let mut clone = self.clone();
        if let Some(preview) = preview {
            clone.apply_preview(preview);
        }
        clone.render()
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
    use crate::data::{LocalizedText, PlanetaryItem, load_game_data};
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

    #[test]
    fn previews_and_applies_surface_buildings() {
        let generated = GeneratedPlanet {
            size_id: "tiny".into(),
            surface_slots: 3,
            orbital_slots: 0,
            orbital_items: Vec::new(),
            surface_type_id: "eden".into(),
            tiles: vec![TileColor::Red, TileColor::Green, TileColor::Blue],
            row_width: 2,
        };

        let mut surface = PlanetSurface::from(&generated);
        let factory = PlanetaryItem {
            id: "factory".into(),
            name: LocalizedText {
                en: "Factory".into(),
                ru: "Фабрика".into(),
            },
            description: LocalizedText {
                en: "Produces".into(),
                ru: "Производит".into(),
            },
            industry_bonus: 1,
            research_bonus: 0,
            prosperity_bonus: 0,
            max_population_bonus: 0,
            slot_size: 2,
            industry_cost: 10,
            tech_index: 0,
        };

        let preview = surface
            .preview_placement(&factory)
            .expect("Should preview placement");
        assert_eq!(preview.tile_indices.len(), 2);

        surface.apply_preview(&preview);
        assert_eq!(surface.used_slots(), 2);

        let error = surface
            .preview_placement(&factory)
            .expect_err("Second placement should exceed capacity");
        assert_eq!(
            error,
            SurfacePlacementError::InsufficientSlots {
                required: 2,
                available: 1
            }
        );
    }

    #[test]
    fn previews_orbital_placement_with_capacity_limits() {
        let generated = GeneratedPlanet {
            size_id: "orbital".into(),
            surface_slots: 1,
            orbital_slots: 1,
            orbital_items: vec![None],
            surface_type_id: "t".into(),
            tiles: vec![TileColor::Black],
            row_width: 1,
        };

        let mut orbitals = PlanetOrbitals::from(&generated);
        let satellite = PlanetaryItem {
            id: "satellite".into(),
            name: LocalizedText {
                en: "Satellite".into(),
                ru: "Спутник".into(),
            },
            description: LocalizedText {
                en: "Watcher".into(),
                ru: "Наблюдатель".into(),
            },
            industry_bonus: 0,
            research_bonus: 0,
            prosperity_bonus: 0,
            max_population_bonus: 0,
            slot_size: 1,
            industry_cost: 5,
            tech_index: 0,
        };

        let preview = orbitals
            .preview_placement(&satellite)
            .expect("Should preview orbital placement");
        assert_eq!(preview.slot_indices, vec![0]);
        orbitals.apply_preview(&preview);
        assert_eq!(orbitals.used_slots(), 1);

        let error = orbitals
            .preview_placement(&satellite)
            .expect_err("No additional orbital capacity");
        assert_eq!(
            error,
            OrbitalPlacementError::InsufficientSlots {
                required: 1,
                available: 0
            }
        );
    }
}
