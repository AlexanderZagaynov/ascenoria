//! Planet surface data structures and procedural generation.
//!
//! This module handles the representation and creation of planet surfaces,
//! which are grid-based maps where players construct buildings.
//!
//! # Module Structure
//! - [`types`] - Core data structures (`PlanetSurface`, `SurfaceTile`, `BuildingType`)
//! - [`generation`] - Procedural generation algorithms
//!
//! # Surface Model
//! Each planet has a rectangular grid of tiles. Tiles can be:
//! - **White**: Buildable terrain
//! - **Black**: Unbuildable terrain (obstacles)
//!
//! Tiles become "connected" (powered) when reachable from the Base building
//! through adjacent white tiles. Only connected tiles can have buildings
//! constructed on them.

mod generation;
mod types;

pub use generation::generate_planet;
pub use types::*;
