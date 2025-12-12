//! Type definitions for the planet view screen.
//!
//! Contains marker components, state resources, and enums for UI elements.

use bevy::prelude::*;

/// State for the planet view.
#[derive(Resource, Default)]
pub struct PlanetViewState {
    /// Index of the currently viewed planet within its star system.
    pub planet_index: usize,
}

/// Marker component for all planet view UI entities (2D overlay).
#[derive(Component)]
pub struct PlanetViewRoot;

/// Marker component for all 3D entities in the planet view.
#[derive(Component)]
pub struct PlanetView3D;

/// Marker for the planet grid mesh.
#[derive(Component)]
pub struct PlanetGrid;

/// Marker for planet thumbnail buttons in the top bar.
#[derive(Component)]
pub struct PlanetThumbnail(pub usize);

/// Marker for surface tiles (now on the 3D sphere).
#[derive(Component)]
pub struct SurfaceTileMarker {
    pub index: usize,
    pub color: crate::planet_data::TileColor,
}

/// Marker for tile cube on the planet surface.
#[derive(Component)]
pub struct TileCube {
    pub tile_index: usize,
}

/// Marker for the population display.
#[derive(Component)]
pub struct PopulationDisplay;

/// Marker for the project display.
#[derive(Component)]
pub struct ProjectDisplay;

/// Marker for the back button.
#[derive(Component)]
pub struct BackButton;

/// Panel button types.
#[derive(Component, Clone, Copy, Debug)]
pub enum PanelButton {
    Back,
}

/// Colors for the planet view UI - inspired by Ascendancy's planet screen.
pub mod colors {
    use bevy::prelude::*;

    /// Dark space background.
    pub const BACKGROUND: Color = Color::srgb(0.02, 0.02, 0.05);
    /// Panel background.
    pub const PANEL_BG: Color = Color::srgb(0.08, 0.10, 0.15);
    /// Border color - teal/cyan accent.
    pub const BORDER: Color = Color::srgb(0.2, 0.5, 0.6);
    /// Header text color.
    pub const HEADER_TEXT: Color = Color::srgb(0.7, 0.85, 0.9);
    /// Normal text color.
    pub const TEXT: Color = Color::srgb(0.6, 0.7, 0.75);
    /// Button normal.
    pub const BUTTON_NORMAL: Color = Color::srgb(0.08, 0.12, 0.20);
    /// Button hovered.
    pub const BUTTON_HOVERED: Color = Color::srgb(0.12, 0.18, 0.28);

    // Tile colors matching TileColor enum
    /// Black/hostile terrain - impassable.
    pub const TILE_BLACK: Color = Color::srgb(0.1, 0.1, 0.1);
    /// White/marginal terrain - buildable but plain.
    pub const TILE_WHITE: Color = Color::srgb(0.75, 0.75, 0.7);
    /// Red/mineral terrain - industry bonus.
    pub const TILE_RED: Color = Color::srgb(0.8, 0.3, 0.2);
    /// Green/fertile terrain - prosperity bonus.
    pub const TILE_GREEN: Color = Color::srgb(0.3, 0.7, 0.3);
    /// Blue/special terrain - research bonus.
    pub const TILE_BLUE: Color = Color::srgb(0.3, 0.5, 0.8);

    /// Tile border.
    pub const TILE_BORDER: Color = Color::srgb(0.3, 0.3, 0.35);
    /// Tile hover highlight.
    pub const TILE_HOVER: Color = Color::srgb(0.9, 0.8, 0.3);

    /// Selected planet thumbnail border.
    pub const THUMBNAIL_SELECTED: Color = Color::srgb(0.9, 0.7, 0.2);
    /// Unselected planet thumbnail border.
    pub const THUMBNAIL_NORMAL: Color = Color::srgb(0.3, 0.4, 0.5);
}
