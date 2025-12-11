//! Type definitions for the galaxy map screen.
//!
//! Contains marker components, state resources, button enums, and other
//! type definitions used by the galaxy map systems.

use bevy::prelude::*;

/// Marker component for all galaxy map UI entities.
#[derive(Component)]
pub struct GalaxyMapRoot;

/// Marker for the 3D galaxy view entities.
#[derive(Component)]
pub struct GalaxyView3D;

/// Marker for star system entities on the map.
#[derive(Component)]
pub struct StarMarker {
    pub system_index: usize,
}

/// Marker for star lane (connection line) entities.
#[derive(Component)]
pub struct StarLane;

/// Marker for the selection indicator around selected star.
#[derive(Component)]
pub struct SelectionIndicator;

/// Marker for the currently selected star.
#[derive(Component)]
pub struct SelectedStar;

/// State for the galaxy map view.
#[derive(Resource)]
pub struct GalaxyMapState {
    pub selected_system: Option<usize>,
    pub camera_offset: Vec2,
    pub turn_number: u32,
    /// Rotation angle around Y axis (horizontal rotation).
    pub rotation_y: f32,
    /// Rotation angle around X axis (vertical tilt).
    pub rotation_x: f32,
    /// Is the user currently dragging to rotate (right/middle click)?
    pub is_dragging: bool,
    /// Is left mouse button held down?
    pub left_mouse_down: bool,
    /// Has left-click drag moved enough to be considered a drag (not a click)?
    pub left_is_dragging: bool,
    /// Last mouse position when dragging started.
    pub last_mouse_pos: Vec2,
    /// Position where left-click started (for drag threshold check).
    pub left_click_start_pos: Vec2,
    /// Camera zoom level (1.0 = default).
    pub zoom: f32,
}

impl Default for GalaxyMapState {
    fn default() -> Self {
        Self {
            selected_system: None,
            camera_offset: Vec2::ZERO,
            turn_number: 0,
            rotation_y: 0.0,
            rotation_x: 0.3, // Slight tilt to see depth
            is_dragging: false,
            left_mouse_down: false,
            left_is_dragging: false,
            last_mouse_pos: Vec2::ZERO,
            left_click_start_pos: Vec2::ZERO,
            zoom: 1.0,
        }
    }
}

/// Side panel button types.
#[derive(Component, Clone, Copy, Debug)]
pub enum PanelButton {
    Planets,
    Ships,
    Research,
    SpecialAbility,
    Species,
    EndTurn,
    NextTurn,
}

/// Star types for visual variety.
#[derive(Clone, Copy, Debug)]
pub enum StarType {
    Yellow,
    Orange,
    Red,
    Blue,
    White,
}

impl StarType {
    pub fn color(&self) -> Color {
        match self {
            StarType::Yellow => Color::srgb(1.0, 0.95, 0.6),
            StarType::Orange => Color::srgb(1.0, 0.6, 0.3),
            StarType::Red => super::colors::STAR_RED,
            StarType::Blue => super::colors::STAR_BLUE,
            StarType::White => Color::srgb(0.95, 0.95, 1.0),
        }
    }

    pub fn from_seed(seed: u64) -> Self {
        match seed % 5 {
            0 => StarType::Yellow,
            1 => StarType::Orange,
            2 => StarType::Red,
            3 => StarType::Blue,
            _ => StarType::White,
        }
    }
}
