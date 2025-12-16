//! Color constants for the game options screen.

use bevy::prelude::*;

/// Dark blue-black background.
pub const BACKGROUND: Color = Color::srgb(0.02, 0.02, 0.06);
/// Panel background (dark navy).
pub const PANEL_BG: Color = Color::srgb(0.05, 0.08, 0.15);
/// Panel border (teal).
pub const PANEL_BORDER: Color = Color::srgb(0.2, 0.5, 0.6);
/// Button normal.
pub const BUTTON_NORMAL: Color = Color::srgb(0.08, 0.12, 0.20);
/// Button hovered.
pub const BUTTON_HOVERED: Color = Color::srgb(0.12, 0.18, 0.28);
/// Button pressed.
pub const BUTTON_PRESSED: Color = Color::srgb(0.16, 0.24, 0.36);
/// Selected item highlight.
pub const SELECTED: Color = Color::srgb(0.15, 0.35, 0.45);
/// Text color - cyan.
pub const TEXT: Color = Color::srgb(0.7, 0.85, 0.9);
/// Title text - green.
pub const TITLE: Color = Color::srgb(0.3, 0.9, 0.5);
/// Description text.
pub const DESCRIPTION: Color = Color::srgb(0.6, 0.75, 0.8);
/// Galaxy info text.
pub const INFO: Color = Color::srgb(0.8, 0.8, 0.6);
/// Player colors.
pub const PLAYER_COLORS: [Color; 8] = [
    Color::srgb(0.2, 0.8, 0.3), // Green
    Color::srgb(0.8, 0.3, 0.2), // Red
    Color::srgb(0.2, 0.5, 0.9), // Blue
    Color::srgb(0.9, 0.8, 0.2), // Yellow
    Color::srgb(0.7, 0.3, 0.8), // Purple
    Color::srgb(0.9, 0.5, 0.2), // Orange
    Color::srgb(0.2, 0.8, 0.8), // Cyan
    Color::srgb(0.8, 0.4, 0.6), // Pink
];
