use bevy::prelude::*;

/// Marker component for all main menu UI entities.
#[derive(Component)]
pub struct MainMenuRoot;

/// Marker for menu buttons with their action type.
#[derive(Component, Clone, Copy)]
pub enum MenuButton {
    NewGame,
    LoadGame,
    SaveGame,
    Settings,
    Exit,
}
