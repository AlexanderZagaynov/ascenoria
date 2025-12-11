//! Type definitions for the game summary screen.

use bevy::prelude::*;

/// Color constants for the game summary screen.
pub mod colors {
    use bevy::prelude::*;

    /// Deep space background.
    pub const BACKGROUND: Color = Color::srgb(0.02, 0.02, 0.05);
    /// Panel background with transparency.
    pub const PANEL_BG: Color = Color::srgba(0.1, 0.1, 0.15, 0.85);
    /// Panel border color.
    pub const PANEL_BORDER: Color = Color::srgb(0.3, 0.4, 0.5);
    /// Title color (golden).
    pub const TITLE: Color = Color::srgb(0.9, 0.8, 0.4);
    /// Subtitle color.
    pub const SUBTITLE: Color = Color::srgb(0.7, 0.7, 0.8);
    /// Main text color.
    pub const TEXT: Color = Color::srgb(0.8, 0.8, 0.85);
    /// Muted text for hints.
    pub const HINT_TEXT: Color = Color::srgb(0.5, 0.5, 0.6);
    /// Portrait placeholder background.
    pub const PORTRAIT_BG: Color = Color::srgb(0.15, 0.15, 0.2);
    /// Portrait border.
    pub const PORTRAIT_BORDER: Color = Color::srgb(0.4, 0.5, 0.6);
    /// Button normal.
    pub const BUTTON_NORMAL: Color = Color::srgb(0.2, 0.25, 0.3);
    /// Button hover.
    pub const BUTTON_HOVER: Color = Color::srgb(0.3, 0.35, 0.4);
    /// Button pressed.
    pub const BUTTON_PRESSED: Color = Color::srgb(0.15, 0.2, 0.25);
}

/// Marker component for all game summary UI entities.
#[derive(Component)]
pub struct GameSummaryRoot;

/// Marker for the scrollable viewport.
#[derive(Component)]
pub struct SummaryScrollViewport;

/// Marker for the scrollable content.
#[derive(Component)]
pub struct SummaryScrollContent;

/// Marker for the continue button.
#[derive(Component)]
pub struct ContinueButton;

/// Marker for the back button.
#[derive(Component)]
pub struct BackButton;

/// Marker for star background entities.
#[derive(Component)]
pub struct StarBackground;
