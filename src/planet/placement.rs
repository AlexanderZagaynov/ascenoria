//! Placement logic for generated planets.

use crate::data::PlanetaryItem;

use super::types::{
    GeneratedPlanet, OrbitalPlacementError, OrbitalPreview, PlanetOrbitals, PlanetSurface,
    SurfacePlacementError, SurfacePreview, SurfaceTile,
};

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
