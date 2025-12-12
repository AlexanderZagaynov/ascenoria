use crate::data::PlanetaryItem;
use crate::planet::types::{
    GeneratedPlanet, PlanetSurface, SurfacePlacementError, SurfacePreview, SurfaceTile,
};

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
