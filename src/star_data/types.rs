//! Type definitions for the star system view.
//!
//! Contains marker components, state resources, and UI-related types.

use bevy::prelude::*;

/// Marker component for all star system view entities.
#[derive(Component)]
pub struct StarRoot;

/// Marker for planet entities in the system view.
#[derive(Component)]
pub struct PlanetMarker {
    pub planet_index: usize,
}

/// Marker for planet selection ring.
#[derive(Component)]
pub struct SelectionRing;

/// Marker for the planet stalk/pole.
#[derive(Component)]
pub struct PlanetStalk {
    pub planet_index: usize,
}

/// Marker for the grid plane.
#[derive(Component)]
pub struct GridPlane;

/// State for the star system view.
#[derive(Resource, Default)]
pub struct StarState {
    /// Currently viewing star index.
    pub star_index: usize,
    /// Currently selected planet (if any).
    pub selected_planet: Option<usize>,
    /// Camera rotation angle (for view rotation).
    pub camera_angle: f32,
    /// Camera zoom level.
    pub zoom: f32,
}

/// Side panel button types for system view.
#[derive(Component, Clone, Copy, Debug)]
pub enum SystemPanelButton {
    /// Navigate to planet (zoom in).
    GotoPlanet,
    /// Send fleet to system.
    SendFleet,
    /// Build ship order.
    BuildShip,
    /// View system info.
    SystemInfo,
    /// Return to galaxy map.
    ReturnToGalaxy,
}

/// Colors for the star system UI.
pub mod colors {
    use bevy::prelude::*;

    /// Black space background.
    pub const SPACE_BLACK: Color = Color::srgb(0.0, 0.0, 0.05);
    /// Grid line color (cyan/teal).
    pub const GRID_LINE: Color = Color::srgb(0.1, 0.4, 0.5);
    /// Grid line highlight.
    pub const GRID_HIGHLIGHT: Color = Color::srgb(0.15, 0.5, 0.6);
    /// Planet stalk/pole color (yellow-green).
    pub const STALK_COLOR: Color = Color::srgb(0.7, 0.8, 0.2);
    /// Selection ring color (bright green).
    pub const SELECTION_GREEN: Color = Color::srgb(0.2, 0.9, 0.3);
    /// Panel background (textured gray-green).
    pub const PANEL_BG: Color = Color::srgb(0.35, 0.42, 0.40);
    /// Panel dark sections.
    pub const PANEL_DARK: Color = Color::srgb(0.22, 0.28, 0.26);
    /// Panel border.
    pub const PANEL_BORDER: Color = Color::srgb(0.18, 0.22, 0.20);
    /// Text on panels.
    pub const PANEL_TEXT: Color = Color::srgb(0.85, 0.90, 0.85);
    /// Dim text.
    pub const PANEL_TEXT_DIM: Color = Color::srgb(0.55, 0.60, 0.55);
    /// Star label text (cyan).
    pub const STAR_LABEL: Color = Color::srgb(0.4, 0.9, 0.8);
    /// Planet name text.
    pub const PLANET_LABEL: Color = Color::srgb(0.3, 0.85, 0.7);
    /// Button icon backgrounds.
    pub const BUTTON_ICON_BG: Color = Color::srgb(0.25, 0.35, 0.45);
}

/// Planet type visual appearance.
#[derive(Clone, Copy, Debug)]
pub enum PlanetVisual {
    Rocky,    // Gray/brown
    Volcanic, // Red/orange with glow
    Oceanic,  // Blue with white clouds
    Desert,   // Tan/yellow
    Lush,     // Green with blue
    Ice,      // White/light blue
    Gas,      // Striped bands
}

impl PlanetVisual {
    pub fn primary_color(&self) -> Color {
        match self {
            PlanetVisual::Rocky => Color::srgb(0.55, 0.45, 0.40),
            PlanetVisual::Volcanic => Color::srgb(0.75, 0.35, 0.20),
            PlanetVisual::Oceanic => Color::srgb(0.25, 0.45, 0.70),
            PlanetVisual::Desert => Color::srgb(0.80, 0.65, 0.40),
            PlanetVisual::Lush => Color::srgb(0.30, 0.60, 0.35),
            PlanetVisual::Ice => Color::srgb(0.85, 0.90, 0.95),
            PlanetVisual::Gas => Color::srgb(0.70, 0.55, 0.45),
        }
    }

    pub fn from_surface_type(surface_id: &str) -> Self {
        match surface_id {
            "eden" | "congenial" | "primordial" => PlanetVisual::Lush,
            "mineral" | "supermineral" => PlanetVisual::Rocky,
            "tycoon" => PlanetVisual::Desert,
            "husk" => PlanetVisual::Volcanic,
            "gas_giant" => PlanetVisual::Gas,
            _ => PlanetVisual::Rocky,
        }
    }
}

/// Generated position data for a planet in the system view.
pub struct PlanetPosition {
    /// Position on the grid (X, Z in 3D terms, mapped to 2D isometric).
    pub grid_pos: Vec2,
    /// Height above the grid (Y in 3D).
    pub height: f32,
    /// Visual size based on planet size.
    pub size: f32,
    /// Planet visual type.
    pub visual: PlanetVisual,
}

/// Convert number to Roman numerals for planet names.
pub fn to_roman(n: usize) -> &'static str {
    match n {
        1 => "I",
        2 => "II",
        3 => "III",
        4 => "IV",
        5 => "V",
        6 => "VI",
        7 => "VII",
        8 => "VIII",
        9 => "IX",
        10 => "X",
        _ => "?",
    }
}
