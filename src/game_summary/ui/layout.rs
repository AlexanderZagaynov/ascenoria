use bevy::ecs::hierarchy::ChildSpawnerCommands;
use bevy::prelude::*;
use super::super::types::colors;

use super::header::spawn_title_section;
use super::content::spawn_content_area;
use super::controls::spawn_button_area;

/// Spawns the main content panel.
pub fn spawn_main_panel(
    parent: &mut ChildSpawnerCommands,
    species_name: &str,
    species_description: &str,
    mission_briefing: &str,
) {
    parent
        .spawn((
            Node {
                width: Val::Px(1000.0),
                height: Val::Px(700.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(30.0)),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(colors::PANEL_BG),
            BorderColor::all(colors::PANEL_BORDER),
            BorderRadius::all(Val::Px(12.0)),
        ))
        .with_children(|panel| {
            // Title section
            spawn_title_section(panel, species_name);

            // Content area (portrait + text)
            spawn_content_area(panel, species_description, mission_briefing);

            // Bottom buttons
            spawn_button_area(panel);
        });
}
