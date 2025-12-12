//! 3D rendering helpers for the planet view.
//!
//! Contains mesh creation and material setup for planet surface visualization.

mod buildings;
mod materials;
mod mesh;

pub use buildings::spawn_surface_buildings;
pub use materials::{create_planet_material, get_planet_base_color, get_planet_thumbnail_color};
pub use mesh::{create_planet_grid_mesh, tile_color_to_linear};
