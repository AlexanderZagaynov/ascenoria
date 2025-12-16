//! Debug formatting utilities for planets.

use super::types::GeneratedPlanet;

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
